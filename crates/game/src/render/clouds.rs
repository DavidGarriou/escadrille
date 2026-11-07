//! Volumetric clouds (M3, increment 1; skill `rendu-visuel`, `nuages-volumetriques.md`).
//!
//! The density of the clouds is written twice: in Rust by `escadrille_weather`, the
//! reference that the game also queries, and in WGSL by `assets/shaders/clouds/
//! density.wesl` for the render (`CLAUDE.md`, CPU/GPU parity). The parity test below runs
//! the shader on the GPU over the same textures and uniforms, and compares.

use bevy::asset::RenderAssetUsages;
use bevy::image::{Image, ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy::math::{Vec2, Vec3};
use bevy::render::render_resource::{Extent3d, ShaderType, TextureDimension, TextureFormat};
use escadrille_weather::clouds::CloudUniforms;
use escadrille_weather::grid::{Plane, Volume};

/// The shader's `CloudUniforms`: those of `escadrille_weather`, field for field and in the
/// same order, laid out by `ShaderType` as WGSL lays them out.
#[derive(ShaderType, Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "bound by the cloud pass, the next step of increment 1"
    )
)]
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
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "loaded by the cloud pass, the next step of increment 1"
    )
)]
pub struct GridTexture {
    pub size: Extent3d,
    pub dimension: TextureDimension,
    pub format: TextureFormat,
}

#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "loaded by the cloud pass, the next step of increment 1"
    )
)]
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
#[cfg_attr(
    not(test),
    expect(
        dead_code,
        reason = "loaded by the cloud pass, the next step of increment 1"
    )
)]
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
        // The GPU's filter weights are rounded to a few bits: 3.5e-3 at worst on the
        // reference GPU (RX 7800 XT, RADV), 2.5e-5 on average, on 2026-11-07.
        assert!(worst < 0.01 && mean < 2e-4, "worst {worst}, mean {mean}");
    }
}
