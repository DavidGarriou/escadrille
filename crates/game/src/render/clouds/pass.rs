//! The clouds in the render world (ADR 0009), two fullscreen passes of the `Core3d`
//! schedule, after the opaque objects and before the transparent ones:
//! - the ray march, at the camera's resolution divided by `divisor` on each side, writes
//!   the light and transmittance of the clouds and two distances (where it stopped, the
//!   mean distance of the clouds) into images of its own;
//! - the temporal pass blends them with the previous frames, reprojected at the distance
//!   of the clouds and clipped to the neighbourhood, into [`super::CloudTarget`], and keeps
//!   a copy for the next frame.
//!
//! Neither touches the main image, so their order against Bevy's `render_sky`, a private
//! system, does not matter: a transparent material blends the clouds after it
//! ([`super::CloudComposite`]).
//!
//! The march's group 0 rebuilds the layout of `render_sky` (bindings 0 to 13 of
//! `bevy_pbr::atmosphere::bindings`, then the depth), so that the shader can import the
//! atmosphere's functions: every resource it binds is public, only Bevy's own bind group
//! is not. A Bevy upgrade must check this layout against `render_sky`'s
//! (`bevy_pbr/src/atmosphere/resources.rs`); a mismatch stops the pipeline, and the
//! capture with it. Group 1 holds the clouds.

use bevy::core_pipeline::FullscreenShader;
use bevy::core_pipeline::core_3d::{main_opaque_pass_3d, main_transparent_pass_3d};
use bevy::core_pipeline::schedule::Core3d;
use bevy::math::{DVec3, Mat4, UVec2, Vec3};
use bevy::pbr::resources::{
    AtmosphereSampler, AtmosphereTextures, AtmosphereTransform, AtmosphereTransforms,
    AtmosphereTransformsOffset, GpuAtmosphere,
};
use bevy::pbr::{
    ExtractedAtmosphere, GpuAtmosphereSettings, GpuLights, GpuScatteringMedium, LightMeta,
    ScatteringMediumSampler, ViewLightsUniformOffset,
};
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy::render::diagnostic::RecordDiagnostics;
use bevy::render::extract_component::{ComponentUniforms, DynamicUniformIndex};
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::binding_types::{
    sampler, texture_2d, texture_3d, uniform_buffer,
};
use bevy::render::render_resource::{
    BindGroup, BindGroupEntries, BindGroupLayoutDescriptor, BindGroupLayoutEntries,
    CachedRenderPipelineId, ColorTargetState, ColorWrites, Extent3d, FilterMode, FragmentState,
    LoadOp, Operations, PipelineCache, RenderPassColorAttachment, RenderPassDescriptor,
    RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages,
    ShaderType, StoreOp, Texture, TextureDescriptor, TextureDimension, TextureFormat,
    TextureSampleType, TextureUsages, TextureView, TextureViewDescriptor, UniformBuffer,
};
use bevy::render::renderer::{RenderContext, RenderDevice, RenderQueue, ViewQuery};
use bevy::render::texture::GpuImage;
use bevy::render::view::{
    ExtractedView, ViewDepthStencilTexture, ViewUniform, ViewUniformOffset, ViewUniforms,
};
use bevy::render::{ExtractSchedule, MainWorld, Render, RenderApp, RenderStartup, RenderSystems};

use super::{
    CloudFrame, CloudImages, CloudTarget, DISTANCE_FORMAT, GpuCloudUniforms, TARGET_FORMAT,
    repeat_linear,
};

/// The shaders of the march and of the temporal pass, under `assets/`.
const MARCH_SHADER: &str = "shaders/clouds/render.wesl";
const TEMPORAL_SHADER: &str = "shaders/clouds/temporal.wesl";

/// Weight of the current frame in the temporal blend: about ten frames averaged, which
/// smooths the jitter of the steps.
const CURRENT_WEIGHT: f32 = 0.12;

/// The shader's `CloudPass`: how the rays march.
#[derive(ShaderType, Debug, Clone, Copy, PartialEq, Default)]
pub struct GpuCloudPass {
    pub frame: u32,
    pub steps: u32,
    pub light_steps: u32,
    /// Divisor of the camera's resolution, on each side.
    pub divisor: u32,
    pub min_step: f32,
    pub max_distance: f32,
}

/// The temporal shader's `CloudTemporal`. The views are centred on the eye: at 2 000 m,
/// a point of the near plane (5 cm away) minus the eye lost a few pixels to the f32
/// rounding, and the history slid down into curtains.
#[derive(ShaderType, Debug, Clone, Copy, PartialEq, Default)]
struct GpuCloudTemporal {
    /// This frame's view, from clip space back to world axes around the eye, unjittered.
    eye_from_clip: Mat4,
    /// The previous frame's view, from world axes around its eye to clip space.
    previous_clip_from_eye: Mat4,
    /// This eye from the previous one, m.
    eye_offset: Vec3,
    /// Weight of the current frame: 1 resets the history.
    current_weight: f32,
    max_distance: f32,
}

