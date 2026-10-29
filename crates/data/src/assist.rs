//! Gains of the flight assists (`assets/aircraft/<id>/assist_gains.ron`; plan
//! `docs/plans/m2-aides-pilotage.md`, step 1; skill `realisme`). They are computed by
//! `fdm_lab assist <id>` from the linearised flight model, never set by hand, and a test
//! of `fdm_lab` checks that the file still follows the model.
//!
//! Each row holds at its equivalent airspeed. Its gains are multiplied by its dynamic
//! pressure: the game interpolates them in equivalent airspeed and divides them by its
//! own dynamic pressure, so that they hold at any speed and altitude.

use std::path::Path;

use serde::Deserialize;

use crate::units::{Angle, Speed};
use crate::{DataError, load_checked};

/// The assists' figures at one speed.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct AssistRow {
    /// Equivalent airspeed.
    pub speed: Speed,
    /// Load factor per radian of incidence, per pascal of dynamic pressure.
    pub load_per_alpha: f64,
    /// Incidence of zero load factor.
    pub zero_load_alpha: Angle,
    /// Elevator command holding the zero-load incidence in steady flight.
    pub elevator_at_zero_load: f64,
    /// Elevator command per radian of incidence in steady flight.
    pub elevator_per_alpha: f64,
    /// Elevator command per rad/s of steady pitch rate (a pull-up, a turn): the pitch
    /// damping to overcome.
    pub elevator_per_pitch_rate: f64,
    /// Rudder command cancelling the yawing moment of a roll rate (per rad/s) and of the
    /// aileron (per unit of command): the adverse yaw a pilot meets with his feet.
    pub rudder_per_roll_rate: f64,
    pub rudder_per_aileron: f64,
    /// Aileron command per radian of bank, per rad/s of roll rate and per rad·s of the
    /// bank's integral, × Pa (u = −K x).
    pub roll: [f64; 3],
    /// Elevator command per radian of incidence, per rad/s of pitch rate and per rad·s
    /// of the incidence's integral, × Pa (all negative feedbacks: u = −K x).
    pub pitch: [f64; 3],
    /// Rudder command per radian of sideslip, per rad/s of yaw rate and per rad·s of the
    /// sideslip's integral, × Pa (u = −K x).
    pub yaw: [f64; 3],
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename = "AssistGains")]
pub struct AssistFile {
    /// Identifier of the aircraft (directory under `assets/aircraft`).
    pub aircraft: String,
    /// Incidence of the airframe's maximum lift in steady flight, the lower of idle and
    /// full throttle.
    pub stall_alpha: Angle,
    /// From the lowest speed to the highest.
    pub rows: Vec<AssistRow>,
}

/// Loads `assets/aircraft/<id>/assist_gains.ron`, if the aircraft has one.
pub fn load_assist_gains(assets: &Path, id: &str) -> Result<Option<AssistFile>, DataError> {
    let path = assets.join("aircraft").join(id).join("assist_gains.ron");
    if !path.exists() {
        return Ok(None);
    }
    let file: AssistFile = load_checked(&path)?;
    let finite = file.rows.iter().all(|r| {
        [
            r.speed.si(),
            r.load_per_alpha,
            r.zero_load_alpha.si(),
            r.elevator_at_zero_load,
            r.elevator_per_alpha,
            r.elevator_per_pitch_rate,
            r.rudder_per_roll_rate,
            r.rudder_per_aileron,
        ]
        .iter()
        .chain(&r.roll)
        .chain(&r.pitch)
        .chain(&r.yaw)
        .all(|v| v.is_finite())
    });
    let increasing = file
        .rows
        .windows(2)
        .all(|w| w[0].speed.si() < w[1].speed.si());
    let alpha = file.stall_alpha.si();
    if file.aircraft != id
        || file.rows.len() < 2
        || !finite
        || !increasing
        || !(0.0..0.6).contains(&alpha)
        || file.rows.iter().any(|r| r.load_per_alpha <= 0.0)
    {
        return Err(DataError::Invalid {
            origin: path.display().to_string(),
            message: format!(
                "assist gains of {id}: at least two rows of finite values, speeds \
                 increasing, positive load per incidence, stall incidence from 0 to 0.6 rad"
            ),
        });
    }
    Ok(Some(file))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn assets() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets")
    }

    #[test]
    fn the_spad_has_its_assist_gains() {
        let gains = load_assist_gains(&assets(), "spad7").unwrap().unwrap();
        assert_eq!(gains.aircraft, "spad7");
        assert!(gains.stall_alpha.si() > 0.1, "{:?}", gains.stall_alpha);
        // An aircraft without the file has no assist, and no error.
        assert!(load_assist_gains(&assets(), "nothing").unwrap().is_none());
    }
}
