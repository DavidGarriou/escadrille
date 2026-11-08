//! Volumetric clouds (M3, increment 1; skill `rendu-visuel`, `nuages-volumetriques.md`).
//!
//! The density of the clouds is written twice: in Rust by `escadrille_weather`, the
//! reference that the game also queries, and in WGSL by `assets/shaders/clouds/
//! density.wesl` for the render (`CLAUDE.md`, CPU/GPU parity). The parity test below runs
//! the shader on the GPU over the same textures and uniforms, and compares.
//!
//! The render (ADR 0009) has three steps:
//! - the ray march, at a fraction of the resolution, writes the light and transmittance
//!   of the clouds and two distances (`pass`);
//! - a temporal pass blends it with the previous frames, reprojected at the distance of
//!   the clouds, into [`CloudTarget`] (`pass`);
//! - a transparent material over the whole screen, [`CloudComposite`], first in the
//!   transparent pass, after Bevy's sky, upsamples that image along the depth of the
//!   scene and blends it over the sky and the opaque objects.
//!
//! Their light on the rest of the scene: their shadow, a map laid on the sun, and the
//! light they send back, drawn into the camera's environment map (`pass`) with the
//! fields below the horizon, in the clouds' mean shadow ([`ground_shade`]), from which
//! Bevy lights every surface. The ground beneath a layer the camera flies above takes the
//! light of the bases the map misses ([`ground_cloud_light`], `render::ground`).
//!
//! The transparent surfaces, drawn after the composite, veil themselves with the fog in
//! front of them (`fog`).

mod fog;
mod pass;