/// What the main world says of the clouds this frame, in the render world.
#[derive(Resource, Debug, Clone)]
struct ExtractedClouds {
    uniforms: GpuCloudUniforms,
    pass: GpuCloudPass,
    origin: DVec3,
    images: CloudImages,
    color: Handle<Image>,
    distance: Handle<Image>,
}

pub fn build(app: &mut App) {
    let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
        return;
    };
    render_app
        .init_resource::<CloudBuffers>()
        .init_resource::<CloudViews>()
        .add_systems(RenderStartup, init_pipelines)
        .add_systems(ExtractSchedule, extract_clouds)
        .add_systems(
            Render,
            (
                write_buffers.in_set(RenderSystems::PrepareResources),
                prepare_views.in_set(RenderSystems::PrepareResources),
                prepare_bind_groups.in_set(RenderSystems::PrepareBindGroups),
            ),
        )
        .add_systems(
            Core3d,
            (march_clouds, resolve_clouds)
                .chain()
                .after(main_opaque_pass_3d)
                .before(main_transparent_pass_3d),
        );
}

/// The layouts, samplers and pipelines of both passes.
#[derive(Resource)]
struct CloudPipelines {
    atmosphere: BindGroupLayoutDescriptor,
    clouds: BindGroupLayoutDescriptor,
    temporal: BindGroupLayoutDescriptor,
    noise_sampler: Sampler,
    history_sampler: Sampler,
    march: CachedRenderPipelineId,
    resolve: CachedRenderPipelineId,
}

fn init_pipelines(
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
    let temporal = BindGroupLayoutDescriptor::new(
        "clouds_temporal_layout",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::FRAGMENT,
            (
                texture_2d(TextureSampleType::Float { filterable: true }),
                texture_2d(TextureSampleType::Float { filterable: false }),
                texture_2d(TextureSampleType::Float { filterable: true }),
                sampler(SamplerBindingType::Filtering),
                uniform_buffer::<GpuCloudTemporal>(false),
            ),
        ),
    );
    let noise_sampler = render_device.create_sampler(&repeat_linear().as_wgpu());
    let history_sampler = render_device.create_sampler(&SamplerDescriptor {
        label: Some("clouds_history_sampler"),
        mag_filter: FilterMode::Linear,
        min_filter: FilterMode::Linear,
        ..default()
    });
    let targets = vec![
        Some(ColorTargetState {
            format: TARGET_FORMAT,
            blend: None,
            write_mask: ColorWrites::ALL,
        }),
        Some(ColorTargetState {
            format: DISTANCE_FORMAT,
            blend: None,
            write_mask: ColorWrites::ALL,
        }),
    ];
    let march = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
        label: Some("clouds_march_pipeline".into()),
        layout: vec![atmosphere.clone(), clouds.clone()],
        vertex: fullscreen_shader.to_vertex_state(),
        fragment: Some(FragmentState {
            shader: asset_server.load(MARCH_SHADER),
            targets: targets.clone(),
            ..default()
        }),
        ..default()
    });
    let resolve = pipeline_cache.queue_render_pipeline(RenderPipelineDescriptor {
        label: Some("clouds_temporal_pipeline".into()),
        layout: vec![temporal.clone()],
        vertex: fullscreen_shader.to_vertex_state(),
        fragment: Some(FragmentState {
            shader: asset_server.load(TEMPORAL_SHADER),
            targets,
            ..default()
        }),
        ..default()
    });
    commands.insert_resource(CloudPipelines {
        atmosphere,
        clouds,
        temporal,
        noise_sampler,
        history_sampler,
        march,
        resolve,
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
            origin: frame.origin,
            images: images.clone(),
            color: target.color.clone(),
            distance: target.distance.clone(),
        }),
        _ if current.is_some() => commands.remove_resource::<ExtractedClouds>(),
        _ => {}
    }
}

/// The uniform buffers of the march's group 1.
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

/// A texture and its view.
struct Owned {
    texture: Texture,
    view: TextureView,
}

