//! International Standard Atmosphere (ISO 2533:1975), troposphere and lower stratosphere.
//!
//! Altitudes are treated as geopotential altitudes; the difference with geometric
//! altitude stays below 0.1 % up to 6 km, the flight envelope of a 1917 fighter.

use crate::constants::{G0, GAMMA_AIR, LAPSE_RATE, P0, R_AIR, RHO0, T0};

/// Altitude of the tropopause, m.
pub const TROPOPAUSE_ALTITUDE: f64 = 11_000.0;

/// Upper limit of the model (top of the isothermal layer), m.
pub const MAX_ALTITUDE: f64 = 20_000.0;

/// Air properties at a given altitude.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AtmosphereState {
    /// Static temperature, K.
    pub temperature: f64,
    /// Static pressure, Pa.
    pub pressure: f64,
    /// Density, kg/m³.
    pub density: f64,
    /// Speed of sound, m/s.
    pub speed_of_sound: f64,
}

impl AtmosphereState {
    /// Density ratio σ = ρ / ρ0.
    pub fn sigma(&self) -> f64 {
        self.density / RHO0
    }
}

/// Exponent g0 / (R · |L|) of the tropospheric pressure law (≈ 5.2559).
fn pressure_exponent() -> f64 {
    G0 / (R_AIR * -LAPSE_RATE)
}

/// Standard atmosphere at `altitude` metres above mean sea level.
///
/// Below sea level the tropospheric law is extrapolated; above [`MAX_ALTITUDE`]
/// the value at that altitude is returned (no 1917 aircraft flies there).
pub fn isa(altitude: f64) -> AtmosphereState {
    let h = altitude.min(MAX_ALTITUDE);
    let t11 = T0 + LAPSE_RATE * TROPOPAUSE_ALTITUDE;
    let (temperature, pressure) = if h <= TROPOPAUSE_ALTITUDE {
        let t = T0 + LAPSE_RATE * h;
        (t, P0 * (t / T0).powf(pressure_exponent()))
    } else {
        let p11 = P0 * (t11 / T0).powf(pressure_exponent());
        (
            t11,
            p11 * (-G0 * (h - TROPOPAUSE_ALTITUDE) / (R_AIR * t11)).exp(),
        )
    };
    AtmosphereState {
        temperature,
        pressure,
        density: pressure / (R_AIR * temperature),
        speed_of_sound: (GAMMA_AIR * R_AIR * temperature).sqrt(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64, rel: f64) -> bool {
        ((a - b) / b).abs() < rel
    }

    #[test]
    fn sea_level_matches_standard_values() {
        let a = isa(0.0);
        assert_eq!(a.temperature, T0);
        assert_eq!(a.pressure, P0);
        assert!(close(a.density, RHO0, 1e-4), "{a:?}");
        assert!(close(a.sigma(), 1.0, 1e-4));
    }

    #[test]
    fn tropopause_matches_iso_2533() {
        // ISO 2533 at 11 000 m: 216.65 K, 22 632 Pa, 0.36392 kg/m³.
        let a = isa(TROPOPAUSE_ALTITUDE);
        assert!(close(a.temperature, 216.65, 1e-9), "{a:?}");
        assert!(close(a.pressure, 22_632.0, 1e-4), "{a:?}");
        assert!(close(a.density, 0.363_92, 1e-3), "{a:?}");
    }

    #[test]
    fn pressure_exponent_is_standard() {
        assert!((pressure_exponent() - 5.2559).abs() < 1e-3);
    }

    #[test]
    fn stratosphere_is_continuous_and_isothermal() {
        let below = isa(TROPOPAUSE_ALTITUDE - 1e-6);
        let above = isa(TROPOPAUSE_ALTITUDE + 1e-6);
        assert!(close(below.pressure, above.pressure, 1e-9));
        assert_eq!(isa(15_000.0).temperature, isa(12_000.0).temperature);
    }

    #[test]
    fn density_decreases_monotonically() {
        let mut previous = isa(-500.0).density;
        for i in 1..=200 {
            let d = isa(-500.0 + f64::from(i) * 100.0).density;
            assert!(d < previous);
            previous = d;
        }
    }

    #[test]
    fn altitude_is_clamped_above_model_limit() {
        assert_eq!(isa(30_000.0), isa(MAX_ALTITUDE));
    }
}
