// Night vision (`crates/game/src/render/night_vision.rs`, whose `seen` is this shader's
// reference): the cones' share from the CIE's mesopic adaptation, the rods' luminance by
// Larson's fit, seen as a cold grey; the reds darken, the blues brighten, the colours go.
// By day, the frame as it is.

import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;

// `NightVision`.
struct NightVision {
    // cd/m² per unit of the frame.
    to_luminance: f32,
    // The cones' share of the eye's adaptation, 0 to 1.
    adapted: f32,
}

@group(0) @binding(0) var frame: texture_2d<f32>;
@group(0) @binding(1) var frame_sampler: sampler;
@group(0) @binding(2) var<uniform> vision: NightVision;

// CIE 191:2010: m = a + b log₁₀ L, 0 at 0.005 cd/m², 1 at 5.
const MESOPIC_A: f32 = 0.7670;
const MESOPIC_B: f32 = 0.3334;
const LOG10_OF_2: f32 = 0.30103;
// Scotopic over photopic luminance of daylight's white (`DAYLIGHT_SCOTOPIC_RATIO`).
const DAYLIGHT_SCOTOPIC_RATIO: f32 = 2.5729;
// The rods' grey, as seen: xy of a black body at 15 000 K (`NIGHT_WHITE`).
const NIGHT_WHITE: vec2<f32> = vec2(0.2637, 0.2672);

// Linear Rec. 709 to CIE XYZ, and back (columns).
const RGB_TO_XYZ: mat3x3<f32> = mat3x3<f32>(
    vec3(0.4124564, 0.2126729, 0.0193339),
    vec3(0.3575761, 0.7151522, 0.1191920),
    vec3(0.1804375, 0.0721750, 0.9503041),
);
const XYZ_TO_RGB: mat3x3<f32> = mat3x3<f32>(
    vec3(3.2404542, -0.9692660, 0.0556434),
    vec3(-1.5371385, 1.8760108, -0.2040259),
    vec3(-0.4985314, 0.0415560, 1.0572252),
);

// The cones' share at a luminance, cd/m².
fn cones_at(luminance: f32) -> f32 {
    return clamp(MESOPIC_A + MESOPIC_B * LOG10_OF_2 * log2(max(luminance, 1e-9)), 0.0, 1.0);
}

// Scotopic over photopic luminance of an XYZ colour (Larson, Rushmeier and Piatko).
fn scotopic_ratio(xyz: vec3<f32>) -> f32 {
    if (xyz.x <= 0.0) {
        return 0.0;
    }
    return max(1.33 * (1.0 + (xyz.y + xyz.z) / xyz.x) - 1.68, 0.0);
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let pixel = textureLoad(frame, vec2<i32>(in.position.xy), 0);
    if (vision.adapted >= 1.0) {
        return pixel;
    }
    let xyz = RGB_TO_XYZ * max(pixel.rgb, vec3(0.0));
    let cones = max(vision.adapted, cones_at(xyz.y * vision.to_luminance));
    if (cones >= 1.0) {
        return pixel;
    }
    let rods = xyz.y * scotopic_ratio(xyz) / DAYLIGHT_SCOTOPIC_RATIO;
    let grey = rods * vec3(
        NIGHT_WHITE.x / NIGHT_WHITE.y,
        1.0,
        (1.0 - NIGHT_WHITE.x - NIGHT_WHITE.y) / NIGHT_WHITE.y,
    );
    let rgb = max(XYZ_TO_RGB * mix(grey, xyz, cones), vec3(0.0));
    return vec4(rgb, pixel.a);
}
