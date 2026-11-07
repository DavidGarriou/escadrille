//! The ray march of the clouds in the render world (ADR 0009): a fullscreen pass of the
//! `Core3d` schedule, after the opaque objects, which writes the light of the clouds and
//! their transmittance into its own image ([`super::CloudTarget`]). The image is blended
//! over the sky by a transparent material, drawn first in the transparent pass
//! ([`super::CloudComposite`]): that pass comes after Bevy's `render_sky`, a private
//! system no `.after()` can name.
//!
//! Group 0 rebuilds the layout of `render_sky` (bindings 0 to 13 of
//! `bevy_pbr::atmosphere::bindings`, then the depth), so that the shader can import the
//! atmosphere's functions: every resource it binds is public, only Bevy's own bind group
//! is not. A Bevy upgrade must check this layout against `render_sky`'s
//! (`bevy_pbr/src/atmosphere/resources.rs`); a mismatch stops the pipeline, and the
//! capture with it. Group 1 holds the clouds.

use bevy::core_pipeline::FullscreenShader;
use bevy::core_pipeline::core_3d::{main_opaque_pass_3d, main_transparent_pass_3d};
use bevy::core_pipeline::schedule::Core3d;
use bevy::pbr::resources::{
    AtmosphereSampler, AtmosphereTextures, AtmosphereTransform, AtmosphereTransforms,
    AtmosphereTransformsOffset, GpuAtmosphere,
};
use bevy::pbr::{
    ExtractedAtmosphere, GpuAtmosphereSettings, GpuLights, GpuScatteringMedium, LightMeta,
    ScatteringMediumSampler, ViewLightsUniformOffset,
};
use bevy::prelude::*;
use bevy::render::diagnostic::RecordDiagnostics;
use bevy::render::extract_component::{ComponentUniforms, DynamicUniformIndex};
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::binding_types::{
    sampler, texture_2d, texture_3d, uniform_buffer,
};
use bevy::render::render_resource::{
    BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries,
    CachedRenderPipelineId, ColorTargetState, ColorWrites, FragmentState, LoadOp, Operations,
    PipelineCache, RenderPassColorAttachment, RenderPassDescriptor, RenderPipelineDescriptor,
    Sampler, SamplerBindingType, ShaderStages, ShaderType, StoreOp, TextureSampleType,
    UniformBuffer,
};
use bevy::render::renderer::{RenderContext, RenderDevice, RenderQueue, ViewQuery};
use bevy::render::texture::GpuImage;
use bevy::render::view::{ViewDepthStencilTexture, ViewUniform, ViewUniformOffset, ViewUniforms};
use bevy::render::{ExtractSchedule, MainWorld, Render, RenderApp, RenderStartup, RenderSystems};

use super::{CloudFrame, CloudImages, CloudTarget, GpuCloudUniforms, TARGET_FORMAT, repeat_linear};

/// The shader of the march, under `assets/`.
const SHADER: &str = "shaders/clouds/render.wesl";

/// The shader's `CloudPass`: how the rays march.
#[derive(ShaderType, Debug, Clone, Copy, PartialEq, Default)]
pub struct GpuCloudPass {
    pub frame: u32,
    pub steps: u32,
    pub light_steps: u32,
    pub min_step: f32,
    pub max_distance: f32,
}

/// What the main world says of the clouds this frame, in the render world.
#[derive(Resource, Debug, Clone)]
struct ExtractedClouds {
    uniforms: GpuCloudUniforms,
    pass: GpuCloudPass,
    images: CloudImages,
    target: Handle<Image>,
}

pub fn build(app: &mut App) {
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
        return;
    };
    render_app
        .init_resource::<CloudBuffers>()
        .add_systems(RenderStartup, init_pipeline)
        .add_systems(ExtractSchedule, extract_clouds)
        .add_systems(
            Render,
            (
                write_buffers.in_set(RenderSystems::PrepareResources),
                prepare_bind_groups.in_set(RenderSystems::PrepareBindGroups),
            ),
        )
        .add_systems(
            Core3d,
            march_clouds
                .after(main_opaque_pass_3d)
                .before(main_transparent_pass_3d),
        );
}

/// The two layouts, the sampler of the cloud textures and the pipeline.
#[derive(Resource)]
struct CloudPipeline {
    atmosphere: BindGroupLayoutDescriptor,
    clouds: BindGroupLayoutDescriptor,
    sampler: Sampler,
    pipeline: CachedRenderPipelineId,
}

