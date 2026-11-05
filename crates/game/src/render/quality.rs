//! Quality settings (skill `rendu-visuel`, « Performances »; plan `m3-beaute.md`, step 0):
//! one level for the whole renderer, read by each costly feature. The reference is
//! « haut » at 60 frames a second in 1440p on the RX 7800 XT.

use bevy::light::DirectionalLightShadowMap;
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

    /// Side of each cascade of the sun's shadow map, texels: « haut » keeps Bevy's own.
    pub fn shadow_map(self) -> usize {
        match self {
            Self::Low => 1024,
            Self::Medium => 2048,
            Self::High => 2048,
            Self::Ultra => 4096,
        }
    }
}

/// The sun's shadow map at the level asked for.
pub fn apply_quality(quality: Res<Quality>, mut shadows: ResMut<DirectionalLightShadowMap>) {
    shadows.size = quality.shadow_map();
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
}
