// The ray march of the clouds (ADR 0009): for each pixel, the light the clouds send
// towards the eye and their transmittance, written into the cloud image, which a
// transparent material then blends over the sky and the opaque objects:
// colour = light + image × transmittance. Used by `crates/game/src/render/clouds/pass.rs`.
//
// Group 0 is the layout of `render_sky`, rebuilt by the pass, so that the functions of
// Bevy's atmosphere work here: the same sun, sky and aerial perspective as the rest of the
// image. Group 1 holds the clouds.
//
// Light of a sample:
// - single scattering of the sun (Schneider 2015, Hillaire 2016): sunlight dimmed by the
//   atmosphere (transmittance table) and by the clouds towards the sun (a short march on
//   the coarse density), with a two-lobe Henyey-Greenstein phase;
// - multiple scattering, by the diffusion approximation: in a thick cloud the sunlight
//   that goes on through it falls as 1 / (1 + ¾ (1 − g) τ), not as e^−τ (two-stream
//   theory, Bohren, "Multiple scattering of light and some of its observable
//   consequences", Am. J. Phys. 55, 1987). Without it, the base of a cumulus only gets the
//   light of the sky and takes its colour: blue, flat, as if seen through;
// - the sky above the cloud and the ground below it, the latter from the ground's albedo
//   lit by the sun, weighted by the height in the layer. The sky's light goes through the
//   cloud like the sun's, by diffusion: the optical depth towards the sun stands for the
//   depth towards the open sky above.
// Each step is integrated exactly over its length (Hillaire 2016). Steps grow with the
// distance, since the far clouds are small on screen, and stride over empty air: only the
// steps inside a cloud count against the budget, so that the march reaches the far ones.

import bevy_render::maths::PI;
import bevy_pbr::atmosphere::{
    bindings::{atmosphere, view, lights},
    functions::{
        sample_transmittance_lut, sample_sky_view_lut, sample_aerial_view_lut,
        uv_to_ray_direction, uv_to_ndc, direction_world_to_atmosphere, get_view_position,
    },
    bruneton_functions::sample_transmittance_lut_segment,
};
import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput;
import super::density::{CloudUniforms, cloud_coarse, cloud_density};

// The depth of the main pass, binding 13 of `render_sky`'s layout.
@group(0) @binding(13) var depth_texture: texture_depth_2d;

// `CloudPass` of the pass.
struct CloudPass {
    // Frame number: the jitter of the steps changes each frame, and the TAA averages it.
    frame: u32,
    // Steps of the march through the layer, at most, and towards the sun.
    steps: u32,
    light_steps: u32,
    // Shortest step of the march, m.
    min_step: f32,
    // Longest distance marched, m.
    max_distance: f32,
}

@group(1) @binding(0) var<uniform> clouds: CloudUniforms;
@group(1) @binding(1) var<uniform> cloud_pass: CloudPass;
@group(1) @binding(2) var shape_noise: texture_3d<f32>;
@group(1) @binding(3) var detail_noise: texture_3d<f32>;
@group(1) @binding(4) var weather_map: texture_2d<f32>;
@group(1) @binding(5) var cloud_sampler: sampler;

// Phase function: forward lobe, backward lobe, and the weight of the forward one.
const FORWARD_G: f32 = 0.8;
const BACKWARD_G: f32 = -0.3;
const FORWARD_WEIGHT: f32 = 0.7;
// Asymmetry of the droplets' phase function, for the diffusion (Mie, water droplets of
// about 10 µm in visible light: 0.85).
const DROPLET_G: f32 = 0.85;
// Length of the first step towards the sun, m; each next one is longer by LIGHT_GROWTH.
const LIGHT_STEP: f32 = 40.0;
const LIGHT_GROWTH: f32 = 1.6;
// Each step of the march is this share of the distance from the eye, between the
// shortest step and MAX_STEP.
const STEP_SHARE: f32 = 0.02;
const MAX_STEP: f32 = 500.0;
// Through empty air the march strides this many steps at once, and it looks at most
// MAX_LOOKS times along a ray, empty or not.
const EMPTY_STRIDE: f32 = 3.0;
const MAX_LOOKS: u32 = 384u;
// Transmittance under which a ray stops: the rest cannot be seen.
const OPAQUE: f32 = 0.01;

fn henyey_greenstein(cos_theta: f32, g: f32) -> f32 {
    let g2 = g * g;
    return (1.0 - g2) / (4.0 * PI * pow(1.0 + g2 - 2.0 * g * cos_theta, 1.5));
}

fn phase(cos_theta: f32) -> f32 {
    return mix(
        henyey_greenstein(cos_theta, BACKWARD_G),
        henyey_greenstein(cos_theta, FORWARD_G),
        FORWARD_WEIGHT,
    );
}

// Interleaved gradient noise (Jimenez, "Next Generation Post Processing in Call of Duty:
// Advanced Warfare", 2014), shifted each frame: the start of each ray in its first step.
fn jitter(pixel: vec2<f32>, frame: u32) -> f32 {
    let p = pixel + 5.588238 * f32(frame % 64u);
    return fract(52.9829189 * fract(dot(p, vec2<f32>(0.06711056, 0.00583715))));
}

// Optical depth of the clouds from a point towards the sun, on the coarse density.
fn optical_depth_to_sun(position: vec3<f32>, to_sun: vec3<f32>) -> f32 {
    var depth = 0.0;
    var travelled = 0.0;
    var step = LIGHT_STEP;
    for (var i = 0u; i < cloud_pass.light_steps; i++) {
        let at = position + to_sun * (travelled + 0.5 * step);
        depth += cloud_coarse(clouds, shape_noise, weather_map, cloud_sampler, at).x * step;
        travelled += step;
        step *= LIGHT_GROWTH;
    }
    return depth * clouds.extinction;
}

