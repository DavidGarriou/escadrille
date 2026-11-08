//! Materials lit with their own normals rather than the normal prepass's.
//!
//! The ambient occlusion requires a normal prepass, and with one Bevy 0.20's forward pass
//! loads each pixel's normal from it instead of the mesh's (`LOAD_PREPASS_NORMALS`,
//! `bevy_pbr` 0.20.0, `render/mesh.rs` and `render/pbr_fragment.wesl`). Two kinds of
//! material go wrong:
//! - the transparent ones, which the prepass does not draw: a blended surface takes the
//!   normal of what lies behind it, or of no surface over the sky. The propeller's disc,
//!   the windscreen and the wires were lit as the ground or the fuselage behind them;
//! - the clearcoat, whose normal is only set when the mesh's is used: left at zero, it
//!   turned the varnished propeller into a white mirror.
//!
//! Bevy itself drops the prepass's normals for specular transmission only. These
//! materials drop them too: [`OwnNormals`] removes the shader definition from their
//! pipelines, and [`light_with_own_normals`] gives it to each material that needs it.
//!
//! Its fragment shader (`assets/shaders/own_normals.wesl`) is Bevy's, but for one step:
//! drawn after the clouds' composite, the blended ones veil themselves with the clouds'
//! fog (`clouds::fog`).

use bevy::mesh::MeshVertexBufferLayoutRef;
use bevy::pbr::{
    ExtendedMaterial, MaterialExtension, MaterialExtensionKey, MaterialExtensionPipeline,
};
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError,
};
use bevy::render::storage::ShaderBuffer;
use bevy::shader::{ShaderDefVal, ShaderRef};

use super::clouds::CloudFog;

const SHADER: &str = "shaders/own_normals.wesl";

/// A standard material lit with its own normals.
pub type OwnNormalsMaterial = ExtendedMaterial<StandardMaterial, OwnNormals>;

/// The clouds' fog (`clouds::fog`, `assets/shaders/clouds/fog.wesl`).
#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
pub struct OwnNormals {
    #[texture(101)]
    pub cloud_light: Handle<Image>,
    #[texture(102, sample_type = "float", filterable = false)]
    pub cloud_distance: Handle<Image>,
    #[storage(103, read_only)]
    pub cloud_fog: Handle<ShaderBuffer>,
}

impl OwnNormals {
    pub fn new(fog: &CloudFog) -> Self {
        Self {
            cloud_light: fog.light.clone(),
            cloud_distance: fog.distance.clone(),
            cloud_fog: fog.fog.clone(),
        }
    }
}

impl MaterialExtension for OwnNormals {
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

/// The fragment shader of the pipeline computes its normals from the mesh.
pub fn use_own_normals(descriptor: &mut RenderPipelineDescriptor) {
    if let Some(fragment) = descriptor.fragment.as_mut() {
        let load = ShaderDefVal::from("LOAD_PREPASS_NORMALS");
        fragment.shader_defs.retain(|def| def != &load);
    }
}

/// Whether a material needs its own normals: blended, or varnished.
pub fn needs_own_normals(material: &StandardMaterial) -> bool {
    let blended = matches!(
        material.alpha_mode,
        AlphaMode::Blend | AlphaMode::Premultiplied | AlphaMode::Add | AlphaMode::Multiply
    );
    blended || material.clearcoat > 0.0
}

/// Gives each mesh whose new material needs its own normals the same material lit with
/// them, one copy per material.
pub fn light_with_own_normals(
    mut commands: Commands,
    meshes: Query<
        (Entity, &MeshMaterial3d<StandardMaterial>),
        Changed<MeshMaterial3d<StandardMaterial>>,
    >,
    standard: Res<Assets<StandardMaterial>>,
    fog: Res<CloudFog>,
    mut own: ResMut<Assets<OwnNormalsMaterial>>,
    mut copies: Local<HashMap<AssetId<StandardMaterial>, Handle<OwnNormalsMaterial>>>,
) {
    for (mesh, material) in &meshes {
        let Some(base) = standard.get(&material.0) else {
            continue;
        };
        if !needs_own_normals(base) {
            continue;
        }
        let copy = copies
            .entry(material.0.id())
            .or_insert_with(|| {
                own.add(ExtendedMaterial {
                    base: base.clone(),
                    extension: OwnNormals::new(&fog),
                })
            })
            .clone();
        commands
            .entity(mesh)
            .remove::<MeshMaterial3d<StandardMaterial>>()
            .insert(MeshMaterial3d(copy));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::render::render_resource::FragmentState;

    #[test]
    fn the_blended_and_the_varnished_need_their_own_normals() {
        let glass = StandardMaterial {
            alpha_mode: AlphaMode::Blend,
            ..default()
        };
        let varnish = StandardMaterial {
            clearcoat: 1.0,
            ..default()
        };
        let linen = StandardMaterial::default();
        let cut_out = StandardMaterial {
            alpha_mode: AlphaMode::Mask(0.5),
            ..default()
        };
        assert!(needs_own_normals(&glass));
        assert!(needs_own_normals(&varnish));
        // Drawn by the prepass: its normals are their own.
        assert!(!needs_own_normals(&linen));
        assert!(!needs_own_normals(&cut_out));
    }

    #[test]
    fn the_prepass_normals_are_no_longer_loaded() {
        let mut descriptor = RenderPipelineDescriptor {
            fragment: Some(FragmentState {
                shader_defs: vec!["LOAD_PREPASS_NORMALS".into(), "VERTEX_UVS".into()],
                ..default()
            }),
            ..default()
        };
        use_own_normals(&mut descriptor);
        let defs = &descriptor.fragment.unwrap().shader_defs;
        assert_eq!(defs, &vec![ShaderDefVal::from("VERTEX_UVS")]);
    }

    #[test]
    fn a_shared_material_is_copied_once() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_asset::<StandardMaterial>()
            .init_asset::<OwnNormalsMaterial>()
            .init_resource::<CloudFog>()
            .add_systems(Update, light_with_own_normals);
        let world = app.world_mut();
        let glass = world
            .resource_mut::<Assets<StandardMaterial>>()
            .add(StandardMaterial {
                alpha_mode: AlphaMode::Blend,
                ..default()
            });
        let linen = world
            .resource_mut::<Assets<StandardMaterial>>()
            .add(StandardMaterial::default());
        let panes = [
            world.spawn(MeshMaterial3d(glass.clone())).id(),
            world.spawn(MeshMaterial3d(glass)).id(),
        ];
        let wing = world.spawn(MeshMaterial3d(linen)).id();
        app.update();
        let world = app.world();
        let own: Vec<_> = panes
            .iter()
            .map(|e| {
                world
                    .get::<MeshMaterial3d<OwnNormalsMaterial>>(*e)
                    .unwrap()
                    .0
                    .clone()
            })
            .collect();
        assert_eq!(own[0], own[1]);
        assert_eq!(world.resource::<Assets<OwnNormalsMaterial>>().len(), 1);
        assert!(
            world
                .get::<MeshMaterial3d<StandardMaterial>>(panes[0])
                .is_none()
        );
        assert!(
            world
                .get::<MeshMaterial3d<StandardMaterial>>(wing)
                .is_some()
        );
    }
}
