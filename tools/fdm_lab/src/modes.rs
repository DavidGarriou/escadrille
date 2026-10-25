//! Eigenmodes around a trim (skill `simulation-vol`, `modele-aero.md`, « Linéarisation et
//! modes propres »): the state derivative of the flight model is linearised by central
//! differences, and the eigenvalues of the longitudinal and lateral parts give the
//! classical modes (short period, phugoid, roll, spiral, dutch roll).
//!
//! State: x = (u, v, w, p, q, r, φ, θ, Ω, ε), heading and position left out (they do not
//! act on the others in still, uniform air). The rotor speed Ω and the downwash lag ε give
//! extra real modes, labelled by their dominant state. The full matrix is also solved, to
//! show the coupling (engine torque, P-factor) between the two parts.

use escadrille_fdm::aero::surface::ControlDeflections;
use escadrille_fdm::aircraft::{AUX_DOWNWASH, Aircraft};
use escadrille_fdm::environment::Environment;
use escadrille_fdm::rigid_body::RigidBodyState;
use glam::{DQuat, DVec3, EulerRot};
use nalgebra::{Complex, DMatrix};

use crate::trim::Trim;

/// Number of states.
pub const STATES: usize = 10;

/// State names, in order.
pub const NAMES: [&str; STATES] = ["u", "v", "w", "p", "q", "r", "φ", "θ", "Ω", "ε"];

/// Longitudinal states (u, w, q, θ, Ω, ε) and lateral states (v, p, r, φ).
pub const LONGITUDINAL: [usize; 6] = [0, 2, 4, 7, 8, 9];
pub const LATERAL: [usize; 4] = [1, 3, 5, 6];

/// Linearised model around a trim.
#[derive(Debug, Clone, PartialEq)]
pub struct Linear {
    /// Trim state vector.
    pub x0: [f64; STATES],
    /// ẋ = A (x − x0).
    pub a: DMatrix<f64>,
    /// Airspeed of the trim, m/s (state scaling).
    pub speed: f64,
}

/// Flight-model derivative for a state vector, controls held.
pub fn derivative(
    aircraft: &Aircraft,
    env: &dyn Environment,
    deflections: &ControlDeflections,
    throttle: f64,
    altitude: f64,
    x: &[f64; STATES],
) -> [f64; STATES] {
    let (phi, theta) = (x[6], x[7]);
    let mut state = RigidBodyState {
        position_ned: DVec3::new(0.0, 0.0, -altitude),
        attitude: DQuat::from_euler(EulerRot::ZYX, 0.0, theta, phi),
        velocity_body: DVec3::new(x[0], x[1], x[2]),
        angular_velocity_body: DVec3::new(x[3], x[4], x[5]),
        rotor_speed: x[8],
        ..RigidBodyState::default()
    };
    state.aux[AUX_DOWNWASH] = x[9];
    let loads = aircraft
        .evaluate_deflected(&state, deflections, throttle, env)
        .loads;
    let d = state.derivative(aircraft.mass(), &loads);
    let (p, q, r) = (x[3], x[4], x[5]);
    [
        d.velocity.x,
        d.velocity.y,
        d.velocity.z,
        d.angular_velocity.x,
        d.angular_velocity.y,
        d.angular_velocity.z,
        p + (q * phi.sin() + r * phi.cos()) * theta.tan(),
        q * phi.cos() - r * phi.sin(),
        d.rotor_speed,
        d.aux[AUX_DOWNWASH],
    ]
}

/// Linearises the flight model around a trim by central differences.
pub fn linearise(aircraft: &Aircraft, env: &dyn Environment, trim: &Trim) -> Linear {
    linearise_with(
        aircraft,
        env,
        trim,
        [1e-4, 1e-4, 1e-4, 1e-5, 1e-5, 1e-5, 1e-6, 1e-6, 1e-3, 1e-7],
    )
}

/// Half-step of the static stability in incidence, sideslip and downwash, rad: a secant
/// over ±1°. The tangent of [`linearise`] reads the slope of a single segment of the
/// piecewise-linear tables (polars, propeller), and the margin jumped with the trim speed,
/// 2 to 6 % from 140 to 170 km/h on the SPAD (aerodynamicist review of 2026-10-25).
const SECANT_ANGLE: f64 = 1.0 * std::f64::consts::PI / 180.0;