use bevy::asset::RenderAssetUsages;
use bevy::camera::visibility::NoFrustumCulling;
use bevy::diagnostic::FrameCount;
use bevy::image::{Image, ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy::light::{DirectionalLightTexture, NotShadowCaster, NotShadowReceiver};
use bevy::math::{DVec3, Vec2, Vec3};
use bevy::mesh::{Indices, MeshVertexBufferLayoutRef, PrimitiveTopology};
use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::render::render_resource::{
    AsBindGroup, CompareFunction, Extent3d, RenderPipelineDescriptor, ShaderType,
    SpecializedMeshPipelineError, TextureDimension, TextureFormat,
};
use bevy::shader::ShaderRef;
use escadrille_weather::clouds::{CloudLayer, CloudTextures, CloudUniforms};
use escadrille_weather::grid::{Plane, Volume};

pub use fog::{CloudDensity, CloudFog};
pub use pass::{GpuCloudPass, GpuCloudShadow};

use super::ground::{GroundAlbedo, GroundLight};
use super::quality::Quality;
use crate::flight::bridge::to_bevy;
use crate::flight::simulation::{Player, RenderOrigin};

/// Longest distance a ray marches, m: the clouds of a layer at 1 500 m seen from 2 000 m
/// reach the horizon at about 50 km (`escadrille_weather`'s flat world).
const MAX_DISTANCE_M: f32 = 60_000.0;
/// Shortest step of a ray, m: a few texels of the detail noise (7.8 m).
const MIN_STEP_M: f32 = 12.0;

/// Format of the cloud image: light (rgb, HDR) and transmittance (a).
const TARGET_FORMAT: TextureFormat = TextureFormat::Rgba16Float;
/// Format of the cloud distances: where the march stopped (the scene, or the longest
/// distance) and the mean distance of the clouds, m.
const DISTANCE_FORMAT: TextureFormat = TextureFormat::Rg32Float;

/// The shadow of the clouds: a map of the sunlight's transmittance through them, seen from
/// the sun, laid on the sun as its light texture (`DirectionalLightTexture`), which veils
/// its light on the ground and the aircraft. Side, texels, and span, m: 23 m a texel,
/// sharper than the penumbra of a cloud 1 km up (9 m for the sun's half degree) is not
/// needed from the air. Beyond the span the map repeats, in the haze.
const SHADOW_SIZE: u32 = 2048;
const SHADOW_SPAN_M: f32 = 48_000.0;
/// The map lies ahead of the eye by this share of its span, horizontally: the ground in
/// view reaches 15 km and more ahead from 2 000 m; behind and aside, much less.
const SHADOW_AHEAD: f32 = 0.35;
/// Looks along each ray of the shadow map, through the layer towards the sun.
const SHADOW_STEPS: u32 = 24;
/// Around the aircraft's line towards the sun, the map holds only the clouds above it: a
/// ray holds one value for all its heights, and an aircraft in the sun above a cloud would
/// otherwise take the shadow the cloud casts on the ground. The ground of that line lies in
/// the aircraft's own shadow, but a cloud between them lets the sun through there: the
/// radius is one texel, the least the map can hold, and the full column comes back over
/// another one.
const SHADOW_FOCUS_RADIUS_M: f32 = SHADOW_SPAN_M / SHADOW_SIZE as f32;
const SHADOW_FORMAT: TextureFormat = TextureFormat::R8Unorm;

/// Share of the ground in the clouds' shadow for a cover: about 0.76 of the cover
/// (`escadrille_weather`, test `a_summer_afternoon_has_scattered_cumulus`). ESTIMÉ. Twin
/// of `SHADED_PER_COVER` in `shadow.wesl`.
const SHADED_PER_COVER: f32 = 0.76;
/// Light the clouds' bases send to the ground besides the sky's, as a share of the sun's
/// horizontal irradiance times the cover. ESTIMÉ: under a field of cumulus the diffuse
/// light is 25 to 35 % of the global irradiance, and nearly neutral, the white flanks and
/// bases of the clouds adding to the blue sky (art director's review of the shadows,
/// 2026-11-07); the clear sky alone left the shadows blue as lakes. 0.6 times a cover of
/// 0.45 gives about a quarter of the sun.
const CLOUD_LIGHT_ON_GROUND: f32 = 0.6;
/// Optical depth of the clear air at the zenith in visible light, for the sun's irradiance
/// on the ground. ESTIMÉ: Rayleigh about 0.1, aerosols 0.1 to 0.2 on a summer day.
const AIR_OPTICAL_DEPTH: f32 = 0.25;

/// The eye's depth in the clouds ([`eye_depth`]): reach and step of the looks around it,
/// m. A big cumulus is a kilometre or two across; 600 m into its core is 54 optical depths,
/// where the sky no longer gets through.
const EYE_DEPTH_REACH_M: f32 = 600.0;
const EYE_DEPTH_STEP_M: f32 = 10.0;

/// The shader of the composite, under `assets/`.
const COMPOSITE_SHADER: &str = "shaders/clouds/composite.wesl";

/// Sort bias of the composite: far behind everything, so that the transparent pass draws
/// it first and the wires and other transparent objects over it.
const COMPOSITE_DEPTH_BIAS: f32 = -1.0e9;

/// The layer of clouds over the world and the seed of its noises; without it, a clear
/// sky. Set by a scene, later by a mission's weather.
#[derive(Resource, Debug, Clone, PartialEq)]
pub struct Clouds {
    pub layer: CloudLayer,
    pub seed: u32,
}

/// The noise textures of the clouds as images, and the seed they come from.
#[derive(Resource, Debug, Clone)]
pub struct CloudImages {
    seed: u32,
    pub shape: Handle<Image>,
    pub detail: Handle<Image>,
    pub weather: Handle<Image>,
}

/// The images of the clouds after the temporal pass, the camera's size divided by
/// `divisor`, and the entity that blends them.
#[derive(Resource, Debug, Clone)]
pub struct CloudTarget {
    pub color: Handle<Image>,
    pub distance: Handle<Image>,
    /// The shadow map, laid on the sun.
    pub shadow: Handle<Image>,
    pub divisor: u32,
    veil: Entity,
}

/// Material of the composite: the cloud images over the whole screen, upsampled along the
/// depth of the scene, premultiplied (light, and 1 − transmittance in alpha), without
/// depth test or culling.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CloudComposite {
    #[texture(0)]
    #[sampler(1)]
    pub clouds: Handle<Image>,
    #[texture(2, sample_type = "float", filterable = false)]
    pub distances: Handle<Image>,
    /// Divisor of the resolution, longest distance of a ray (m), unused, unused.
    #[uniform(3)]
    pub params: Vec4,
}

impl Material for CloudComposite {
    fn vertex_shader() -> ShaderRef {
        COMPOSITE_SHADER.into()
    }

    fn fragment_shader() -> ShaderRef {
        COMPOSITE_SHADER.into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        AlphaMode::Premultiplied
    }

    fn depth_bias(&self) -> f32 {
        COMPOSITE_DEPTH_BIAS
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

/// A triangle that covers the screen in clip space: the composite's vertex shader passes
/// its positions through.
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

/// What the cloud pass needs this frame: the uniforms of the density, how the rays march
/// from the camera and in the environment map, and the render origin, whose jump resets
/// the temporal pass.
#[derive(Resource, Debug, Clone, Copy)]
pub struct CloudFrame {
    pub uniforms: GpuCloudUniforms,
    pub pass: GpuCloudPass,
    pub environment: GpuCloudPass,
    pub origin: DVec3,
    pub shadow: GpuCloudShadow,
}

pub struct CloudsPlugin;

impl Plugin for CloudsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<CloudComposite>::default())
            .add_systems(Startup, fog::create)
            .add_systems(
                PostUpdate,
                (
                    (
                        spawn_veil,
                        generate_images,
                        size_target,
                        update_frame,
                        fog::update,
                    )
                        .chain()
                        .run_if(resource_exists::<Clouds>),
                    (clear_sky, fog::clear).run_if(not(resource_exists::<Clouds>)),
                )
                    .before(TransformSystems::Propagate),
            );
        pass::build(app);
    }
}

