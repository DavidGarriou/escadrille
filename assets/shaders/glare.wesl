// The sun's glare veil (`crates/game/src/render/glare.rs`): the CIE's general disability
// glare equation over the whole frame (`exposure::veil_per_lux`, the same in Rust), added
// to it, pre-exposed as Bevy's lights are, gone as far as an object or the clouds hide the
// sun.

import bevy_pbr::render::mesh_view_bindings::view;
import bevy_pbr::render::mesh_view_bindings as view_bindings;

struct Vertex {
    @location(0) position: vec3<f32>,
}

struct Varyings {
    @builtin(position) position: vec4<f32>,
}

// `CloudFog` of `clouds/fog.rs`.
struct CloudFog {
    extinction: f32,
    divisor: f32,
}

// Direction towards the sun, Bevy axes (xyz).
@group(constants::MATERIAL_BIND_GROUP) @binding(0) var<uniform> to_sun: vec4<f32>;
// The sun's illuminance at the eye, lux (rgb).
@group(constants::MATERIAL_BIND_GROUP) @binding(1) var<uniform> sun_light: vec4<f32>;
// Light (rgb) and transmittance (a) of the clouds.
@group(constants::MATERIAL_BIND_GROUP) @binding(2) var cloud_light: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(3) var<storage, read> cloud_fog: CloudFog;
// 1 + (A/62.5)⁴, the eye's pigmentation, the nearest angle in degrees.
@group(constants::MATERIAL_BIND_GROUP) @binding(4) var<uniform> eye: vec4<f32>;
// How much of the sun shows, written at the first pixel for the exposure to read back.
@group(constants::MATERIAL_BIND_GROUP) @binding(5) var<storage, read_write> seen_out: f32;

// The sun's angular radius, rad, and the points of its disc tested for an object: a
// sunflower spiral, even over its area.
const SUN_RADIUS: f32 = 0.00465;
const DISC_POINTS: i32 = 16;
const GOLDEN_ANGLE: f32 = 2.39996323;
// The frame is half floats, up to 65 504: room left for what else adds there.
const HALF_FLOAT_ROOM: f32 = 30000.0;

@vertex
fn vertex(vertex: Vertex) -> Varyings {
    return Varyings(vec4<f32>(vertex.position.xy, 0.0, 1.0));
}

// The direction of the pixel `frag`, world axes.
fn pixel_direction(frag: vec2<f32>) -> vec3<f32> {
    let uv = (frag - view.viewport.xy) / view.viewport.zw;
    let ndc = vec4<f32>(uv * vec2<f32>(2.0, -2.0) + vec2<f32>(-1.0, 1.0), 1.0, 1.0);
    let p = view.view_from_clip * ndc;
    let in_view = normalize(p.xyz / p.w);
    return normalize((view.world_from_view * vec4<f32>(in_view, 0.0)).xyz);
}

// The pixel where `direction` (world axes) falls, and whether it lies on the screen.
fn screen_point(direction: vec3<f32>) -> vec3<f32> {
    let clip = view.clip_from_world * vec4<f32>(direction, 0.0);
    if clip.w <= 0.0 {
        return vec3<f32>(0.0, 0.0, 0.0);
    }
    let ndc = clip.xy / clip.w;
    if any(abs(ndc) > vec2<f32>(1.0)) {
        return vec3<f32>(0.0, 0.0, 0.0);
    }
    let uv = ndc * vec2<f32>(0.5, -0.5) + 0.5;
    return vec3<f32>(uv * view.viewport.zw + view.viewport.xy, 1.0);
}

// How much of the sun shows: 1 off the screen, else the share of five points of its disc
// that no object covers, times the clouds' transmittance at its pixel.
fn sun_seen() -> f32 {
    let sun = normalize(to_sun.xyz);
    let centre = screen_point(sun);
    if centre.z == 0.0 {
        return 1.0;
    }
    let side = normalize(cross(sun, vec3<f32>(0.0, 1.0, 0.0001)));
    let up = cross(side, sun);
    var clear = 0.0;
    for (var i = 0; i < DISC_POINTS; i++) {
        let r = SUN_RADIUS * sqrt((f32(i) + 0.5) / f32(DISC_POINTS));
        let a = f32(i) * GOLDEN_ANGLE;
        let p = screen_point(normalize(sun + (side * cos(a) + up * sin(a)) * r));
        var depth = 0.0;
        @if(DEPTH_PREPASS)
        depth = textureLoad(view_bindings::depth_prepass_texture, vec2<i32>(p.xy), 0);
        clear += select(0.0, 1.0, depth <= 0.0);
    }
    clear /= f32(DISC_POINTS);
    var through = 1.0;
    if cloud_fog.divisor > 0.0 {
        let last = vec2<i32>(textureDimensions(cloud_light)) - 1;
        let texel = clamp(
            vec2<i32>((centre.xy - view.viewport.xy) / cloud_fog.divisor),
            vec2<i32>(0),
            last,
        );
        through = textureLoad(cloud_light, texel, 0).a;
    }
    return clear * through;
}

// Veiling luminance per lux at the eye, cd/m², at `theta` degrees: the CIE's equation.
fn veil_per_lux(angle: f32) -> f32 {
    let theta = max(angle, eye.z);
    let p = eye.y;
    return 10.0 / (theta * theta * theta)
        + (5.0 / (theta * theta) + 0.1 * p / theta) * eye.x
        + 0.0025 * p;
}

@fragment
fn fragment(in: Varyings) -> @location(0) vec4<f32> {
    let direction = pixel_direction(in.position.xy);
    let angle = degrees(acos(clamp(dot(direction, normalize(to_sun.xyz)), -1.0, 1.0)));
    let seen = sun_seen();
    if all(vec2<i32>(in.position.xy) == vec2<i32>(view.viewport.xy)) {
        seen_out = seen;
    }
    let luminance = sun_light.rgb * veil_per_lux(angle) * seen;
    // Within the largest half float of the frame: the disc's core is white anyway.
    return vec4<f32>(min(luminance * view.exposure, vec3<f32>(HALF_FLOAT_ROOM)), 0.0);
}
