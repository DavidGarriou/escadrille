//! Where the sun stands for a date, a time and a place (plan `m3-heure-du-jour.md`): the
//! algorithm of the NOAA solar calculator, from Meeus ("Astronomical Algorithms", 2nd ed.,
//! 1998, chapter 25, low accuracy), within about 0.01° for the years of the war, then the
//! refraction of the air near the horizon. Times are universal (UT); the legal time of
//! 1917 is the game's (`docs/recherche/heure-legale-1917.md`).

/// A moment in universal time, Gregorian calendar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Utc {
    pub year: i32,
    pub month: u32,
    pub day: u32,
    /// Hours since midnight, fractional.
    pub hour: f64,
}

impl Utc {
    /// Julian day (Meeus, chapter 7).
    pub fn julian_day(self) -> f64 {
        let (mut y, mut m) = (self.year, self.month as i32);
        if m <= 2 {
            y -= 1;
            m += 12;
        }
        let a = (y as f64 / 100.0).floor();
        let b = 2.0 - a + (a / 4.0).floor();
        (365.25 * (y as f64 + 4716.0)).floor()
            + (30.6001 * (m as f64 + 1.0)).floor()
            + self.day as f64
            + b
            - 1524.5
            + self.hour / 24.0
    }
}

/// A place on the Earth: latitude north and longitude east, degrees.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Place {
    pub latitude_deg: f64,
    pub longitude_deg: f64,
}

/// The sun seen from a place: azimuth from north towards east, and elevation above the
/// horizon, refraction included, degrees.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SunPosition {
    pub azimuth_deg: f64,
    pub elevation_deg: f64,
}

/// The sun's apparent place in the sky, whatever the observer: declination, degrees, and
/// equation of time, minutes (apparent minus mean solar time).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SunEphemeris {
    pub right_ascension_deg: f64,
    pub declination_deg: f64,
    pub equation_of_time_min: f64,
}

/// The sun's ephemeris at `julian_day` (Meeus, chapter 25, and the equation of time of
/// chapter 28 in Smart's form, as the NOAA calculator writes it).
pub fn ephemeris(julian_day: f64) -> SunEphemeris {
    let t = (julian_day - 2_451_545.0) / 36_525.0;
    let mean_longitude = (280.46646 + t * (36000.76983 + t * 0.0003032)).rem_euclid(360.0);
    let mean_anomaly = 357.52911 + t * (35999.05029 - 0.0001537 * t);
    let eccentricity = 0.016_708_634 - t * (0.000_042_037 + 0.0000001267 * t);
    let m = mean_anomaly.to_radians();
    let centre = m.sin() * (1.914_602 - t * (0.004_817 + 0.000_014 * t))
        + (2.0 * m).sin() * (0.019_993 - 0.000_101 * t)
        + (3.0 * m).sin() * 0.000_289;
    let true_longitude = mean_longitude + centre;
    let omega = (125.04 - 1_934.136 * t).to_radians();
    let apparent_longitude = (true_longitude - 0.005_69 - 0.004_78 * omega.sin()).to_radians();
    let mean_obliquity =
        23.0 + (26.0 + (21.448 - t * (46.815 + t * (0.000_59 - t * 0.001_813))) / 60.0) / 60.0;
    let obliquity = (mean_obliquity + 0.002_56 * omega.cos()).to_radians();
    let right_ascension = (obliquity.cos() * apparent_longitude.sin())
        .atan2(apparent_longitude.cos())
        .to_degrees()
        .rem_euclid(360.0);
    let declination = (obliquity.sin() * apparent_longitude.sin())
        .asin()
        .to_degrees();
    let y = (obliquity / 2.0).tan().powi(2);
    let l0 = mean_longitude.to_radians();
    let e = eccentricity;
    let equation = y * (2.0 * l0).sin() - 2.0 * e * m.sin()
        + 4.0 * e * y * m.sin() * (2.0 * l0).cos()
        - 0.5 * y * y * (4.0 * l0).sin()
        - 1.25 * e * e * (2.0 * m).sin();
    SunEphemeris {
        right_ascension_deg: right_ascension,
        declination_deg: declination,
        equation_of_time_min: 4.0 * equation.to_degrees(),
    }
}

/// Refraction of the air at a true elevation, degrees: the NOAA calculator's fit of the
/// standard atmosphere (Bennett's formula near the horizon), nothing above 85°.
pub fn refraction_deg(elevation_deg: f64) -> f64 {
    let e = elevation_deg;
    let arcsec = if e > 85.0 {
        0.0
    } else if e > 5.0 {
        let t = e.to_radians().tan();
        58.1 / t - 0.07 / t.powi(3) + 0.000_086 / t.powi(5)
    } else if e > -0.575 {
        1_735.0 + e * (-518.2 + e * (103.4 + e * (-12.79 + e * 0.711)))
    } else {
        -20.772 / e.to_radians().tan()
    };
    arcsec / 3_600.0
}

