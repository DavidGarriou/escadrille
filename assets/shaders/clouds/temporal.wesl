// The temporal pass of the clouds (ADR 0009): this frame's ray march blended with the
// previous frames, at the march's resolution. Used by
// `crates/game/src/render/clouds/pass.rs`.
//
// Each texel looks for its clouds in the previous frame where they were: its ray, at the
// mean distance of the clouds (or far, through clear sky), projected by the previous
// view. The history found there is clipped to the neighbourhood of this frame (its mean
// ± CLIP_SIGMAS standard deviations: Salvi, "An Excursion in Temporal Supersampling",
// GDC 2016), so that what moved does not leave trails, then blended with this frame. The
// march jitters its steps every frame: the blend averages them.

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;

// `GpuCloudTemporal` of the pass. Centred on the eye: subtracting the eye from a point of
// the near plane, 5 cm away, at 2 000 m lost a few pixels to the f32 rounding.
struct CloudTemporal {
    // This frame's view, from clip space back to world axes around the eye, unjittered.
    eye_from_clip: mat4x4<f32>,
    // The previous frame's view, from world axes around its eye to clip space.
    previous_clip_from_eye: mat4x4<f32>,
    // This eye from the previous one, m.
    eye_offset: vec3<f32>,
    // Weight of this frame: 1 resets the history.
    current_weight: f32,
    // Longest distance of a ray, m.
    max_distance: f32,
}

@group(0) @binding(0) var current_color: texture_2d<f32>;
@group(0) @binding(1) var current_distance: texture_2d<f32>;
@group(0) @binding(2) var history_color: texture_2d<f32>;
@group(0) @binding(3) var history_sampler: sampler;
@group(0) @binding(4) var<uniform> temporal: CloudTemporal;

const CLIP_SIGMAS: f32 = 1.25;

struct Resolved {
    @location(0) color: vec4<f32>,
    @location(1) distance: vec2<f32>,
}

fn uv_to_ndc(uv: vec2<f32>) -> vec2<f32> {
    return uv * vec2<f32>(2.0, -2.0) + vec2<f32>(-1.0, 1.0);
}

fn ndc_to_uv(ndc: vec2<f32>) -> vec2<f32> {
    return ndc * vec2<f32>(0.5, -0.5) + vec2<f32>(0.5);
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> Resolved {
    let texel = vec2<i32>(in.position.xy);
    let last = vec2<i32>(textureDimensions(current_color)) - 1;
    let current = textureLoad(current_color, texel, 0);
    let distances = textureLoad(current_distance, texel, 0).xy;

    // Mean and spread of this frame around the texel.
    var mean = vec4<f32>(0.0);
    var square = vec4<f32>(0.0);
    for (var j = -1; j <= 1; j++) {
        for (var i = -1; i <= 1; i++) {
            let c = textureLoad(current_color, clamp(texel + vec2<i32>(i, j), vec2<i32>(0), last), 0);
            mean += c;
            square += c * c;
        }
    }
    mean /= 9.0;
    let sigma = sqrt(max(square / 9.0 - mean * mean, vec4<f32>(0.0)));

    // Where the clouds of this texel were in the previous frame. Reversed depth: the near
    // plane is at z = 1.
    let near = temporal.eye_from_clip * vec4<f32>(uv_to_ndc(in.uv), 1.0, 1.0);
    let direction = normalize(near.xyz / near.w);
    let position = direction * min(distances.y, temporal.max_distance) + temporal.eye_offset;
    let previous = temporal.previous_clip_from_eye * vec4<f32>(position, 1.0);
    let previous_uv = ndc_to_uv(previous.xy / previous.w);

    var weight = temporal.current_weight;
    if previous.w <= 0.0 || any(previous_uv < vec2<f32>(0.0)) || any(previous_uv > vec2<f32>(1.0)) {
        weight = 1.0;
    }
    let history = textureSampleLevel(history_color, history_sampler, previous_uv, 0.0);
    let clipped = clamp(history, mean - CLIP_SIGMAS * sigma, mean + CLIP_SIGMAS * sigma);
    return Resolved(mix(clipped, current, weight), distances);
}