/// The cloud images and the entity that blends them, once. The images are the fog's,
/// which the transparent materials bind for every sky.
fn spawn_veil(
    target: Option<Res<CloudTarget>>,
    quality: Res<Quality>,
    fog: Res<CloudFog>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CloudComposite>>,
    mut commands: Commands,
) {
    if target.is_some() {
        return;
    }
    let color = fog.light.clone();
    let distance = fog.distance.clone();
    let shadow = images.add(Image::new_target_texture(
        SHADOW_SIZE,
        SHADOW_SIZE,
        SHADOW_FORMAT,
        None,
    ));
    let divisor = quality.cloud_divisor();
    let veil = commands
        .spawn((
            Name::new("Cloud veil"),
            Mesh3d(meshes.add(screen_triangle())),
            MeshMaterial3d(materials.add(CloudComposite {
                clouds: color.clone(),
                distances: distance.clone(),
                params: Vec4::new(divisor as f32, MAX_DISTANCE_M, 0.0, 0.0),
            })),
            Transform::default(),
            NoFrustumCulling,
            NotShadowCaster,
            NotShadowReceiver,
        ))
        .id();
    commands.insert_resource(CloudTarget {
        color,
        distance,
        shadow,
        divisor,
        veil,
    });
}

/// The size of the cloud images: the 3D camera's target divided by the divisor, rounded up.
pub fn cloud_size(target: UVec2, divisor: u32) -> UVec2 {
    target.map(|side| side.div_ceil(divisor))
}

/// The cloud images keep the size of the 3D camera's target, divided.
fn size_target(
    target: Res<CloudTarget>,
    cameras: Query<&Camera, With<Camera3d>>,
    mut images: ResMut<Assets<Image>>,
) {
    let Some(size) = cameras.iter().find_map(Camera::physical_target_size) else {
        return;
    };
    let size = cloud_size(size, target.divisor);
    for handle in [&target.color, &target.distance] {
        if images.get(handle).is_some_and(|image| image.size() != size)
            && let Some(mut image) = images.get_mut(handle)
        {
            image.resize(Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            });
        }
    }
}

/// The noise textures, generated again when the seed changes (0.2 s on 16 cores), as
/// images and on the CPU.
fn generate_images(
    clouds: Res<Clouds>,
    current: Option<Res<CloudImages>>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    if current.is_some_and(|c| c.seed == clouds.seed) {
        return;
    }
    let textures = CloudTextures::generate(clouds.seed);
    let shape = GridTexture::of_volume(&textures.shape).image(textures.shape.data.clone());
    let detail = GridTexture::of_volume(&textures.detail).image(textures.detail.data.clone());
    let weather = GridTexture::of_plane(&textures.weather).image(textures.weather.data.clone());
    commands.insert_resource(CloudDensity(textures));
    commands.insert_resource(CloudImages {
        seed: clouds.seed,
        shape: images.add(shape),
        detail: images.add(detail),
        weather: images.add(weather),
    });
}

/// The 3D camera, not the sun.
type CameraOnly = (With<Camera3d>, Without<DirectionalLight>);

/// The player's aircraft, neither the sun nor the camera.
type PlayerOnly = (With<Player>, Without<DirectionalLight>, Without<Camera3d>);

/// The sun, which carries the shadow map, and whether it carries it already.
type Suns<'w, 's> = Query<
    'w,
    's,
    (
        Entity,
        &'static DirectionalLight,
        &'static mut Transform,
        Has<DirectionalLightTexture>,
    ),
>;

/// Where the shadow map lies: ahead of the eye along its horizontal view, on the map's own
/// grid of texels in the sun's plane, so that the shadows do not crawl as the eye moves.
pub fn shadow_center(eye: Vec3, forward: Vec3, sun: Quat) -> Vec3 {
    let ahead = Vec3::new(forward.x, 0.0, forward.z).normalize_or_zero();
    let wanted = eye + ahead * SHADOW_AHEAD * SHADOW_SPAN_M;
    let texel = SHADOW_SPAN_M / SHADOW_SIZE as f32;
    let snap = |v: f32| (v / texel).round() * texel;
    let (right, up, to_sun) = (sun * Vec3::X, sun * Vec3::Y, sun * Vec3::Z);
    right * snap(wanted.dot(right)) + up * snap(wanted.dot(up)) + to_sun * wanted.dot(to_sun)
}

/// Share of the ground's sunlight left under a layer of the cover, on average: the
/// fields seen below the horizon of the environment map.
pub fn ground_shade(cover: f32) -> f32 {
    1.0 - SHADED_PER_COVER * cover.clamp(0.0, 1.0)
}

/// The sun's irradiance on level ground, lux: its illuminance through the clear air, by
/// its height (`to_sun`, the direction towards it).
pub fn sun_on_ground(illuminance: f32, to_sun: Vec3) -> f32 {
    let height = to_sun.y;
    if height <= 0.0 {
        return 0.0;
    }
    illuminance * height * (-AIR_OPTICAL_DEPTH / height).exp()
}

