//! Scenarios (`assets/scenarios/<name>.ron`): where and how a flight starts: in steady
//! flight (the test flight of plan `docs/plans/m2-vol-essai.md`, the final approach of
//! plan `docs/plans/m2-train-sol.md`) or at rest on the ground. The states are the flight
//! model's own, solved by `fdm_lab` (the game cannot solve them: `nalgebra` belongs to the
//! tools, CLAUDE.md) and checked there by tests, so that the files follow the flight
//! model when it changes.

use std::path::Path;

use escadrille_fdm::aircraft::{AUX_DOWNWASH, PilotControls};
use escadrille_fdm::rigid_body::RigidBodyState;
use glam::{DQuat, DVec3, EulerRot};
use serde::Deserialize;

use crate::units::{Angle, Length, RotationSpeed, Speed};
use crate::{DataError, load_checked};

/// Steady straight flight, wings level, at the scenario's point.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct EquilibriumSpec {
    pub alpha: Angle,
    pub beta: Angle,
    /// Pilot commands holding it: stick and rudder from −1 to 1, throttle from 0 to 1.
    pub roll: f64,
    pub pitch: f64,
    pub yaw: f64,
    pub throttle: f64,
    pub rotor_speed: RotationSpeed,
    /// Downwash at the tail (the flight model's lagged state).
    pub downwash: Angle,
}

/// At rest on the ground, wings level, engine at idle.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct RestSpec {
    /// Height of the centre of gravity above the ground.
    pub height: Length,
    pub pitch: Angle,
    pub rotor_speed: RotationSpeed,
    /// Elevator command the stick starts around: the one of the climb after the take-off.
    pub elevator: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum StartSpec {
    Flight {
        altitude: Length,
        /// True airspeed.
        speed: Speed,
        /// Angle of the path to the horizon, level when left out.
        #[serde(default)]
        flight_path: Option<Angle>,
        equilibrium: EquilibriumSpec,
    },
    Ground(RestSpec),
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename = "Scenario")]
pub struct ScenarioFile {
    /// Identifier of the aircraft (directory under `assets/aircraft`).
    pub aircraft: String,
    /// Share of the fuel tanks filled.
    pub fuel_fraction: f64,
    /// Heading, from north towards east.
    pub heading: Angle,
    /// North and east of the origin of the map, m.
    #[serde(default)]
    pub position_m: [f64; 2],
    pub start: StartSpec,
}

impl ScenarioFile {
    /// Rigid-body state of the start: steady flight in still air, at the equilibrium's
    /// incidence and sideslip, or at rest on the ground.
    pub fn initial_state(&self) -> RigidBodyState {
        let [north, east] = self.position_m;
        let heading = self.heading.si();
        match self.start {
            StartSpec::Flight {
                altitude,
                speed,
                flight_path,
                equilibrium: e,
            } => {
                let (alpha, beta) = (e.alpha.si(), e.beta.si());
                let path = flight_path.map_or(0.0, Angle::si);
                let mut state = RigidBodyState {
                    position_ned: DVec3::new(north, east, -altitude.si()),
                    attitude: DQuat::from_euler(EulerRot::ZYX, heading, alpha + path, 0.0),
                    velocity_body: DVec3::new(
                        alpha.cos() * beta.cos(),
                        beta.sin(),
                        alpha.sin() * beta.cos(),
                    ) * speed.si(),
                    rotor_speed: e.rotor_speed.si(),
                    ..RigidBodyState::default()
                };
                state.aux[AUX_DOWNWASH] = e.downwash.si();
                state
            }
            StartSpec::Ground(r) => RigidBodyState {
                position_ned: DVec3::new(north, east, -r.height.si()),
                attitude: DQuat::from_euler(EulerRot::ZYX, heading, r.pitch.si(), 0.0),
                rotor_speed: r.rotor_speed.si(),
                ..RigidBodyState::default()
            },
        }
    }

    /// The pilot commands of the start: those holding the equilibrium, or the throttle
    /// closed on the ground.
    pub fn commands(&self) -> PilotControls {
        match self.start {
            StartSpec::Flight { equilibrium: e, .. } => PilotControls {
                roll: e.roll,
                pitch: e.pitch,
                yaw: e.yaw,
                throttle: e.throttle,
            },
            StartSpec::Ground(r) => PilotControls {
                pitch: r.elevator,
                ..PilotControls::default()
            },
        }
    }
}

/// Loads `assets/scenarios/<name>.ron`.
pub fn load_scenario(assets: &Path, name: &str) -> Result<ScenarioFile, DataError> {
    let path = assets.join("scenarios").join(format!("{name}.ron"));
    let file: ScenarioFile = load_checked(&path)?;
    let start_valid = match &file.start {
        StartSpec::Flight {
            altitude,
            speed,
            equilibrium: e,
            ..
        } => {
            altitude.si() > 0.0
                && speed.si() > 0.0
                && [e.roll, e.pitch, e.yaw].iter().all(|c| c.abs() <= 1.0)
                && (0.0..=1.0).contains(&e.throttle)
                && e.rotor_speed.si() >= 0.0
        }
        StartSpec::Ground(r) => {
            r.height.si() > 0.0 && r.elevator.abs() <= 1.0 && r.rotor_speed.si() >= 0.0
        }
    };
    let valid = (0.0..=1.0).contains(&file.fuel_fraction) && start_valid;
    if !valid {
        return Err(DataError::Invalid {
            origin: path.display().to_string(),
            message: "fuel fraction and throttle from 0 to 1, commands from −1 to 1, \
                      positive altitude, speed and rotor speed"
                .to_owned(),
        });
    }
    Ok(file)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn assets() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets")
    }

    #[test]
    fn the_test_flight_starts_level_at_1000_m_and_160_kmh() {
        let s = load_scenario(&assets(), "vol-essai").unwrap();
        assert_eq!(s.aircraft, "spad7");
        let state = s.initial_state();
        assert!((state.position_ned.z + 1000.0).abs() < 1e-9);
        assert!((state.velocity_body.length() - 160.0 / 3.6).abs() < 1e-9);
        // Heading east: the nose (body x) points east (NED y), level.
        let nose = state.attitude * DVec3::X;
        assert!(nose.y > 0.999, "{nose}");
        // In level flight the velocity is horizontal.
        let velocity = state.attitude * state.velocity_body;
        assert!(velocity.z.abs() < 1e-9, "{velocity}");
    }

    #[test]
    fn on_the_field_the_aircraft_stands_still_nose_up_throttle_closed() {
        let s = load_scenario(&assets(), "terrain").unwrap();
        let state = s.initial_state();
        assert_eq!(state.velocity_body, DVec3::ZERO);
        let nose = state.attitude * DVec3::X;
        assert!(nose.y > 0.9 && nose.z < -0.2, "{nose}");
        assert_eq!(s.commands().throttle, 0.0);
    }

    #[test]
    fn the_final_approach_comes_down_towards_the_field() {
        let s = load_scenario(&assets(), "finale").unwrap();
        let state = s.initial_state();
        let velocity = state.attitude * state.velocity_body;
        assert!(velocity.z > 0.0 && velocity.y > 30.0, "{velocity}");
        assert!(state.position_ned.y < 0.0, "{}", state.position_ned);
    }
}