// Radiance of the sky over a cloud, averaged over its upper half: the zenith and four
// directions 30° above the horizon, from the sky-view table at the camera's height.
fn sky_radiance(r: f32) -> vec3<f32> {
    let s = 0.5;
    let c = 0.866;
    let directions = array<vec3<f32>, 5>(
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(c, s, 0.0),
        vec3<f32>(-c, s, 0.0),
        vec3<f32>(0.0, s, c),
        vec3<f32>(0.0, s, -c),
    );
    var sum = vec3<f32>(0.0);
    for (var i = 0u; i < 5u; i++) {
        sum += sample_sky_view_lut(r, direction_world_to_atmosphere(directions[i]));
    }
    return sum / 5.0;
}

@fragment
fn fragment(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let clear = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    let direction = uv_to_ray_direction(in.uv);
    let eye = view.world_position;

    // How far the ray goes: to the first opaque object, or to the longest distance.
    var far = cloud_pass.max_distance;
    let depth = textureLoad(depth_texture, vec2<i32>(in.position.xy), 0);
    if depth > 0.0 {
        let p = view.view_from_clip * vec4<f32>(uv_to_ndc(in.uv), depth, 1.0);
        far = min(far, length(p.xyz / p.w));
    }

    // Where it crosses the layer, between its base and its highest top.
    let bottom = clouds.base_altitude - clouds.altitude_offset;
    let top = bottom + max(clouds.thickness.x, max(clouds.thickness.y, clouds.thickness.z));
    var near = 0.0;
    if abs(direction.y) < 1e-6 {
        if eye.y < bottom || eye.y > top {
            return clear;
        }
    } else {
        let to_bottom = (bottom - eye.y) / direction.y;
        let to_top = (top - eye.y) / direction.y;
        near = max(near, min(to_bottom, to_top));
        far = min(far, max(to_bottom, to_top));
    }
    if far <= near {
        return clear;
    }

    // The sun (the first directional light), the sky, and the ground under the clouds.
    let view_position = get_view_position();
    let r_view = length(view_position);
    let sky = sky_radiance(r_view);
    var to_sun = vec3<f32>(0.0, 1.0, 0.0);
    var sun = vec3<f32>(0.0);
    if lights.n_directional_lights > 0u {
        to_sun = lights.directional_lights[0].direction_to_light;
        sun = lights.directional_lights[0].color.rgb;
    }
    let ground_r = atmosphere.inner_radius;
    let sun_on_ground = sun * sample_transmittance_lut(ground_r, to_sun.y) * max(to_sun.y, 0.0);
    let ground = atmosphere.ground_albedo / PI * (sun_on_ground + PI * sky);
    let cos_theta = dot(direction, to_sun);
    let single_phase = phase(cos_theta);

    var transmittance = 1.0;
    var light = vec3<f32>(0.0);
    var weighted_distance = 0.0;
    var weights = 0.0;
    // The first look is shifted within its step by the jitter, the others follow it.
    var t = near + clamp(near * STEP_SHARE, cloud_pass.min_step, MAX_STEP)
        * jitter(in.position.xy, cloud_pass.frame);
    var steps = 0u;
    for (var look = 0u; look < MAX_LOOKS && steps < cloud_pass.steps; look++) {
        let step = clamp(t * STEP_SHARE, cloud_pass.min_step, MAX_STEP);
        let at = t;
        if at > far {
            break;
        }
        let position = eye + direction * at;
        if cloud_coarse(clouds, shape_noise, weather_map, cloud_sampler, position).x <= 0.0 {
            t += step * EMPTY_STRIDE;
            continue;
        }
        t += step;
        steps++;
        let density = cloud_density(
            clouds, shape_noise, detail_noise, weather_map, cloud_sampler, position,
        );
        if density <= 0.0 {
            continue;
        }
        let extinction = density * clouds.extinction;

        // Sunlight reaching the sample, through the atmosphere and the clouds.
        let atmosphere_position = (atmosphere.world_to_atmosphere * vec4<f32>(position, 1.0)).xyz;
        let r = length(atmosphere_position);
        let up = atmosphere_position / r;
        let sun_through_air = sun * sample_transmittance_lut(r, dot(to_sun, up));
        let depth_to_sun = optical_depth_to_sun(position, to_sun);
        let single = single_phase * exp(-depth_to_sun);
        let diffused = 1.0 / (1.0 + 0.75 * (1.0 - DROPLET_G) * depth_to_sun);
        let multiple = diffused / (4.0 * PI);

        // Sky above, through the cloud; ground below.
        let height = saturate((position.y - bottom) / (top - bottom));
        let ambient = mix(ground, sky * diffused, height);

        // Scattering coefficient = extinction (the albedo of cloud droplets is about 1).
        let source = extinction * (sun_through_air * (single + multiple) + ambient);
        let through_step = exp(-extinction * step);
        light += transmittance * (source - source * through_step) / extinction;
        let absorbed = transmittance * (1.0 - through_step);
        weighted_distance += absorbed * at;
        weights += absorbed;
        transmittance *= through_step;
        if transmittance < OPAQUE {
            transmittance = 0.0;
            break;
        }
    }
    if weights <= 0.0 {
        return clear;
    }

    // Aerial perspective between the eye and the clouds, at their mean distance.
    let distance = weighted_distance / weights;
    let mu_view = dot(direction, view_position / r_view);
    let air = sample_transmittance_lut_segment(r_view, mu_view, distance);
    let haze = sample_aerial_view_lut(in.uv, distance);
    let color = light * air + haze * (1.0 - transmittance);
    return vec4<f32>(color * view.exposure, transmittance);
}
