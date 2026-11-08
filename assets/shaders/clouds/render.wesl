// The clouds seen from the camera (ADR 0009): for each pixel of an image `divisor` times
// smaller than the camera's on each side, the light the clouds send towards the eye and
// their transmittance, and two distances: where the march stopped (the nearest object of
// the pixel's block, or the longest distance) and the mean distance of the clouds. A
// temporal pass blends it with the previous frames (`temporal.wesl`), then a transparent
// material upsamples it over the sky and the opaque objects (`composite.wesl`):
// colour = light + image × transmittance. Used by `crates/game/src/render/clouds/pass.rs`.
//
// Group 0 is the layout of `render_sky`, rebuilt by the pass, with the depth of the main
// pass; group 1 holds the clouds. The march itself is `march.wesl`'s.

import bevy_pbr::atmosphere::{
    bindings::view,
    functions::{sample_aerial_view_lut, uv_to_ray_direction, uv_to_ndc, get_view_position},
};
import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import super::march::{air_transmittance, cloud_pass, jitter, march};

// The depth of the main pass, binding 13 of `render_sky`'s layout.
@group(0) @binding(13) var depth_texture: texture_depth_2d;

struct MarchOutput {
    // Light (rgb, exposed) and transmittance (a).
    @location(0) color: vec4<f32>,
    // Where the march stopped, and the mean distance of the clouds, m.
    @location(1) distance: vec2<f32>,
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> MarchOutput {
    let direction = uv_to_ray_direction(in.uv);

    // How far the ray goes: to the nearest opaque object of the pixel's block (reversed
    // depth: the largest value), or to the longest distance.
    let divisor = i32(cloud_pass.divisor);
    let block = vec2<i32>(in.position.xy) * divisor;
    let last = vec2<i32>(textureDimensions(depth_texture)) - 1;
    var depth = 0.0;
    for (var j = 0; j < divisor; j++) {
        for (var i = 0; i < divisor; i++) {
            depth = max(depth, textureLoad(depth_texture, min(block + vec2<i32>(i, j), last), 0));
        }
    }
    var stop = cloud_pass.max_distance;
    if depth > 0.0 {
        let p = view.view_from_clip * vec4<f32>(uv_to_ndc(in.uv), depth, 1.0);
        stop = min(stop, length(p.xyz / p.w));
    }

    let marched = march(
        view.world_position, direction, stop, jitter(in.position.xy, cloud_pass.frame),
    );
    if marched.transmittance >= 1.0 {
        return MarchOutput(vec4<f32>(0.0, 0.0, 0.0, 1.0), vec2<f32>(stop, stop));
    }

    // Aerial perspective between the eye and the clouds, at their mean distance.
    let view_position = get_view_position();
    let r_view = length(view_position);
    let mu_view = dot(direction, view_position / r_view);
    let air = air_transmittance(r_view, mu_view, marched.distance);
    let haze = sample_aerial_view_lut(in.uv, marched.distance);
    let color = marched.light * air + haze * (1.0 - marched.transmittance);
    return MarchOutput(
        vec4<f32>(color * view.exposure, marched.transmittance),
        vec2<f32>(stop, marched.distance),
    );
}
