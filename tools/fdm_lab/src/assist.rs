//! Gains of the flight assists (plan `docs/plans/m2-aides-pilotage.md`, step 1; skill
//! `realisme`): the game's assists move the commands as a pilot would, and their gains
//! come from the flight model, never from hand tuning.
//!
//! At each speed of [`SPEEDS_KMH`], at sea level, the model is trimmed and linearised by
//! secants ([`crate::modes::linearise_secant`]: the piecewise-linear tables read over a
//! degree, not one segment). Three loops, each with an integrator:
//! - **bank**, by the aileron, on (p, φ): [`ROLL_RESPONSE`], with the airframe's own roll
//!   damping where it is more than the response asks (no rate gain; the third pole is
//!   placed where that damping puts it);
//! - **incidence**, by the elevator, on (α, q) with the rotor and the downwash settled:
//!   designs at [`DAMPING`] from the lowest of [`PITCH_FREQUENCIES`], the gentlest kept
//!   whose rate gain still damps and whose closed loop on the whole longitudinal model,
//!   downwash lag included, is damped to [`PITCH_DAMPING_SOUGHT`] (aerodynamicist review
//!   of 2026-10-29);
//! - **sideslip**, by the rudder, on (β, r): the airframe's natural frequency, at least
//!   [`MIN_YAW_FREQUENCY`], damped to [`DAMPING`].
//!
//! The integrators sit at [`INTEGRAL_FRACTION`] of their loop's frequency. The statics
//! are nonlinear: the elevator holding an incidence at the row's speed, and the load
//! factor it gives, by secants from the level flight's incidence to the stall guard's
//! limit and over a pitch rate; the rudder meeting the adverse yaw and the aileron
//! holding a roll rate come from the secant model. The stall sweep ([`crate::performance`]) gives the stall incidence.

use escadrille_data::assist::STALL_MARGIN;
use escadrille_fdm::aero::surface::ControlDeflections;
use escadrille_fdm::aircraft::Aircraft;
use escadrille_fdm::constants::{G0, RHO0};
use escadrille_fdm::environment::Environment;
use nalgebra::{DMatrix, DVector, Matrix3, Vector3};

use crate::modes::{LATERAL, LONGITUDINAL, Linear, STATES, derivative, linearise_secant};
use crate::performance::stall_speed_at;
use crate::trim::{Flight, LEVEL_AT_SPEED, PATH_AT_SPEED, Trim, solve_robust};

/// Equivalent airspeeds of the rows, km/h: from just above the stall (85 km/h at idle) to
/// a steep dive.
pub const SPEEDS_KMH: [f64; 10] = [
    90.0, 100.0, 110.0, 120.0, 140.0, 160.0, 180.0, 200.0, 230.0, 260.0,
];
/// Bank loop: natural frequency, rad/s, and damping. About a second to settle: a
/// beginner's turn, not an aerobatic roll; the aileron saturates before on a large bank.
pub const ROLL_RESPONSE: (f64, f64) = (3.0, 0.8);
/// Damping of the two-state designs.
pub const DAMPING: f64 = 0.8;
/// Design frequencies of the incidence loop tried, rad/s: from, to, step. From 4 rad/s:
/// the incidence answers in about half a second, a little quicker than the airframe's
/// own slow incidence root (1 to 2.6 s⁻¹ on the SPAD).
pub const PITCH_FREQUENCIES: (f64, f64, f64) = (4.0, 12.0, 0.25);
/// Damping sought for the incidence loop's closed loop on the whole longitudinal model.
pub const PITCH_DAMPING_SOUGHT: f64 = 0.7;
/// Lowest natural frequency of the sideslip loop, rad/s.
pub const MIN_YAW_FREQUENCY: f64 = 3.0;
/// Pole of the integrators, as a fraction of their loop's frequency.
pub const INTEGRAL_FRACTION: f64 = 0.25;
/// Closed-loop poles slower than this, rad/s, are the phugoid and the spiral, which the
/// loops leave to the pilot (and the instructor's outer laws): not counted in the damping.
pub const SLOW_MODES: f64 = 0.8;
/// Half-step of the control columns' secants, unit of command.
const CONTROL_SECANT: f64 = 0.02;