/// Half-step of the static stability in rotor speed, fraction of the trimmed speed.
const SECANT_ROTOR: f64 = 0.02;

/// Same as [`linearise`] with the given half-steps of central differences.
fn linearise_with(
    aircraft: &Aircraft,
    env: &dyn Environment,
    trim: &Trim,
    steps: [f64; STATES],
) -> Linear {
    let f = trim.flight;
    let state = f.state();
    let v = state.velocity_body;
    let x0 = [
        v.x,
        v.y,
        v.z,
        0.0,
        0.0,
        0.0,
        0.0,
        f.pitch(),
        f.rotor_speed,
        f.downwash,
    ];
    let mut a = DMatrix::<f64>::zeros(STATES, STATES);
    for j in 0..STATES {
        let (mut plus, mut minus) = (x0, x0);
        plus[j] += steps[j];
        minus[j] -= steps[j];
        let fp = derivative(aircraft, env, &f.deflections, f.throttle, f.altitude, &plus);
        let fm = derivative(
            aircraft,
            env,
            &f.deflections,
            f.throttle,
            f.altitude,
            &minus,
        );
        for i in 0..STATES {
            a[(i, j)] = (fp[i] - fm[i]) / (2.0 * steps[j]);
        }
    }
    Linear {
        x0,
        a,
        speed: f.speed,
    }
}

/// One mode: an eigenvalue (with its conjugate for an oscillation).
#[derive(Debug, Clone, PartialEq)]
pub struct Mode {
    pub name: String,
    pub eigenvalue: Complex<f64>,
    /// Damping ratio ζ = −Re λ / |λ|.
    pub damping: f64,
    /// Undamped natural frequency |λ|, rad/s.
    pub natural_frequency: f64,
    /// Period of the oscillation 2π / Im λ, s (`None` for a real mode).
    pub period: Option<f64>,
    /// Time to half amplitude (stable) or to double (unstable), s.
    pub time_to_half_or_double: f64,
    /// Dominant state of the eigenvector (scaled).
    pub dominant: &'static str,
}

impl Mode {
    pub fn stable(&self) -> bool {
        self.eigenvalue.re < 0.0
    }
}

/// Scale of each state for comparing eigenvector components: velocities over the
/// airspeed, rates over V / 5 m (a few chords), angles as such, rotor over 100 rad/s,
/// downwash over 0.5 rad (it follows half the angle of attack or so).
fn scales(speed: f64) -> [f64; STATES] {
    let v = speed.max(1.0);
    let rate = v / 5.0;
    [v, v, v, rate, rate, rate, 1.0, 1.0, 100.0, 0.5]
}

/// Eigenvector of `a` for `lambda`: right singular vector of the smallest singular value
/// of A − λ I.
fn eigenvector(a: &DMatrix<f64>, lambda: Complex<f64>) -> Vec<Complex<f64>> {
    let n = a.nrows();
    let shifted = DMatrix::from_fn(n, n, |i, j| {
        Complex::new(a[(i, j)], 0.0)
            - if i == j {
                lambda
            } else {
                Complex::new(0.0, 0.0)
            }
    });
    let svd = shifted.svd(false, true);
    let k = (0..n)
        .min_by(|&i, &j| svd.singular_values[i].total_cmp(&svd.singular_values[j]))
        .unwrap_or(0);
    let v_t = svd.v_t.expect("requested");
    (0..n).map(|j| v_t[(k, j)].conj()).collect()
}