/// How deep the eye is in the clouds: the least optical depth over the first
/// [`EYE_DEPTH_REACH_M`] around it, looking up, along the four horizontal directions and
/// towards the sun; zero in clear air. One value for the whole frame, which the march adds
/// to the depth back along each ray: from inside a cloud, a sample sees the sky through
/// the eye's side only beyond the cloud around the eye. Taken per pixel, back along each
/// ray, it drew the shape of the cloud behind the eye on the fog in front, with sharp
/// edges that turned with the view (art director's review of `traversee_nuage`).
pub fn eye_depth(
    textures: &CloudTextures,
    uniforms: &CloudUniforms,
    eye: Vec3,
    to_sun: Vec3,
) -> f32 {
    [Vec3::Y, Vec3::X, Vec3::NEG_X, Vec3::Z, Vec3::NEG_Z, to_sun]
        .into_iter()
        .map(|d| textures.optical_depth(uniforms, eye, d, EYE_DEPTH_REACH_M, EYE_DEPTH_STEP_M))
        .fold(f32::INFINITY, f32::min)
}

/// The light of the clouds' bases the ground takes besides the environment map's, cd/m²:
/// in the share of the layer the eye is above (`altitude`, between its `bottom` and `top`,
/// m), since the map, seen from the eye, then misses the bases. A Lambertian surface under
/// an even radiance L receives π L.
pub fn ground_cloud_light(cover: f32, sun_on_ground: f32, altitude: f32, layer: (f32, f32)) -> f32 {
    let (bottom, top) = layer;
    let above = ((altitude - bottom) / (top - bottom).max(1.0)).clamp(0.0, 1.0);
    above * CLOUD_LIGHT_ON_GROUND * cover * sun_on_ground / std::f32::consts::PI
}

/// The uniforms of this frame: the clouds carried by the wind for the game's time, around
/// the render origin. The sun carries the shadow map, centred on the camera, its side the
/// map's span (a light texture spans the light's local −1 to 1).
#[expect(
    clippy::too_many_arguments,
    reason = "the clouds, their density and clocks, and the three bodies the shadow map follows"
)]
fn update_frame(
    clouds: Res<Clouds>,
    density: Option<Res<CloudDensity>>,
    target: Res<CloudTarget>,
    time: Res<Time>,
    origin: Option<Res<RenderOrigin>>,
    frames: Res<FrameCount>,
    quality: Res<Quality>,
    ground: Res<GroundAlbedo>,
    cameras: Query<&Transform, CameraOnly>,
    player: Query<&Transform, PlayerOnly>,
    mut suns: Suns,
    mut commands: Commands,
) {
    let camera = cameras.iter().next();
    let eye = camera.map_or(Vec3::ZERO, |t| t.translation);
    let forward = camera.map_or(Vec3::NEG_Z, |t| *t.forward());
    let focus = player.iter().next().map_or(eye, |t| t.translation);
    let mut shadow = GpuCloudShadow::default();
    let mut on_ground = 0.0;
    let mut to_sun = Vec3::Y;
    if let Some((sun, light, mut transform, textured)) = suns.iter_mut().next() {
        let half = 0.5 * SHADOW_SPAN_M;
        let center = shadow_center(eye, forward, transform.rotation);
        transform.translation = center;
        transform.scale = Vec3::new(half, half, 1.0);
        if !textured {
            commands.entity(sun).insert(DirectionalLightTexture {
                image: target.shadow.clone(),
                tiled: true,
            });
        }
        to_sun = transform.rotation * Vec3::Z;
        shadow = GpuCloudShadow {
            center,
            half_span: half,
            right: transform.rotation * Vec3::X,
            steps: SHADOW_STEPS,
            up: transform.rotation * Vec3::Y,
            focus_radius: SHADOW_FOCUS_RADIUS_M,
            to_sun,
            focus,
        };
        on_ground = sun_on_ground(light.illuminance, to_sun);
    }
    let layer = &clouds.layer;
    let bottom = layer.base_altitude as f32;
    let top = bottom + layer.thickness.iter().copied().fold(0.0, f64::max) as f32;
    let cover = layer.cover as f32;
    commands.insert_resource(GroundLight(Vec3::splat(ground_cloud_light(
        cover,
        on_ground,
        eye.y,
        (bottom, top),
    ))));
    let origin = origin.map_or(DVec3::ZERO, |o| to_bevy(o.0));
    let uniforms = clouds.layer.uniforms(time.elapsed_secs_f64(), origin);
    let eye_depth = density.map_or(0.0, |d| eye_depth(&d.0, &uniforms, eye, to_sun));
    let pass = |(steps, light_steps)| GpuCloudPass {
        frame: frames.0,
        steps,
        light_steps,
        divisor: quality.cloud_divisor(),
        min_step: MIN_STEP_M,
        max_distance: MAX_DISTANCE_M,
        ground_albedo: ground.0,
        ground_shade: ground_shade(cover),
        eye_depth,
    };
    commands.insert_resource(CloudFrame {
        uniforms: GpuCloudUniforms::from(&uniforms),
        pass: pass(quality.cloud_steps()),
        environment: pass(quality.cloud_environment_steps()),
        origin,
        shadow,
    });
}

