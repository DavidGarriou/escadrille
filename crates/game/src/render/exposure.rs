//! The exposure that follows the light (plan `m3-heure-du-jour.md`, step 2), in three
//! parts, in the order the frame meets them:
//!
//! 1. **Base** ([`metered_ev100`], the camera's `Exposure`): EV100 13 in full daylight
//!    ([`DAYLIGHT_EV100`], chosen on `avion-studio`), following the light down at dusk
//!    and at night, so that the frame stays within the range the adaptation reads.
//! 2. **Adaptation** (Bevy's auto exposure, [`adapt_to_the_frame`]): the frame's mean
//!    luminance brought part of the way back to the neutral of the approved reference
//!    scenes, within [`ADAPTATION_EV`] either way: the eye adapts to what it looks at.
//! 3. **Look** ([`look_ev`], the camera's `ColorGrading`, applied after the adaptation
//!    reads the frame, so it does not undo it): a dusk darker than a noon, and the sun's
//!    glare in the eye.
//!
//! Measured on 2026-11-11: a base following the illuminance all day opened a sun 9° high
//! by 2.6 stops, beyond what the adaptation closes; the frame's mean alone opened the
//! contre-jour, mostly cloud faces in shade, by half a stop; a glare put in the base was
//! undone by the adaptation.

use std::f32::consts::PI;

use bevy::camera::Exposure;
use bevy::curve::LinearSpline;
use bevy::light::SunDisk;
use bevy::post_process::auto_exposure::{AutoExposure, AutoExposureCompensationCurve};
use bevy::prelude::*;
use bevy::render::view::ColorGrading;

use crate::camera::DAYLIGHT_EV100;
use crate::dev::capture::CaptureRequest;
use crate::render::glare::SunSeen;
use crate::world::sky::{Sky, towards};

/// Full daylight, lux: above it, the base stays at [`DAYLIGHT_EV100`] (`ESTIMÉ`: the
/// light of a sun about 8° high in a clear sky).
pub const FULL_DAYLIGHT_LUX: f32 = 10_000.0;
/// The darkest light the base follows, lux: a starlit night (`ESTIMÉ`).
pub const STARLIGHT_LUX: f32 = 0.001;
/// Share of the fall of the light below full daylight that the look shows (`ESTIMÉ`): the
/// eye makes up for most of it, not all.
pub const DUSK_SHOWN: f32 = 0.3;
/// The most the night darkens the look, stops (`ESTIMÉ`, to set with the night).
pub const NIGHT_DARKENING_EV: f32 = 5.0;

/// The air mass towards a sun `elevation_deg` high: Kasten and Young's formula ("Revised
/// optical air mass tables and approximation formula", Applied Optics 28, 1989), true to
/// the horizon, where 1 / sin h, which ignores the Earth's curve, overstates it (19 instead
/// of 15 at 3°).
pub fn air_mass(elevation_deg: f32) -> f32 {
    let h = elevation_deg.max(0.0);
    1.0 / (h.to_radians().sin() + 0.50572 * (h + 6.07995).powf(-1.6364))
}

/// The sun's illuminance on a surface facing it, lux, at `elevation_deg` above the
/// horizon: the IES model of a clear sky, 127.5 klux × e^(−0.21 m), m the air mass
/// ([`air_mass`]).
pub fn direct_normal_lux(elevation_deg: f32) -> f32 {
    if elevation_deg <= 0.0 {
        return 0.0;
    }
    127_500.0 * (-0.21 * air_mass(elevation_deg)).exp()
}

/// Clear-sky daylight on level ground, lux, the sun at `elevation_deg`. Above the horizon,
/// the IES model of a clear sky: the direct light ([`direct_normal_lux`]) and the diffuse
/// 0.8 + 15.5 √(sin h) klux. Below it, twilight falls from the diffuse 800 lux at sunset
/// to 3.4 lux at the end of civil twilight, 6° down (`ESTIMÉ`: a common figure), and on by
/// the same slope.
pub fn ground_illuminance_lux(elevation_deg: f32) -> f32 {
    if elevation_deg > 0.0 {
        let s = elevation_deg.to_radians().sin();
        direct_normal_lux(elevation_deg) * s + 800.0 + 15_500.0 * s.sqrt()
    } else {
        800.0 * 10f32.powf(TWILIGHT_DECADES_PER_DEG * elevation_deg)
    }
}