/// Modes of the sub-matrix of `linear` over `states`.
pub fn modes(linear: &Linear, states: &[usize]) -> Vec<Mode> {
    let n = states.len();
    let sub = DMatrix::from_fn(n, n, |i, j| linear.a[(states[i], states[j])]);
    let scale = scales(linear.speed);
    let mut eigenvalues: Vec<Complex<f64>> = sub
        .complex_eigenvalues()
        .iter()
        .copied()
        .filter(|l| l.im >= 0.0)
        .collect();
    eigenvalues.sort_by(|a, b| a.norm().total_cmp(&b.norm()));
    eigenvalues
        .into_iter()
        .map(|lambda| {
            let vector = eigenvector(&sub, lambda);
            let dominant = (0..n)
                .max_by(|&i, &j| {
                    let w = |k: usize| vector[k].norm() / scale[states[k]];
                    w(i).total_cmp(&w(j))
                })
                .map_or("?", |k| NAMES[states[k]]);
            let norm = lambda.norm();
            Mode {
                name: String::new(),
                eigenvalue: lambda,
                damping: if norm > 0.0 { -lambda.re / norm } else { 0.0 },
                natural_frequency: norm,
                period: (lambda.im > 1e-9).then(|| 2.0 * std::f64::consts::PI / lambda.im),
                time_to_half_or_double: std::f64::consts::LN_2 / lambda.re.abs(),
                dominant,
            }
        })
        .collect()
}

/// Longitudinal modes, named: the slowest oscillation is the phugoid, the fastest the
/// short period; real modes are named by their dominant state.
pub fn longitudinal(linear: &Linear) -> Vec<Mode> {
    let mut modes = modes(linear, &LONGITUDINAL);
    let oscillations: Vec<usize> = (0..modes.len())
        .filter(|&i| modes[i].period.is_some())
        .collect();
    for (rank, &i) in oscillations.iter().enumerate() {
        modes[i].name = match (rank, oscillations.len()) {
            (0, n) if n > 1 => "phugoïde".into(),
            (r, n) if r + 1 == n => "oscillation d'incidence".into(),
            _ => "oscillation".into(),
        };
    }
    for mode in modes.iter_mut().filter(|m| m.name.is_empty()) {
        mode.name = match mode.dominant {
            "Ω" => "régime du rotor".into(),
            "ε" => "retard de la déflexion".into(),
            "u" | "θ" => "phugoïde apériodique".into(),
            _ => "incidence apériodique".into(),
        };
    }
    modes
}

/// Lateral modes, named by their eigenvector rather than their rank, so that an unusual
/// aircraft is not given textbook names: an oscillation driven by sideslip and yaw is the
/// dutch roll, a slow one driven by bank a coupled spiral and roll oscillation; the real
/// mode driven by the roll rate is the roll subsidence, a slow one driven by bank or yaw
/// the spiral.
pub fn lateral(linear: &Linear) -> Vec<Mode> {
    let mut modes = modes(linear, &LATERAL);
    for mode in &mut modes {
        mode.name = match (mode.period.is_some(), mode.dominant) {
            (true, "v" | "r") => "roulis hollandais".into(),
            (true, _) => "oscillation spirale-roulis".into(),
            (false, "p") => "roulis".into(),
            (false, "φ" | "r") if mode.natural_frequency < 0.5 => "spirale".into(),
            (false, dominant) => format!("apériodique ({dominant})"),
        };
    }
    modes
}

/// Model reduced to the `slow` states, the `fast` ones in quasi-static equilibrium
/// (Schur complement A_ss − A_sf A_ff⁻¹ A_fs): the way to compare variants without
/// freezing states, which would describe another aircraft (aerodynamicist review).
pub fn reduced(linear: &Linear, slow: &[usize], fast: &[usize]) -> Option<DMatrix<f64>> {
    let block = |rows: &[usize], cols: &[usize]| {
        DMatrix::from_fn(rows.len(), cols.len(), |i, j| linear.a[(rows[i], cols[j])])
    };
    let inverse = block(fast, fast).try_inverse()?;
    Some(block(slow, slow) - block(slow, fast) * inverse * block(fast, slow))
}

/// Static stability around a trim.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StaticStability {
    /// Stick-fixed static margin, fraction of the mean wing chord, rotor speed and
    /// downwash in quasi-static equilibrium: K = M_w / (Z_w c̄).
    pub static_margin: f64,
    /// Mean chord of the wings used, m.
    pub mean_chord: f64,
    /// Yawing moment per unit of sideslip velocity, N·m / (m/s); positive is stable
    /// (weathercock).
    pub n_v: f64,
    /// Rolling moment per unit of sideslip velocity, N·m / (m/s); negative is the
    /// dihedral effect.
    pub l_v: f64,
}

