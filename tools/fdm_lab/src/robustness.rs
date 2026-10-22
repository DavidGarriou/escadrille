//! Robustness scenarios of `essais.ron` (M1 plan, step 7; skill `simulation-vol`,
//! « Validation »): scripted flights through the corners of the envelope. They pass when
//! the simulation stays finite and within physical bounds; what the aircraft does (spin
//! turns, recovery, top speed) is reported, not judged: that is for the qualitative tests
//! drawn from the pilots' testimonies.

use escadrille_data::essais::Robustness;
use escadrille_fdm::aircraft::{Aircraft, PilotControls};
use escadrille_fdm::environment::Environment;
use escadrille_fdm::rigid_body::RigidBodyState;
use glam::{DQuat, DVec3, EulerRot};
use std::cell::Cell;

use crate::performance::max_level_speed;

/// Integration step, s (ADR 0005: 120 Hz with two substeps).
const DT: f64 = 1.0 / 240.0;

/// Starting altitude of the scenarios, m.
const ALTITUDE: f64 = 2_000.0;

/// Physical bounds: beyond them, the simulation has diverged.
const MAX_SPEED: f64 = 300.0;
const MAX_RATE: f64 = 30.0;
const MAX_ROTOR_SPEED: f64 = 1_000.0;

#[derive(Debug, Clone, PartialEq)]
pub struct RobustnessOutcome {
    /// Scenario name, in French.
    pub name: String,
    /// Finite and within bounds throughout.
    pub passed: bool,
    /// What happened, in French.
    pub details: String,
}

/// Flight record of a scenario.
#[derive(Debug, Clone, Copy, Default)]
struct Record {
    finite: bool,
    bounded: bool,
    max_speed: f64,
    max_rate: f64,
    max_rotor: f64,
    /// Accumulated heading change, rad.
    yaw: f64,
    /// Largest angle of attack of the body, rad.
    max_alpha: f64,
    /// Largest aerodynamic load factor (aerodynamic force over weight).
    max_load_factor: f64,
    /// Time spent stalled and yawing (angle of attack above [`SPIN_ALPHA`], yaw rate
    /// above [`SPIN_YAW_RATE`]), s.
    spinning_time: f64,
}

/// Angle of attack above which the wings are stalled in a spin, rad (E:, well past the
/// stall of the period's thin sections).
const SPIN_ALPHA: f64 = 20.0 * std::f64::consts::PI / 180.0;

/// Yaw rate of a spin, rad/s (E:).
const SPIN_YAW_RATE: f64 = 0.5;

/// Time stalled and yawing that makes a spin, s (E:).
const SPIN_TIME: f64 = 3.0;

fn fly(
    aircraft: &Aircraft,
    env: &dyn Environment,
    mut state: RigidBodyState,
    seconds: f64,
    controls: impl Fn(f64, &RigidBodyState) -> PilotControls,
) -> (RigidBodyState, Record) {
    let mut record = Record {
        finite: true,
        bounded: true,
        ..Record::default()
    };
    let steps = (seconds / DT).round() as u32;
    let mut last_heading = heading(&state);
    for i in 0..steps {
        let c = controls(f64::from(i) * DT, &state);
        state = aircraft.step(&state, &c, env, DT);
        let values = [
            state.velocity_body.to_array(),
            state.angular_velocity_body.to_array(),
            state.position_ned.to_array(),
        ];
        if !(values.iter().flatten().all(|v| v.is_finite())
            && state.rotor_speed.is_finite()
            && state.aux.iter().all(|v| v.is_finite()))
        {
            record.finite = false;
            break;
        }
        let speed = state.velocity_body.length();
        let rate = state.angular_velocity_body.length();
        record.max_speed = record.max_speed.max(speed);
        record.max_rate = record.max_rate.max(rate);
        record.max_rotor = record.max_rotor.max(state.rotor_speed.abs());
        record.bounded &=
            speed < MAX_SPEED && rate < MAX_RATE && state.rotor_speed.abs() < MAX_ROTOR_SPEED;
        let new_heading = heading(&state);
        record.yaw += wrap(new_heading - last_heading);
        last_heading = new_heading;
        let v = state.velocity_body;
        let alpha = v.z.atan2(v.x);
        record.max_alpha = record.max_alpha.max(alpha);
        let force = aircraft.loads(&state, &c, env).wrench.force;
        let weight = aircraft.mass().mass() * escadrille_fdm::constants::G0;
        record.max_load_factor = record.max_load_factor.max(force.length() / weight);
        if alpha > SPIN_ALPHA && state.angular_velocity_body.z.abs() > SPIN_YAW_RATE {
            record.spinning_time += DT;
        }
    }
    (state, record)
}