fn init_pipeline(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    asset_server: Res<AssetServer>,
    fullscreen_shader: Res<FullscreenShader>,
    pipeline_cache: Res<PipelineCache>,
) {
    // `render_sky`'s layout, without multisampling (the game's cameras have no MSAA).
    let atmosphere = BindGroupLayoutDescriptor::new(
        "clouds_atmosphere_layout",
        &BindGroupLayoutEntries::with_indices(
            ShaderStages::FRAGMENT,
            (
                (0, uniform_buffer::<GpuAtmosphere>(true)),
                (1, uniform_buffer::<GpuAtmosphereSettings>(true)),
                (2, uniform_buffer::<AtmosphereTransform>(true)),
                (3, uniform_buffer::<ViewUniform>(true)),
                (4, uniform_buffer::<GpuLights>(true)),
                (5, texture_2d(TextureSampleType::default())),
                (6, texture_2d(TextureSampleType::default())),
                (7, sampler(SamplerBindingType::Filtering)),
                (8, texture_2d(TextureSampleType::default())),
                (9, texture_2d(TextureSampleType::default())),
                (10, texture_2d(TextureSampleType::default())),
                (11, texture_3d(TextureSampleType::default())),
                (12, sampler(SamplerBindingType::Filtering)),
                (13, texture_2d(TextureSampleType::Depth)),
            ),
        ),
    );
    let clouds = BindGroupLayoutDescriptor::new(
        "clouds_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::FRAGMENT,
            (
                uniform_buffer::<GpuCloudUniforms>(false),
                uniform_buffer::<GpuCloudPass>(false),
                texture_3d(TextureSampleType::default()),
                texture_3d(TextureSampleType::default()),
                texture_2d(TextureSampleType::default()),
                sampler(SamplerBindingType::Filtering),
            ),
        ),
    );
    let sampler = render_device.create_sampler(&repeat_linear().as_wgpu());
    let pipeline = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
        label: Some("clouds_pipeline".into()),
        layout: vec![atmosphere.clone(), clouds.clone()],
        vertex: fullscreen_shader.to_vertex_state(),
        fragment: Some(FragmentState {
            shader: asset_server.load(SHADER),
            targets: vec![Some(ColorTargetState {
                format: TARGET_FORMAT,
                blend: None,
                write_mask: ColorWrites::ALL,
            })],
            ..default()
        }),
        ..default()
    });
    commands.insert_resource(CloudPipeline {
        atmosphere,
        clouds,
        sampler,
        pipeline,
    });
}

/// The clouds of the main world, or none: the render world follows a clear sky too.
fn extract_clouds(
    mut commands: Commands,
    main_world: Res<MainWorld>,
    current: Option<Res<ExtractedClouds>>,
) {
    let frame = main_world.get_resource::<CloudFrame>();
    let images = main_world.get_resource::<CloudImages>();
    let target = main_world.get_resource::<CloudTarget>();
    match (frame, images, target) {
        (Some(frame), Some(images), Some(target)) => commands.insert_resource(ExtractedClouds {
            uniforms: frame.uniforms,
            pass: frame.pass,
            images: images.clone(),
            target: target.image.clone(),
        }),
        _ if current.is_some() => commands.remove_resource::<ExtractedClouds>(),
        _ => {}
    }
}

/// The uniform buffers of group 1.
#[derive(Resource, Default)]
struct CloudBuffers {
    uniforms: UniformBuffer<GpuCloudUniforms>,
    pass: UniformBuffer<GpuCloudPass>,
}

fn write_buffers(
    clouds: Option<Res<ExtractedClouds>>,
    mut buffers: ResMut<CloudBuffers>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
) {
    let Some(clouds) = clouds else {
        return;
    };
    let CloudBuffers { uniforms, pass } = buffers.as_mut();
    uniforms.set(clouds.uniforms);
    pass.set(clouds.pass);
    uniforms.write_buffer(&render_device, &render_queue);
    pass.write_buffer(&render_device, &render_queue);
}

/// Bind groups of one view.
#[derive(Component)]
struct CloudBindGroups {
    atmosphere: BindGroup,
    clouds: BindGroup,
}