/// Static margin and lateral derivatives around a trim, from a linear model by secants
/// ([`SECANT_ANGLE`], [`SECANT_ROTOR`]).
pub fn static_stability(
    aircraft: &Aircraft,
    env: &dyn Environment,
    trim: &Trim,
) -> Option<StaticStability> {
    let f = trim.flight;
    let (speed, angle) = (f.speed * SECANT_ANGLE.tan(), SECANT_ANGLE);
    let rotor = (SECANT_ROTOR * f.rotor_speed).max(1e-3);
    let linear = &linearise_with(
        aircraft,
        env,
        trim,
        [
            speed, speed, speed, 1e-5, 1e-5, 1e-5, 1e-6, 1e-6, rotor, angle,
        ],
    );
    // Longitudinal: u, w, q, θ with Ω and ε eliminated.
    let long = reduced(linear, &[0, 2, 4, 7], &[8, 9])?;
    let (z_w, m_w) = (long[(1, 1)], long[(2, 1)]);
    let mass = aircraft.mass();
    let inertia = mass.airframe_inertia();
    let surfaces = aircraft.lifting().surfaces();
    let wings: Vec<usize> = aircraft
        .downwash()
        .map_or_else(|| vec![0], |d| d.wings.clone());
    let (mut area, mut span) = (0.0, 0.0);
    for &i in &wings {
        let strips = surfaces[i].strips();
        area += strips.iter().map(|s| s.area).sum::<f64>();
        // Strip edges: centre ± half its width (area / chord).
        let right = strips
            .iter()
            .map(|s| s.position.y + 0.5 * s.area / s.chord)
            .fold(f64::MIN, f64::max);
        let left = strips
            .iter()
            .map(|s| s.position.y - 0.5 * s.area / s.chord)
            .fold(f64::MAX, f64::min);
        span += right - left;
    }
    if !(area > 0.0 && span > 0.0 && z_w != 0.0) {
        return None;
    }
    let mean_chord = area / span;
    let static_margin = (inertia.y_axis.y * m_w) / (mass.mass() * z_w * mean_chord);
    // Lateral: moments from the angular accelerations, M = I' ω̇.
    let rates = glam::DVec3::new(linear.a[(3, 1)], linear.a[(4, 1)], linear.a[(5, 1)]);
    let moments = inertia * rates;
    Some(StaticStability {
        static_margin,
        mean_chord,
        n_v: moments.z,
        l_v: moments.x,
    })
}

