//! The sky's clock (plan `m3-heure-du-jour.md`): the sun and the moon of a scene's date and
//! hour over the sector of Ypres, and the one light they share. The sun keeps it by day
//! and through the twilight, whose glow Bevy's atmosphere draws from a sun below the
//! horizon; the moon takes it in the full night. One light, so that the clouds, which
//! take the first directional light for their sun, are lit by the moon at night.

use bevy::light::{SunDisk, light_consts::lux};
use bevy::math::DVec3;
use bevy::prelude::*;
use escadrille_weather::moon::{self, MoonPosition};
use escadrille_weather::sun::{self, Place, SunPosition, Utc};

use crate::flight::bridge::to_bevy;

/// The game's place on the Earth: Ypres, its cloth hall (`docs/recherche/heure-legale-1917.md`
/// § 6.1, confidence A), until the real ground; the fields of the SPA 3 at Bierne put the
/// sun 1.9 min later, no visible difference.
pub const YPRES: Place = Place {
    latitude_deg: 50.851,
    longitude_deg: 2.885,
};

/// Below this height of the sun, the full night: the moon takes the light. At −12°, the
/// end of nautical twilight, the sky's glow from the sun has fallen to the moon's.
pub const NIGHT_SUN_DEG: f64 = -12.0;

/// The sun's disc as drawn: its size, and a hundredth of its brightness. At its own, about
/// 2·10⁹ cd/m², the disc passed the largest half float of the frame (65 504) once exposed
/// in daylight, and its edge came out as a darker ring, 5 levels deep; at a tenth, still 3
/// (2026-11-11). At a hundredth it stays thousands of times the white once exposed, noon or
/// sunset, so the tonemapping keeps it white; the light, the glare veil and the clouds take
/// the sun's illuminance elsewhere.
pub const SUN_DISC: SunDisk = SunDisk {
    angular_size: SunDisk::EARTH.angular_size,
    intensity: 0.01,
};

/// Illuminance of a star of magnitude 0 outside the atmosphere, lux (`ESTIMÉ`: the common
/// figure, which gives the sun its 127 klux).
pub const MAGNITUDE_ZERO_LUX: f64 = 2.54e-6;

/// The sky of a scene.
#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub struct Sky {
    pub place: Place,
    /// When, if the scene has a time.
    pub when: Option<Utc>,
    pub sun: SunPosition,
    pub moon: Option<MoonPosition>,
}

impl Sky {
    /// The sun and the moon of `when` over `place`.
    pub fn at(when: Utc, place: Place) -> Self {
        Self {
            place,
            when: Some(when),
            sun: sun::position(when, place),
            moon: Some(moon::position(when, place)),
        }
    }

    /// A sun set by hand, without a clock: no moon, no stars.
    pub fn sun_only(sun: SunPosition) -> Self {
        Self {
            place: YPRES,
            when: None,
            sun,
            moon: None,
        }
    }

    pub fn is_night(&self) -> bool {
        self.sun.elevation_deg < NIGHT_SUN_DEG
    }

    /// The moon's illuminance outside the atmosphere on a surface facing it, lux: its
    /// magnitude by the phase law of Allen ("Astrophysical Quantities"), m = −12.73 +
    /// 0.026 |i| + 4·10⁻⁹ i⁴, i its phase angle in degrees.
    pub fn moonlight_lux(&self) -> f64 {
        let Some(moon) = self.moon else {
            return 0.0;
        };
        let i = (2.0 * moon.illuminated - 1.0)
            .clamp(-1.0, 1.0)
            .acos()
            .to_degrees();
        let magnitude = -12.73 + 0.026 * i + 4e-9 * i.powi(4);
        MAGNITUDE_ZERO_LUX * 10f64.powf(-0.4 * magnitude)
    }

    /// Local apparent solar time, hours: the afternoon when the scene has no clock.
    pub fn solar_hour(&self) -> f64 {
        self.when
            .map_or(AFTERNOON_SOLAR_HOUR, |w| sun::solar_hour(w, self.place))
    }

    /// Local sidereal angle, degrees, for the stars.
    pub fn sidereal_deg(&self) -> Option<f64> {
        self.when
            .map(|w| sun::local_sidereal_deg(w.julian_day(), self.place.longitude_deg))
    }
}

/// The local solar time of a scene without a clock, hours: the test flight's afternoon.
pub const AFTERNOON_SOLAR_HOUR: f64 = 15.0;

/// Direction towards a body of the sky, Bevy axes.
pub fn towards(position: SunPosition) -> Vec3 {
    let (az, el) = (
        position.azimuth_deg.to_radians(),
        position.elevation_deg.to_radians(),
    );
    // North, east, down, as the flight model's frame.
    let ned = DVec3::new(el.cos() * az.cos(), el.cos() * az.sin(), -el.sin());
    to_bevy(ned).as_vec3()
}

/// The light of the star of the hour: the sun, or the moon in the full night.
pub fn light_the_sky(
    sky: Option<Res<Sky>>,
    mut lights: Query<(&mut DirectionalLight, &mut Transform, &mut SunDisk)>,
) {
    let Some(sky) = sky else {
        return;
    };
    let (body, illuminance, disc) = match sky.moon {
        Some(moon) if sky.is_night() => (moon.sky, sky.moonlight_lux() as f32, 0.0),
        _ => (sky.sun, lux::RAW_SUNLIGHT, SUN_DISC.intensity),
    };
    let rotation = Transform::default()
        .looking_to(-towards(body), Vec3::Y)
        .rotation;
    for (mut light, mut transform, mut sun_disk) in &mut lights {
        if transform.rotation != rotation {
            transform.rotation = rotation;
        }
        if light.illuminance != illuminance {
            light.illuminance = illuminance;
        }
        if sun_disk.intensity != disc {
            sun_disk.intensity = disc;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_full_moon_lights_a_quarter_of_a_lux() {
        let full = Sky {
            moon: Some(MoonPosition {
                sky: SunPosition {
                    azimuth_deg: 180.0,
                    elevation_deg: 40.0,
                },
                illuminated: 1.0,
            }),
            ..Sky::sun_only(SunPosition {
                azimuth_deg: 0.0,
                elevation_deg: -30.0,
            })
        };
        let lux = full.moonlight_lux();
        assert!((0.25..0.35).contains(&lux), "{lux}");
        // At the first quarter, about a tenth: the opposition surge is gone.
        let quarter = Sky {
            moon: full.moon.map(|m| MoonPosition {
                illuminated: 0.5,
                ..m
            }),
            ..full
        };
        let ratio = quarter.moonlight_lux() / lux;
        assert!((0.05..0.15).contains(&ratio), "{ratio}");
    }

    #[test]
    fn the_sun_points_where_it_stands() {
        // South-west, 40° high: towards the south (Bevy +z) and the west (−x), and up.
        let d = towards(SunPosition {
            azimuth_deg: 225.0,
            elevation_deg: 40.0,
        });
        assert!(d.x < 0.0 && d.z > 0.0 && d.y > 0.6, "{d}");
        assert!((d.length() - 1.0).abs() < 1e-5);
    }
}