#[expect(
    clippy::too_many_arguments,
    reason = "every resource of render_sky's layout, as Bevy's own system gathers them"
)]
fn prepare_bind_groups(
    mut commands: Commands,
    views: Query<(
        Entity,
        &ExtractedAtmosphere,
        &AtmosphereTextures,
        &ViewDepthStencilTexture,
    )>,
    clouds: Option<Res<ExtractedClouds>>,
    buffers: Res<CloudBuffers>,
    pipeline: Option<Res<CloudPipeline>>,
    images: Res<RenderAssets<GpuImage>>,
    render_device: Res<RenderDevice>,
    pipeline_cache: Res<PipelineCache>,
    atmosphere_sampler: Res<AtmosphereSampler>,
    view_uniforms: Res<ViewUniforms>,
    lights: Res<LightMeta>,
    transforms: Res<AtmosphereTransforms>,
    atmospheres: Res<ComponentUniforms<GpuAtmosphere>>,
    settings: Res<ComponentUniforms<GpuAtmosphereSettings>>,
    media: Res<RenderAssets<GpuScatteringMedium>>,
    medium_sampler: Res<ScatteringMediumSampler>,
) {
    let (Some(clouds), Some(pipeline)) = (clouds, pipeline) else {
        return;
    };
    let (Some(shape), Some(detail), Some(weather)) = (
        images.get(&clouds.images.shape),
        images.get(&clouds.images.detail),
        images.get(&clouds.images.weather),
    ) else {
        return;
    };
    let (
        Some(atmosphere_binding),
        Some(settings_binding),
        Some(transforms_binding),
        Some(view_binding),
        Some(lights_binding),
        Some(uniforms_binding),
        Some(pass_binding),
    ) = (
        atmospheres.binding(),
        settings.binding(),
        transforms.uniforms().binding(),
        view_uniforms.uniforms.binding(),
        lights.view_gpu_lights.binding(),
        buffers.uniforms.binding(),
        buffers.pass.binding(),
    )
    else {
        return;
    };
    let cloud_group = render_device.create_bind_group(
        "clouds_bind_group",
        &pipeline_cache.get_bind_group_layout(&pipeline.clouds),
        &BindGroupEntries::sequential((
            uniforms_binding,
            pass_binding,
            &shape.texture_view,
            &detail.texture_view,
            &weather.texture_view,
            &pipeline.sampler,
        )),
    );
    for (entity, atmosphere, textures, depth) in &views {
        let Some(depth_view) = depth.attachment.depth_stencil_views().depth_only_view() else {
            continue;
        };
        let Some(medium) = media.get(atmosphere.medium) else {
            continue;
        };
        let atmosphere_group = render_device.create_bind_group(
            "clouds_atmosphere_bind_group",
            &pipeline_cache.get_bind_group_layout(&pipeline.atmosphere),
            &BindGroupEntries::with_indices((
                (0, atmosphere_binding.clone()),
                (1, settings_binding.clone()),
                (2, transforms_binding.clone()),
                (3, view_binding.clone()),
                (4, lights_binding.clone()),
                (5, &medium.density_lut_view),
                (6, &medium.scattering_lut_view),
                (7, medium_sampler.sampler()),
                (8, &textures.transmittance_lut.default_view),
                (9, &textures.multiscattering_lut.default_view),
                (10, &textures.sky_view_lut.default_view),
                (11, &textures.aerial_view_lut.default_view),
                (12, &**atmosphere_sampler),
                (13, depth_view),
            )),
        );
        commands.entity(entity).insert(CloudBindGroups {
            atmosphere: atmosphere_group,
            clouds: cloud_group.clone(),
        });
    }
}

/// The march, into the cloud image: light of the clouds (rgb) and their transmittance (a).
#[expect(
    clippy::type_complexity,
    reason = "the dynamic offsets of render_sky's layout, one component each"
)]
fn march_clouds(
    view: ViewQuery<(
        &CloudBindGroups,
        &DynamicUniformIndex<GpuAtmosphere>,
        &DynamicUniformIndex<GpuAtmosphereSettings>,
        &AtmosphereTransformsOffset,
        &ViewUniformOffset,
        &ViewLightsUniformOffset,
    )>,
    clouds: Option<Res<ExtractedClouds>>,
    pipeline: Option<Res<CloudPipeline>>,
    pipeline_cache: Res<PipelineCache>,
    images: Res<RenderAssets<GpuImage>>,
    mut ctx: RenderContext,
) {
    let (Some(clouds), Some(pipeline)) = (clouds, pipeline) else {
        return;
    };
    let (Some(render_pipeline), Some(target)) = (
        pipeline_cache.get_render_pipeline(pipeline.pipeline),
        images.get(&clouds.target),
    ) else {
        return;
    };
    let (groups, atmosphere, settings, transforms, view_offset, lights) = view.into_inner();

    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let mut pass = ctx
        .command_encoder()
        .begin_render_pass(&RenderPassDescriptor {
            label: Some("clouds"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &target.texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Default::default()),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
    let span = diagnostics.pass_span(&mut pass, "clouds");
    pass.set_pipeline(render_pipeline);
    pass.set_bind_group(
        0,
        &groups.atmosphere,
        &[
            atmosphere.index(),
            settings.index(),
            transforms.index(),
            view_offset.offset,
            lights.offset,
        ],
    );
    pass.set_bind_group(1, &groups.clouds, &[]);
    pass.draw(0..3, 0..1);
    span.end(&mut pass);
}
