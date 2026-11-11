//! The sun's glare veil (plan `m3-heure-du-jour.md`, art director's review of 2026-11-11):
//! the light the eye scatters around the sun, added over the whole frame. The
//! Stiles–Holladay formula of disability glare (CIE 146:2002) gives it, L = 10 E / θ²
//! cd/m², E the sun's illuminance at the eye, lux, θ the angle from the sun, degrees: the
//! same as the glare that closes the eye (`exposure::glare_ev`). With the closing alone,
//! the sky round the sun turned slate; with the veil, it stays bright while the eye
//! closes, and an enemy diving from the sun is lost in it.
//!
//! The veil goes when the sun is hidden: by an object (the depth at five points of its
//! disc) or by the clouds (their transmittance at its pixel). Off the screen, the sun is
//! taken as seen.

use bevy::asset::RenderAssetUsages;
use bevy::camera::visibility::NoFrustumCulling;
use bevy::ecs::system::SystemParam;
use bevy::light::{NotShadowCaster, NotShadowReceiver, SunDisk};
use bevy::mesh::{Indices, MeshVertexBufferLayoutRef, PrimitiveTopology};
use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::render::render_resource::{
    AsBindGroup, CompareFunction, RenderPipelineDescriptor, SpecializedMeshPipelineError,
};
use bevy::render::storage::ShaderBuffer;
use bevy::shader::ShaderRef;

use super::clouds::CloudFog;
use super::exposure::{FollowsDaylight, direct_normal_lux};
use crate::world::atmosphere::AIR_SCATTERING;
use crate::world::sky::{Sky, towards};

const SHADER: &str = "shaders/glare.wesl";

/// The Stiles–Holladay coefficient, cd/m² per lux and per degree squared.
pub const HOLLADAY: f32 = 10.0;
/// The formula holds from 1° from the sun: nearer, the veil stays at its 1° value.
pub const NEAREST_DEG: f32 = 1.0;
/// Drawn after every other transparent surface: the veil is in the eye.
const LAST: f32 = 1.0e9;
/// The scale height of the air, m, for the sun's colour through it.
const AIR_SCALE_HEIGHT_M: f32 = 8_000.0;

/// The veil's material: the sun's direction, its veil's strength, and the clouds' image to
/// tell whether they hide it.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct GlareVeil {
    /// Direction towards the sun, Bevy axes (xyz; w unused).
    #[uniform(0)]
    pub to_sun: Vec4,
    /// HOLLADAY × the sun's illuminance at the eye, per channel (rgb), and the nearest
    /// angle, degrees (a).
    #[uniform(1)]
    pub veil: Vec4,
    #[texture(2, sample_type = "float", filterable = false)]
    pub cloud_light: Handle<Image>,
    #[storage(3, read_only)]
    pub cloud_fog: Handle<ShaderBuffer>,
}

impl Material for GlareVeil {
    fn vertex_shader() -> ShaderRef {
        SHADER.into()
    }

    fn fragment_shader() -> ShaderRef {
        SHADER.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        // Premultiplied with an alpha of 0: added to the frame.
        AlphaMode::Premultiplied
    }

    fn depth_bias(&self) -> f32 {
        LAST
    }

    fn enable_prepass() -> bool {
        false
    }

    fn enable_shadows() -> bool {
        false
    }

    fn specialize(
        _pipeline: &MaterialPipeline,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = None;
        if let Some(depth) = descriptor.depth_stencil.as_mut() {
            depth.depth_compare = Some(CompareFunction::Always);
            depth.depth_write_enabled = Some(false);
        }
        Ok(())
    }
}

/// The glare veil's entity.
#[derive(Component, Debug, Clone, Copy)]
pub struct Glare;

/// The sun's illuminance at the eye, lux, per channel, the sun at `elevation_deg`: the IES
/// clear sky's ([`direct_normal_lux`]) for its luminance, its colour from the air's
/// scattering along the air mass (bluer light scattered out first; the haze takes all
/// colours alike).
pub fn sun_at_the_eye(elevation_deg: f32) -> Vec3 {
    let lux = direct_normal_lux(elevation_deg);
    if lux <= 0.0 {
        return Vec3::ZERO;
    }
    let air_mass = 1.0 / elevation_deg.to_radians().sin().max(0.03);
    let through = (-AIR_SCATTERING * AIR_SCALE_HEIGHT_M * air_mass).exp();
    lux * through / through.y
}

