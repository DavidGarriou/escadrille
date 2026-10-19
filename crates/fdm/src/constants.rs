//! Physical constants of the International Standard Atmosphere (ISO 2533:1975).

/// Standard acceleration of gravity, m/s².
pub const G0: f64 = 9.806_65;

/// Specific gas constant of dry air, J/(kg·K).
pub const R_AIR: f64 = 287.052_87;

/// Ratio of specific heats of air (dimensionless).
pub const GAMMA_AIR: f64 = 1.4;

/// Sea-level standard temperature, K.
pub const T0: f64 = 288.15;

/// Sea-level standard pressure, Pa.
pub const P0: f64 = 101_325.0;

/// Sea-level standard density, kg/m³.
pub const RHO0: f64 = 1.225;

/// Temperature lapse rate in the troposphere (0 to 11 km), K/m.
pub const LAPSE_RATE: f64 = -0.0065;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sea_level_density_matches_ideal_gas_law() {
        let rho = P0 / (R_AIR * T0);
        assert!((rho - RHO0).abs() < 1e-3, "rho from p/(R T) = {rho}");
    }

    #[test]
    fn sea_level_speed_of_sound_is_standard() {
        // ISO 2533 sea-level speed of sound: 340.294 m/s.
        let a = (GAMMA_AIR * R_AIR * T0).sqrt();
        assert!((a - 340.294).abs() < 1e-3, "a0 = {a}");
    }
}