/// Eigenvalues of the full matrix (couplings included).
pub fn coupled(linear: &Linear) -> Vec<Mode> {
    modes(linear, &(0..STATES).collect::<Vec<_>>())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trim::tests::{STILL_AIR, biplane};
    use crate::trim::{Flight, LEVEL_AT_SPEED, solve_robust};

    fn trimmed(speed: f64) -> (Aircraft, Trim) {
        let a = biplane();
        let trim = solve_robust(
            &a,
            &STILL_AIR,
            Flight::guess(1_000.0, speed, 0.4),
            LEVEL_AT_SPEED,
        )
        .unwrap();
        (a, trim)
    }

    #[test]
    fn linear_model_predicts_small_perturbations() {
        let (a, trim) = trimmed(35.0);
        let lin = linearise(&a, &STILL_AIR, &trim);
        let f = trim.flight;
        let at = |x: &[f64; STATES]| {
            derivative(&a, &STILL_AIR, &f.deflections, f.throttle, f.altitude, x)
        };
        // The trim is an equilibrium.
        let f0 = at(&lin.x0);
        assert!(f0.iter().take(6).all(|v| v.abs() < 1e-7), "{f0:?}");
        // A small mixed perturbation, on both sides: the linear model is right to second
        // order. The centred difference ½ (f(x₀ + dx) − f(x₀ − dx)) cancels the curvature,
        // and the first-order error of a slope break of the piecewise-linear tables at the
        // trim point itself, as the Jacobian (aerodynamicist review of 2026-10-25).
        let dx = [
            0.05, 0.03, -0.04, 0.002, -0.001, 0.0015, 0.001, -0.0008, 0.5, 0.0002,
        ];
        let (mut plus, mut minus) = (lin.x0, lin.x0);
        for i in 0..STATES {
            plus[i] += dx[i];
            minus[i] -= dx[i];
        }
        let (f_plus, f_minus) = (at(&plus), at(&minus));
        for i in 0..STATES {
            let predicted: f64 = (0..STATES).map(|j| lin.a[(i, j)] * dx[j]).sum();
            let actual = 0.5 * (f_plus[i] - f_minus[i]);
            assert!(
                (predicted - actual).abs() < 0.02 * actual.abs().max(1e-3),
                "{}: {predicted} vs {actual}",
                NAMES[i]
            );
        }
    }

    #[test]
    fn classical_modes_appear() {
        let (a, trim) = trimmed(35.0);
        let lin = linearise(&a, &STILL_AIR, &trim);
        let long = longitudinal(&lin);
        let lat = lateral(&lin);
        let named = |modes: &[Mode], name: &str| modes.iter().find(|m| m.name == name).cloned();

        // Phugoid and short period: two oscillations far apart, the fast one well
        // damped (statically stable aircraft). The phugoid period is not compared to
        // Lanchester's here: this synthetic biplane has a large slipstream on its wings.
        let phugoid = named(&long, "phugoïde").expect("phugoid");
        let short = named(&long, "oscillation d'incidence").expect("short period");
        assert!(
            phugoid.period.unwrap() > 5.0 * short.period.unwrap(),
            "{long:?}"
        );
        assert!(short.stable() && short.damping > 0.2, "{short:?}");
        assert!(named(&long, "régime du rotor").is_some(), "{long:?}");

        // Roll subsidence: fast and stable, dominated by the roll rate.
        let roll = named(&lat, "roulis").expect("roll mode");
        assert!(roll.stable() && roll.natural_frequency > 1.0, "{roll:?}");
        assert_eq!(roll.dominant, "p");
        // With a plausible propeller normal force (aerodynamicist review: 0.3 made the
        // biplane directionally unstable), the dutch roll and the spiral are there.
        let dutch = named(&lat, "roulis hollandais").expect("dutch roll");
        assert!(dutch.period.unwrap() < 10.0 && dutch.stable(), "{dutch:?}");
        let spiral = named(&lat, "spirale").expect("spiral");
        assert!(spiral.time_to_half_or_double > 10.0, "slow: {spiral:?}");

        let st = static_stability(&a, &STILL_AIR, &trim).unwrap();
        assert!(st.n_v > 0.0 && st.l_v < 0.0, "{st:?}");
        assert!(st.static_margin > 0.0 && st.static_margin < 0.3, "{st:?}");

        // The full matrix keeps one mode per real eigenvalue or conjugate pair.
        let full = coupled(&lin);
        assert!(full.len() * 2 >= STATES && full.len() <= STATES, "{full:?}");
    }
}

#[cfg(test)]
mod kinematics {
    use super::*;
    use crate::trim::tests::{STILL_AIR, biplane};

    #[test]
    fn euler_rates_match_the_attitude_quaternion() {
        // φ̇ and θ̇ of the linear model against the Euler angles of the quaternion
        // advanced by a small time (banked and pitched, all three rates).
        let a = biplane();
        let (phi, theta) = (0.4_f64, 0.25_f64);
        let w = DVec3::new(0.3, -0.2, 0.25);
        let x = [40.0, 2.0, 3.0, w.x, w.y, w.z, phi, theta, 180.0, 0.02];
        let d = derivative(
            &a,
            &STILL_AIR,
            &ControlDeflections::default(),
            0.5,
            1_000.0,
            &x,
        );
        let q0 = DQuat::from_euler(EulerRot::ZYX, 0.0, theta, phi);
        let h = 1e-6;
        let q1 = (q0 * DQuat::from_scaled_axis(w * h)).normalize();
        let (_, theta1, phi1) = q1.to_euler(EulerRot::ZYX);
        assert!(
            ((phi1 - phi) / h - d[6]).abs() < 1e-5,
            "φ̇ {} vs {}",
            (phi1 - phi) / h,
            d[6]
        );
        assert!(
            ((theta1 - theta) / h - d[7]).abs() < 1e-5,
            "θ̇ {} vs {}",
            (theta1 - theta) / h,
            d[7]
        );
    }
}