fn clear_sky(
    frame: Option<Res<CloudFrame>>,
    target: Option<Res<CloudTarget>>,
    mut suns: Query<(Entity, &mut Transform), With<DirectionalLightTexture>>,
    ground: Option<Res<GroundLight>>,
    mut commands: Commands,
) {
    if frame.is_some() {
        commands.remove_resource::<CloudFrame>();
    }
    for (sun, mut transform) in &mut suns {
        transform.scale = Vec3::ONE;
        commands.entity(sun).remove::<DirectionalLightTexture>();
    }
    if ground.is_some_and(|g| g.0 != Vec3::ZERO) {
        commands.insert_resource(GroundLight::default());
    }
    if let Some(target) = target {
        commands.entity(target.veil).despawn();
        commands.remove_resource::<CloudTarget>();
    }
}

/// The shader's `CloudUniforms`: those of `escadrille_weather`, field for field and in the
/// same order, laid out by `ShaderType` as WGSL lays them out.
#[derive(ShaderType, Debug, Clone, Copy, PartialEq, Default)]
pub struct GpuCloudUniforms {
    pub shape_offset: Vec3,
    pub shape_inverse_scale: f32,
    pub detail_offset: Vec3,
    pub detail_inverse_scale: f32,
    pub weather_offset: Vec2,
    pub weather_inverse_scale: f32,
    pub altitude_offset: f32,
    pub thickness: Vec3,
    pub base_altitude: f32,
    pub cover: f32,
    pub cover_softness: f32,
    pub cloud_type: f32,
    pub type_variation: f32,
    pub shape_erosion: f32,
    pub detail_erosion: f32,
    pub extinction: f32,
}

impl From<&CloudUniforms> for GpuCloudUniforms {
    fn from(u: &CloudUniforms) -> Self {
        Self {
            shape_offset: u.shape_offset,
            shape_inverse_scale: u.shape_inverse_scale,
            detail_offset: u.detail_offset,
            detail_inverse_scale: u.detail_inverse_scale,
            weather_offset: u.weather_offset,
            weather_inverse_scale: u.weather_inverse_scale,
            altitude_offset: u.altitude_offset,
            thickness: u.thickness,
            base_altitude: u.base_altitude,
            cover: u.cover,
            cover_softness: u.cover_softness,
            cloud_type: u.cloud_type,
            type_variation: u.type_variation,
            shape_erosion: u.shape_erosion,
            detail_erosion: u.detail_erosion,
            extinction: u.extinction,
        }
    }
}

/// A grid of `escadrille_weather` as the GPU stores it: a cube of `R8Unorm` texels or a
/// square of `Rg8Unorm` texels, one mip level, the data unchanged.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GridTexture {
    pub size: Extent3d,
    pub dimension: TextureDimension,
    pub format: TextureFormat,
}

impl GridTexture {
    pub fn of_volume(volume: &Volume) -> Self {
        Self {
            size: Extent3d {
                width: volume.size,
                height: volume.size,
                depth_or_array_layers: volume.size,
            },
            dimension: TextureDimension::D3,
            format: TextureFormat::R8Unorm,
        }
    }

    pub fn of_plane(plane: &Plane) -> Self {
        Self {
            size: Extent3d {
                width: plane.size,
                height: plane.size,
                depth_or_array_layers: 1,
            },
            dimension: TextureDimension::D2,
            format: TextureFormat::Rg8Unorm,
        }
    }

    /// The image of a grid's data, with the sampler that the reference reproduces.
    pub fn image(self, data: Vec<u8>) -> Image {
        let mut image = Image::new(
            self.size,
            self.dimension,
            data,
            self.format,
            RenderAssetUsages::RENDER_WORLD,
        );
        image.sampler = ImageSampler::Descriptor(repeat_linear());
        image
    }
}