/// A triangle that covers the screen in clip space.
fn screen_triangle() -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![[-1.0, -1.0, 0.0], [3.0, -1.0, 0.0], [-1.0, 3.0, 0.0]],
    )
    .with_inserted_indices(Indices::U16(vec![0, 1, 2]))
}

/// Where the sun is read: the sky's clock, or the light that has a disc.
#[derive(SystemParam)]
pub struct SunSource<'w, 's> {
    sky: Option<Res<'w, Sky>>,
    suns: Query<'w, 's, &'static GlobalTransform, (With<DirectionalLight>, With<SunDisk>)>,
}

/// The sun now: towards it, and its height, degrees. From the sky's clock when the scene
/// has one (none at night: the light is the moon's), else from the light that has a disc
/// (the scenes built in code).
fn the_sun(source: &SunSource) -> Option<(Vec3, f32)> {
    match source.sky.as_deref() {
        Some(sky) => Some((towards(sky.sun), sky.sun.elevation_deg as f32)),
        None => source.suns.iter().next().map(|sun| {
            // The light shines along its forward axis: the sun lies behind it.
            let to_sun = *sun.back();
            (to_sun, to_sun.y.clamp(-1.0, 1.0).asin().to_degrees())
        }),
    }
}

/// The veil's values for a sun.
fn veil_of((to_sun, elevation): (Vec3, f32)) -> (Vec4, Vec4) {
    let sun = sun_at_the_eye(elevation);
    (to_sun.extend(0.0), (HOLLADAY * sun).extend(NEAREST_DEG))
}

/// The veil, once the scene has a camera that follows the daylight.
pub fn spawn_glare(
    mut commands: Commands,
    source: SunSource,
    fog: Option<Res<CloudFog>>,
    cameras: Query<(), With<FollowsDaylight>>,
    existing: Query<(), With<Glare>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<GlareVeil>>,
) {
    let Some(fog) = fog else {
        return;
    };
    if cameras.is_empty() || !existing.is_empty() {
        return;
    }
    let Some(sun) = the_sun(&source) else {
        return;
    };
    let (to_sun, veil) = veil_of(sun);
    commands.spawn((
        Name::new("Glare veil"),
        Glare,
        Mesh3d(meshes.add(screen_triangle())),
        MeshMaterial3d(materials.add(GlareVeil {
            to_sun,
            veil,
            cloud_light: fog.light.clone(),
            cloud_fog: fog.fog.clone(),
        })),
        Transform::default(),
        NoFrustumCulling,
        NotShadowCaster,
        NotShadowReceiver,
    ));
}

/// The veil follows the sun.
pub fn follow_the_sun(
    source: SunSource,
    glares: Query<&MeshMaterial3d<GlareVeil>, With<Glare>>,
    mut materials: ResMut<Assets<GlareVeil>>,
) {
    let Some(sun) = the_sun(&source) else {
        return;
    };
    let (to_sun, veil) = veil_of(sun);
    for handle in &glares {
        let changed = materials
            .get(&handle.0)
            .is_some_and(|m| m.to_sun != to_sun || m.veil != veil);
        if changed && let Some(mut material) = materials.get_mut(&handle.0) {
            material.to_sun = to_sun;
            material.veil = veil;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_low_sun_is_redder_and_dimmer() {
        let high = sun_at_the_eye(60.0);
        let low = sun_at_the_eye(3.0);
        assert!(low.y < high.y);
        assert!(low.x / low.z > high.x / high.z, "{low} {high}");
        assert_eq!(sun_at_the_eye(-1.0), Vec3::ZERO);
    }

    #[test]
    fn the_veil_at_the_contre_jour_s_line_of_sight_is_the_closing_s() {
        // A sun 9° high, 16° from the line of sight: the luminance that closes the eye
        // (exposure::glare_ev) is the one the veil adds there.
        let sun = sun_at_the_eye(9.0).y;
        let veil = HOLLADAY * sun / 16.0f32.powi(2);
        assert!((veil - 10.0 * direct_normal_lux(9.0) / 256.0).abs() < 1.0);
    }
}