/// Fall of the twilight, decades of lux per degree: 800 to 3.4 lux over 6°.
const TWILIGHT_DECADES_PER_DEG: f32 = 0.395;

/// Stops of light below full daylight, under `ground_lux` on level ground: 0 by day,
/// negative at dusk and at night.
fn below_daylight(ground_lux: f32) -> f32 {
    (ground_lux.max(STARLIGHT_LUX) / FULL_DAYLIGHT_LUX)
        .log2()
        .min(0.0)
}

/// The base exposure under `ground_lux` on level ground, EV100.
pub fn metered_ev100(ground_lux: f32) -> f32 {
    DAYLIGHT_EV100 + below_daylight(ground_lux)
}

/// The look's exposure offset, stops (negative: darker), under `ground_lux`, the sun at
/// `elevation_deg` and `angle_deg` from the line of sight: the dusk the eye does not make
/// up for, and the sun's glare.
pub fn look_ev(ground_lux: f32, elevation_deg: f32, angle_deg: f32, seen: f32) -> f32 {
    (DUSK_SHOWN * below_daylight(ground_lux)).max(-NIGHT_DARKENING_EV)
        - glare_ev(elevation_deg, angle_deg, seen)
}

/// Reflectance of a mean scene, the photographer's grey card.
const MEAN_REFLECTANCE: f32 = 0.18;
/// The most the glare closes the eye, stops (`ESTIMÉ`).
pub const GLARE_MAX_EV: f32 = 2.0;

/// The age of the pilot's eye, years, and its pigmentation (0 very dark, 0.5 brown, 1 blue),
/// for the glare (`ESTIMÉ`: a young pilot, brown eyes).
pub const PILOT_AGE: f32 = 25.0;
pub const EYE_PIGMENTATION: f32 = 0.5;
/// The narrowest angle of the glare equation, degrees.
pub const GLARE_NEAREST_DEG: f32 = 0.1;

/// Veiling luminance per lux at the eye, cd/m², at `angle_deg` from the glare's source: the
/// CIE's general disability glare equation (CIE 146:2002, after Vos), from 0.1° to 100°,
/// L / E = 10/θ³ + (5/θ² + 0.1 p/θ)(1 + (A/62.5)⁴) + 0.0025 p. From 1° to 30°, close to
/// Stiles and Holladay's 10/θ²; nearer, it goes on rising, where theirs stops at 1°.
pub fn veil_per_lux(angle_deg: f32) -> f32 {
    let theta = angle_deg.max(GLARE_NEAREST_DEG);
    let age = 1.0 + (PILOT_AGE / 62.5).powi(4);
    10.0 / theta.powi(3)
        + (5.0 / theta.powi(2) + 0.1 * EYE_PIGMENTATION / theta) * age
        + 0.0025 * EYE_PIGMENTATION
}

/// How far the sun's glare closes the eye, stops, the sun at `elevation_deg` and
/// `angle_deg` from the line of sight: the veiling luminance ([`veil_per_lux`]) of the sun's
/// illuminance at the eye adds to the luminance the eye adapts to, a grey card under the
/// day's light.
/// Only the part of the sun that shows, `seen` (0 to 1), counts.
pub fn glare_ev(elevation_deg: f32, angle_deg: f32, seen: f32) -> f32 {
    let sun = direct_normal_lux(elevation_deg) * seen;
    // Out of the field of view, no glare.
    if sun <= 0.0 || angle_deg >= 90.0 {
        return 0.0;
    }
    let veil = sun * veil_per_lux(angle_deg);
    let scene = MEAN_REFLECTANCE * ground_illuminance_lux(elevation_deg) / PI;
    (1.0 + veil / scene).log2().min(GLARE_MAX_EV)
}

/// A camera whose exposure follows the daylight and adapts to its frame.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct FollowsDaylight;

