// The composite of the clouds (ADR 0009): the image of the ray march
// (`shaders/clouds/render.wesl`) blended over the whole screen by a transparent material,
// drawn first in the transparent pass, after Bevy's sky. The image holds the light of the
// clouds and their transmittance T; premultiplied blending with alpha 1 − T gives
// light + image × T. Used by `CloudComposite` of `crates/game/src/render/clouds.rs`.

struct Vertex {
    @location(0) position: vec3<f32>,
}

struct Varyings {
    @builtin(position) position: vec4<f32>,
}

@group(constants::MATERIAL_BIND_GROUP) @binding(0) var clouds: texture_2d<f32>;
@group(constants::MATERIAL_BIND_GROUP) @binding(1) var clouds_sampler: sampler;

// The triangle is already in clip space; the depth test is off.
@vertex
fn vertex(vertex: Vertex) -> Varyings {
    return Varyings(vec4<f32>(vertex.position.xy, 0.0, 1.0));
}

// The cloud image has the size of the target: one texel per pixel.
@fragment
fn fragment(in: Varyings) -> @location(0) vec4<f32> {
    let cloud = textureLoad(clouds, vec2<i32>(in.position.xy), 0);
    return vec4<f32>(cloud.rgb, 1.0 - cloud.a);
}
