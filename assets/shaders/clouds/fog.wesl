// The clouds' fog over a transparent surface (`crates/game/src/render/clouds/fog.rs`),
// drawn after the composite of the clouds, which veils only the sky and the opaque
// objects. Imported by the fragment shaders of the wires (`wire_aa.wesl`) and of the
// blended standard materials (`own_normals.wesl`), whose materials bind the cloud images
// and the fog's buffer at 101 to 103.
//
// The fog in front of the surface comes from the cloud image at its pixel: the light and
// transmittance of the march there, when the march stopped at the surface's distance (on
// the aircraft behind it); when it went on behind the surface, the transmittance of the
// clouds' extinction at the eye over the surface's distance, and the march's light in the
// same share of its fog. Out of the clouds, the extinction at the eye is zero: no fog,
// whatever clouds lie behind.

import bevy_pbr::render::mesh_view_bindings::view;

// `CloudFog` of `fog.rs`.
struct CloudFog {
    // Extinction of the clouds at the eye, 1/m.
    extinction: f32,
    // Divisor of the cloud images; 0 under a clear sky.
    divisor: f32,
}

// Light (rgb, exposed) and transmittance (a) of the clouds after the temporal pass.
@group(constants::MATERIAL_BIND_GROUP) @binding(101) var cloud_light: texture_2d<f32>;
// Where each march stopped, m, and the mean distance of its clouds.
@group(constants::MATERIAL_BIND_GROUP) @binding(102) var cloud_distance: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(103) var<storage, read> cloud_fog: CloudFog;

// A march that stopped within this share beyond the surface stopped on it, or on what lies
// just behind: the aircraft under its wires.
const SAME_DISTANCE: f32 = 1.1;

// The colour of a surface at `world_position`, drawn at the pixel `frag_coord`, seen
// through the fog in front of it: colour × T + light, before any premultiplication.
// `lit` is false for a surface that only adds light, which the fog dims but does not
// replace.
fn veil(color: vec4<f32>, frag_coord: vec2<f32>, world_position: vec3<f32>, lit: bool) -> vec4<f32> {
    if cloud_fog.divisor <= 0.0 {
        return color;
    }
    let distance = length(world_position - view.world_position);
    let last = vec2<i32>(textureDimensions(cloud_light)) - 1;
    let texel = clamp(
        vec2<i32>((frag_coord - view.viewport.xy) / cloud_fog.divisor),
        vec2<i32>(0),
        last,
    );
    let clouds = textureLoad(cloud_light, texel, 0);
    let stop = textureLoad(cloud_distance, texel, 0).x;
    var through = clouds.a;
    var light = clouds.rgb;
    if stop > distance * SAME_DISTANCE {
        through = max(clouds.a, exp(-cloud_fog.extinction * distance));
        light *= min(1.0, (1.0 - through) / max(1.0 - clouds.a, 1e-4));
    }
    if !lit {
        light = vec3<f32>(0.0);
    }
    return vec4<f32>(color.rgb * through + light, color.a);
}
