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
    let steps = [1e-4, 1e-4, 1e-4, 1e-5, 1e-5, 1e-5, 1e-6, 1e-6, 1e-3, 1e-7];
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
        // A small mixed perturbation: the linear model is right to second order.
        let dx = [
            0.05, 0.03, -0.04, 0.002, -0.001, 0.0015, 0.001, -0.0008, 0.5, 0.0002,
        ];
        let mut x = lin.x0;
        for i in 0..STATES {
            x[i] += dx[i];
        }
        let f1 = at(&x);
        for i in 0..STATES {
            let predicted: f64 = (0..STATES).map(|j| lin.a[(i, j)] * dx[j]).sum();
            let actual = f1[i] - f0[i];
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
        assert!(lat.len() >= 2 && lat.len() <= 4, "{lat:?}");

        // The full matrix keeps one mode per real eigenvalue or conjugate pair.
        let full = coupled(&lin);
        assert!(full.len() * 2 >= STATES && full.len() <= STATES, "{full:?}");
    }
}
