//! Steady straight flight, wings level (trim): Newton's method on the state derivative
//! of the flight model (skill `simulation-vol`, `modele-aero.md`, « Linéarisation »).
//!
//! A [`Flight`] holds ten variables; eight of them are solved so that the eight
//! derivatives that must vanish do: body accelerations (3), angular accelerations (3),
//! rotor acceleration and downwash-lag rate. Which eight depends on the question: level
//! flight at a given speed solves the throttle, maximum speed solves the speed at full
//! throttle, a climb solves the flight-path angle. The control surfaces are solved as
//! deflections, not limited to their travel, so that the solver never sees them
//! saturate; [`Trim::commands`] tells whether the pilot can actually hold the trim.

use escadrille_fdm::aero::surface::ControlDeflections;
use escadrille_fdm::aircraft::{AUX_DOWNWASH, Aircraft, PilotControls};
use escadrille_fdm::constants::G0;
use escadrille_fdm::environment::Environment;
use escadrille_fdm::rigid_body::RigidBodyState;
use glam::{DQuat, DVec3, EulerRot};
use nalgebra::{DMatrix, DVector};

/// Number of unknowns and of equations.
pub const N: usize = 8;

/// Maximum Newton iterations.
const MAX_ITERATIONS: usize = 100;

/// Largest scaled residual accepted as a solution.
pub const TOLERANCE: f64 = 1e-10;

/// Variables of a steady straight flight, wings level.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Flight {
    /// Altitude, m.
    pub altitude: f64,
    /// True airspeed, m/s.
    pub speed: f64,
    /// Flight-path angle γ, rad (positive climbing).
    pub flight_path: f64,
    pub alpha: f64,
    pub beta: f64,
    pub deflections: ControlDeflections,
    /// Throttle lever, 0 to 1.
    pub throttle: f64,
    /// Rotor speed, rad/s.
    pub rotor_speed: f64,
    /// Downwash at the tail, rad.
    pub downwash: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unknown {
    Alpha,
    Beta,
    Elevator,
    Aileron,
    Rudder,
    Throttle,
    Speed,
    FlightPath,
    RotorSpeed,
    Downwash,
}

impl Unknown {
    /// Step of the central differences.
    fn step(self, value: f64) -> f64 {
        match self {
            Self::Speed => 1e-5 * value.abs().max(1.0),
            Self::RotorSpeed => 1e-4 * value.abs().max(1.0),
            Self::Downwash => 1e-7,
            _ => 1e-6,
        }
    }
}

/// Level flight at a given speed: everything but the speed and the flight path.
pub const LEVEL_AT_SPEED: [Unknown; N] = [
    Unknown::Alpha,
    Unknown::Beta,
    Unknown::Elevator,
    Unknown::Aileron,
    Unknown::Rudder,
    Unknown::Throttle,
    Unknown::RotorSpeed,
    Unknown::Downwash,
];

/// Given throttle and flight path: the speed (maximum speed at full throttle).
pub const SPEED_AT_THROTTLE: [Unknown; N] = [
    Unknown::Alpha,
    Unknown::Beta,
    Unknown::Elevator,
    Unknown::Aileron,
    Unknown::Rudder,
    Unknown::Speed,
    Unknown::RotorSpeed,
    Unknown::Downwash,
];

/// Given speed and throttle: the flight path (climb or glide).
pub const PATH_AT_SPEED: [Unknown; N] = [
    Unknown::Alpha,
    Unknown::Beta,
    Unknown::Elevator,
    Unknown::Aileron,
    Unknown::Rudder,
    Unknown::FlightPath,
    Unknown::RotorSpeed,
    Unknown::Downwash,
];

/// Given angle of attack and throttle: speed and flight path (stall search).
pub const AT_ALPHA: [Unknown; N] = [
    Unknown::Beta,
    Unknown::Elevator,
    Unknown::Aileron,
    Unknown::Rudder,
    Unknown::Speed,
    Unknown::FlightPath,
    Unknown::RotorSpeed,
    Unknown::Downwash,
];

impl Flight {
    /// A starting point: small angle of attack, controls centred, engine turning.
    pub fn guess(altitude: f64, speed: f64, throttle: f64) -> Self {
        Self {
            altitude,
            speed,
            flight_path: 0.0,
            alpha: 0.05,
            beta: 0.0,
            deflections: ControlDeflections::default(),
            throttle,
            rotor_speed: 180.0,
            downwash: 0.0,
        }
    }