/// The eye's adaptation, EV either way around the base (art director's review of
/// 2026-11-11).
pub const ADAPTATION_EV: f32 = 1.5;
/// Share of the way back to the neutral the adaptation goes (`ESTIMÉ`): the eye does not
/// make up for all of it, a bright scene stays bright. Brought fully back, a frame of
/// white cloud turned grey, as snow on a photograph (`traversee_nuage`, 2026-11-11).
pub const ADAPTATION_SHARE: f32 = 0.5;

/// The span of the histogram either way around the neutral, EV: wide, so that the bright
/// sky round the sun weighs with its true value.
const METERED_EV: f32 = 8.0;

/// The mean log₂ luminance of a frame, after the base exposure, that the adaptation
/// leaves as it is. Set so that `vol-essai-poursuite` keeps its approved exposure: by a
/// sweep of −1, 0 and 1, it came out 0.39 stop darker, 0.17 stop brighter, 0.56 stop
/// brighter (2026-11-11). A value exact in binary: Bevy checks that the curve's segments
/// join by exact equality.
pub const NEUTRAL_LOG_LUMINANCE: f32 = -0.25;

/// Speed of the adaptation, stops a second: Bevy's, the eye closing faster than it opens.
/// Captures adapt fast, so that their frames of stabilisation reach the end.
const SPEED_BRIGHTEN: f32 = 3.0;
const SPEED_DARKEN: f32 = 1.0;
const CAPTURE_SPEED: f32 = 20.0;

/// The base and the look of every daylight camera, from the sun's height, the moon's light
/// and the sun's glare. Without a sky's clock (the scenes built in code), the sun is the
/// light that has a disc.
pub fn follow_daylight(
    sky: Option<Res<Sky>>,
    seen: Option<Res<SunSeen>>,
    suns: Query<&GlobalTransform, (With<DirectionalLight>, With<SunDisk>)>,
    mut cameras: Query<(&GlobalTransform, &mut Exposure, &mut ColorGrading), With<FollowsDaylight>>,
) {
    let (to_sun, ground_lux) = match sky {
        Some(sky) => {
            let moon = sky.moon.map_or(0.0, |m| {
                (sky.moonlight_lux() * m.sky.elevation_deg.to_radians().sin().max(0.0)) as f32
            });
            let sun = ground_illuminance_lux(sky.sun.elevation_deg as f32);
            (towards(sky.sun), sun + moon)
        }
        None => {
            let Some(sun) = suns.iter().next() else {
                return;
            };
            // The light shines along its forward axis: the sun lies behind it.
            let to_sun = *sun.back();
            let elevation = to_sun.y.clamp(-1.0, 1.0).asin().to_degrees();
            (to_sun, ground_illuminance_lux(elevation))
        }
    };
    let elevation = to_sun.y.clamp(-1.0, 1.0).asin().to_degrees();
    let base = metered_ev100(ground_lux);
    for (camera, mut exposure, mut grading) in &mut cameras {
        if exposure.ev100 != base {
            exposure.ev100 = base;
        }
        let angle = camera.forward().dot(to_sun).clamp(-1.0, 1.0).acos();
        let seen = seen.as_deref().copied().unwrap_or_default().0;
        let look = look_ev(ground_lux, elevation, angle.to_degrees(), seen);
        if grading.global.exposure != look {
            grading.global.exposure = look;
        }
    }
}

