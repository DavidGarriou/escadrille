//! Where the moon stands, and how much of it is lit (plan `m3-heure-du-jour.md`, step 4):
//! the low-precision formulae of the Astronomical Almanac for its ecliptic longitude and
//! latitude (within about 0.3°, as quoted by Meeus, "Astronomical Algorithms", 1998, and
//! the Almanac's section D), turned into the sky of a place through the sidereal time
//! (Meeus, chapter 12), with the parallax of its nearness; the lit fraction from its
//! elongation from the sun (Meeus, chapter 48).

use crate::sun::{Place, SunPosition, Utc, ephemeris, local_sidereal_deg, refraction_deg};

/// The moon seen from a place: where it stands, and the lit fraction of its disc.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MoonPosition {
    pub sky: SunPosition,
    pub illuminated: f64,
}

/// The moon's ecliptic longitude and latitude, degrees, at `julian_day`.
pub fn ecliptic(julian_day: f64) -> (f64, f64) {
    let t = (julian_day - 2_451_545.0) / 36_525.0;
    let s = |a: f64, b: f64| (a + b * t).to_radians().sin();
    let longitude = 218.32 + 481_267.881 * t + 6.29 * s(135.0, 477_198.87)
        - 1.27 * s(259.3, -413_335.36)
        + 0.66 * s(235.7, 890_534.22)
        + 0.21 * s(269.9, 954_397.74)
        - 0.19 * s(357.5, 35_999.05)
        - 0.11 * s(186.5, 966_404.03);
    let latitude = 5.13 * s(93.3, 483_202.02) + 0.28 * s(228.2, 960_400.89)
        - 0.28 * s(318.3, 6_003.15)
        - 0.17 * s(217.6, -407_332.21);
    (longitude.rem_euclid(360.0), latitude)
}

/// The moon's horizontal parallax, degrees: its nearness lowers it in the sky by up to
/// this much (Almanac, low precision).
fn horizontal_parallax(julian_day: f64) -> f64 {
    let t = (julian_day - 2_451_545.0) / 36_525.0;
    let c = |a: f64, b: f64| (a + b * t).to_radians().cos();
    0.9508
        + 0.0518 * c(135.0, 477_198.87)
        + 0.0095 * c(259.3, -413_335.36)
        + 0.0078 * c(235.7, 890_534.22)
        + 0.0028 * c(269.9, 954_397.74)
}

/// The moon seen from `place` at `when`.
pub fn position(when: Utc, place: Place) -> MoonPosition {
    let jd = when.julian_day();
    let (longitude, latitude) = ecliptic(jd);
    let t = (jd - 2_451_545.0) / 36_525.0;
    let obliquity = (23.439_291 - 0.013_004_2 * t).to_radians();
    let (l, b) = (longitude.to_radians(), latitude.to_radians());
    let right_ascension = (l.sin() * obliquity.cos() - b.tan() * obliquity.sin()).atan2(l.cos());
    let declination = (b.sin() * obliquity.cos() + b.cos() * obliquity.sin() * l.sin()).asin();
    // Local sidereal time, then the hour angle.
    let sidereal = local_sidereal_deg(jd, place.longitude_deg);
    let hour_angle = (sidereal.to_radians() - right_ascension).rem_euclid(std::f64::consts::TAU);
    let lat = place.latitude_deg.to_radians();
    let elevation =
        (lat.sin() * declination.sin() + lat.cos() * declination.cos() * hour_angle.cos()).asin();
    let azimuth = (-hour_angle.sin())
        .atan2(declination.tan() * lat.cos() - lat.sin() * hour_angle.cos())
        .to_degrees()
        .rem_euclid(360.0);
    // Seen from the surface, not from the Earth's centre.
    let geocentric = elevation.to_degrees();
    let topocentric = geocentric - horizontal_parallax(jd) * elevation.cos();
    // Lit fraction from the elongation from the sun.
    let sun_longitude = sun_longitude(jd);
    let elongation = (b.cos() * (longitude - sun_longitude).to_radians().cos()).acos();
    let phase_angle = std::f64::consts::PI - elongation;
    MoonPosition {
        sky: SunPosition {
            azimuth_deg: azimuth,
            elevation_deg: topocentric + refraction_deg(topocentric),
        },
        illuminated: (1.0 + phase_angle.cos()) / 2.0,
    }
}

/// The sun's apparent ecliptic longitude, degrees, from its right ascension and
/// declination.
fn sun_longitude(julian_day: f64) -> f64 {
    let sun = ephemeris(julian_day);
    let t = (julian_day - 2_451_545.0) / 36_525.0;
    let obliquity = (23.439_291 - 0.013_004_2 * t).to_radians();
    let (a, d) = (
        sun.right_ascension_deg.to_radians(),
        sun.declination_deg.to_radians(),
    );
    (a.sin() * obliquity.cos() + d.tan() * obliquity.sin())
        .atan2(a.cos())
        .to_degrees()
        .rem_euclid(360.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_moon_s_place_is_meeus_s_example() {
        // Meeus, example 47.a: 1992 April 12, 0h TD, λ = 133.162655°, β = −3.229126°; the
        // low-precision formulae hold within about 0.3°.
        let (longitude, latitude) = ecliptic(2_448_724.5);
        assert!((longitude - 133.162_655).abs() < 0.3, "{longitude}");
        assert!((latitude + 3.229_126).abs() < 0.2, "{latitude}");
    }

    #[test]
    fn the_lit_fraction_is_meeus_s_example() {
        // Meeus, example 48.a: the same moment, k = 0.6786.
        let ypres = Place {
            latitude_deg: 50.85,
            longitude_deg: 2.89,
        };
        let moon = position(
            Utc {
                year: 1992,
                month: 4,
                day: 12,
                hour: 0.0,
            },
            ypres,
        );
        assert!(
            (moon.illuminated - 0.6786).abs() < 0.02,
            "{}",
            moon.illuminated
        );
    }

    #[test]
    fn the_moon_crosses_the_sky_once_a_day_and_some() {
        // Over a day, it rises and sets: the highest and lowest are far apart, and it
        // stands highest near the south.
        let ypres = Place {
            latitude_deg: 50.85,
            longitude_deg: 2.89,
        };
        let sky: Vec<SunPosition> = (0..48)
            .map(|half| {
                position(
                    Utc {
                        year: 1917,
                        month: 8,
                        day: 2,
                        hour: half as f64 * 0.5,
                    },
                    ypres,
                )
                .sky
            })
            .collect();
        let highest = sky
            .iter()
            .max_by(|a, b| a.elevation_deg.total_cmp(&b.elevation_deg))
            .unwrap();
        let lowest = sky
            .iter()
            .min_by(|a, b| a.elevation_deg.total_cmp(&b.elevation_deg))
            .unwrap();
        assert!(highest.elevation_deg > 0.0 && lowest.elevation_deg < 0.0);
        assert!((highest.azimuth_deg - 180.0).abs() < 20.0, "{highest:?}");
    }
}
