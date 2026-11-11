// The moon (`crates/game/src/render/night.rs`): a sphere lit on the sun's side, with a
// narrow terminator, its radiance pre-exposed as Bevy's lights are.

import bevy_pbr::render::{forward_io::VertexOutput, mesh_view_bindings::view};

// Direction towards the sun, Bevy axes (xyz).
@group(constants::MATERIAL_BIND_GROUP) @binding(0) var<uniform> to_sun: vec4<f32>;
// Radiance of the lit surface, cd/m² (rgb).
@group(constants::MATERIAL_BIND_GROUP) @binding(1) var<uniform> radiance: vec4<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let n = normalize(in.world_normal);
    let lit = smoothstep(-0.02, 0.02, dot(n, to_sun.xyz));
    return vec4<f32>(radiance.rgb * lit * view.exposure, 1.0);
}
