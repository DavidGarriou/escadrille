// Cloud density: the GPU twin of `CloudTextures::density` of the crate
// `escadrille_weather` (crates/weather/src/clouds.rs), which documents the model. Same
// textures, same steps, same order: any change goes to both, and the parity test of the
// game (crates/game/src/render/clouds.rs) compares them.
//
// Positions are relative to the origin of the uniforms (the render origin), in the game's
// world frame: x east, y up, z south, metres. Textures and sampler are parameters, so
// that each pass binds them where it likes; the sampler filters linearly and repeats.
// Plain WGSL, without imports: the parity test compiles it as it is.

// `CloudUniforms` of `escadrille_weather`, field for field.
struct CloudUniforms {
    // Offset of the shape noise, in repeats, and its repeats per metre.
    shape_offset: vec3<f32>,
    shape_inverse_scale: f32,
    detail_offset: vec3<f32>,
    detail_inverse_scale: f32,
    // Offset of the weather map (x east, z south), in repeats, and its repeats per metre.
    weather_offset: vec2<f32>,
    weather_inverse_scale: f32,
    // Altitude of the origin, m.
    altitude_offset: f32,
    // Thickness of a layer of stratus, stratocumulus and cumulus, m.
    thickness: vec3<f32>,
    // Cloud base, m above mean sea level.
    base_altitude: f32,
    cover: f32,
    cover_softness: f32,
    cloud_type: f32,
    type_variation: f32,
    shape_erosion: f32,
    detail_erosion: f32,
    // Extinction coefficient at full density, 1/m.
    extinction: f32,
}

// Height fraction where each type (stratus, stratocumulus, cumulus) reaches its full
// profile, and where it starts to fall off towards its top.
const CLOUD_RISE_END = vec3<f32>(0.1, 0.15, 0.06);
const CLOUD_FALL_START = vec3<f32>(0.4, 0.5, 0.65);

// Height fraction over which the detail noise turns from wisps (at the base) to billows.
const CLOUD_WISPS_HEIGHT: f32 = 0.03;

// A quantity of the three types of cloud, blended for a type between them: stratus at 0,
// stratocumulus at ½, cumulus at 1.
fn cloud_blend_types(values: vec3<f32>, cloud_type: f32) -> f32 {
    return mix(
        mix(values.x, values.y, saturate(2.0 * cloud_type)),
        values.z,
        saturate(2.0 * cloud_type - 1.0),
    );
}

// Vertical profile of a type of cloud at a height fraction of its layer.
fn cloud_profile(height: f32, cloud_type: f32) -> f32 {
    let rise = vec3<f32>(
        smoothstep(0.0, CLOUD_RISE_END.x, height),
        smoothstep(0.0, CLOUD_RISE_END.y, height),
        smoothstep(0.0, CLOUD_RISE_END.z, height),
    );
    let fall = vec3<f32>(
        smoothstep(CLOUD_FALL_START.x, 1.0, height),
        smoothstep(CLOUD_FALL_START.y, 1.0, height),
        smoothstep(CLOUD_FALL_START.z, 1.0, height),
    );
    return cloud_blend_types(rise * (vec3<f32>(1.0) - fall), cloud_type);
}

// Density before the detail noise (x) and height fraction in the layer (y): the cheap
// first look of a ray march.
fn cloud_coarse(
    u: CloudUniforms,
    shape: texture_3d<f32>,
    weather: texture_2d<f32>,
    repeat_linear: sampler,
    position: vec3<f32>,
) -> vec2<f32> {
    let w = textureSampleLevel(
        weather, repeat_linear, position.xz * u.weather_inverse_scale + u.weather_offset, 0.0,
    ).rg;
    let cover = saturate((w.x - (1.0 - u.cover)) / u.cover_softness);
    let cloud_type = saturate(u.cloud_type + (w.y - 0.5) * u.type_variation);
    let thickness = cloud_blend_types(u.thickness, cloud_type);
    let height = (position.y + u.altitude_offset - u.base_altitude) / thickness;
    if height <= 0.0 || height >= cover {
        return vec2<f32>(0.0);
    }
    let s = textureSampleLevel(
        shape, repeat_linear, position * u.shape_inverse_scale + u.shape_offset, 0.0,
    ).r;
    let envelope = cover * cloud_profile(height / cover, cloud_type);
    let eaten = (1.0 - s) * u.shape_erosion;
    return vec2<f32>(max((envelope - eaten) / (1.0 - eaten), 0.0), height);
}

// Density of the clouds, 0 to 1: the share of the full extinction.
fn cloud_density(
    u: CloudUniforms,
    shape: texture_3d<f32>,
    detail: texture_3d<f32>,
    weather: texture_2d<f32>,
    repeat_linear: sampler,
    position: vec3<f32>,
) -> f32 {
    let coarse = cloud_coarse(u, shape, weather, repeat_linear, position);
    if coarse.x <= 0.0 {
        return 0.0;
    }
    let d = textureSampleLevel(
        detail, repeat_linear, position * u.detail_inverse_scale + u.detail_offset, 0.0,
    ).r;
    let billows = mix(d, 1.0 - d, saturate(coarse.y / CLOUD_WISPS_HEIGHT));
    let erosion = billows * u.detail_erosion;
    return saturate((coarse.x - erosion) / (1.0 - erosion));
}
