//! Wires thinner than a pixel (skill `rendu-visuel`, "phone-wire AA"). A flying wire of
//! 4.5 mm is a fifth of a pixel at 30 m: drawn as it is, it breaks into dashes and
//! shimmers. The meshes of a model's `wire_*` nodes take a material whose vertex shader
//! (`assets/shaders/wire_aa.wesl`) widens them to [`MIN_PIXELS`] on screen and whose
//! fragment shader cuts their opacity in proportion. Each node carries the diameter of
//! its wires in its glTF extras (`diameter_m`, written by `art/<id>/build.py`).
//!
//! Blended, the wires are not drawn by the normal prepass: they are lit with their own
//! normals (`own_normals`); drawn after the clouds' composite, they veil themselves with
//! the clouds' fog (`clouds::fog`).

use bevy::gltf::GltfExtras;
use bevy::mesh::MeshVertexBufferLayoutRef;
use bevy::pbr::{
    ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline,
};
use bevy::prelude::*;
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, ShaderType, SpecializedMeshPipelineError,
};
use bevy::render::storage::ShaderBuffer;
use bevy::shader::ShaderRef;

use super::clouds::CloudFog;
use super::extra_number;
use super::own_normals::use_own_normals;

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
    /// The clouds' fog (`clouds::fog`, `assets/shaders/clouds/fog.wesl`).
    #[texture(101)]
    pub cloud_light: Handle<Image>,
    #[texture(102, sample_type = "float", filterable = false)]
    pub cloud_distance: Handle<Image>,
    #[storage(103, read_only)]
    pub cloud_fog: Handle<ShaderBuffer>,
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
    pub fn new(diameter_m: f32, fog: &CloudFog) -> Self {
        Self {
            wire: WireUniform {
                radius: 0.5 * diameter_m,
                min_pixels: MIN_PIXELS,
                _padding_a: 0.0,
                _padding_b: 0.0,
            },
            cloud_light: fog.light.clone(),
            cloud_distance: fog.distance.clone(),
            cloud_fog: fog.fog.clone(),
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

    fn specialize(
        _pipeline: &MaterialExtensionPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialExtensionKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        use_own_normals(descriptor);
        Ok(())
    }
}

/// Gives the meshes of each new `wire_*` or `dmg_wire_*` node of a model the wire material, made from
/// their own material in alpha blending.
pub fn widen_wires(
    mut commands: Commands,
    nodes: Query<(&Name, &GltfExtras, &Children), Added<GltfExtras>>,
    meshes: Query<&MeshMaterial3d<StandardMaterial>>,
    standard: Res<Assets<StandardMaterial>>,
    fog: Res<CloudFog>,
    mut wires: ResMut<Assets<WireMaterial>>,
) {
    for (name, extras, children) in &nodes {
        // The wires, and their halves once cut.
        if !(name.as_str().starts_with("wire_") || name.as_str().starts_with("dmg_wire_")) {
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
                extension: WireAa::new(diameter as f32, &fog),
            });
            commands
                .entity(child)
                .remove::<MeshMaterial3d<StandardMaterial>>()
                .insert(MeshMaterial3d(material));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_shader_gets_the_radius() {
        let wire = WireAa::new(0.0045, &CloudFog::default()).wire;
        assert!((wire.radius - 0.00225).abs() < 1e-9);
        assert_eq!(wire.min_pixels, MIN_PIXELS);
    }
}
