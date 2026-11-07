// The composite of the clouds (ADR 0009): the cloud image, `divisor` times smaller than the
// camera's on each side, upsampled over the whole screen by a transparent material drawn
// first in the transparent pass, after Bevy's sky. The image holds the light of the
// clouds and their transmittance T; premultiplied blending with alpha 1 − T gives
// light + image × T. Used by `CloudComposite` of `crates/game/src/render/clouds.rs`.
//
// Upsampling along the depth of the scene: of the four texels around a pixel, those whose
// march stopped at the pixel's own depth weigh the most (bilinear weight over the
// mismatch of the distances). A sky pixel by the aircraft's edge so takes the clouds of
// the sky texels, not those cut short by the aircraft; and a texel whose clouds lie behind
// the pixel's object leaves it clear.

import bevy_pbr::render::mesh_view_bindings::view;
import bevy_pbr::prepass::utils::prepass_depth;

struct Vertex {
    @location(0) position: vec3<f32>,
}

struct Varyings {
    @builtin(position) position: vec4<f32>,
}

@group(constants::MATERIAL_BIND_GROUP) @binding(0) var clouds: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(1) var clouds_sampler: sampler;
@group(constants::MATERIAL_BIND_GROUP) @binding(2) var distances: texture_2d<f32>;
// Divisor of the resolution, longest distance of a ray (m), unused, unused.
@group(constants::MATERIAL_BIND_GROUP) @binding(3) var<uniform> params: vec4<f32>;

// Relative mismatch of distances under which a texel counts as the pixel's own.
const MATCH: f32 = 0.01;

// The triangle is already in clip space; the depth test is off.
@vertex
fn vertex(vertex: Vertex) -> Varyings {
    return Varyings(vec4<f32>(vertex.position.xy, 0.0, 1.0));
}

// Distance of the pixel's scene from the eye, m: the longest distance for the sky.
fn scene_distance(frag: vec4<f32>) -> f32 {
    var depth = 0.0;
    @if(DEPTH_PREPASS)
    depth = prepass_depth(frag, 0u);
    if depth <= 0.0 {
        return params.y;
    }
    let uv = (frag.xy - view.viewport.xy) / view.viewport.zw;
    let ndc = vec3<f32>(uv * vec2<f32>(2.0, -2.0) + vec2<f32>(-1.0, 1.0), depth);
    let p = view.view_from_clip * vec4<f32>(ndc, 1.0);
    return min(length(p.xyz / p.w), params.y);
}

@fragment
fn fragment(in: Varyings) -> @location(0) vec4<f32> {
    let scene = scene_distance(in.position);
    let last = vec2<i32>(textureDimensions(clouds)) - 1;
    let coord = (in.position.xy - view.viewport.xy) / params.x - 0.5;
    let base = floor(coord);
    let f = coord - base;
    var sum = vec4<f32>(0.0);
    var total = 0.0;
    for (var j = 0; j < 2; j++) {
        for (var i = 0; i < 2; i++) {
            let texel = clamp(vec2<i32>(base) + vec2<i32>(i, j), vec2<i32>(0), last);
            let bilinear = select(1.0 - f.x, f.x, i == 1) * select(1.0 - f.y, f.y, j == 1);
            // Where the texel's march stopped, and the mean distance of its clouds.
            let d = textureLoad(distances, texel, 0).xy;
            var cloud = textureLoad(clouds, texel, 0);
            if d.y > scene && d.x > scene * 1.05 {
                cloud = vec4<f32>(0.0, 0.0, 0.0, 1.0);
            }
            let weight = bilinear / (MATCH + abs(d.x - scene) / max(scene, 1.0));
            sum += cloud * weight;
            total += weight;
        }
    }
    let cloud = sum / max(total, 1e-6);
    return vec4<f32>(cloud.rgb, 1.0 - cloud.a);
}
