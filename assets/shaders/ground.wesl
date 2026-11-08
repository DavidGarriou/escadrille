// The ground under the clouds (`crates/game/src/render/ground.rs`): the standard
// material's light, and the light of the clouds' bases overhead that the view's
// environment map misses when the camera flies above them. A Lambertian surface under an
// even radiance L from its upper half takes albedo × L × (1 + n·up) / 2, darkened in its
// hollows like the rest of the indirect light.

import bevy_pbr::render::{
    forward_io::{VertexOutput, FragmentOutput},
    mesh_view_bindings::view,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
};

struct CloudLitGround {
    // Radiance of the clouds' bases, cd/m² (rgb; a unused).
    light: vec4<f32>,
}

@group(constants::MATERIAL_BIND_GROUP) @binding(100)
var<uniform> ground: CloudLitGround;

@fragment
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);
    pbr_input.material.base_color = alpha_discard(
        pbr_input.material.flags,
        pbr_input.material.alpha_cutoff,
        pbr_input.material.base_color
    );
    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    let overhead = 0.5 * (1.0 + pbr_input.N.y);
    let clouds = pbr_input.material.base_color.rgb * ground.light.rgb * overhead
        * pbr_input.diffuse_occlusion;
    out.color += vec4<f32>(clouds * view.exposure, 0.0);
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
