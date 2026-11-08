// The standard materials lit with their own normals (`crates/game/src/render/
// own_normals.rs`): Bevy's forward fragment shader (`bevy_pbr`, `render/pbr.wesl`), with
// the steps this game uses, and one more: the blended ones, drawn after the clouds'
// composite, are veiled by the clouds' fog in front of them (`clouds/fog.wesl`). The
// opaque ones (the varnished propeller) are veiled by the composite itself.

import bevy_pbr::render::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_types,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::{alpha_discard, apply_pbr_lighting, main_pass_post_lighting_processing},
};
import super::clouds::fog::veil;

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
    if (pbr_input.material.flags & pbr_types::STANDARD_MATERIAL_FLAGS_UNLIT_BIT) == 0u {
        out.color = apply_pbr_lighting(pbr_input);
    } else {
        out.color = pbr_input.material.base_color;
    }
    let alpha_mode = pbr_input.material.flags
        & pbr_types::STANDARD_MATERIAL_FLAGS_ALPHA_MODE_RESERVED_BITS;
    if alpha_mode == pbr_types::STANDARD_MATERIAL_FLAGS_ALPHA_MODE_BLEND
        || alpha_mode == pbr_types::STANDARD_MATERIAL_FLAGS_ALPHA_MODE_PREMULTIPLIED {
        out.color = veil(out.color, in.position.xy, in.world_position.xyz, true);
    } else if alpha_mode == pbr_types::STANDARD_MATERIAL_FLAGS_ALPHA_MODE_ADD {
        out.color = veil(out.color, in.position.xy, in.world_position.xyz, false);
    }
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
