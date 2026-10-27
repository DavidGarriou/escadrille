//! Scenarios (`assets/scenarios/<name>.ron`): where and how a flight starts. The first is
//! the test flight of plan `docs/plans/m2-vol-essai.md`: in the air, in steady level
//! flight. The equilibrium is the flight model's own, solved by `fdm_lab` (the game
//! cannot solve it: `nalgebra` belongs to the tools, CLAUDE.md) and checked there by a
//! test, so that the file follows the flight model when it changes.

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

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename = "Scenario")]
pub struct ScenarioFile {
    /// Identifier of the aircraft (directory under `assets/aircraft`).
    pub aircraft: String,
    /// Share of the fuel tanks filled.
    pub fuel_fraction: f64,
    /// Heading, from north towards east.
    pub heading: Angle,
    pub altitude: Length,
    /// True airspeed.
    pub speed: Speed,
    pub equilibrium: EquilibriumSpec,
}

impl ScenarioFile {
    /// Rigid-body state of the start, over the origin of the NED frame: level flight in
    /// still air, at the equilibrium's incidence and sideslip.
    pub fn initial_state(&self) -> RigidBodyState {
        let e = &self.equilibrium;
        let (alpha, beta) = (e.alpha.si(), e.beta.si());
        let mut state = RigidBodyState {
            position_ned: DVec3::new(0.0, 0.0, -self.altitude.si()),
            attitude: DQuat::from_euler(EulerRot::ZYX, self.heading.si(), alpha, 0.0),
            velocity_body: DVec3::new(
                alpha.cos() * beta.cos(),
                beta.sin(),
                alpha.sin() * beta.cos(),
            ) * self.speed.si(),
            rotor_speed: e.rotor_speed.si(),
            ..RigidBodyState::default()
        };
        state.aux[AUX_DOWNWASH] = e.downwash.si();
        state
    }

    /// The pilot commands holding the equilibrium.
    pub fn commands(&self) -> PilotControls {
        let e = &self.equilibrium;
        PilotControls {
            roll: e.roll,
            pitch: e.pitch,
            yaw: e.yaw,
            throttle: e.throttle,
        }
    }
}

/// Loads `assets/scenarios/<name>.ron`.
pub fn load_scenario(assets: &Path, name: &str) -> Result<ScenarioFile, DataError> {
    let path = assets.join("scenarios").join(format!("{name}.ron"));
    let file: ScenarioFile = load_checked(&path)?;
    let e = &file.equilibrium;
    let valid = (0.0..=1.0).contains(&file.fuel_fraction)
        && file.altitude.si() > 0.0
        && file.speed.si() > 0.0
        && [e.roll, e.pitch, e.yaw].iter().all(|c| c.abs() <= 1.0)
        && (0.0..=1.0).contains(&e.throttle)
        && e.rotor_speed.si() >= 0.0;
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
}
