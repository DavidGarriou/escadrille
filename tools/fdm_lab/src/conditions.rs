//! Flight-test conditions other than the standard atmosphere, to bracket period figures
//! whose conditions are unknown (plan `docs/plans/m1-ecarts-spad7.md`): a day warmer or
//! colder than the standard atmosphere, read in pressure altitudes as a period altimeter
//! did, and climbs from an airfield above sea level.
//!
//! On such a day the vertical coordinate is the pressure altitude h_p: the pressure is
//! that of the standard atmosphere at h_p, the temperature is offset by ΔT, the density
//! follows. Hydrostatics (dp = −ρ g dz, and dp = −ρ_ISA(h_p) g dh_p by definition of h_p)
//! turn a geometric rate of climb w into dh_p/dt = w ρ / ρ_ISA = w T_ISA / T: on a hot
//! day the aircraft must climb further to reach the same altimeter reading.

use escadrille_fdm::aircraft::Aircraft;
use escadrille_fdm::atmosphere::{AtmosphereState, isa};
use escadrille_fdm::constants::{GAMMA_AIR, R_AIR};
use escadrille_fdm::environment::{Environment, altitude};
use glam::DVec3;

use crate::performance::{CLIMB_STEP, PerformanceError, best_climb, integrate_climb};

/// Still air, standard pressures, temperatures offset by `delta_t` (K); altitudes are
/// pressure altitudes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NonStandardDay {
    pub delta_t: f64,
}

/// Largest temperature offset accepted, K: beyond any climate (and the temperature must
/// stay positive).
pub const MAX_DELTA_T: f64 = 60.0;

impl NonStandardDay {
    /// A day `delta_t` warmer than the standard atmosphere, or `None` beyond ±[`MAX_DELTA_T`].
    pub fn new(delta_t: f64) -> Option<Self> {
        (delta_t.abs() <= MAX_DELTA_T).then_some(Self { delta_t })
    }
}

impl Environment for NonStandardDay {
    fn atmosphere(&self, position_ned: DVec3) -> AtmosphereState {
        let standard = isa(altitude(position_ned));
        let temperature = standard.temperature + self.delta_t;
        AtmosphereState {
            temperature,
            pressure: standard.pressure,
            density: standard.pressure / (R_AIR * temperature),
            speed_of_sound: (GAMMA_AIR * R_AIR * temperature).sqrt(),
        }
    }

    fn wind_ned(&self, _position_ned: DVec3) -> DVec3 {
        DVec3::ZERO
    }

    fn ground_altitude(&self, _position_ned: DVec3) -> f64 {
        0.0
    }
}

/// Best rate of climb in pressure altitude at the pressure altitude `h`, m/s.
pub fn pressure_climb_rate(
    aircraft: &Aircraft,
    day: &NonStandardDay,
    h: f64,
) -> Result<f64, PerformanceError> {
    let standard = isa(h).temperature;
    Ok(best_climb(aircraft, day, h)?.rate * standard / (standard + day.delta_t))
}