fn at(attitude: DQuat, velocity_body: DVec3, rotor_speed: f64) -> RigidBodyState {
    RigidBodyState {
        position_ned: DVec3::new(0.0, 0.0, -ALTITUDE),
        attitude,
        velocity_body,
        rotor_speed,
        ..RigidBodyState::default()
    }
}

fn heading(state: &RigidBodyState) -> f64 {
    state.attitude.to_euler(EulerRot::ZYX).0
}

/// Wraps an angle difference into (−π, π].
fn wrap(delta: f64) -> f64 {
    escadrille_fdm::aero::polar::wrap_angle(delta)
}

fn pitch_of(state: &RigidBodyState) -> f64 {
    state.attitude.to_euler(EulerRot::ZYX).1
}

const IDLE: PilotControls = PilotControls {
    roll: 0.0,
    pitch: 0.0,
    yaw: 0.0,
    throttle: 0.0,
};

/// Runs one scenario.
pub fn run(aircraft: &Aircraft, env: &dyn Environment, scenario: &Robustness) -> RobustnessOutcome {
    let (name, record, details) = match *scenario {
        Robustness::ZeroAirspeed => {
            let start = at(DQuat::IDENTITY, DVec3::ZERO, 60.0);
            let (end, record) = fly(aircraft, env, start, 15.0, |_, _| IDLE);
            let speed = end.velocity_body.length();
            (
                "Vitesse nulle (lâché à l'arrêt, ailes à plat)",
                record,
                format!(
                    "vitesse après 15 s : {:.1} km/h ; assiette {:.0}°",
                    speed * 3.6,
                    pitch_of(&end).to_degrees()
                ),
            )
        }
        Robustness::TailSlide => {
            let attitude = DQuat::from_euler(EulerRot::ZYX, 0.0, 80f64.to_radians(), 0.0);
            let start = at(attitude, DVec3::new(-5.0, 0.0, 0.0), 120.0);
            let (end, record) = fly(aircraft, env, start, 15.0, |_, _| IDLE);
            (
                "Glissade sur la queue (cabré à 80°, recule à 5 m/s)",
                record,
                format!(
                    "assiette après 15 s : {:.0}° ; vitesse {:.1} km/h",
                    pitch_of(&end).to_degrees(),
                    end.velocity_body.length() * 3.6
                ),
            )
        }
        Robustness::SpinEntryRecovery { turns } => {
            // Slow, nose high, then full back stick and full right rudder until the
            // requested turns of heading (or 20 s), then recovery: opposite rudder, stick
            // forward.
            let attitude = DQuat::from_euler(EulerRot::ZYX, 0.0, 15f64.to_radians(), 0.0);
            let start = at(attitude, DVec3::new(25.0, 0.0, 5.0), 150.0);
            let target = f64::from(turns) * 2.0 * std::f64::consts::PI;
            let yawed = Cell::new(0.0);
            let last = Cell::new(heading(&start));
            let recovery_at = Cell::new(None::<f64>);
            let yaw_at_recovery = Cell::new(0.0);
            let (end, record) = fly(aircraft, env, start, 40.0, |t, s| {
                let h = heading(s);
                yawed.set(yawed.get() + wrap(h - last.get()));
                last.set(h);
                if recovery_at.get().is_none() && (yawed.get().abs() >= target || t >= 20.0) {
                    recovery_at.set(Some(t));
                    yaw_at_recovery.set(yawed.get());
                }
                if recovery_at.get().is_none() {
                    PilotControls {
                        pitch: 1.0,
                        yaw: 1.0,
                        ..IDLE
                    }
                } else {
                    PilotControls {
                        pitch: -0.5,
                        yaw: -1.0,
                        ..IDLE
                    }
                }
            });
            let turns = |yaw: f64| yaw.abs() / (2.0 * std::f64::consts::PI);
            // A spin is stalled and yawing for a while; otherwise the aircraft only
            // tightened into a spiral dive (aerodynamicist review).
            let what = if record.spinning_time >= SPIN_TIME {
                format!(
                    "**vrille établie** ({:.1} s décroché en lacet)",
                    record.spinning_time
                )
            } else {
                format!(
                    "pas de vrille : spirale engagée ou décrochage sans autorotation ({:.1} s décroché en lacet)",
                    record.spinning_time
                )
            };
            (
                "Vrille : entrée puis sortie",
                record,
                format!(
                    "{what} ; {:.1} tours de cap avant la sortie, commandée à {:.1} s ; incidence maximale {:.0}°, facteur de charge maximal {:.1} ; à la fin : rotation {:.2} rad/s, assiette {:.0}°, vitesse {:.0} km/h",
                    turns(yaw_at_recovery.get()),
                    recovery_at.get().unwrap_or(f64::NAN),
                    record.max_alpha.to_degrees(),
                    record.max_load_factor,
                    end.angular_velocity_body.length(),
                    pitch_of(&end).to_degrees(),
                    end.velocity_body.length() * 3.6
                ),
            )
        }
        Robustness::Inverted { seconds } => {
            let speed =
                max_level_speed(aircraft, env, ALTITUDE).map_or(45.0, |t| 0.8 * t.flight.speed);
            let attitude = DQuat::from_euler(EulerRot::ZYX, 0.0, 0.0, std::f64::consts::PI);
            let start = at(attitude, DVec3::new(speed, 0.0, 0.0), 180.0);
            let push = PilotControls {
                pitch: -0.6,
                throttle: 1.0,
                ..IDLE
            };
            let (end, record) = fly(aircraft, env, start, seconds, |_, _| push);
            (
                "Vol sur le dos",
                record,
                format!(
                    "après {seconds:.0} s : vitesse {:.1} km/h, altitude perdue {:.0} m",
                    end.velocity_body.length() * 3.6,
                    ALTITUDE + end.position_ned.z
                ),
            )
        }
        Robustness::MaxDive => {
            // Variant of the RAE (historian): "vertical" dives were 45 to 60°.
            let attitude = DQuat::from_euler(EulerRot::ZYX, 0.0, -60f64.to_radians(), 0.0);
            let start = at(attitude, DVec3::new(50.0, 0.0, 0.0), 200.0);
            let full = PilotControls {
                throttle: 1.0,
                ..IDLE
            };
            let mut start = start;
            start.position_ned.z = -4_000.0;
            let (_, record) = fly(aircraft, env, start, 25.0, |_, _| full);
            (
                "Piqué à 60°, pleins gaz",
                record,
                format!(
                    "vitesse maximale {:.0} km/h ; régime maximal {:.0} tr/min",
                    record.max_speed * 3.6,
                    record.max_rotor * 30.0 / std::f64::consts::PI
                ),
            )
        }
    };
    let passed = record.finite && record.bounded;
    let details = if record.finite {
        details
    } else {
        "**valeurs non finies (NaN)**".to_owned()
    };
    RobustnessOutcome {
        name: name.to_owned(),
        passed,
        details,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::trim::tests::{STILL_AIR, biplane};

    #[test]
    fn every_scenario_stays_finite_and_bounded() {
        let a = biplane();
        for scenario in [
            Robustness::ZeroAirspeed,
            Robustness::TailSlide,
            Robustness::SpinEntryRecovery { turns: 3 },
            Robustness::Inverted { seconds: 10.0 },
            Robustness::MaxDive,
        ] {
            let outcome = run(&a, &STILL_AIR, &scenario);
            assert!(outcome.passed, "{outcome:?}");
        }
    }

    #[test]
    fn dropped_at_rest_it_falls_nose_down_and_gathers_speed() {
        let a = biplane();
        let start = at(DQuat::IDENTITY, DVec3::ZERO, 60.0);
        let (end, record) = fly(&a, &STILL_AIR, start, 15.0, |_, _| IDLE);
        assert!(record.finite && record.bounded);
        assert!(end.velocity_body.length() > 15.0, "{end:?}");
        assert!(end.velocity_body.x > 0.0, "flying forwards again: {end:?}");
    }
}
