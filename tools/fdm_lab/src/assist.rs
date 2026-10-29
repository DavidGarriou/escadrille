//! Gains of the flight assists (plan `docs/plans/m2-aides-pilotage.md`, step 1; skill
//! `realisme`): the game's assists move the commands as a pilot would, and their gains
//! come from the flight model, never from hand tuning.
//!
//! At each speed of [`SPEEDS_KMH`], at sea level, the model is trimmed and linearised
//! ([`crate::modes`]). Three reduced models give three loops:
//! - bank, by the aileron: roll rate and bank (p, φ), and the bank's integral;
//! - incidence, by the elevator: incidence and pitch rate (α, q), and the incidence's
//!   integral;
//! - sideslip, by the rudder: sideslip and yaw rate (β, r), and the sideslip's integral.
//!
//! Their poles are placed at a chosen response: [`ROLL_RESPONSE`] for the bank; for the
//! others, the airframe's own natural frequency, at least [`MIN_PITCH_FREQUENCY`] or
//! [`MIN_YAW_FREQUENCY`], damped to [`DAMPING`] (the loops damp and hold, they do not
//! stiffen the aeroplane), the integrator at [`INTEGRAL_FRACTION`] of that frequency.
//! The statics give the incidence of a load factor, the elevator holding an incidence
//! and a pitch rate, and the rudder meeting the adverse yaw of the roll rate and the
//! aileron; the stall sweep ([`crate::performance`]) gives the incidence of maximum lift.

use escadrille_fdm::aero::surface::ControlDeflections;
use escadrille_fdm::aircraft::Aircraft;
use escadrille_fdm::constants::{G0, RHO0};
use escadrille_fdm::environment::Environment;
use nalgebra::{DMatrix, DVector};

use crate::modes::{Linear, STATES, derivative, linearise};
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
/// Damping of the incidence and sideslip loops.
pub const DAMPING: f64 = 0.8;
/// Lowest natural frequencies of the incidence and sideslip loops, rad/s.
pub const MIN_PITCH_FREQUENCY: f64 = 3.0;
pub const MIN_YAW_FREQUENCY: f64 = 1.5;
/// Pole of the integrators, as a fraction of their loop's frequency.
pub const INTEGRAL_FRACTION: f64 = 0.25;

/// Indices of the states of [`crate::modes`].
const V: usize = 1;
const W: usize = 2;
const P: usize = 3;
const Q: usize = 4;
const R: usize = 5;
const OMEGA: usize = 8;
const EPSILON: usize = 9;

