//! The ground under the clouds. Bevy lights every surface with one environment map, seen
//! from the camera, into which the clouds are drawn (`clouds`). Above the layer the camera
//! sees a clear sky overhead, but the ground beneath the layer sees the clouds' bases:
//! their light, which the map misses, is added to the ground's material
//! ([`CloudLitGround`]), in the share of the layer the camera is above
//! ([`super::clouds`] sets [`GroundLight`]). Below the bases the map already holds them.

use bevy::pbr::{ExtendedMaterial, MaterialExtension};
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

const SHADER: &str = "shaders/ground.wesl";

/// A standard material that also takes the clouds' light from above.
pub type GroundMaterial = ExtendedMaterial<StandardMaterial, CloudLitGround>;

#[derive(Asset, AsBindGroup, Reflect, Debug, Clone, Default)]
pub struct CloudLitGround {
    /// Radiance of the clouds' bases over the ground that the environment map misses,
    /// cd/m² (rgb; a unused).
    #[uniform(100)]
    pub light: Vec4,
}

impl MaterialExtension for CloudLitGround {
    fn fragment_shader() -> ShaderRef {
        SHADER.into()
    }
}

/// The ground: its material takes the clouds' light.
#[derive(Component, Debug, Clone, Copy, Default)]
pub struct Ground;

/// Mean albedo of the ground, linear: the fields seen below the horizon of the
/// environment map. By default the clouds' own ground (`escadrille_weather::scattering`).
#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub struct GroundAlbedo(pub Vec3);

impl Default for GroundAlbedo {
    fn default() -> Self {
        Self(Vec3::splat(escadrille_weather::scattering::GROUND_ALBEDO))
    }
}

/// The clouds' light the ground takes this frame, cd/m²: none under a clear sky.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Default)]
pub struct GroundLight(pub Vec3);

/// The ground's meshes whose standard material is new.
type NewGrounds<'w, 's> = Query<
    'w,
    's,
    (Entity, &'static MeshMaterial3d<StandardMaterial>),
    (With<Ground>, Changed<MeshMaterial3d<StandardMaterial>>),
>;

/// Gives the ground's new material the clouds' light, one copy per material.
pub fn light_ground(
    mut commands: Commands,
    grounds: NewGrounds,
    standard: Res<Assets<StandardMaterial>>,
    mut lit: ResMut<Assets<GroundMaterial>>,
    mut copies: Local<HashMap<AssetId<StandardMaterial>, Handle<GroundMaterial>>>,
) {
    for (ground, material) in &grounds {
        let Some(base) = standard.get(&material.0) else {
            continue;
        };
        let copy = copies
            .entry(material.0.id())
            .or_insert_with(|| {
                lit.add(ExtendedMaterial {
                    base: base.clone(),
                    extension: CloudLitGround::default(),
                })
            })
            .clone();
        commands
            .entity(ground)
            .remove::<MeshMaterial3d<StandardMaterial>>()
            .insert(MeshMaterial3d(copy));
    }
}

/// The clouds' light into the ground's materials, when it changes.
pub fn update_ground_light(
    light: Res<GroundLight>,
    grounds: Query<&MeshMaterial3d<GroundMaterial>>,
    mut materials: ResMut<Assets<GroundMaterial>>,
) {
    let wanted = light.0.extend(0.0);
    for material in &grounds {
        // Look before taking it mutably: taking it uploads it again.
        if materials
            .get(&material.0)
            .is_some_and(|m| m.extension.light != wanted)
            && let Some(mut m) = materials.get_mut(&material.0)
        {
            m.extension.light = wanted;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_ground_takes_the_clouds_light() {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()))
            .init_asset::<StandardMaterial>()
            .init_asset::<GroundMaterial>()
            .insert_resource(GroundLight(Vec3::splat(2_000.0)))
            .add_systems(Update, (light_ground, update_ground_light).chain());
        let world = app.world_mut();
        let fields = world
            .resource_mut::<Assets<StandardMaterial>>()
            .add(StandardMaterial::default());
        let ground = world.spawn((Ground, MeshMaterial3d(fields.clone()))).id();
        let wing = world.spawn(MeshMaterial3d(fields)).id();
        app.update();
        app.update();
        let world = app.world();
        let lit = world.get::<MeshMaterial3d<GroundMaterial>>(ground).unwrap();
        let light = world
            .resource::<Assets<GroundMaterial>>()
            .get(&lit.0)
            .unwrap();
        assert_eq!(
            light.extension.light,
            Vec4::new(2_000.0, 2_000.0, 2_000.0, 0.0)
        );
        // Only the ground.
        assert!(
            world
                .get::<MeshMaterial3d<StandardMaterial>>(wing)
                .is_some()
        );
    }
}