impl Owned {
    fn new(
        render_device: &RenderDevice,
        label: &'static str,
        size: UVec2,
        format: TextureFormat,
    ) -> Self {
        let texture = render_device.create_texture(&TextureDescriptor {
            label: Some(label),
            size: Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format,
            usage: TextureUsages::RENDER_ATTACHMENT
                | TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let view = texture.create_view(&TextureViewDescriptor::default());
        Self { texture, view }
    }
}

/// What a view keeps from one frame to the next: the march's images, the previous
/// frame's cloud image, the previous view and render origin.
struct CloudView {
    size: UVec2,
    color: Owned,
    distance: Owned,
    history: Owned,
    /// The previous view around its eye, the eye, and the render origin.
    previous: Option<(Mat4, Vec3, DVec3)>,
    temporal: UniformBuffer<GpuCloudTemporal>,
    march_groups: Option<(BindGroup, BindGroup)>,
    temporal_group: Option<BindGroup>,
}

/// The views' state, by render-world view entity.
#[derive(Resource, Default)]
struct CloudViews(HashMap<Entity, CloudView>);

/// For each view: the march's images at the cloud size, and the temporal uniforms. The
/// history resets when the size or the render origin changes.
fn prepare_views(
    clouds: Option<Res<ExtractedClouds>>,
    views: Query<(Entity, &ExtractedView), With<ExtractedAtmosphere>>,
    images: Res<RenderAssets<GpuImage>>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    mut state: ResMut<CloudViews>,
) {
    let Some(clouds) = clouds else {
        state.0.clear();
        return;
    };
    let Some(target) = images.get(&clouds.color) else {
        return;
    };
    let size = UVec2::new(
        target.texture_descriptor.size.width,
        target.texture_descriptor.size.height,
    );
    state.0.retain(|entity, _| views.contains(*entity));
    for (entity, view) in &views {
        let fresh = || CloudView {
            size,
            color: Owned::new(&render_device, "clouds_march_color", size, TARGET_FORMAT),
            distance: Owned::new(
                &render_device,
                "clouds_march_distance",
                size,
                DISTANCE_FORMAT,
            ),
            history: Owned::new(&render_device, "clouds_history", size, TARGET_FORMAT),
            previous: None,
            temporal: UniformBuffer::default(),
            march_groups: None,
            temporal_group: None,
        };
        let cloud_view = state.0.entry(entity).or_insert_with(fresh);
        if cloud_view.size != size {
            *cloud_view = fresh();
        }
        let eye = view.world_from_view.translation();
        let rotation = Mat4::from_quat(view.world_from_view.rotation());
        let clip_from_eye = view.clip_from_view * rotation.inverse();
        let (previous_clip_from_eye, eye_offset, current_weight) = match cloud_view.previous {
            Some((previous, previous_eye, origin)) if origin == clouds.origin => {
                (previous, eye - previous_eye, CURRENT_WEIGHT)
            }
            _ => (clip_from_eye, Vec3::ZERO, 1.0),
        };
        cloud_view.temporal.set(GpuCloudTemporal {
            eye_from_clip: clip_from_eye.inverse(),
            previous_clip_from_eye,
            eye_offset,
            current_weight,
            max_distance: clouds.pass.max_distance,
        });
        cloud_view
            .temporal
            .write_buffer(&render_device, &render_queue);
        cloud_view.previous = Some((clip_from_eye, eye, clouds.origin));
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "every resource of render_sky's layout, as Bevy's own system gathers them"
)]
fn prepare_bind_groups(
    views: Query<(
        Entity,
        &ExtractedAtmosphere,
        &AtmosphereTextures,
        &ViewDepthStencilTexture,
    )>,
    clouds: Option<Res<ExtractedClouds>>,
    buffers: Res<CloudBuffers>,
    pipelines: Option<Res<CloudPipelines>>,
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
    mut state: ResMut<CloudViews>,
) {
    let (Some(clouds), Some(pipelines)) = (clouds, pipelines) else {
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
        &pipeline_cache.get_bind_group_layout(&pipelines.clouds),
        &BindGroupEntries::sequential((
            uniforms_binding,
            pass_binding,
            &shape.texture_view,
            &detail.texture_view,
            &weather.texture_view,
            &pipelines.noise_sampler,
        )),
    );
    for (entity, atmosphere, textures, depth) in &views {
        let Some(cloud_view) = state.0.get_mut(&entity) else {
            continue;
        };
        let Some(depth_view) = depth.attachment.depth_stencil_views().depth_only_view() else {
            continue;
        };
        let Some(medium) = media.get(atmosphere.medium) else {
            continue;
        };
        let Some(temporal_binding) = cloud_view.temporal.binding() else {
            continue;
        };
        let atmosphere_group = render_device.create_bind_group(
            "clouds_atmosphere_bind_group",
            &pipeline_cache.get_bind_group_layout(&pipelines.atmosphere),
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
        let temporal_group = render_device.create_bind_group(
            "clouds_temporal_bind_group",
            &pipeline_cache.get_bind_group_layout(&pipelines.temporal),
            &BindGroupEntries::sequential((
                &cloud_view.color.view,
                &cloud_view.distance.view,
                &cloud_view.history.view,
                &pipelines.history_sampler,
                temporal_binding,
            )),
        );
        cloud_view.march_groups = Some((atmosphere_group, cloud_group.clone()));
        cloud_view.temporal_group = Some(temporal_group);
    }
}

/// A colour attachment cleared, then stored.
fn attachment(view: &TextureView) -> Option<RenderPassColorAttachment<'_>> {
    Some(RenderPassColorAttachment {
        view,
        depth_slice: None,
        resolve_target: None,
        ops: Operations {
            load: LoadOp::Clear(Default::default()),
            store: StoreOp::Store,
        },
    })
}

/// The march, into the view's images: light of the clouds (rgb) and their transmittance
/// (a); where the march stopped and the mean distance of the clouds.
#[expect(
    clippy::type_complexity,
    reason = "the dynamic offsets of render_sky's layout, one component each"
)]
fn march_clouds(
    view: ViewQuery<(
        Entity,
        &DynamicUniformIndex<GpuAtmosphere>,
        &DynamicUniformIndex<GpuAtmosphereSettings>,
        &AtmosphereTransformsOffset,
        &ViewUniformOffset,
        &ViewLightsUniformOffset,
    )>,
    clouds: Option<Res<ExtractedClouds>>,
    pipelines: Option<Res<CloudPipelines>>,
    pipeline_cache: Res<PipelineCache>,
    state: Res<CloudViews>,
    mut ctx: RenderContext,
) {
    let (Some(_), Some(pipelines)) = (clouds, pipelines) else {
        return;
    };
    let (entity, atmosphere, settings, transforms, view_offset, lights) = view.into_inner();
    let Some(cloud_view) = state.0.get(&entity) else {
        return;
    };
    let (Some(pipeline), Some((atmosphere_group, cloud_group))) = (
        pipeline_cache.get_render_pipeline(pipelines.march),
        cloud_view.march_groups.as_ref(),
    ) else {
        return;
    };

    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let mut pass = ctx
        .command_encoder()
        .begin_render_pass(&RenderPassDescriptor {
            label: Some("clouds"),
            color_attachments: &[
                attachment(&cloud_view.color.view),
                attachment(&cloud_view.distance.view),
            ],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
    let span = diagnostics.pass_span(&mut pass, "clouds");
    pass.set_pipeline(pipeline);
    pass.set_bind_group(
        0,
        atmosphere_group,
        &[
            atmosphere.index(),
            settings.index(),
            transforms.index(),
            view_offset.offset,
            lights.offset,
        ],
    );
    pass.set_bind_group(1, cloud_group, &[]);
    pass.draw(0..3, 0..1);
    span.end(&mut pass);
}

/// The temporal pass, into the cloud images of [`super::CloudTarget`], then their colour
/// copied as the next frame's history.
fn resolve_clouds(
    view: ViewQuery<Entity>,
    clouds: Option<Res<ExtractedClouds>>,
    pipelines: Option<Res<CloudPipelines>>,
    pipeline_cache: Res<PipelineCache>,
    images: Res<RenderAssets<GpuImage>>,
    state: Res<CloudViews>,
    mut ctx: RenderContext,
) {
    let (Some(clouds), Some(pipelines)) = (clouds, pipelines) else {
        return;
    };
    let entity = view.into_inner();
    let Some(cloud_view) = state.0.get(&entity) else {
        return;
    };
    let (Some(pipeline), Some(group), Some(color), Some(distance)) = (
        pipeline_cache.get_render_pipeline(pipelines.resolve),
        cloud_view.temporal_group.as_ref(),
        images.get(&clouds.color),
        images.get(&clouds.distance),
    ) else {
        return;
    };

    let diagnostics = ctx.diagnostic_recorder();
    let diagnostics = diagnostics.as_deref();
    let encoder = ctx.command_encoder();
    {
        let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("clouds_temporal"),
            color_attachments: &[
                attachment(&color.texture_view),
                attachment(&distance.texture_view),
            ],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        let span = diagnostics.pass_span(&mut pass, "clouds_temporal");
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, group, &[]);
        pass.draw(0..3, 0..1);
        span.end(&mut pass);
    }
    encoder.copy_texture_to_texture(
        color.texture.as_image_copy(),
        cloud_view.history.texture.as_image_copy(),
        Extent3d {
            width: cloud_view.size.x,
            height: cloud_view.size.y,
            depth_or_array_layers: 1,
        },
    );
}