/// Time to climb between two pressure altitudes at the best rate of climb, s.
pub fn time_to_pressure_altitude(
    aircraft: &Aircraft,
    day: &NonStandardDay,
    from: f64,
    to: f64,
) -> Result<f64, PerformanceError> {
    integrate_climb(
        |h| pressure_climb_rate(aircraft, day, h),
        from,
        to,
        CLIMB_STEP,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::performance::time_to_climb;
    use crate::trim::tests::{STILL_AIR, biplane};

    fn at(h: f64) -> DVec3 {
        DVec3::new(0.0, 0.0, -h)
    }

    #[test]
    fn a_standard_day_is_the_standard_atmosphere() {
        let day = NonStandardDay { delta_t: 0.0 };
        for h in [0.0, 2_000.0, 5_000.0] {
            let (a, b) = (day.atmosphere(at(h)), isa(h));
            assert!(
                (a.density - b.density).abs() < 1e-12 * b.density,
                "{a:?} {b:?}"
            );
            assert!((a.speed_of_sound - b.speed_of_sound).abs() < 1e-9);
            assert_eq!(a.pressure, b.pressure);
        }
    }

    #[test]
    fn a_hot_day_keeps_the_pressure_and_thins_the_air() {
        let hot = NonStandardDay { delta_t: 20.0 };
        let (a, b) = (hot.atmosphere(at(2_000.0)), isa(2_000.0));
        assert_eq!(a.pressure, b.pressure);
        assert!((a.temperature - b.temperature - 20.0).abs() < 1e-12);
        let expected = b.density * b.temperature / a.temperature;
        assert!((a.density - expected).abs() < 1e-12 * expected);
    }

    #[test]
    fn on_a_standard_day_pressure_and_geometric_climbs_agree() {
        let a = biplane();
        let day = NonStandardDay { delta_t: 0.0 };
        let pressure = time_to_pressure_altitude(&a, &day, 0.0, 2_000.0).unwrap();
        let geometric = time_to_climb(&a, &STILL_AIR, 0.0, 2_000.0).unwrap();
        assert!(
            (pressure - geometric).abs() < 1e-9 * geometric,
            "{pressure} {geometric}"
        );
    }

    /// The same hot day described in geometric altitude: the pressure integrated along a
    /// column whose temperature at each pressure is the standard one plus ΔT,
    /// z(h_p) = h_p + ΔT (R / g) ln(p₀ / p), an independent check of w T_std / T.
    struct GeometricDay {
        delta_t: f64,
    }

    impl GeometricDay {
        fn height(&self, pressure_altitude: f64) -> f64 {
            use escadrille_fdm::constants::{G0, P0};
            pressure_altitude
                + self.delta_t * R_AIR / G0 * (P0 / isa(pressure_altitude).pressure).ln()
        }
    }

    impl Environment for GeometricDay {
        fn atmosphere(&self, position_ned: DVec3) -> AtmosphereState {
            let z = altitude(position_ned);
            let (mut lo, mut hi) = (z - 2_000.0, z + 2_000.0);
            for _ in 0..100 {
                let mid = 0.5 * (lo + hi);
                if self.height(mid) < z {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            NonStandardDay {
                delta_t: self.delta_t,
            }
            .atmosphere(DVec3::new(0.0, 0.0, -0.5 * (lo + hi)))
        }

        fn wind_ned(&self, _position_ned: DVec3) -> DVec3 {
            DVec3::ZERO
        }

        fn ground_altitude(&self, _position_ned: DVec3) -> f64 {
            0.0
        }
    }

    #[test]
    fn the_pressure_altitude_climb_matches_a_geometric_column() {
        let a = biplane();
        let delta_t = 15.0;
        let pressure =
            time_to_pressure_altitude(&a, &NonStandardDay { delta_t }, 0.0, 2_000.0).unwrap();
        let column = GeometricDay { delta_t };
        let geometric = time_to_climb(&a, &column, 0.0, column.height(2_000.0)).unwrap();
        assert!(
            (pressure - geometric).abs() < 2e-3 * geometric,
            "{pressure} {geometric}"
        );
    }

    #[test]
    fn days_beyond_any_climate_are_refused() {
        assert!(NonStandardDay::new(20.0).is_some());
        assert!(NonStandardDay::new(-61.0).is_none());
        assert!(NonStandardDay::new(f64::NAN).is_none());
    }

    #[test]
    fn a_hot_day_slows_the_climb_and_a_cold_one_speeds_it_up() {
        let a = biplane();
        let time = |delta_t: f64| {
            time_to_pressure_altitude(&a, &NonStandardDay { delta_t }, 0.0, 2_000.0).unwrap()
        };
        let (cold, standard, hot) = (time(-15.0), time(0.0), time(15.0));
        assert!(cold < standard && standard < hot, "{cold} {standard} {hot}");
        // Thinner air and a longer geometric climb: more than the density ratio alone.
        assert!(hot / standard > 1.03, "{hot} {standard}");
    }
}
