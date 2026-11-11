// The stars (`crates/game/src/render/night.rs`): squares whose radiance, cd/m², is in the
// vertex colour, pre-exposed as Bevy's lights are, shown as the night falls.

import bevy_pbr::render::{forward_io::VertexOutput, mesh_view_bindings::view};

// x: how much of the stars shows.
@group(constants::MATERIAL_BIND_GROUP) @binding(0) var<uniform> fade: vec4<f32>;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var radiance = vec3<f32>(0.0);
    @if(VERTEX_COLORS)
    radiance = in.color.rgb;
    return vec4<f32>(radiance * fade.x * view.exposure, 1.0);
}
