//! Rendering: how the game's assets are prepared for the GPU (skill `rendu-visuel`).

pub mod clouds;
pub mod damage;
pub mod exposure;
pub mod fabric;
pub mod ground;
pub mod mipmaps;
pub mod night;
pub mod own_normals;
pub mod quality;
pub mod wires;

use bevy::prelude::*;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            MaterialPlugin::<wires::WireMaterial>::default(),
            MaterialPlugin::<own_normals::OwnNormalsMaterial>::default(),
            MaterialPlugin::<ground::GroundMaterial>::default(),
            MaterialPlugin::<night::MoonMaterial>::default(),
            MaterialPlugin::<night::StarsMaterial>::default(),
            clouds::CloudsPlugin,
            bevy::post_process::auto_exposure::AutoExposurePlugin,
        ))
        .init_resource::<quality::Quality>()
        .init_resource::<ground::GroundAlbedo>()
        .init_resource::<ground::GroundLight>()
        .add_observer(damage::hide_damage_nodes)
        .add_systems(PreStartup, quality::apply_quality)
        .add_systems(
            Update,
            (
                mipmaps::fill_mip_chains,
                wires::widen_wires,
                fabric::translucent_fabric,
                quality::apply_occlusion_quality,
                own_normals::light_with_own_normals,
                (ground::light_ground, ground::update_ground_light).chain(),
                (night::spawn_night_sky, night::place_night_sky).chain(),
                (exposure::adapt_to_the_frame, exposure::follow_daylight)
                    .after(crate::world::sky::light_the_sky),
            ),
        );
    }
}

/// A number of a flat JSON object, as glTF extras hold them: `{"diameter_m": 0.0045}`.
pub fn extra_number(json: &str, key: &str) -> Option<f64> {
    let quoted = format!("\"{key}\"");
    let rest = &json[json.find(&quoted)? + quoted.len()..];
    let rest = rest.trim_start().strip_prefix(':')?.trim_start();
    let end = rest
        .find(|c: char| !(c.is_ascii_digit() || "+-.eE".contains(c)))
        .unwrap_or(rest.len());
    rest[..end].parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_diameter_is_read_from_the_extras() {
        assert_eq!(
            extra_number(r#"{"diameter_m":0.0045}"#, "diameter_m"),
            Some(0.0045)
        );
        assert_eq!(
            extra_number(r#"{ "other": 2, "diameter_m" : 3.5e-3 }"#, "diameter_m"),
            Some(0.0035)
        );
        assert_eq!(extra_number(r#"{"other": 2}"#, "diameter_m"), None);
        assert_eq!(
            extra_number(r#"{"diameter_m": "thin"}"#, "diameter_m"),
            None
        );
    }
}
