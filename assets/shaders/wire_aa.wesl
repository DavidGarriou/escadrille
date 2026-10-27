// Wires thinner than a pixel ("phone-wire AA", skill rendu-visuel): each wire is drawn
// at least `min_pixels` wide on screen, and its opacity cut in proportion to the
// coverage of its true width, so that it neither breaks into dashes nor shimmers.
// The wire's mesh is a smooth tube: its normals point away from its axis, along which
// the vertex shader moves each vertex. Used by `crates/game/src/render/wires.rs` with
// the StandardMaterial of the model's `wire_*` nodes, in alpha blending.

import bevy_pbr::render::{
    mesh_functions,
    forward_io::{Vertex, VertexOutput, FragmentOutput, decompress_vertex},
    mesh_view_bindings::view,
    view_transformations::{position_world_to_clip, position_world_to_view},
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
};

struct Wire {
    // True radius of the wire, m.
    radius: f32,
    // Smallest width drawn on screen, pixels.
    min_pixels: f32,
    _padding_a: f32,
    _padding_b: f32,
}

@group(constants::MATERIAL_BIND_GROUP) @binding(100)
var<uniform> wire: Wire;

// Width in metres of one pixel of the view at a world position (perspective).
fn pixel_size(world_position: vec3<f32>) -> f32 {
    let distance = max(-position_world_to_view(world_position).z, 1e-4);
    return 2.0 * distance / (view.clip_from_view[1][1] * view.viewport.w);
}

// Radius drawn at a world position: the true one, or half the smallest width.
fn drawn_radius(world_position: vec3<f32>) -> f32 {
    return max(wire.radius, 0.5 * wire.min_pixels * pixel_size(world_position));
}

@vertex
fn vertex(vertex_in: Vertex) -> VertexOutput {
    var out: VertexOutput;
    let vertex = decompress_vertex(vertex_in, vertex_in.instance_index);
    let world_from_local = mesh_functions::get_world_from_local(vertex_in.instance_index);
    out.world_normal = mesh_functions::mesh_normal_local_to_world(
        vertex.normal,
        vertex_in.instance_index
    );
    let surface = mesh_functions::mesh_position_local_to_world(
        world_from_local,
        vec4<f32>(vertex.position, 1.0)
    ).xyz;
    let widened = surface + normalize(out.world_normal) * (drawn_radius(surface) - wire.radius);
    out.world_position = vec4<f32>(widened, 1.0);
    out.position = position_world_to_clip(widened);
@if(VERTEX_UVS_A)
    out.uv = vertex.uv;
@if(VERTEX_UVS_B)
    out.uv_b = vertex.uv_b;
@if(VERTEX_TANGENTS)
    out.world_tangent = mesh_functions::mesh_tangent_local_to_world(
        world_from_local,
        vertex.tangent,
        vertex_in.instance_index
    );
@if(VERTEX_COLORS)
    out.color = vertex.color;
@if(VERTEX_OUTPUT_INSTANCE_INDEX)
    out.instance_index = vertex_in.instance_index;
@if(VISIBILITY_RANGE_DITHER)
    out.visibility_range_dither = mesh_functions::get_visibility_range_dither_level(
        vertex_in.instance_index, world_from_local[3]);
    return out;
}

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    // The share of the drawn width the true wire covers.
    let coverage = wire.radius / drawn_radius(in.world_position.xyz);
    pbr_input.material.base_color.a *= coverage;
    pbr_input.material.base_color = alpha_discard(
        pbr_input.material.flags,
        pbr_input.material.alpha_cutoff,
        pbr_input.material.base_color
    );
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
