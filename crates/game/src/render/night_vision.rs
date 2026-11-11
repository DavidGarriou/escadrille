//! Night vision (plan `m3-heure-du-jour.md`, art director's review of 2026-11-11, reserve
//! 2): as the light falls, the eye passes from the cones to the rods, and a night reads as
//! a night, not as an underexposed day. The colours go; the reds darken and the blues
//! brighten (the Purkinje shift); what is left takes a cold tinge.
//!
//! A pass over the frame before the tone mapping (Bevy's `FullscreenMaterial`,
//! `shaders/night_vision.wesl`), pixel by pixel, in the frame's linear light:
//! - **the cones' share** `m`, from the luminance the eye is adapted to, by the CIE's
//!   system for mesopic photometry (CIE 191:2010, [`mesopic_share`]): rods alone at
//!   0.005 cd/m² and below, cones alone at 5 and above. A pixel bright enough for the cones
//!   keeps its colour whatever the eye's adaptation: a fire, a lit instrument (`ESTIMÉ`);
//! - **the rods' luminance**, from the pixel's XYZ by Larson, Rushmeier and Piatko's
//!   formula (`scotopic_ratio`), scaled so that daylight's white keeps its luminance: a
//!   grey stays as bright, and the exposure, set on greys, still holds;
//! - **the rods' colour**: none, but seen as a cold grey, the white of a body at
//!   15 000 K (`ESTIMÉ`, judged on the dusk's contact sheet of 2026-11-11).
//!
//! By day the pass leaves the frame as it is. The functions the pass needs only on the GPU
//! are here for the tests, as the shader's reference (`seen`).

use std::f32::consts::PI;

use bevy::anti_alias::taa::temporal_anti_alias;
use bevy::camera::Exposure;
use bevy::core_pipeline::Core3dSystems;
use bevy::core_pipeline::fullscreen_material::FullscreenMaterial;
use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::BoxedSystem;
use bevy::prelude::*;
use bevy::render::RenderApp;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_resource::ShaderType;
use bevy::shader::ShaderRef;

use super::exposure::MEAN_REFLECTANCE;
use crate::camera::DAYLIGHT_EV100;

const SHADER: &str = "shaders/night_vision.wesl";

/// The CIE's mesopic adaptation, m = a + b log₁₀ L_mes, L_mes in cd/m² (CIE 191:2010):
/// 0 at 0.005 cd/m², 1 at 5.
const MESOPIC_A: f32 = 0.7670;
const MESOPIC_B: f32 = 0.3334;
/// The scotopic efficiency at 555 nm, where the photopic one peaks: V′(λ₀) = 683 / 1699.
const SCOTOPIC_AT_PEAK: f32 = 683.0 / 1699.0;
/// Steps of the CIE's iteration for the adaptation; it settles to 10⁻⁴ within ten.
const MESOPIC_STEPS: usize = 20;

/// Linear Rec. 709 (the frame's primaries, white D65) to CIE XYZ, and back.
#[cfg(test)]
const RGB_TO_XYZ: Mat3 = Mat3::from_cols_array(&[
    0.412_456_4,
    0.212_672_9,
    0.019_333_9,
    0.357_576_1,
    0.715_152_2,
    0.119_192,
    0.180_437_5,
    0.072_175,
    0.950_304_1,
]);
#[cfg(test)]
const XYZ_TO_RGB: Mat3 = Mat3::from_cols_array(&[
    3.240_454_2,
    -0.969_266,
    0.055_643_4,
    -1.537_138_5,
    1.876_010_8,
    -0.204_025_9,
    -0.498_531_4,
    0.041_556,
    1.057_225_2,
]);

/// Scotopic over photopic luminance of daylight's white, D65 (`scotopic_ratio`; the
/// lighting tables give about 2.5).
pub const DAYLIGHT_SCOTOPIC_RATIO: f32 = 2.5729;

/// The rods' grey, as seen, CIE 1931 xy: the white of a black body at 15 000 K (Kim et
/// al.'s fit of the Planckian locus; `ESTIMÉ`, judged with the art director on the dusk's
/// contact sheet of 2026-11-11: at 10 000 K, the 13 % of cones under a full moon, in its
/// warm light, left the clouds a neutral grey; at 15 000 K, a slate grey, not a violet).
#[cfg(test)]
const NIGHT_WHITE: Vec2 = Vec2::new(0.2637, 0.2672);