/// Linear filtering and repeat addressing, without mipmaps: the sampling of
/// `escadrille_weather::grid`.
pub fn repeat_linear() -> ImageSamplerDescriptor {
    ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Nearest,
        ..Default::default()
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::{DVec3, Vec4};
    use bevy::render::render_resource::encase::{StorageBuffer, UniformBuffer};
    use bevy::render::render_resource::{
        BindGroupDescriptor, BindGroupEntry, BindingResource, BufferDescriptor,
        BufferInitDescriptor, BufferUsages, CommandEncoderDescriptor, ComputePassDescriptor,
        MapMode, PipelineCompilationOptions, PollType, RawComputePipelineDescriptor,
        ShaderModuleDescriptor, ShaderSource, ShaderType, TextureDataOrder, TextureDescriptor,
        TextureUsages, TextureViewDescriptor,
    };
    use bevy::render::renderer::{RenderDevice, RenderQueue, initialize_renderer};
    use bevy::render::settings::{Backends, RenderResources, WgpuSettings};
    use escadrille_weather::clouds::{CloudLayer, CloudTextures};
    use escadrille_weather::noise::pcg3d;

    use super::*;

    /// The density module, as the game loads it.
    const DENSITY: &str = include_str!("../../../../assets/shaders/clouds/density.wesl");

    /// A compute entry point around the module: the density at each position.
    const PARITY: &str = r"
@group(0) @binding(0) var<uniform> uniforms: CloudUniforms;
@group(0) @binding(1) var shape_noise: texture_3d<f32>;
@group(0) @binding(2) var detail_noise: texture_3d<f32>;
@group(0) @binding(3) var weather_map: texture_2d<f32>;
@group(0) @binding(4) var cloud_sampler: sampler;
@group(0) @binding(5) var<storage, read> positions: array<vec4<f32>>;
@group(0) @binding(6) var<storage, read_write> densities: array<f32>;

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x < arrayLength(&densities) {
        densities[id.x] = cloud_density(
            uniforms, shape_noise, detail_noise, weather_map, cloud_sampler, positions[id.x].xyz,
        );
    }
}
";

    fn gpu() -> (RenderDevice, RenderQueue) {
        let settings = WgpuSettings::default();
        let backends = settings.backends.unwrap_or(Backends::all());
        let RenderResources(device, queue, ..) =
            bevy::tasks::block_on(initialize_renderer(backends, None, &settings));
        (device, queue)
    }

    /// The densities that the shader computes at positions, on the GPU.
    fn gpu_densities(
        textures: &CloudTextures,
        uniforms: &CloudUniforms,
        positions: &[Vec3],
    ) -> Vec<f32> {
        let (device, queue) = gpu();
        let view = |grid: GridTexture, data: &[u8]| {
            let descriptor = TextureDescriptor {
                label: None,
                size: grid.size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: grid.dimension,
                format: grid.format,
                usage: TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            };
            device
                .create_texture_with_data(&queue, &descriptor, TextureDataOrder::LayerMajor, data)
                .create_view(&TextureViewDescriptor::default())
        };
        let shape = view(
            GridTexture::of_volume(&textures.shape),
            &textures.shape.data,
        );
        let detail = view(
            GridTexture::of_volume(&textures.detail),
            &textures.detail.data,
        );
        let weather = view(
            GridTexture::of_plane(&textures.weather),
            &textures.weather.data,
        );
        let sampler = device.create_sampler(&repeat_linear().as_wgpu());

        let mut uniform_bytes = UniformBuffer::new(Vec::<u8>::new());
        uniform_bytes
            .write(&GpuCloudUniforms::from(uniforms))
            .unwrap();
        let uniform_buffer = device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: &uniform_bytes.into_inner(),
            usage: BufferUsages::UNIFORM,
        });
        let mut position_bytes = StorageBuffer::new(Vec::<u8>::new());
        let padded: Vec<Vec4> = positions.iter().map(|p| p.extend(0.0)).collect();
        position_bytes.write(&padded).unwrap();
        let position_buffer = device.create_buffer_with_data(&BufferInitDescriptor {
            label: None,
            contents: &position_bytes.into_inner(),
            usage: BufferUsages::STORAGE,
        });
        let size = 4 * positions.len() as u64;
        let output = device.create_buffer(&BufferDescriptor {
            label: None,
            size,
            usage: BufferUsages::STORAGE | BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let readback = device.create_buffer(&BufferDescriptor {
            label: None,
            size,
            usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let module = device.create_and_validate_shader_module(ShaderModuleDescriptor {
            label: Some("cloud density parity"),
            source: ShaderSource::Wgsl(format!("{DENSITY}\n{PARITY}").into()),
        });
        let pipeline = device.create_compute_pipeline(&RawComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &module,
            entry_point: Some("main"),
            compilation_options: PipelineCompilationOptions::default(),
            cache: None,
        });
        let bind_group = device
            .wgpu_device()
            .create_bind_group(&BindGroupDescriptor {
                label: None,
                layout: &pipeline.get_bind_group_layout(0),
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: uniform_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::TextureView(&shape),
                    },
                    BindGroupEntry {
                        binding: 2,
                        resource: BindingResource::TextureView(&detail),
                    },
                    BindGroupEntry {
                        binding: 3,
                        resource: BindingResource::TextureView(&weather),
                    },
                    BindGroupEntry {
                        binding: 4,
                        resource: BindingResource::Sampler(&sampler),
                    },
                    BindGroupEntry {
                        binding: 5,
                        resource: position_buffer.as_entire_binding(),
                    },
                    BindGroupEntry {
                        binding: 6,
                        resource: output.as_entire_binding(),
                    },
                ],
            });

        let mut encoder = device.create_command_encoder(&CommandEncoderDescriptor::default());
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor::default());
            pass.set_pipeline(&pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            pass.dispatch_workgroups(positions.len().div_ceil(64) as u32, 1, 1);
        }
        encoder.copy_buffer_to_buffer(&output, 0, &readback, 0, size);
        queue.submit([encoder.finish()]);
        readback
            .slice(..)
            .map_async(MapMode::Read, |result| result.unwrap());
        device.poll(PollType::wait_indefinitely()).unwrap();
        let bytes = readback.slice(..).get_mapped_range().unwrap();
        bytes
            .as_chunks::<4>()
            .0
            .iter()
            .map(|b| f32::from_le_bytes(*b))
            .collect()
    }

    #[test]
    fn the_shadow_map_lies_ahead_on_its_grid() {
        let sun = Transform::default()
            .looking_to(Vec3::new(0.3, -0.85, 0.4), Vec3::Y)
            .rotation;
        let texel = SHADOW_SPAN_M / SHADOW_SIZE as f32;
        let eye = Vec3::new(123.4, 2_000.0, -567.8);
        let center = shadow_center(eye, Vec3::new(0.0, -0.1, -1.0), sun);
        for axis in [sun * Vec3::X, sun * Vec3::Y] {
            let across = center.dot(axis) / texel;
            assert!((across - across.round()).abs() < 1e-2, "{across}");
        }
        // Ahead of the eye, horizontally, by about the share of the span (in the sun's
        // plane, within a texel).
        let ahead = (center - eye) - (sun * Vec3::Z) * (center - eye).dot(sun * Vec3::Z);
        let wanted = Vec3::NEG_Z * SHADOW_AHEAD * SHADOW_SPAN_M;
        let wanted = wanted - (sun * Vec3::Z) * wanted.dot(sun * Vec3::Z);
        assert!(
            (ahead - wanted).length() < texel,
            "{ahead} against {wanted}"
        );
    }

    #[test]
    fn the_eye_is_deep_in_a_cloud_only_inside_it_and_sinks_in_smoothly() {
        // The cumulus of traversee_nuage, on the path east at 1 000 m.
        let textures = CloudTextures::generate(85);
        let layer = CloudLayer {
            base_altitude: 600.0,
            cover: 0.45,
            ..CloudLayer::default()
        };
        let uniforms = layer.uniforms(0.0, DVec3::ZERO);
        let to_sun = Vec3::new(0.0, 0.85, 0.53);
        let at = |east: f32| eye_depth(&textures, &uniforms, Vec3::new(east, 1_000.0, 0.0), to_sun);
        assert_eq!(at(100.0), 0.0);
        assert!(at(950.0) > 5.0, "core: {}", at(950.0));
        // Entering, by quarters of a metre: a slope (0.7 per metre at most), no jump.
        let depths: Vec<f32> = (0..600).map(|i| at(700.0 + 0.25 * i as f32)).collect();
        for pair in depths.windows(2) {
            assert!((pair[1] - pair[0]).abs() < 0.25, "{pair:?}");
        }
    }

    #[test]
    fn the_fields_below_the_horizon_lose_the_clouds_mean_shadow() {
        assert_eq!(ground_shade(0.0), 1.0);
        // A cover of 0.45 shades about a third of the ground; overcast, three quarters.
        assert!((ground_shade(0.45) - 0.658).abs() < 1e-3);
        assert!((ground_shade(1.0) - 0.24).abs() < 1e-3);
        assert_eq!(ground_shade(2.0), ground_shade(1.0));
    }

    #[test]
    fn above_the_layer_the_ground_takes_about_a_quarter_of_the_sun_from_the_bases() {
        let noon = Vec3::new(0.0, 58f32.to_radians().sin(), -58f32.to_radians().cos());
        let sun = sun_on_ground(100_000.0, noon);
        // On level ground, the sun's height times what gets through the air: about 75 %
        // at 58°.
        let through = sun / (100_000.0 * noon.y);
        assert!((0.7..0.8).contains(&through), "{through}");
        let layer = (1_000.0, 1_700.0);
        let above = std::f32::consts::PI * ground_cloud_light(0.45, sun, 2_000.0, layer);
        assert!((0.2..0.35).contains(&(above / sun)), "{}", above / sun);
        // Under the bases the environment map holds them already; half way, half.
        assert_eq!(ground_cloud_light(0.45, sun, 500.0, layer), 0.0);
        let half = ground_cloud_light(0.45, sun, 1_350.0, layer);
        assert!((half / ground_cloud_light(0.45, sun, 2_000.0, layer) - 0.5).abs() < 1e-3);
        assert_eq!(ground_cloud_light(0.0, sun, 2_000.0, layer), 0.0);
        assert_eq!(sun_on_ground(100_000.0, Vec3::new(1.0, -0.1, 0.0)), 0.0);
    }

    #[test]
    fn the_cloud_images_cover_the_camera_s_rounded_up() {
        assert_eq!(cloud_size(UVec2::new(2560, 1440), 2), UVec2::new(1280, 720));
        assert_eq!(cloud_size(UVec2::new(2561, 1441), 2), UVec2::new(1281, 721));
        assert_eq!(
            cloud_size(UVec2::new(1920, 1080), 1),
            UVec2::new(1920, 1080)
        );
    }

    #[test]
    fn the_screen_triangle_covers_the_whole_screen() {
        let mesh = screen_triangle();
        let Some(bevy::mesh::VertexAttributeValues::Float32x3(p)) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        else {
            panic!("positions");
        };
        // Each corner of clip space is inside the triangle (left of every edge).
        let inside = |x: f32, y: f32| {
            (0..3).all(|i| {
                let (a, b) = (p[i], p[(i + 1) % 3]);
                (b[0] - a[0]) * (y - a[1]) - (b[1] - a[1]) * (x - a[0]) >= 0.0
            })
        };
        assert!(
            [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)]
                .iter()
                .all(|&(x, y)| inside(x, y))
        );
    }

    #[test]
    fn the_uniforms_have_the_layout_of_the_shader() {
        // Six rows of four 32-bit numbers: the vec3s take a row with the number after them.
        assert_eq!(GpuCloudUniforms::min_size().get(), 96);
    }

    #[test]
    fn grids_become_textures_of_their_size_and_format() {
        let volume = Volume {
            size: 4,
            data: vec![0; 64],
        };
        let grid = GridTexture::of_volume(&volume);
        assert_eq!(grid.dimension, TextureDimension::D3);
        assert_eq!(grid.size.depth_or_array_layers, 4);
        let image = grid.image(volume.data);
        assert_eq!(image.texture_descriptor.format, TextureFormat::R8Unorm);
        let plane = Plane {
            size: 2,
            data: vec![0; 8],
        };
        let image = GridTexture::of_plane(&plane).image(plane.data);
        assert_eq!(image.texture_descriptor.format, TextureFormat::Rg8Unorm);
        assert_eq!(image.texture_descriptor.size.depth_or_array_layers, 1);
    }

    #[test]
    fn the_gpu_density_is_the_reference() {
        let textures = CloudTextures::generate(7);
        let layer = CloudLayer {
            wind: DVec3::new(8.0, 0.0, -3.0),
            ..CloudLayer::default()
        };
        let uniforms = layer.uniforms(137.0, DVec3::new(12_500.0, 300.0, -7_300.0));
        // Through the layer and around it, over 32 km: altitudes 1 100 to 2 600 m.
        let positions: Vec<Vec3> = (0..65_536)
            .map(|n| {
                let [x, y, z] = pcg3d(n, 17, 23).map(|h| h as f32 / 4_294_967_296.0);
                Vec3::new(
                    (x - 0.5) * 32_000.0,
                    800.0 + 1_500.0 * y,
                    (z - 0.5) * 32_000.0,
                )
            })
            .collect();
        let cpu: Vec<f32> = positions
            .iter()
            .map(|p| textures.density(&uniforms, *p))
            .collect();
        let gpu = gpu_densities(&textures, &uniforms, &positions);

        let cloudy = cpu.iter().filter(|d| **d > 0.0).count();
        assert!(
            cloudy > positions.len() / 20,
            "too few cloudy samples: {cloudy}"
        );
        let errors: Vec<f32> = cpu.iter().zip(&gpu).map(|(c, g)| (c - g).abs()).collect();
        let worst = errors.iter().copied().fold(0.0, f32::max);
        let mean = errors.iter().sum::<f32>() / errors.len() as f32;
        println!("{cloudy} cloudy samples; error worst {worst:.2e}, mean {mean:.2e}");
        // The GPU's filter weights are rounded to a few bits, and the density's gain of 4
        // (its sharpness) multiplies that: 1.6e-2 at worst on the reference GPU (RX 7800
        // XT, RADV), 2.8e-5 on average, on 2026-11-07.
        assert!(worst < 0.04 && mean < 2e-4, "worst {worst}, mean {mean}");

        // Where the summit vanishes: a cover so low that the map's strongest value lies
        // before the summit's start (a denominator under zero, guarded), and no cover at
        // all. No cloud on either side, and no NaN from the GPU.
        for cover in [0.02, 0.0] {
            let uniforms = CloudLayer {
                cover,
                ..CloudLayer::default()
            }
            .uniforms(0.0, DVec3::ZERO);
            let gpu = gpu_densities(&textures, &uniforms, &positions[..4_096]);
            assert!(gpu.iter().all(|d| *d == 0.0), "cover {cover}");
            assert!(
                positions[..4_096]
                    .iter()
                    .all(|p| textures.density(&uniforms, *p) == 0.0)
            );
        }
    }
}
