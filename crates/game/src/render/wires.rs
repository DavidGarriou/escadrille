//! Wires thinner than a pixel (skill `rendu-visuel`, "phone-wire AA"). A flying wire of
//! 4.5 mm is a fifth of a pixel at 30 m: drawn as it is, it breaks into dashes and
//! shimmers. The meshes of a model's `wire_*` nodes take a material whose vertex shader
//! (`assets/shaders/wire_aa.wesl`) widens them to [`MIN_PIXELS`] on screen and whose
//! fragment shader cuts their opacity in proportion. Each node carries the diameter of
//! its wires in its glTF extras (`diameter_m`, written by `art/<id>/build.py`).

use bevy::gltf::GltfExtras;
use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;

const SHADER: &str = "shaders/wire_aa.wesl";

/// Smallest width of a wire on screen, pixels: one and a half, as a six-sided tube is
/// only 0.87 of its diameter across its flats, and a line under a pixel wide misses
/// pixel centres and breaks into dashes (seen on the captures at 1.0).
pub const MIN_PIXELS: f32 = 1.5;

pub type WireMaterial = ExtendedMaterial<StandardMaterial, WireAa>;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone)]
pub struct WireAa {
    #[uniform(100)]
    pub wire: WireUniform,
}

/// The shader's `Wire`, padded to 16 bytes.
#[derive(ShaderType, Reflect, Debug, Clone, Copy, PartialEq)]
pub struct WireUniform {
    /// True radius of the wire, m.
    pub radius: f32,
    pub min_pixels: f32,
    pub _padding_a: f32,
    pub _padding_b: f32,
}

impl WireAa {
    pub fn new(diameter_m: f32) -> Self {
        Self {
            wire: WireUniform {
                radius: 0.5 * diameter_m,
                min_pixels: MIN_PIXELS,
                _padding_a: 0.0,
                _padding_b: 0.0,
            },
        }
    }
}

impl MaterialExtension for WireAa {
    fn vertex_shader() -> ShaderRef {
        SHADER.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER.into()
    }
}

/// Gives the meshes of each new `wire_*` node of a model the wire material, made from
/// their own material in alpha blending.
pub fn widen_wires(
    mut commands: Commands,
    nodes: Query<(&Name, &GltfExtras, &Children), Added<GltfExtras>>,
    meshes: Query<&MeshMaterial3d<StandardMaterial>>,
    standard: Res<Assets<StandardMaterial>>,
    mut wires: ResMut<Assets<WireMaterial>>,
) {
    for (name, extras, children) in &nodes {
        if !name.as_str().starts_with("wire_") {
            continue;
        }
        let Some(diameter) = extra_number(&extras.value, "diameter_m") else {
            warn!("{name}: no diameter_m in its glTF extras, drawn as it is");
            continue;
        };
        for child in children.iter() {
            let Some(base) = meshes.get(child).ok().and_then(|m| standard.get(&m.0)) else {
                continue;
            };
            let material = wires.add(ExtendedMaterial {
                base: StandardMaterial {
                    alpha_mode: AlphaMode::Blend,
                    ..base.clone()
                },
                extension: WireAa::new(diameter as f32),
            });
            commands
                .entity(child)
                .remove::<MeshMaterial3d<StandardMaterial>>()
                .insert(MeshMaterial3d(material));
        }
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

    #[test]
    fn the_shader_gets_the_radius() {
        let wire = WireAa::new(0.0045).wire;
        assert!((wire.radius - 0.00225).abs() < 1e-9);
        assert_eq!(wire.min_pixels, MIN_PIXELS);
    }
}
