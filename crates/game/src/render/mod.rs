//! Rendering: how the game's assets are prepared for the GPU (skill `rendu-visuel`).

pub mod fabric;
pub mod mipmaps;
pub mod wires;

use bevy::prelude::*;

pub struct RenderingPlugin;

impl Plugin for RenderingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<wires::WireMaterial>::default())
            .add_systems(
                Update,
                (
                    mipmaps::fill_mip_chains,
                    wires::widen_wires,
                    fabric::translucent_fabric,
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
