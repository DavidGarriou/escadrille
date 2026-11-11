// The sun's glare veil (`crates/game/src/render/glare.rs`): L = 10 E / θ² cd/m² over the
// whole frame (Stiles–Holladay), added to it, pre-exposed as Bevy's lights are, gone when
// an object or the clouds hide the sun.

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
// 10 × the sun's illuminance at the eye (rgb), the nearest angle in degrees (a).
@group(constants::MATERIAL_BIND_GROUP) @binding(1) var<uniform> veil: vec4<f32>;
// Light (rgb) and transmittance (a) of the clouds.
@group(constants::MATERIAL_BIND_GROUP) @binding(2) var cloud_light: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(3) var<storage, read> cloud_fog: CloudFog;

// The sun's angular radius, rad: the five points of its disc tested for an object.
const SUN_RADIUS: f32 = 0.00465;

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
    let offsets = array<vec3<f32>, 5>(
        vec3<f32>(0.0), side, -side, up, -up,
    );
    var clear = 0.0;
    for (var i = 0; i < 5; i++) {
        let p = screen_point(normalize(sun + offsets[i] * SUN_RADIUS));
        var depth = 0.0;
        @if(DEPTH_PREPASS)
        depth = textureLoad(view_bindings::depth_prepass_texture, vec2<i32>(p.xy), 0);
        clear += select(0.0, 0.2, depth <= 0.0);
    }
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

@fragment
fn fragment(in: Varyings) -> @location(0) vec4<f32> {
    let direction = pixel_direction(in.position.xy);
    let angle = degrees(acos(clamp(dot(direction, normalize(to_sun.xyz)), -1.0, 1.0)));
    let theta = max(angle, veil.a);
    let luminance = veil.rgb / (theta * theta) * sun_seen();
    return vec4<f32>(luminance * view.exposure, 0.0);
}