/// The assists' figures at one speed (`escadrille_data::assist::AssistRow`).
#[derive(Debug, Clone, PartialEq)]
pub struct Row {
    /// Equivalent airspeed (true airspeed at sea level), m/s.
    pub speed: f64,
    /// How the row was trimmed, for the file.
    pub flight: String,
    /// Load factor per radian of incidence, per pascal.
    pub load_per_alpha: f64,
    pub zero_load_alpha: f64,
    pub elevator_at_zero_load: f64,
    pub elevator_per_alpha: f64,
    pub elevator_per_pitch_rate: f64,
    pub rudder_per_roll_rate: f64,
    pub rudder_per_aileron: f64,
    /// Gains × dynamic pressure: bank, roll rate, integral.
    pub roll: [f64; 3],
    /// Incidence, pitch rate, integral.
    pub pitch: [f64; 3],
    /// Sideslip, yaw rate, integral.
    pub yaw: [f64; 3],
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

/// (s² + 2ζω s + ω²), times (s + f ω) with an integrator, from the constant term up.
fn response(omega: f64, zeta: f64, integral: bool) -> Vec<f64> {
    let pair = [omega * omega, 2.0 * zeta * omega, 1.0];
    if !integral {
        return pair.to_vec();
    }
    let i = INTEGRAL_FRACTION * omega;
    vec![
        i * pair[0],
        pair[0] + i * pair[1],
        pair[1] + i * pair[2],
        pair[2],
    ]
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

/// State derivative per unit of each command (aileron, elevator, rudder) at the trim.
fn control_columns(
    aircraft: &Aircraft,
    env: &dyn Environment,
    trim: &Trim,
    linear: &Linear,
) -> [DVector<f64>; 3] {
    let f = trim.flight;
    let travel = aircraft.travel();
    let h = 1e-4;
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

/// Level flight at a speed, or, beyond the maximum level speed, a dive at full throttle.
fn trim_at(
    aircraft: &Aircraft,
    env: &dyn Environment,
    speed: f64,
) -> Result<(Trim, String), String> {
    if let Ok(t) = solve_robust(
        aircraft,
        env,
        Flight::guess(0.0, speed, 0.5),
        LEVEL_AT_SPEED,
    ) && t.feasible(aircraft)
    {
        let note = format!("level flight, throttle {:.2}", t.flight.throttle);
        return Ok((t, note));
    }
    let mut guess = Flight::guess(0.0, speed, 1.0);
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

/// One row: trim, linearisation, the three loops and the statics.
pub fn row(aircraft: &Aircraft, env: &dyn Environment, speed: f64) -> Result<Row, String> {
    let (trim, flight) = trim_at(aircraft, env, speed)?;
    let linear = linearise(aircraft, env, &trim);
    let [aileron, elevator, rudder] = control_columns(aircraft, env, &trim, &linear);
    let a = &linear.a;
    let dynamic_pressure = 0.5 * RHO0 * speed * speed;
    let failed = |what: &str| format!("{what} at {:.0} km/h", speed * 3.6);

    // Bank: ṗ = L_p p + L_δa δa, φ̇ = p, and the bank's integral, u = −K (φ, p, ∫φ):
    // s³ + (L_δa k_p − L_p) s² + L_δa k_φ s + L_δa k_i is matched to the response. The
    // airframe's roll damping is often more than the response asks: the rate gain stays
    // zero then, and the poles are faster and as stable (Routh).
    let (omega, zeta) = ROLL_RESPONSE;
    let (l_p, l_a) = (a[(P, P)], aileron[P]);
    let wanted = response(omega, zeta, true);
    let roll = [
        wanted[1] / l_a,
        (wanted[2] + l_p).max(0.0) / l_a,
        wanted[0] / l_a,
    ];

    // Incidence (α = w / V) and pitch rate, then its integral.
    let alpha_q = DMatrix::from_row_slice(
        2,
        2,
        &[a[(W, W)], a[(W, Q)] / speed, a[(Q, W)] * speed, a[(Q, Q)]],
    );
    let b_alpha = DVector::from_vec(vec![elevator[W] / speed, elevator[Q]]);
    let omega = natural_frequency(&alpha_q, MIN_PITCH_FREQUENCY);
    let (aa, bb) = augmented(&alpha_q, &b_alpha);
    let k = place(&aa, &bb, &response(omega, DAMPING, true))
        .ok_or_else(|| failed("no incidence loop"))?;
    let pitch = [k[0], k[1], k[2]];

    // Sideslip (β = v / V) and yaw rate, then its integral.
    let beta_r = DMatrix::from_row_slice(
        2,
        2,
        &[a[(V, V)], a[(V, R)] / speed, a[(R, V)] * speed, a[(R, R)]],
    );
    let b_beta = DVector::from_vec(vec![rudder[V] / speed, rudder[R]]);
    let omega = natural_frequency(&beta_r, MIN_YAW_FREQUENCY);
    let (aa, bb) = augmented(&beta_r, &b_beta);
    let k = place(&aa, &bb, &response(omega, DAMPING, true))
        .ok_or_else(|| failed("no sideslip loop"))?;
    let yaw = [k[0], k[1], k[2]];

    // Statics at constant speed, rotor and downwash settled.
    let (s, b) = residualised(&linear, &elevator, &[W, Q], &[OMEGA, EPSILON])
        .ok_or_else(|| failed("no static incidence"))?;
    let load_per_alpha = -s[(0, 0)] * speed / G0;
    let elevator_per_alpha = -s[(1, 0)] * speed / b[1];
    let elevator_per_pitch_rate = -s[(1, 1)] / b[1];
    // Feet with the hands: the rudder cancelling the yawing acceleration of the roll
    // rate and of the aileron.
    let rudder_per_roll_rate = -a[(R, P)] / rudder[R];
    let rudder_per_aileron = -aileron[R] / rudder[R];
    let alpha = trim.flight.alpha;
    let zero_load_alpha = alpha - trim.flight.flight_path.cos() / load_per_alpha;
    let elevator_at_zero_load =
        trim.commands(aircraft).pitch - (alpha - zero_load_alpha) * elevator_per_alpha;

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
        roll: roll.map(normalised),
        pitch: pitch.map(normalised),
        yaw: yaw.map(normalised),
    })
}

/// The whole table and the stall incidence.
pub fn assist_gains(aircraft: &Aircraft, env: &dyn Environment) -> Result<AssistGains, String> {
    let rows = SPEEDS_KMH
        .iter()
        .map(|v| row(aircraft, env, v / 3.6))
        .collect::<Result<Vec<_>, _>>()?;
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
            "        // {}\n        (speed: Kmh({}), load_per_alpha: {}, zero_load_alpha: Rad({}), elevator_at_zero_load: {}, elevator_per_alpha: {}, elevator_per_pitch_rate: {}, rudder_per_roll_rate: {}, rudder_per_aileron: {}, roll: ({}), pitch: ({}), yaw: ({})),\n",
            r.flight,
            number(r.speed * 3.6),
            number(r.load_per_alpha),
            number(r.zero_load_alpha),
            number(r.elevator_at_zero_load),
            number(r.elevator_per_alpha),
            number(r.elevator_per_pitch_rate),
            number(r.rudder_per_roll_rate),
            number(r.rudder_per_aileron),
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
        let wanted = response(4.0, 0.8, true);
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