    pub fn get(&self, unknown: Unknown) -> f64 {
        match unknown {
            Unknown::Alpha => self.alpha,
            Unknown::Beta => self.beta,
            Unknown::Elevator => self.deflections.elevator,
            Unknown::Aileron => self.deflections.aileron,
            Unknown::Rudder => self.deflections.rudder,
            Unknown::Throttle => self.throttle,
            Unknown::Speed => self.speed,
            Unknown::FlightPath => self.flight_path,
            Unknown::RotorSpeed => self.rotor_speed,
            Unknown::Downwash => self.downwash,
        }
    }

    pub fn set(&mut self, unknown: Unknown, value: f64) {
        match unknown {
            Unknown::Alpha => self.alpha = value,
            Unknown::Beta => self.beta = value,
            Unknown::Elevator => self.deflections.elevator = value,
            Unknown::Aileron => self.deflections.aileron = value,
            Unknown::Rudder => self.deflections.rudder = value,
            Unknown::Throttle => self.throttle = value,
            Unknown::Speed => self.speed = value,
            Unknown::FlightPath => self.flight_path = value,
            Unknown::RotorSpeed => self.rotor_speed = value,
            Unknown::Downwash => self.downwash = value,
        }
    }

    /// Pitch angle for the flight path with wings level: sin γ = cos β sin(θ − α).
    pub fn pitch(&self) -> f64 {
        self.alpha
            + (self.flight_path.sin() / self.beta.cos())
                .clamp(-1.0, 1.0)
                .asin()
    }

    /// Rigid-body state of this flight (still air, heading north, no rotation).
    pub fn state(&self) -> RigidBodyState {
        let (a, b) = (self.alpha, self.beta);
        let mut state = RigidBodyState {
            position_ned: DVec3::new(0.0, 0.0, -self.altitude),
            attitude: DQuat::from_euler(EulerRot::ZYX, 0.0, self.pitch(), 0.0),
            velocity_body: DVec3::new(a.cos() * b.cos(), b.sin(), a.sin() * b.cos()) * self.speed,
            rotor_speed: self.rotor_speed,
            ..RigidBodyState::default()
        };
        state.aux[AUX_DOWNWASH] = self.downwash;
        state
    }

    /// Rate of climb, m/s.
    pub fn climb_rate(&self) -> f64 {
        self.speed * self.flight_path.sin()
    }
}

/// Scaled derivatives that vanish in steady straight flight.
pub fn residuals(aircraft: &Aircraft, env: &dyn Environment, flight: &Flight) -> [f64; N] {
    let state = flight.state();
    let loads = aircraft
        .evaluate_deflected(&state, &flight.deflections, flight.throttle, env)
        .loads;
    let d = state.derivative(aircraft.mass(), &loads);
    [
        d.velocity.x / G0,
        d.velocity.y / G0,
        d.velocity.z / G0,
        d.angular_velocity.x,
        d.angular_velocity.y,
        d.angular_velocity.z,
        d.rotor_speed / 100.0,
        d.aux[AUX_DOWNWASH],
    ]
}

fn norm(r: &[f64; N]) -> f64 {
    r.iter().map(|v| v * v).sum::<f64>().sqrt()
}

/// A solved steady flight.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Trim {
    pub flight: Flight,
    pub iterations: usize,
    /// Largest scaled residual left.
    pub residual: f64,
}

impl Trim {
    /// Pilot commands holding the trim (may be beyond ±1 or [0, 1]).
    pub fn commands(&self, aircraft: &Aircraft) -> PilotControls {
        let t = aircraft.travel();
        let d = self.flight.deflections;
        let ratio = |deflection: f64, travel: f64| {
            if travel > 0.0 {
                deflection / travel
            } else {
                0.0
            }
        };
        PilotControls {
            roll: ratio(d.aileron, t.aileron),
            pitch: ratio(d.elevator, t.elevator),
            yaw: ratio(d.rudder, t.rudder),
            throttle: self.flight.throttle,
        }
    }