/// The eye's adaptation on each new daylight camera: the correction brings the frame's
/// mean luminance [`ADAPTATION_SHARE`] of the way back to [`NEUTRAL_LOG_LUMINANCE`],
/// within [`ADAPTATION_EV`].
pub fn adapt_to_the_frame(
    mut commands: Commands,
    cameras: Query<Entity, Added<FollowsDaylight>>,
    mut curves: ResMut<Assets<AutoExposureCompensationCurve>>,
    capture: Option<Res<CaptureRequest>>,
) {
    for camera in &cameras {
        let neutral = NEUTRAL_LOG_LUMINANCE;
        // The correction is the curve minus the frame's mean: a share of the way back to
        // the neutral, k (n − m), up to the bound, reached at the bound over k; beyond, the
        // curve follows the mean, the correction staying at the bound.
        let (low, high) = (neutral - METERED_EV, neutral + METERED_EV);
        let reach = ADAPTATION_EV / ADAPTATION_SHARE;
        let curve = AutoExposureCompensationCurve::from_curve(LinearSpline::new([
            vec2(low, low + ADAPTATION_EV),
            vec2(neutral - reach, neutral - reach + ADAPTATION_EV),
            vec2(neutral + reach, neutral + reach - ADAPTATION_EV),
            vec2(high, high - ADAPTATION_EV),
        ]))
        .expect("an increasing curve");
        let (brighten, darken) = if capture.is_some() {
            (CAPTURE_SPEED, CAPTURE_SPEED)
        } else {
            (SPEED_BRIGHTEN, SPEED_DARKEN)
        };
        commands.entity(camera).insert(AutoExposure {
            range: low..=high,
            speed_brighten: brighten,
            speed_darken: darken,
            compensation_curve: curves.add(curve),
            ..default()
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_daylight_keeps_its_exposure() {
        // The plain's afternoon, the high noon of the cumulus, the sun of the contre-jour.
        for h in [9.0, 40.0, 58.0, 90.0] {
            let lux = ground_illuminance_lux(h);
            assert_eq!(metered_ev100(lux), DAYLIGHT_EV100, "{h}°");
            assert!(look_ev(lux, h, 180.0, 1.0).abs() < 0.01, "{h}°");
        }
    }

    #[test]
    fn daylight_grows_with_the_sun_and_fades_through_twilight() {
        let heights = [-12.0, -6.0, -2.0, 0.0, 0.5, 5.0, 20.0, 40.0, 60.0, 90.0];
        let light: Vec<f32> = heights.iter().map(|h| ground_illuminance_lux(*h)).collect();
        assert!(light.windows(2).all(|w| w[0] < w[1]), "{light:?}");
        // Continuous at sunset.
        assert!((ground_illuminance_lux(1e-4) / ground_illuminance_lux(0.0) - 1.0).abs() < 0.05);
        // The end of civil twilight, and a high summer sun of about 100 klux.
        assert!((ground_illuminance_lux(-6.0) - 3.4).abs() < 0.2);
        assert!((80_000.0..130_000.0).contains(&ground_illuminance_lux(60.0)));
    }

    #[test]
    fn dusk_opens_the_base_and_darkens_the_look() {
        let at = |h: f32| metered_ev100(ground_illuminance_lux(h));
        let base: Vec<f32> = [8.0, 4.0, 0.0, -3.0, -6.0].iter().map(|h| at(*h)).collect();
        assert!(base.windows(2).all(|w| w[0] > w[1]), "{base:?}");
        // At sunset, the light is 3.6 stops below full daylight: the look shows 1.1.
        let sunset = ground_illuminance_lux(0.0);
        assert!((metered_ev100(sunset) - (DAYLIGHT_EV100 - 3.64)).abs() < 0.05);
        assert!((look_ev(sunset, 0.0, 180.0, 1.0) + 1.09).abs() < 0.05);
        // Deep in the night, the base stops at starlight and the look at its floor.
        assert_eq!(at(-30.0), at(-40.0));
        let night = ground_illuminance_lux(-30.0);
        assert_eq!(look_ev(night, -30.0, 180.0, 1.0), -NIGHT_DARKENING_EV);
    }

    #[test]
    fn the_sun_in_the_eye_closes_it_and_behind_it_does_not() {
        // The contre-jour: a sun 9° high, 16° from the line of sight.
        let facing = glare_ev(9.0, 16.0, 1.0);
        assert!((1.0..GLARE_MAX_EV + 1e-6).contains(&facing), "{facing}");
        // Further from the line of sight, less; the sun behind, nothing to speak of.
        assert!(glare_ev(9.0, 30.0, 1.0) < facing);
        assert!(glare_ev(40.0, 120.0, 1.0) < 0.05);
        // Looking straight at it, the bound; set, no glare.
        assert_eq!(glare_ev(9.0, 0.0, 1.0), GLARE_MAX_EV);
        assert_eq!(glare_ev(-2.0, 5.0, 1.0), 0.0);
        // Hidden, it closes nothing.
        assert_eq!(glare_ev(9.0, 16.0, 0.0), 0.0);
    }
}
