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

mod pass;

use bevy::asset::RenderAssetUsages;
use bevy::camera::visibility::NoFrustumCulling;
use bevy::diagnostic::FrameCount;
use bevy::image::{Image, ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy::light::{NotShadowCaster, NotShadowReceiver};
use bevy::math::{DVec3, Vec2, Vec3};
use bevy::mesh::{Indices, MeshVertexBufferLayoutRef, PrimitiveTopology};
use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::render::render_resource::{
    AsBindGroup, CompareFunction, Extent3d, RenderPipelineDescriptor, ShaderType,
    SpecializedMeshPipelineError, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::shader::ShaderRef;
use escadrille_weather::clouds::{CloudLayer, CloudTextures, CloudUniforms};
use escadrille_weather::grid::{Plane, Volume};

pub use pass::GpuCloudPass;

use super::quality::Quality;
use crate::flight::bridge::to_bevy;
use crate::flight::simulation::RenderOrigin;

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

/// What the cloud pass needs this frame: the uniforms of the density, how the rays march,
/// and the render origin, whose jump resets the temporal pass.
#[derive(Resource, Debug, Clone, Copy)]
pub struct CloudFrame {
    pub uniforms: GpuCloudUniforms,
    pub pass: GpuCloudPass,
    pub origin: DVec3,
}

pub struct CloudsPlugin;

impl Plugin for CloudsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<CloudComposite>::default())
            .add_systems(
                PostUpdate,
                (
                    (spawn_veil, generate_images, size_target, update_frame)
                        .chain()
                        .run_if(resource_exists::<Clouds>),
                    clear_sky.run_if(not(resource_exists::<Clouds>)),
                ),
            );
        pass::build(app);
    }
}

/// The cloud images and the entity that blends them, once.
fn spawn_veil(
    target: Option<Res<CloudTarget>>,
    quality: Res<Quality>,
    mut images: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CloudComposite>>,
    mut commands: Commands,
) {
    if target.is_some() {
        return;
    }
    let image = |format| {
        let mut image = Image::new_target_texture(1, 1, format, None);
        // The temporal pass keeps a copy for the next frame.
        image.texture_descriptor.usage |= TextureUsages::COPY_SRC;
        image
    };
    let color = images.add(image(TARGET_FORMAT));
    let distance = images.add(image(DISTANCE_FORMAT));
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

/// The noise textures, generated again when the seed changes (0.2 s on 16 cores).
fn generate_images(
    clouds: Res<Clouds>,
    current: Option<Res<CloudImages>>,
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
) {
    if current.is_some_and(|c| c.seed == clouds.seed) {
        return;
    }
    let CloudTextures {
        shape,
        detail,
        weather,
    } = CloudTextures::generate(clouds.seed);
    let shape = GridTexture::of_volume(&shape).image(shape.data);
    let detail = GridTexture::of_volume(&detail).image(detail.data);
    let weather = GridTexture::of_plane(&weather).image(weather.data);
    commands.insert_resource(CloudImages {
        seed: clouds.seed,
        shape: images.add(shape),
        detail: images.add(detail),
        weather: images.add(weather),
    });
}

/// The uniforms of this frame: the clouds carried by the wind for the game's time, around
/// the render origin.
fn update_frame(
    clouds: Res<Clouds>,
    time: Res<Time>,
    origin: Option<Res<RenderOrigin>>,
    frames: Res<FrameCount>,
    quality: Res<Quality>,
    mut commands: Commands,
) {
    let origin = origin.map_or(DVec3::ZERO, |o| to_bevy(o.0));
    let uniforms = clouds.layer.uniforms(time.elapsed_secs_f64(), origin);
    let (steps, light_steps) = quality.cloud_steps();
    commands.insert_resource(CloudFrame {
        uniforms: GpuCloudUniforms::from(&uniforms),
        pass: GpuCloudPass {
            frame: frames.0,
            steps,
            light_steps,
            divisor: quality.cloud_divisor(),
            min_step: MIN_STEP_M,
            max_distance: MAX_DISTANCE_M,
        },
        origin,
    });
}

fn clear_sky(
    frame: Option<Res<CloudFrame>>,
    target: Option<Res<CloudTarget>>,
    mut commands: Commands,
) {
    if frame.is_some() {
        commands.remove_resource::<CloudFrame>();
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
    }
}
