//! Quality settings (skill `rendu-visuel`, « Performances »; plan `m3-beaute.md`, step 0):
//! one level for the whole renderer, read by each costly feature. The reference is
//! « haut » at 60 frames a second in 1440p on the RX 7800 XT.

use bevy::light::DirectionalLightShadowMap;
use bevy::pbr::{ScreenSpaceAmbientOcclusion, ScreenSpaceAmbientOcclusionQualityLevel};
use bevy::prelude::*;
use clap::ValueEnum;

#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum Quality {
    #[value(name = "bas")]
    Low,
    #[value(name = "moyen")]
    Medium,
    /// The reference of the performance budget.
    #[default]
    #[value(name = "haut")]
    High,
    #[value(name = "ultra")]
    Ultra,
}

impl Quality {
    /// Its name, as the command line and the capture report give it.
    pub fn name(self) -> &'static str {
        match self {
            Self::Low => "bas",
            Self::Medium => "moyen",
            Self::High => "haut",
            Self::Ultra => "ultra",
        }
    }

    /// Steps of a ray through the clouds, at most, and towards the sun (ADR 0009).
    pub fn cloud_steps(self) -> (u32, u32) {
        match self {
            Self::Low => (48, 4),
            Self::Medium => (64, 5),
            Self::High => (96, 6),
            Self::Ultra => (160, 8),
        }
    }

    /// Divisor of the resolution of the clouds' ray march, on each side: 2 marches a
    /// quarter of the pixels, which the temporal pass and the upsampling make up for.
    pub fn cloud_divisor(self) -> u32 {
        match self {
            Self::Low => 3,
            Self::Medium | Self::High => 2,
            Self::Ultra => 1,
        }
    }

    /// Side of each cascade of the sun's shadow map, texels: « haut » keeps Bevy's own.
    pub fn shadow_map(self) -> usize {
        match self {
            Self::Low => 1024,
            Self::Medium => 2048,
            Self::High => 2048,
            Self::Ultra => 4096,
        }
    }

    /// Samples of the ambient occlusion: Bevy's levels, one for one.
    pub fn occlusion(self) -> ScreenSpaceAmbientOcclusionQualityLevel {
        match self {
            Self::Low => ScreenSpaceAmbientOcclusionQualityLevel::Low,
            Self::Medium => ScreenSpaceAmbientOcclusionQualityLevel::Medium,
            Self::High => ScreenSpaceAmbientOcclusionQualityLevel::High,
            Self::Ultra => ScreenSpaceAmbientOcclusionQualityLevel::Ultra,
        }
    }
}

/// The sun's shadow map at the level asked for.
pub fn apply_quality(quality: Res<Quality>, mut shadows: ResMut<DirectionalLightShadowMap>) {
    shadows.size = quality.shadow_map();
}

/// The ambient occlusion of each new camera at the level asked for.
pub fn apply_occlusion_quality(
    quality: Res<Quality>,
    mut cameras: Query<&mut ScreenSpaceAmbientOcclusion, Added<ScreenSpaceAmbientOcclusion>>,
) {
    for mut occlusion in &mut cameras {
        occlusion.quality_level = quality.occlusion();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_levels_climb_and_the_reference_keeps_bevy_s_shadows() {
        let levels = [Quality::Low, Quality::Medium, Quality::High, Quality::Ultra];
        for pair in levels.windows(2) {
            assert!(pair[0].shadow_map() <= pair[1].shadow_map());
        }
        assert_eq!(
            Quality::High.shadow_map(),
            DirectionalLightShadowMap::default().size
        );
        assert!(levels.iter().all(|q| q.shadow_map().is_power_of_two()));
    }

    #[test]
    fn the_clouds_march_at_a_quarter_of_the_pixels_on_the_reference() {
        let levels = [Quality::Low, Quality::Medium, Quality::High, Quality::Ultra];
        assert_eq!(Quality::High.cloud_divisor(), 2);
        assert!(
            levels
                .windows(2)
                .all(|pair| pair[0].cloud_divisor() >= pair[1].cloud_divisor())
        );
    }

    #[test]
    fn the_occlusion_takes_the_level_of_its_camera() {
        let mut app = App::new();
        app.insert_resource(Quality::Low)
            .add_systems(Update, apply_occlusion_quality);
        let camera = app
            .world_mut()
            .spawn(ScreenSpaceAmbientOcclusion::default())
            .id();
        app.update();
        let occlusion = app
            .world()
            .get::<ScreenSpaceAmbientOcclusion>(camera)
            .unwrap();
        assert_eq!(
            occlusion.quality_level,
            ScreenSpaceAmbientOcclusionQualityLevel::Low
        );
        assert_eq!(
            Quality::High.occlusion(),
            ScreenSpaceAmbientOcclusionQualityLevel::default()
        );
    }

    #[test]
    fn the_clouds_get_more_steps_as_the_level_climbs() {
        let levels = [Quality::Low, Quality::Medium, Quality::High, Quality::Ultra];
        for pair in levels.windows(2) {
            let (low, high) = (pair[0].cloud_steps(), pair[1].cloud_steps());
            assert!(low.0 < high.0 && low.1 < high.1);
        }
    }
}