/// Scotopic over photopic luminance of a colour given in CIE XYZ: Larson, Rushmeier and
/// Piatko's fit, V = Y [1.33 (1 + (Y + Z) / X) − 1.68] ("A visibility matching tone
/// reproduction operator for high dynamic range scenes", IEEE TVCG 3, 1997). About 0.4
/// for the red of the frame, 7 for its blue.
#[cfg(test)]
fn scotopic_ratio(xyz: Vec3) -> f32 {
    if xyz.x <= 0.0 {
        return 0.0;
    }
    (1.33 * (1.0 + (xyz.y + xyz.z) / xyz.x) - 1.68).max(0.0)
}

/// The cones' share at a mesopic luminance `luminance`, cd/m², from 0 to 1.
fn cones_at(luminance: f32) -> f32 {
    (MESOPIC_A + MESOPIC_B * luminance.max(1e-9).log10()).clamp(0.0, 1.0)
}

/// The cones' share of an eye adapted to a field of `photopic` and `scotopic` luminance
/// (cd/m², scotopic in the rods' own unit): the CIE's recommended system for mesopic
/// photometry (CIE 191:2010), whose mesopic luminance and adaptation depend on each other,
/// found by iteration from m = 0.5.
pub fn mesopic_share(photopic: f32, scotopic: f32) -> f32 {
    let mut m = 0.5;
    for _ in 0..MESOPIC_STEPS {
        let weight = m + (1.0 - m) * SCOTOPIC_AT_PEAK;
        m = cones_at((m * photopic + (1.0 - m) * scotopic * SCOTOPIC_AT_PEAK) / weight);
    }
    m
}

/// The cones' share of the pilot's eye under `ground_lux` on level ground: adapted to a
/// grey card in that light, of daylight's colour (`ESTIMÉ` for the moon's and the
/// twilight's, a little redder and a little bluer).
pub fn adapted_share(ground_lux: f32) -> f32 {
    let photopic = MEAN_REFLECTANCE * ground_lux / PI;
    mesopic_share(photopic, photopic * DAYLIGHT_SCOTOPIC_RATIO)
}

/// cd/m² per unit of the frame before the tone mapping, the camera at `ev100`: Bevy's
/// exposure undone.
pub fn luminance_per_unit(ev100: f32) -> f32 {
    1.0 / Exposure { ev100 }.exposure()
}

/// The eye's state for the night vision pass.
#[derive(Component, ExtractComponent, Debug, Clone, Copy, PartialEq, ShaderType)]
#[extract_app(RenderApp)]
pub struct NightVision {
    /// cd/m² per unit of the frame ([`luminance_per_unit`]).
    pub to_luminance: f32,
    /// The cones' share of the eye's adaptation, 0 to 1 ([`adapted_share`]).
    pub adapted: f32,
}

impl Default for NightVision {
    /// Full daylight: the frame as it is.
    fn default() -> Self {
        Self {
            to_luminance: luminance_per_unit(DAYLIGHT_EV100),
            adapted: 1.0,
        }
    }
}

impl NightVision {
    /// The eye under `ground_lux`, the camera at `ev100`.
    pub fn under(ground_lux: f32, ev100: f32) -> Self {
        Self {
            to_luminance: luminance_per_unit(ev100),
            adapted: adapted_share(ground_lux),
        }
    }
}

impl FullscreenMaterial for NightVision {
    fn fragment_shader() -> ShaderRef {
        SHADER.into()
    }

    /// After the anti-aliasing, before the adaptation reads the frame and the bloom spreads
    /// it.
    fn schedule_configs(system: ScheduleConfigs<BoxedSystem>) -> ScheduleConfigs<BoxedSystem> {
        system
            .in_set(Core3dSystems::EarlyPostProcess)
            .after(temporal_anti_alias)
    }
}