/// Indices of the states of [`crate::modes`].
const U: usize = 0;
const V: usize = 1;
const W: usize = 2;
const P: usize = 3;
const Q: usize = 4;
const R: usize = 5;
const PHI: usize = 6;
const THETA: usize = 7;
const OMEGA: usize = 8;
const EPSILON: usize = 9;

/// The assists' figures at one speed (`escadrille_data::assist::AssistRow`).
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    /// Equivalent airspeed (true airspeed at sea level), m/s.
    pub speed: f64,
    /// How the row was trimmed and its incidence loop chosen, for the file.
    pub flight: String,
    /// Load factor per radian of incidence, per pascal.
    pub load_per_alpha: f64,
    pub zero_load_alpha: f64,
    pub elevator_at_zero_load: f64,
    pub elevator_per_alpha: f64,
    pub elevator_per_pitch_rate: f64,
    pub rudder_per_roll_rate: f64,
    pub rudder_per_aileron: f64,
    pub aileron_per_roll_rate: f64,
    /// Gains × dynamic pressure: bank, roll rate, integral.
    pub roll: [f64; 3],
    /// Incidence, pitch rate, integral.
    pub pitch: [f64; 3],
    /// Sideslip, yaw rate, integral.
    pub yaw: [f64; 3],
}

impl Row {
    /// The gains at a dynamic pressure, as the game divides them.
    pub fn at(&self, dynamic_pressure: f64) -> ([f64; 3], [f64; 3], [f64; 3]) {
        let k = |g: [f64; 3]| g.map(|v| v / dynamic_pressure);
        (k(self.roll), k(self.pitch), k(self.yaw))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AssistGains {
    pub stall_alpha: f64,
    /// Speeds and throttles of the stall sweeps, for the file.
    pub stall_note: String,
    pub rows: Vec<Row>,
}

/// Coefficients of det(sI − M), from the constant term up, the leading 1 included
/// (Faddeev–LeVerrier).
fn characteristic(m: &DMatrix<f64>) -> Vec<f64> {
    let n = m.nrows();
    let mut c = vec![0.0; n + 1];
    c[n] = 1.0;
    let mut mk = DMatrix::zeros(n, n);
    for k in 1..=n {
        mk = m * &mk + DMatrix::identity(n, n) * c[n - k + 1];
        c[n - k] = -(m * &mk).trace() / k as f64;
    }
    c
}

/// The feedback u = −K x that gives ẋ = A x + B u the characteristic polynomial
/// `desired` (coefficients from the constant term, leading 1 included). With one input,
/// det(sI − A + B K) is affine in K: its slopes are read one gain at a time.
fn place(a: &DMatrix<f64>, b: &DVector<f64>, desired: &[f64]) -> Option<DVector<f64>> {
    let n = a.nrows();
    let open = characteristic(a);
    let mut slopes = DMatrix::zeros(n, n);
    for column in 0..n {
        let mut e = DVector::zeros(n);
        e[column] = 1.0;
        let closed = characteristic(&(a - b * e.transpose()));
        for row in 0..n {
            slopes[(row, column)] = closed[row] - open[row];
        }
    }
    let rhs = DVector::from_fn(n, |i, _| desired[i] - open[i]);
    slopes.lu().solve(&rhs)
}

/// (s² + 2ζω s + ω²)(s + c), from the constant term up.
fn with_real_pole(omega: f64, zeta: f64, c: f64) -> Vec<f64> {
    let pair = [omega * omega, 2.0 * zeta * omega, 1.0];
    vec![
        c * pair[0],
        pair[0] + c * pair[1],
        pair[1] + c * pair[2],
        pair[2],
    ]
}

/// (s² + 2ζω s + ω²)(s + f ω): a pair and its integrator.
fn response(omega: f64, zeta: f64) -> Vec<f64> {
    with_real_pole(omega, zeta, INTEGRAL_FRACTION * omega)
}

/// A 2-state loop (y, rate) with the integral of y appended: ż = y.
fn augmented(a: &DMatrix<f64>, b: &DVector<f64>) -> (DMatrix<f64>, DVector<f64>) {
    let mut aa = DMatrix::zeros(3, 3);
    aa.view_mut((0, 0), (2, 2)).copy_from(a);
    aa[(2, 0)] = 1.0;
    let bb = DVector::from_vec(vec![b[0], b[1], 0.0]);
    (aa, bb)
}

/// The natural frequency of a 2-state model, or `min` if it is not oscillating or
/// slower.
fn natural_frequency(a: &DMatrix<f64>, min: f64) -> f64 {
    a.determinant().max(0.0).sqrt().max(min)
}

/// State derivative per unit of each command (aileron, elevator, rudder) at the trim,
/// by secants of [`CONTROL_SECANT`].
fn control_columns(
    aircraft: &Aircraft,
    env: &dyn Environment,
    trim: &Trim,
    linear: &Linear,
) -> [DVector<f64>; 3] {
    let f = trim.flight;
    let travel = aircraft.travel();
    let h = CONTROL_SECANT;
    let column = |set: &dyn Fn(&mut ControlDeflections, f64)| {
        let (mut plus, mut minus) = (f.deflections, f.deflections);
        set(&mut plus, h);
        set(&mut minus, -h);
        let fp = derivative(aircraft, env, &plus, f.throttle, f.altitude, &linear.x0);
        let fm = derivative(aircraft, env, &minus, f.throttle, f.altitude, &linear.x0);
        DVector::from_fn(STATES, |i, _| (fp[i] - fm[i]) / (2.0 * h))
    };
    [
        column(&|d, h| d.aileron += h * travel.aileron),
        column(&|d, h| d.elevator += h * travel.elevator),
        column(&|d, h| d.rudder += h * travel.rudder),
    ]
}

/// The slow states' model with the fast ones in quasi-static equilibrium (Schur
/// complement, as [`crate::modes::reduced`]), its control column with them.
fn residualised(
    linear: &Linear,
    b: &DVector<f64>,
    slow: &[usize],
    fast: &[usize],
) -> Option<(DMatrix<f64>, DVector<f64>)> {
    let block = |rows: &[usize], cols: &[usize]| {
        DMatrix::from_fn(rows.len(), cols.len(), |i, j| linear.a[(rows[i], cols[j])])
    };
    let column = |rows: &[usize]| DVector::from_fn(rows.len(), |i, _| b[rows[i]]);
    let inverse = block(fast, fast).try_inverse()?;
    let coupling = block(slow, fast) * inverse;
    Some((
        block(slow, slow) - &coupling * block(fast, slow),
        column(slow) - coupling * column(fast),
    ))
}

/// Level flight at a speed and altitude, or, beyond the maximum level speed, a dive at
/// full throttle; and how, for the file.
pub fn trim_at(
    aircraft: &Aircraft,
    env: &dyn Environment,
    altitude: f64,
    speed: f64,
) -> Result<(Trim, String), String> {
    if let Ok(t) = solve_robust(
        aircraft,
        env,
        Flight::guess(altitude, speed, 0.5),
        LEVEL_AT_SPEED,
    ) && t.feasible(aircraft)
    {
        let note = format!("level flight, throttle {:.2}", t.flight.throttle);
        return Ok((t, note));
    }
    let mut guess = Flight::guess(altitude, speed, 1.0);
    guess.flight_path = -0.1;
    let t = solve_robust(aircraft, env, guess, PATH_AT_SPEED)
        .map_err(|e| format!("no steady flight at {:.0} km/h: {e:?}", speed * 3.6))?;
    if !t.feasible(aircraft) {
        return Err(format!("the pilot cannot hold {:.0} km/h", speed * 3.6));
    }
    let note = format!(
        "dive of {:.1}° at full throttle",
        -t.flight.flight_path.to_degrees()
    );
    Ok((t, note))
}

/// Linear incidence (w / V) of the states of [`crate::modes`], about the trim.
fn incidence_row(linear: &Linear) -> [f64; STATES] {
    let (u0, w0) = (linear.x0[U], linear.x0[W]);
    let v2 = u0 * u0 + w0 * w0;
    let mut c = [0.0; STATES];
    c[U] = -w0 / v2;
    c[W] = u0 / v2;
    c
}

/// The longitudinal model ([`LONGITUDINAL`], rotor and downwash lag included) with the
/// incidence loop closed, u = −(k_α α + k_q q + k_i ∫α), the integral appended.
pub fn pitch_closed(linear: &Linear, elevator: &DVector<f64>, k: [f64; 3]) -> DMatrix<f64> {
    let s = LONGITUDINAL;
    let n = s.len();
    let c = incidence_row(linear);
    let mut m = DMatrix::zeros(n + 1, n + 1);
    for (i, &si) in s.iter().enumerate() {
        for (j, &sj) in s.iter().enumerate() {
            let feedback = k[0] * c[sj] + if sj == Q { k[1] } else { 0.0 };
            m[(i, j)] = linear.a[(si, sj)] - elevator[si] * feedback;
        }
        m[(i, n)] = -elevator[si] * k[2];
    }
    for (j, &sj) in s.iter().enumerate() {
        m[(n, j)] = c[sj];
    }
    m
}

/// The lateral model ([`LATERAL`]) with the coordination closed and, for the
/// instructor, the bank loop, both integrals appended (bank, sideslip), as the game's
/// laws linearised about wings level.
pub fn lateral_closed(
    linear: &Linear,
    [aileron, rudder]: [&DVector<f64>; 2],
    row: &Row,
    gains: ([f64; 3], [f64; 3]),
    bank_loop: bool,
) -> DMatrix<f64> {
    let s = LATERAL;
    let n = s.len();
    let (roll, yaw) = gains;
    let (u0, w0) = (linear.x0[U], linear.x0[W]);
    let speed = (u0 * u0 + w0 * w0).sqrt();
    let theta = linear.x0[THETA];
    // Commands as rows over (states of `s`, bank integral, sideslip integral).
    let mut a_cmd = DVector::zeros(n + 2);
    if bank_loop {
        a_cmd[s.iter().position(|&x| x == PHI).unwrap()] = -roll[0];
        a_cmd[s.iter().position(|&x| x == P).unwrap()] = -roll[1];
        a_cmd[n] = -roll[2];
    }
    let mut r_cmd = &a_cmd * row.rudder_per_aileron;
    for (j, &sj) in s.iter().enumerate() {
        r_cmd[j] += match sj {
            V => -yaw[0] / speed,
            R => -yaw[1],
            // The yaw rate of the turn and of a roll about the flight path, left alone.
            PHI => yaw[1] * G0 * theta.cos() / speed,
            P => row.rudder_per_roll_rate + yaw[1] * (w0 / u0),
            _ => 0.0,
        };
    }
    r_cmd[n + 1] = -yaw[2];
    let mut m = DMatrix::zeros(n + 2, n + 2);
    for (i, &si) in s.iter().enumerate() {
        for (j, &sj) in s.iter().enumerate() {
            m[(i, j)] = linear.a[(si, sj)];
        }
        for j in 0..n + 2 {
            m[(i, j)] += aileron[si] * a_cmd[j] + rudder[si] * r_cmd[j];
        }
    }
    m[(n, s.iter().position(|&x| x == PHI).unwrap())] = if bank_loop { 1.0 } else { 0.0 };
    m[(n + 1, s.iter().position(|&x| x == V).unwrap())] = 1.0 / speed;
    m
}

/// The least damping ratio of a closed loop's modes faster than [`SLOW_MODES`]: a real
/// stable mode counts as 1, an unstable one as −1.
pub fn least_damping(m: &DMatrix<f64>) -> f64 {
    m.complex_eigenvalues()
        .iter()
        .filter(|l| l.norm() > SLOW_MODES)
        .map(|l| {
            if l.re >= 0.0 {
                -1.0
            } else if l.im.abs() > 1e-9 {
                -l.re / l.norm()
            } else {
                1.0
            }
        })
        .fold(1.0, f64::min)
}

/// Elevator command, and load factor, holding an incidence and a pitch rate at the
/// trim's speed, sideslip and throttle: wings level, flight path horizontal, rotor and
/// downwash settled (Newton on q̇, Ω̇ and ε̇).
fn held(
    aircraft: &Aircraft,
    env: &dyn Environment,
    trim: &Trim,
    alpha: f64,
    pitch_rate: f64,
) -> Option<(f64, f64)> {
    let f = trim.flight;
    let (sa, ca) = alpha.sin_cos();
    let state = |x: &Vector3<f64>| -> [f64; STATES] {
        let mut s = [0.0; STATES];
        s[U] = f.speed * ca * f.beta.cos();
        s[V] = f.speed * f.beta.sin();
        s[W] = f.speed * sa * f.beta.cos();
        s[Q] = pitch_rate;
        s[THETA] = alpha;
        s[OMEGA] = x[1];
        s[EPSILON] = x[2];
        s
    };
    let evaluate = |x: &Vector3<f64>| {
        let mut d = f.deflections;
        d.elevator = x[0];
        derivative(aircraft, env, &d, f.throttle, f.altitude, &state(x))
    };
    let residual = |x: &Vector3<f64>| {
        let r = evaluate(x);
        Vector3::new(r[Q], r[OMEGA] / 100.0, r[EPSILON])
    };
    let mut x = Vector3::new(f.deflections.elevator, f.rotor_speed, f.downwash);
    for _ in 0..50 {
        let r = residual(&x);
        if r.amax() < 1e-11 {
            let w_rate = evaluate(&x)[W];
            let load = -(w_rate - G0 * ca - pitch_rate * state(&x)[U]) / G0;
            return Some((x[0] / aircraft.travel().elevator, load));
        }
        let steps = [1e-6, 1e-4 * x[1].abs().max(1.0), 1e-7];
        let mut jacobian = Matrix3::zeros();
        for (c, h) in steps.iter().enumerate() {
            let (mut plus, mut minus) = (x, x);
            plus[c] += h;
            minus[c] -= h;
            jacobian.set_column(c, &((residual(&plus) - residual(&minus)) / (2.0 * h)));
        }
        x -= jacobian.lu().solve(&r)?;
    }
    None
}

/// One row: trim, secant linearisation, the three loops and the statics, for a stall
/// incidence.
pub fn row(
    aircraft: &Aircraft,
    env: &dyn Environment,
    speed: f64,
    stall_alpha: f64,
) -> Result<Row, String> {
    let (trim, flight) = trim_at(aircraft, env, 0.0, speed)?;
    let linear = linearise_secant(aircraft, env, &trim);
    let [aileron, elevator, rudder] = control_columns(aircraft, env, &trim, &linear);
    let a = &linear.a;
    let dynamic_pressure = 0.5 * RHO0 * speed * speed;
    let failed = |what: &str| format!("{what} at {:.0} km/h", speed * 3.6);

    // Bank: ṗ = L_p p + L_δa δa, φ̇ = p, and the integral, u = −(k_φ φ + k_p p + k_i ∫φ):
    // s³ + (L_δa k_p − L_p) s² + L_δa k_φ s + L_δa k_i. The pair of the response and a
    // real pole c, at the integral's place, or further out where the airframe's roll
    // damping alone exceeds the response's (no rate gain then).
    let (omega, zeta) = ROLL_RESPONSE;
    let (l_p, l_a) = (a[(P, P)], aileron[P]);
    let c = (INTEGRAL_FRACTION * omega).max(-l_p - 2.0 * zeta * omega);
    let wanted = with_real_pole(omega, zeta, c);
    let roll = [wanted[1] / l_a, (wanted[2] + l_p) / l_a, wanted[0] / l_a];

    // Incidence (α = w / V) and pitch rate, then its integral, on the model with the
    // rotor and the downwash settled: its stiffness is the aeroplane's true static one
    // (the downwash frozen, a near-neutral aeroplane looked stiff, and the loops built on
    // it were poorly damped once the lag was back: aerodynamicist, 2026-10-29). The
    // gentlest design from the lowest frequency whose rate gain still damps and whose
    // closed loop, on the whole longitudinal model, is damped enough; or else the best
    // damped.
    let (s, b) = residualised(&linear, &elevator, &[W, Q], &[OMEGA, EPSILON])
        .ok_or_else(|| failed("no settled incidence model"))?;
    let settled = DMatrix::from_row_slice(
        2,
        2,
        &[s[(0, 0)], s[(0, 1)] / speed, s[(1, 0)] * speed, s[(1, 1)]],
    );
    let (aa, bb) = augmented(&settled, &DVector::from_vec(vec![b[0] / speed, b[1]]));
    let (from, to, step) = PITCH_FREQUENCIES;
    let mut chosen: Option<([f64; 3], f64, f64)> = None;
    let mut best: Option<([f64; 3], f64, f64)> = None;
    let mut omega = from;
    while omega <= to + 1e-9 && chosen.is_none() {
        if let Some(k) = place(&aa, &bb, &response(omega, DAMPING)) {
            let k = [k[0], k[1], k[2]];
            let damping = least_damping(&pitch_closed(&linear, &elevator, k));
            if k[1] >= 0.0 && damping >= PITCH_DAMPING_SOUGHT {
                chosen = Some((k, omega, damping));
            } else if best.is_none_or(|(_, _, z)| damping > z) {
                best = Some((k, omega, damping));
            }
        }
        omega += step;
    }
    let (pitch, omega, damping) = chosen.or(best).ok_or_else(|| failed("no incidence loop"))?;
    let flight =
        format!("{flight}; incidence loop at {omega:.2} rad/s, closed-loop damping {damping:.2}");

    // Sideslip (β = v / V) and yaw rate, then its integral.
    let beta_r = DMatrix::from_row_slice(
        2,
        2,
        &[a[(V, V)], a[(V, R)] / speed, a[(R, V)] * speed, a[(R, R)]],
    );
    let b_beta = DVector::from_vec(vec![rudder[V] / speed, rudder[R]]);
    let omega = natural_frequency(&beta_r, MIN_YAW_FREQUENCY);
    let (aa, bb) = augmented(&beta_r, &b_beta);
    let k = place(&aa, &bb, &response(omega, DAMPING)).ok_or_else(|| failed("no sideslip loop"))?;
    let yaw = [k[0], k[1], k[2]];

    // Statics: secants from the level flight's incidence to the guard's limit, at this
    // speed, and over the pitch rate of one more g.
    let alpha = trim.flight.alpha;
    let far = (stall_alpha - STALL_MARGIN).max(alpha + 1.0_f64.to_radians());
    let held_at = |alpha: f64, rate: f64| {
        held(aircraft, env, &trim, alpha, rate).ok_or_else(|| failed("no held incidence"))
    };
    let (e1, n1) = held_at(alpha, 0.0)?;
    let (e2, n2) = held_at(far, 0.0)?;
    let rate = G0 / speed;
    let (e3, _) = held_at(alpha, rate)?;
    let load_per_alpha = (n2 - n1) / (far - alpha);
    let elevator_per_alpha = (e2 - e1) / (far - alpha);
    let elevator_per_pitch_rate = (e3 - e1) / rate;
    let zero_load_alpha = alpha - n1 / load_per_alpha;
    let elevator_at_zero_load = e1 - (alpha - zero_load_alpha) * elevator_per_alpha;
    // Feet with the hands: the rudder cancelling the yawing acceleration of the roll
    // rate and of the aileron.
    let rudder_per_roll_rate = -a[(R, P)] / rudder[R];
    let rudder_per_aileron = -aileron[R] / rudder[R];
    let aileron_per_roll_rate = -l_p / l_a;

    let normalised = |k: f64| k * dynamic_pressure;
    Ok(Row {
        speed,
        flight,
        load_per_alpha: load_per_alpha / dynamic_pressure,
        zero_load_alpha,
        elevator_at_zero_load,
        elevator_per_alpha,
        elevator_per_pitch_rate,
        rudder_per_roll_rate,
        rudder_per_aileron,
        aileron_per_roll_rate,
        roll: roll.map(normalised),
        pitch: pitch.map(normalised),
        yaw: yaw.map(normalised),
    })
}

/// The stall incidence, and the whole table.
pub fn assist_gains(aircraft: &Aircraft, env: &dyn Environment) -> Result<AssistGains, String> {
    let mut stall_alpha = f64::INFINITY;
    let mut notes = Vec::new();
    for (throttle, label) in [(0.0, "idle"), (1.0, "full throttle")] {
        let s = stall_speed_at(aircraft, env, 0.0, throttle)
            .map_err(|e| format!("no stall at {label}: {e:?}"))?;
        stall_alpha = stall_alpha.min(s.alpha);
        notes.push(format!(
            "{label} {:.1}° at {:.0} km/h{}",
            s.alpha.to_degrees(),
            s.speed * 3.6,
            if s.elevator_limited {
                ", elevator at its stop"
            } else {
                ""
            }
        ));
    }
    let rows = SPEEDS_KMH
        .iter()
        .map(|v| row(aircraft, env, v / 3.6, stall_alpha))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(AssistGains {
        stall_alpha,
        stall_note: notes.join(", "),
        rows,
    })
}

/// A number with six significant figures.
fn number(v: f64) -> String {
    if v == 0.0 || !v.is_finite() {
        return format!("{v:.1}");
    }
    let decimals = (5 - v.abs().log10().floor() as i32).max(1) as usize;
    format!("{v:.decimals$}")
}

/// The file `assets/aircraft/<id>/assist_gains.ron`.
pub fn to_ron(id: &str, gains: &AssistGains) -> String {
    let mut text = format!(
        "//! C: computed by `fdm_lab assist {id}` from the linearised flight model (plan docs/plans/m2-aides-pilotage.md, step 1, and tools/fdm_lab/src/assist.rs): do not edit, run the command again when the model changes (a test of fdm_lab checks the file)\n\
         //! C: each row at its equivalent airspeed, trimmed at sea level as its comment says; gains multiplied by the row's dynamic pressure, Pa (schema: crates/data/src/assist.rs)\n\
         (\n    aircraft: \"{id}\",\n    stall_alpha: Rad({}), // C: incidence of the lowest steady speed: {}\n    rows: [\n",
        number(gains.stall_alpha),
        gains.stall_note,
    );
    let list = |v: &[f64]| v.iter().map(|x| number(*x)).collect::<Vec<_>>().join(", ");
    for r in &gains.rows {
        text.push_str(&format!(
            "        // {}\n        (speed: Kmh({}), load_per_alpha: {}, zero_load_alpha: Rad({}), elevator_at_zero_load: {}, elevator_per_alpha: {}, elevator_per_pitch_rate: {}, rudder_per_roll_rate: {}, rudder_per_aileron: {}, aileron_per_roll_rate: {}, roll: ({}), pitch: ({}), yaw: ({})),\n",
            r.flight,
            number(r.speed * 3.6),
            number(r.load_per_alpha),
            number(r.zero_load_alpha),
            number(r.elevator_at_zero_load),
            number(r.elevator_per_alpha),
            number(r.elevator_per_pitch_rate),
            number(r.rudder_per_roll_rate),
            number(r.rudder_per_aileron),
            number(r.aileron_per_roll_rate),
            list(&r.roll),
            list(&r.pitch),
            list(&r.yaw),
        ));
    }
    text.push_str("    ],\n)\n");
    text
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use escadrille_data::aircraft::{Loading, load_aircraft};
    use escadrille_data::assist::load_assist_gains;
    use nalgebra::Complex;

    use super::*;
    use crate::trim::tests::STILL_AIR;

    #[test]
    fn the_placed_poles_are_where_asked() {
        // A lightly damped oscillator with its integrator: the closed loop has the
        // asked poles.
        let a = DMatrix::from_row_slice(2, 2, &[-0.5, 1.0, -9.0, -0.8]);
        let b = DVector::from_vec(vec![-0.1, -12.0]);
        let (aa, bb) = augmented(&a, &b);
        let wanted = response(4.0, 0.8);
        let k = place(&aa, &bb, &wanted).unwrap();
        let closed = &aa - &bb * k.transpose();
        let got = characteristic(&closed);
        for (g, w) in got.iter().zip(&wanted) {
            assert!(
                (g - w).abs() < 1e-9 * w.abs().max(1.0),
                "{got:?} {wanted:?}"
            );
        }
        let poles = closed.complex_eigenvalues();
        let expected = Complex::new(-0.8 * 4.0, 4.0 * (1.0 - 0.64_f64).sqrt());
        assert!(
            poles.iter().any(|p| (p - expected).norm() < 1e-6),
            "{poles}"
        );
        assert!(
            poles
                .iter()
                .any(|p| (p - Complex::new(-1.0, 0.0)).norm() < 1e-6),
            "{poles}"
        );
    }

    /// The loops closed on the whole linear model (secants, all states, the downwash lag
    /// and the rotor included), with the gains of the file's rows at their equivalent
    /// airspeed, at sea level and at 4 000 m, tank full and empty: every mode faster than
    /// the phugoid and the spiral damped to 0.5 at least (aerodynamicist, 2026-10-29).
    #[test]
    fn the_closed_loops_are_damped_across_the_envelope() {
        use escadrille_fdm::atmosphere::isa;

        let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
        let data = load_aircraft(&assets, "spad7").unwrap();
        let full = data
            .build(&Loading { fuel_fraction: 1.0 })
            .unwrap()
            .aircraft;
        let gains = assist_gains(&full, &STILL_AIR).unwrap();
        let mut worst = (f64::INFINITY, String::new());
        for fuel in [1.0, 0.0] {
            let aircraft = data
                .build(&Loading {
                    fuel_fraction: fuel,
                })
                .unwrap()
                .aircraft;
            for altitude in [0.0, 4_000.0] {
                let sigma = isa(altitude).density / RHO0;
                for r in &gains.rows {
                    let speed = r.speed / sigma.sqrt();
                    let Ok((trim, _)) = trim_at(&aircraft, &STILL_AIR, altitude, speed) else {
                        continue;
                    };
                    let linear = linearise_secant(&aircraft, &STILL_AIR, &trim);
                    let [aileron, elevator, rudder] =
                        control_columns(&aircraft, &STILL_AIR, &trim, &linear);
                    let (roll, pitch, yaw) = r.at(0.5 * RHO0 * r.speed * r.speed);
                    let loops = [
                        ("incidence", pitch_closed(&linear, &elevator, pitch)),
                        (
                            "instructor's bank and coordination",
                            lateral_closed(&linear, [&aileron, &rudder], r, (roll, yaw), true),
                        ),
                        (
                            "coordination",
                            lateral_closed(&linear, [&aileron, &rudder], r, (roll, yaw), false),
                        ),
                    ];
                    for (name, m) in loops {
                        let z = least_damping(&m);
                        if z < worst.0 {
                            worst = (
                                z,
                                format!(
                                    "{name} at {:.0} km/h EAS, {altitude} m, fuel {fuel}",
                                    r.speed * 3.6
                                ),
                            );
                        }
                    }
                }
            }
        }
        eprintln!("least damping {:.2}: {}", worst.0, worst.1);
        assert!(worst.0 >= 0.5, "least damping {:.2}: {}", worst.0, worst.1);
    }

    /// The file follows the flight model: when the model changes, this fails and prints
    /// the file to write (or run `fdm_lab assist spad7`).
    #[test]
    fn the_assist_gains_follow_the_flight_model() {
        let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
        let aircraft = load_aircraft(&assets, "spad7")
            .unwrap()
            .build(&Loading { fuel_fraction: 1.0 })
            .unwrap()
            .aircraft;
        let gains = assist_gains(&aircraft, &STILL_AIR).unwrap();
        let file = load_assist_gains(&assets, "spad7").unwrap().unwrap();
        let close = |a: f64, b: f64| (a - b).abs() <= 1e-4 * a.abs().max(b.abs()) + 1e-9;
        let same = file.rows.len() == gains.rows.len()
            && close(file.stall_alpha.si(), gains.stall_alpha)
            && file.rows.iter().zip(&gains.rows).all(|(f, g)| {
                close(f.speed.si(), g.speed)
                    && close(f.load_per_alpha, g.load_per_alpha)
                    && close(f.zero_load_alpha.si(), g.zero_load_alpha)
                    && close(f.elevator_at_zero_load, g.elevator_at_zero_load)
                    && close(f.elevator_per_alpha, g.elevator_per_alpha)
                    && close(f.elevator_per_pitch_rate, g.elevator_per_pitch_rate)
                    && close(f.rudder_per_roll_rate, g.rudder_per_roll_rate)
                    && close(f.rudder_per_aileron, g.rudder_per_aileron)
                    && close(f.aileron_per_roll_rate, g.aileron_per_roll_rate)
                    && f.roll.iter().zip(&g.roll).all(|(a, b)| close(*a, *b))
                    && f.pitch.iter().zip(&g.pitch).all(|(a, b)| close(*a, *b))
                    && f.yaw.iter().zip(&g.yaw).all(|(a, b)| close(*a, *b))
            });
        assert!(
            same,
            "assist_gains.ron is stale:\n{}",
            to_ron("spad7", &gains)
        );
    }
}