    /// Whether the pilot can hold it: commands within their travel, throttle in [0, 1].
    pub fn feasible(&self, aircraft: &Aircraft) -> bool {
        let c = self.commands(aircraft);
        [c.roll, c.pitch, c.yaw].iter().all(|v| v.abs() <= 1.0) && (0.0..=1.0).contains(&c.throttle)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TrimError {
    /// Newton's method stalled: no steady flight near the guess (or none at all).
    NoConvergence { residual: f64, flight: Flight },
    /// The Jacobian is singular: an unknown has no effect.
    Singular { flight: Flight },
}

/// Solves the eight `free` unknowns from `guess`; the others keep their value.
///
/// Levenberg-Marquardt with Marquardt's scaling: close to Newton's method near the
/// solution, but robust where an unknown has no effect, as the throttle when the engine
/// is limited by full throttle at low speed (a real dead zone of the carburettor).
pub fn solve(
    aircraft: &Aircraft,
    env: &dyn Environment,
    guess: Flight,
    free: [Unknown; N],
) -> Result<Trim, TrimError> {
    let mut flight = guess;
    let mut r = residuals(aircraft, env, &flight);
    let mut damping = 1e-3;
    for iteration in 0..MAX_ITERATIONS {
        let worst = r.iter().fold(0.0_f64, |m, v| m.max(v.abs()));
        if worst < TOLERANCE {
            return Ok(Trim {
                flight,
                iterations: iteration,
                residual: worst,
            });
        }
        // Jacobian by central differences.
        let mut jacobian = DMatrix::<f64>::zeros(N, N);
        for (column, &unknown) in free.iter().enumerate() {
            let value = flight.get(unknown);
            let h = unknown.step(value);
            let (mut plus, mut minus) = (flight, flight);
            plus.set(unknown, value + h);
            minus.set(unknown, value - h);
            let (rp, rm) = (
                residuals(aircraft, env, &plus),
                residuals(aircraft, env, &minus),
            );
            for row in 0..N {
                jacobian[(row, column)] = (rp[row] - rm[row]) / (2.0 * h);
            }
        }
        if jacobian.iter().all(|v| v.is_finite()) && jacobian.column_iter().all(|c| c.norm() == 0.0)
        {
            return Err(TrimError::Singular { flight });
        }
        let rv = DVector::from_row_slice(&r);
        let normal = jacobian.transpose() * &jacobian;
        let gradient = jacobian.transpose() * rv;
        let current = norm(&r);
        // Raise the damping until a step lowers the residual.
        let mut accepted = false;
        for _ in 0..40 {
            let mut system = normal.clone();
            for i in 0..N {
                system[(i, i)] += damping * normal[(i, i)].max(1e-12);
            }
            let Some(step) = system.lu().solve(&(-&gradient)) else {
                damping *= 4.0;
                continue;
            };
            let mut trial = flight;
            for (i, &unknown) in free.iter().enumerate() {
                trial.set(unknown, flight.get(unknown) + step[i]);
            }
            let rt = residuals(aircraft, env, &trial);
            if rt.iter().all(|v| v.is_finite()) && norm(&rt) < current {
                flight = trial;
                r = rt;
                damping = (damping / 3.0).max(1e-12);
                accepted = true;
                break;
            }
            damping *= 4.0;
        }
        if !accepted {
            break;
        }
    }
    let worst = r.iter().fold(0.0_f64, |m, v| m.max(v.abs()));
    if worst < TOLERANCE {
        return Ok(Trim {
            flight,
            iterations: MAX_ITERATIONS,
            residual: worst,
        });
    }
    Err(TrimError::NoConvergence {
        residual: worst,
        flight,
    })
}

/// [`solve`] from `guess`, then, if it fails, from other throttle settings and rotor
/// speeds: the full-throttle dead zone can trap the solver far from the solution.
pub fn solve_robust(
    aircraft: &Aircraft,
    env: &dyn Environment,
    guess: Flight,
    free: [Unknown; N],
) -> Result<Trim, TrimError> {
    let first = solve(aircraft, env, guess, free);
    if first.is_ok() {
        return first;
    }
    let throttles: &[f64] = if free.contains(&Unknown::Throttle) {
        &[0.2, 0.4, 0.6]
    } else {
        &[guess.throttle]
    };
    for &throttle in throttles {
        for rotor_speed in [100.0, 140.0, 180.0, 220.0] {
            let start = Flight {
                throttle,
                rotor_speed,
                ..guess
            };
            if let Ok(trim) = solve(aircraft, env, start, free) {
                return Ok(trim);
            }
        }
    }
    first
}

#[cfg(test)]
pub(crate) mod tests {
    use std::path::PathBuf;

    use escadrille_data::aircraft::{Loading, load_aircraft};
    use escadrille_fdm::environment::StandardEnvironment;

    use super::*;

    /// The synthetic biplane of the data crate's tests (not historical data).
    pub(crate) fn biplane() -> Aircraft {
        let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates/data/tests/fixtures/assets");
        load_aircraft(&assets, "test_biplane")
            .unwrap()
            .build(&Loading { fuel_fraction: 1.0 })
            .unwrap()
            .aircraft
    }

    pub(crate) const STILL_AIR: StandardEnvironment = StandardEnvironment {
        wind_ned: DVec3::ZERO,
        ground_altitude: 0.0,
    };

    #[test]
    fn level_flight_trim_is_an_equilibrium_of_the_integrator() {
        let a = biplane();
        let trim = solve_robust(
            &a,
            &STILL_AIR,
            Flight::guess(1_000.0, 40.0, 0.5),
            LEVEL_AT_SPEED,
        )
        .unwrap();
        assert!(trim.residual < TOLERANCE, "{trim:?}");
        assert!(trim.feasible(&a), "{trim:?}");
        let f = trim.flight;
        assert!(f.alpha > 0.0 && f.alpha < 0.2, "{f:?}");
        assert!(f.throttle > 0.0 && f.throttle < 1.0, "{f:?}");
        // Propeller torque and P-factor need a little aileron and rudder, and a sideslip.
        assert!(f.deflections.aileron != 0.0 && f.deflections.rudder != 0.0);

        // Flown by the integrator with the trim commands, nothing moves.
        let controls = trim.commands(&a);
        let mut state = f.state();
        let start = state;
        for _ in 0..240 {
            state = a.step(&state, &controls, &STILL_AIR, 1.0 / 240.0);
        }
        assert!(
            (state.velocity_body - start.velocity_body).length() < 1e-6,
            "{state:?}"
        );
        assert!(state.angular_velocity_body.length() < 1e-6);
        assert!((state.rotor_speed - start.rotor_speed).abs() < 1e-6);
    }

    #[test]
    fn full_throttle_gives_a_faster_level_flight() {
        let a = biplane();
        let cruise = solve_robust(
            &a,
            &STILL_AIR,
            Flight::guess(1_000.0, 40.0, 0.5),
            LEVEL_AT_SPEED,
        )
        .unwrap();
        let mut guess = cruise.flight;
        guess.throttle = 1.0;
        guess.speed = 70.0;
        let fast = solve(&a, &STILL_AIR, guess, SPEED_AT_THROTTLE).unwrap();
        assert!(fast.flight.speed > 40.0, "{fast:?}");
        // At that speed, level flight needs the full-throttle power: the trimmed
        // throttle may be lower, the engine being already saturated at that rotor speed
        // (the carburettor's dead zone), but the power must be the same.
        let mut check = fast.flight;
        check.throttle = 0.5;
        let again = solve_robust(&a, &STILL_AIR, check, LEVEL_AT_SPEED).unwrap();
        let engine = &a.propulsion().unwrap().engine;
        let sigma = escadrille_fdm::atmosphere::isa(1_000.0).sigma();
        let rotor = again.flight.rotor_speed;
        let trimmed = engine
            .torques(rotor, again.flight.throttle, sigma)
            .combustion;
        let full = engine.torques(rotor, 1.0, sigma).combustion;
        assert!((trimmed - full).abs() < 1e-6 * full, "{again:?}");
        assert!((again.flight.speed - fast.flight.speed).abs() < 1e-9);
    }

    #[test]
    fn climb_at_full_throttle_and_glide_at_idle() {
        let a = biplane();
        let mut guess = Flight::guess(1_000.0, 40.0, 1.0);
        guess.flight_path = 0.05;
        let climb = solve(&a, &STILL_AIR, guess, PATH_AT_SPEED).unwrap();
        assert!(climb.flight.climb_rate() > 0.0, "{climb:?}");
        guess.throttle = 0.0;
        guess.flight_path = -0.05;
        let glide = solve(&a, &STILL_AIR, guess, PATH_AT_SPEED).unwrap();
        assert!(glide.flight.climb_rate() < 0.0, "{glide:?}");
    }
}