/// A pixel of the frame, linear `rgb`, as the eye sees it: the shader's reference.
#[cfg(test)]
fn seen(rgb: Vec3, vision: NightVision) -> Vec3 {
    let xyz = RGB_TO_XYZ * rgb.max(Vec3::ZERO);
    let cones = vision.adapted.max(cones_at(xyz.y * vision.to_luminance));
    if cones >= 1.0 {
        return rgb;
    }
    let rods = xyz.y * scotopic_ratio(xyz) / DAYLIGHT_SCOTOPIC_RATIO;
    let grey = rods
        * Vec3::new(
            NIGHT_WHITE.x / NIGHT_WHITE.y,
            1.0,
            (1.0 - NIGHT_WHITE.x - NIGHT_WHITE.y) / NIGHT_WHITE.y,
        );
    (XYZ_TO_RGB * grey.lerp(xyz, cones)).max(Vec3::ZERO)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn luminance(rgb: Vec3) -> f32 {
        (RGB_TO_XYZ * rgb).y
    }

    /// The frame at night: a grey card under a full moon's 0.1 lux, at the camera's
    /// exposure of then.
    fn moonlit() -> NightVision {
        NightVision::under(0.1, super::super::exposure::metered_ev100(0.1))
    }

    #[test]
    fn the_matrices_are_inverse_and_white_is_d65() {
        let white = RGB_TO_XYZ * Vec3::ONE;
        assert!((white - Vec3::new(0.9505, 1.0, 1.0888)).abs().max_element() < 1e-3);
        let back = XYZ_TO_RGB * RGB_TO_XYZ;
        assert!(
            (back - Mat3::IDENTITY)
                .abs()
                .to_cols_array()
                .iter()
                .all(|&e| e < 1e-4)
        );
    }

    #[test]
    fn larson_s_ratio_of_daylight_is_the_lighting_tables_s() {
        assert!((scotopic_ratio(RGB_TO_XYZ * Vec3::ONE) - DAYLIGHT_SCOTOPIC_RATIO).abs() < 1e-3);
        // The frame's red is a quarter of its green to the rods, its blue more than twice.
        let ratio = |rgb: Vec3| scotopic_ratio(RGB_TO_XYZ * rgb);
        assert!(
            ratio(Vec3::X) < 0.5 && ratio(Vec3::Z) > 7.0,
            "{} {}",
            ratio(Vec3::X),
            ratio(Vec3::Z)
        );
    }

    #[test]
    fn the_cie_adaptation_spans_from_rods_to_cones() {
        assert_eq!(mesopic_share(0.001, 0.0025), 0.0);
        assert_eq!(mesopic_share(10.0, 25.0), 1.0);
        // Its two ends hold for any colour of light.
        assert!(cones_at(0.005) < 1e-3 && cones_at(5.0) > 0.999);
        // At 0.1 cd/m², about half way.
        let m = mesopic_share(0.1, 0.25);
        assert!((0.3..0.6).contains(&m), "{m}");
    }

    #[test]
    fn day_and_dusk_are_photopic_and_a_moonlit_night_mostly_scotopic() {
        assert_eq!(adapted_share(10_000.0), 1.0);
        // The sun at the horizon, about 800 lux.
        assert_eq!(adapted_share(800.0), 1.0);
        // Civil twilight's end, 3.4 lux: half way.
        let dusk = adapted_share(3.4);
        assert!((0.3..0.7).contains(&dusk), "{dusk}");
        let moon = adapted_share(0.1);
        assert!((0.05..0.25).contains(&moon), "{moon}");
        assert_eq!(adapted_share(0.001), 0.0);
    }

    #[test]
    fn by_day_the_frame_is_left_as_it_is() {
        let day = NightVision::default();
        for rgb in [
            Vec3::new(0.8, 0.2, 0.1),
            Vec3::new(1e-6, 2e-6, 0.0),
            Vec3::ZERO,
        ] {
            assert_eq!(seen(rgb, day), rgb);
        }
    }

    #[test]
    fn at_night_a_grey_stays_as_bright_and_turns_cold() {
        let grey = Vec3::splat(1e-4);
        let night = seen(grey, moonlit());
        assert!((luminance(night) / luminance(grey) - 1.0).abs() < 0.02);
        assert!(night.z > night.x, "{night}");
    }

    #[test]
    fn at_night_the_reds_darken_and_the_blues_brighten() {
        let red = Vec3::new(1e-4, 0.2e-4, 0.1e-4);
        let blue = Vec3::new(0.1e-4, 0.2e-4, 1e-4);
        let night = moonlit();
        assert!(luminance(seen(red, night)) < 0.6 * luminance(red));
        assert!(luminance(seen(blue, night)) > 1.5 * luminance(blue));
        // And their colours go.
        let red = seen(red, night);
        assert!(red.x / red.z < 2.0, "{red}");
    }

    #[test]
    fn at_night_a_light_bright_enough_for_the_cones_keeps_its_colour() {
        let night = moonlit();
        // 10 cd/m² of orange fire, in the frame's units.
        let fire = Vec3::new(1.0, 0.4, 0.1) * 10.0
            / night.to_luminance
            / luminance(Vec3::new(1.0, 0.4, 0.1));
        assert_eq!(seen(fire, night), fire);
    }
}