/// The sun seen from `place` at `when`.
pub fn position(when: Utc, place: Place) -> SunPosition {
    let sun = ephemeris(when.julian_day());
    // True solar time, minutes, and the hour angle, degrees (west of the meridian
    // positive).
    let solar_time = (when.hour * 60.0 + sun.equation_of_time_min + 4.0 * place.longitude_deg)
        .rem_euclid(1_440.0);
    let hour_angle = solar_time / 4.0 - 180.0;
    let (lat, dec, ha) = (
        place.latitude_deg.to_radians(),
        sun.declination_deg.to_radians(),
        hour_angle.to_radians(),
    );
    let cos_zenith = (lat.sin() * dec.sin() + lat.cos() * dec.cos() * ha.cos()).clamp(-1.0, 1.0);
    let zenith = cos_zenith.acos();
    let elevation = 90.0 - zenith.to_degrees();
    // Azimuth from north towards east: east of north in the morning, west in the
    // afternoon.
    let cos_azimuth =
        ((lat.sin() * cos_zenith - dec.sin()) / (lat.cos() * zenith.sin())).clamp(-1.0, 1.0);
    let from_south = cos_azimuth.acos().to_degrees();
    let azimuth = if hour_angle > 0.0 {
        (from_south + 180.0).rem_euclid(360.0)
    } else {
        (540.0 - from_south).rem_euclid(360.0)
    };
    SunPosition {
        azimuth_deg: azimuth,
        elevation_deg: elevation + refraction_deg(elevation),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Ypres, the plan's default place.
    const YPRES: Place = Place {
        latitude_deg: 50.85,
        longitude_deg: 2.89,
    };

    /// The UT of the sun's highest point on a day at `place`, by search, hours.
    fn noon(year: i32, month: u32, day: u32, place: Place) -> (f64, SunPosition) {
        let mut best = (0.0, -90.0);
        for minute in 0..1_440 {
            let hour = minute as f64 / 60.0;
            let e = position(
                Utc {
                    year,
                    month,
                    day,
                    hour,
                },
                place,
            )
            .elevation_deg;
            if e > best.1 {
                best = (hour, e);
            }
        }
        (
            best.0,
            position(
                Utc {
                    year,
                    month,
                    day,
                    hour: best.0,
                },
                place,
            ),
        )
    }

    #[test]
    fn the_julian_day_is_meeus_s() {
        // Meeus, example 7.a: 1957 October 4.81 = JD 2436116.31.
        let sputnik = Utc {
            year: 1957,
            month: 10,
            day: 4,
            hour: 0.81 * 24.0,
        };
        assert!((sputnik.julian_day() - 2_436_116.31).abs() < 1e-6);
        // The epoch J2000.0.
        let j2000 = Utc {
            year: 2000,
            month: 1,
            day: 1,
            hour: 12.0,
        };
        assert_eq!(j2000.julian_day(), 2_451_545.0);
    }

    #[test]
    fn the_sun_s_place_is_meeus_s_example() {
        // Meeus, example 25.a: 1992 October 13.0 TD, α = 198.38083°, δ = −7.78507°.
        let sun = ephemeris(2_448_908.5);
        assert!(
            (sun.right_ascension_deg - 198.380_83).abs() < 0.01,
            "{sun:?}"
        );
        assert!((sun.declination_deg + 7.785_07).abs() < 0.01, "{sun:?}");
    }

    #[test]
    fn the_equation_of_time_has_its_known_extremes() {
        // About −14.2 min in mid-February and +16.4 min in early November.
        let february = ephemeris(
            Utc {
                year: 1917,
                month: 2,
                day: 11,
                hour: 12.0,
            }
            .julian_day(),
        );
        let november = ephemeris(
            Utc {
                year: 1917,
                month: 11,
                day: 3,
                hour: 12.0,
            }
            .julian_day(),
        );
        assert!(
            (february.equation_of_time_min + 14.2).abs() < 0.3,
            "{february:?}"
        );
        assert!(
            (november.equation_of_time_min - 16.4).abs() < 0.3,
            "{november:?}"
        );
    }

    #[test]
    fn at_noon_over_ypres_the_sun_climbs_to_ninety_minus_latitude_plus_declination() {
        for (month, day, declination) in [(3, 21, 0.0), (6, 21, 23.44), (12, 22, -23.44)] {
            let (_, sun) = noon(1917, month, day, YPRES);
            let expected = 90.0 - YPRES.latitude_deg + declination;
            // Within the declination's change over the day of the solstice or equinox
            // and the refraction (a few hundredths of a degree at these heights).
            assert!(
                (sun.elevation_deg - expected).abs() < 0.5,
                "1917-{month}-{day}: {} against {expected}",
                sun.elevation_deg
            );
            assert!((sun.azimuth_deg - 180.0).abs() < 1.0, "{sun:?}");
        }
    }

    #[test]
    fn noon_over_ypres_comes_before_noon_at_greenwich_by_the_longitude_and_the_equation() {
        // 2.89° east: 11.6 min early; mid-February, the sun 14.2 min late.
        let (hour, _) = noon(1917, 2, 11, YPRES);
        let expected = 12.0 - (4.0 * 2.89 - 14.2) / 60.0;
        assert!(
            (hour - expected).abs() < 1.5 / 60.0,
            "{hour} against {expected}"
        );
    }

    #[test]
    fn the_sun_rises_in_the_east_and_sets_in_the_west() {
        let morning = position(
            Utc {
                year: 1917,
                month: 3,
                day: 21,
                hour: 6.0,
            },
            YPRES,
        );
        let evening = position(
            Utc {
                year: 1917,
                month: 3,
                day: 21,
                hour: 18.0,
            },
            YPRES,
        );
        assert!((morning.azimuth_deg - 90.0).abs() < 8.0, "{morning:?}");
        assert!((evening.azimuth_deg - 270.0).abs() < 8.0, "{evening:?}");
        assert!(morning.elevation_deg.abs() < 5.0 && evening.elevation_deg.abs() < 5.0);
    }

    #[test]
    fn the_air_lifts_the_setting_sun_by_about_half_a_degree() {
        assert!(
            (refraction_deg(0.0) - 0.48).abs() < 0.05,
            "{}",
            refraction_deg(0.0)
        );
        assert!(refraction_deg(45.0) < 0.02);
        assert_eq!(refraction_deg(89.0), 0.0);
    }
}
