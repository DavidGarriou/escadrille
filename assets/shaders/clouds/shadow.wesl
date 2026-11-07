// The shadow map of the clouds (ADR 0009): for each texel, the sunlight's transmittance
// through the layer of clouds along its ray towards the sun, e^−τ on the coarse density.
// The sun carries the map as its light texture (`DirectionalLightTexture`), which Bevy
// samples by projecting each lit point along the sun's direction: the ground and the
// aircraft under a cloud lose its direct light. Used by
// `crates/game/src/render/clouds/pass.rs`.

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import super::density::{CloudUniforms, cloud_coarse};

// `GpuCloudShadow` of the pass.
struct CloudShadow {
    center: vec3<f32>,
    half_span: f32,
    right: vec3<f32>,
    steps: u32,
    up: vec3<f32>,
    focus_radius: f32,
    to_sun: vec3<f32>,
    // The aircraft: along its line towards the sun, only the clouds above it count, since
    // a ray holds one value for all its heights.
    focus: vec3<f32>,
}

@group(0) @binding(0) var<uniform> shadow: CloudShadow;
@group(1) @binding(0) var<uniform> clouds: CloudUniforms;
@group(1) @binding(2) var shape_noise: texture_3d<f32>;
@group(1) @binding(4) var weather_map: texture_2d<f32>;
@group(1) @binding(5) var cloud_sampler: sampler;

// Share of the ground in the clouds' shadow for a cover: about 0.76 of the cover
// (`escadrille_weather`, test `a_summer_afternoon_has_scattered_cumulus`). ESTIMÉ.
const SHADED_PER_COVER: f32 = 0.76;
// The outer share of the map fades to the mean transmittance of the layer: the map
// repeats beyond its edge, and an even veil reads better far away than a copy of the
// near shadows.
const EDGE_FADE: f32 = 0.2;

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let lit = vec4<f32>(1.0, 0.0, 0.0, 1.0);
    let to_sun = shadow.to_sun;
    if to_sun.y <= 1e-3 {
        return lit;
    }
    // Bevy's mapping of a light texture: uv = (−x, y) / 2 + ½ in the light's local frame.
    let local = vec2<f32>(1.0 - 2.0 * in.uv.x, 2.0 * in.uv.y - 1.0) * shadow.half_span;
    let base = shadow.center + shadow.right * local.x + shadow.up * local.y;

    let bottom = clouds.base_altitude - clouds.altitude_offset;
    let top = bottom + max(clouds.thickness.x, max(clouds.thickness.y, clouds.thickness.z));
    // Near the aircraft's line, the column starts at the aircraft.
    let offset = base - shadow.focus;
    let across = length(offset - to_sun * dot(offset, to_sun));
    let near_focus = 1.0 - smoothstep(shadow.focus_radius, 2.0 * shadow.focus_radius, across);
    let low = mix(bottom, max(bottom, shadow.focus.y), near_focus);

    let start = (low - base.y) / to_sun.y;
    let end = (top - base.y) / to_sun.y;
    if end <= start {
        return lit;
    }
    let step = (end - start) / f32(shadow.steps);
    var density = 0.0;
    for (var i = 0u; i < shadow.steps; i++) {
        let at = base + to_sun * (start + (f32(i) + 0.5) * step);
        density += cloud_coarse(clouds, shape_noise, weather_map, cloud_sampler, at).x;
    }
    let transmittance = exp(-density * step * clouds.extinction);
    let mean = 1.0 - SHADED_PER_COVER * clouds.cover;
    let edge = max(abs(local.x), abs(local.y)) / shadow.half_span;
    let fade = smoothstep(1.0 - EDGE_FADE, 1.0, edge);
    return vec4<f32>(mix(transmittance, mean, fade), 0.0, 0.0, 1.0);
}
