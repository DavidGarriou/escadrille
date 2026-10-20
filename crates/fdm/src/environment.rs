//! What the flight model needs to know about the world, provided by the caller
//! (the game, the AI, `fdm_lab`, the tests) through the [`Environment`] trait.
//!
//! Positions are in the local NED frame (x north, y east, z down) whose origin is
//! at mean sea level below the map reference point: altitude = −z.

use glam::DVec3;

use crate::atmosphere::{AtmosphereState, isa};

pub trait Environment {
    /// Air properties at a NED position.
    fn atmosphere(&self, position_ned: DVec3) -> AtmosphereState;

    /// Wind velocity at a NED position, in the NED frame, m/s (where the air goes).
    fn wind_ned(&self, position_ned: DVec3) -> DVec3;

    /// Ground altitude above mean sea level below a NED position, m.
    fn ground_altitude(&self, position_ned: DVec3) -> f64;
}

/// Altitude above mean sea level of a NED position, m.
pub fn altitude(position_ned: DVec3) -> f64 {
    -position_ned.z
}

/// ISA atmosphere, uniform wind, flat ground: the reference environment of the flight tests.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct StandardEnvironment {
    pub wind_ned: DVec3,
    pub ground_altitude: f64,
}

impl Environment for StandardEnvironment {
    fn atmosphere(&self, position_ned: DVec3) -> AtmosphereState {
        isa(altitude(position_ned))
    }

    fn wind_ned(&self, _position_ned: DVec3) -> DVec3 {
        self.wind_ned
    }

    fn ground_altitude(&self, _position_ned: DVec3) -> f64 {
        self.ground_altitude
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn altitude_is_minus_down_coordinate() {
        assert_eq!(altitude(DVec3::new(10.0, -3.0, -2_000.0)), 2_000.0);
    }

    #[test]
    fn standard_environment_uses_isa_and_uniform_wind() {
        let env = StandardEnvironment {
            wind_ned: DVec3::new(0.0, 5.0, 0.0),
            ground_altitude: 30.0,
        };
        let p = DVec3::new(100.0, 200.0, -1_500.0);
        assert_eq!(env.atmosphere(p), isa(1_500.0));
        assert_eq!(env.wind_ned(p), DVec3::new(0.0, 5.0, 0.0));
        assert_eq!(env.ground_altitude(p), 30.0);
    }
}
