// The ray march of the clouds (ADR 0009), shared by the pass seen from the camera
// (`render.wesl`) and the pass that draws the clouds into the environment map
// (`environment.wesl`): along a ray, the light the clouds send back towards its origin,
// their transmittance and their mean distance. Both bind the clouds as group 1 and the
// layout of `render_sky` as group 0, so that the functions of Bevy's atmosphere work here:
// the same sun, sky and transmittance as the rest of the image.
//
// Light of a sample: the model of `escadrille_weather::scattering`, whose tests check its
// energy against two-stream theory; the functions and constants below are its twins, to
// be changed together.
// - Sunlight, dimmed by the atmosphere (transmittance table), scattered once (two-lobe
//   Henyey-Greenstein phase, e^−τ towards the sun) and many times (a fast part near the
//   lit surface, e^−τ, and the slow fall of diffusion deep in the cloud, 1 / (1 + k τ)).
//   The optical depth towards the sun comes from a short march on the coarse density.
// - The sky above and the ground below, each from its half of the sphere, dimmed through
//   the cloud on its side: up to the top of the envelope, and down to the base of the
//   layer, at the density of the sample. The ground is the albedo of fields lit by the
//   sun, outside the clouds' shadows, and by the sky.
// Each step is integrated exactly over its length (Hillaire 2016), the last one cut where
// the ray ends. Steps grow with the distance, since the far clouds are small on screen,
// and stride over empty air: only the steps inside a cloud count against the budget, so
// that the march reaches the far ones.

import bevy_render::maths::PI;
import bevy_pbr::atmosphere::{
    bindings::{atmosphere, lights},
    functions::{
        sample_transmittance_lut, sample_sky_view_lut, direction_world_to_atmosphere,
        get_view_position, sample_density_lut, get_local_r, ABSORPTION_DENSITY,
        SCATTERING_DENSITY,
    },
};
import super::density::{CloudUniforms, cloud_coarse, cloud_density};

// `CloudPass` of the pass.
struct CloudPass {
    // Frame number: the jitter of the steps changes each frame, and the TAA averages it.
    frame: u32,
    // Steps of the march through the layer, at most, and towards the sun.
    steps: u32,
    light_steps: u32,
    // Divisor of the camera's resolution, on each side.
    divisor: u32,
    // Shortest step of the march, m.
    min_step: f32,
    // Longest distance marched, m.
    max_distance: f32,
    // Mean albedo of the fields, linear, seen below the horizon of the environment map,
    // and the share of their sunlight the clouds' mean shadow leaves them.
    ground_albedo: vec3<f32>,
    ground_shade: f32,
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
// Multiple scattering, ambient light, ground: `escadrille_weather::scattering`.
const SURFACE_LEVEL: f32 = 1.8;
const SURFACE_DEPTH: f32 = 1.0;
const DIFFUSE_LEVEL: f32 = 2.1;
const DIFFUSE_FALL: f32 = 0.055;
const AMBIENT_LEVEL: f32 = 2.3;
const GROUND_ALBEDO: f32 = 0.18;
// Length of the first step towards the sun, m, a few texels of the detail noise so that
// the buds shade one another; each next one is longer by LIGHT_GROWTH (6 steps: 2.9 km,
// the width of a tower crossed by a low sun; at 900 m, the dark side of a tower against a
// sun at 9° kept three times the light two-stream theory lets through).
// The first DETAILED_LIGHT_STEPS look at the full density, the others at the coarse one:
// three, 104 m, so that buds of a few tens of metres shade one another (2026-11-08: the
// sunlit edges of the contre-jour's crest brightened from 1.45 to 1.63 times the sky
// beside them; with two, 32 m, the light beyond saw only the smooth envelope).
const LIGHT_STEP: f32 = 8.0;
const LIGHT_GROWTH: f32 = 3.0;
const DETAILED_LIGHT_STEPS: u32 = 3u;
// Each step of the march is this share of the distance from the eye, between the
// shortest step and MAX_STEP.
const STEP_SHARE: f32 = 0.02;
const MAX_STEP: f32 = 500.0;
// Through empty air the march strides this many steps at once, and it looks at most
// MAX_LOOKS times along a ray, empty or not. On meeting a cloud after a stride it steps
// back and goes on at its plain step: otherwise it enters the clouds at distances
// quantised by the stride, which the jitter of the first step does not cover, and the
// shells of equal distance show as stripes. It strides only outside every envelope of
// cloud, where nothing can be missed, and the jitter spreads the first look over a whole
// stride: else each row of the screen shares the same grid of looks, and a top barely
// grazed shows on one row and not on the next.
const EMPTY_STRIDE: f32 = 3.0;
const MAX_LOOKS: u32 = 384u;
// Transmittance under which a ray stops: the rest cannot be seen.
const OPAQUE: f32 = 0.01;
// Offset of the second jitter from the first, the golden ratio's fraction: the place of
// the sample of the stretch before the first look.
const SECOND_JITTER: f32 = 0.618034;
// The march towards the sky from a sample (`optical_depth_to_sky`): first step, m, growth
// of each next one, steps (10, 30, 90, 270 m: 400 m), and how far it leans towards the
// eye's side, horizontal against vertical (45° when the eye looks across). Twins of
// `SKY_*` in `escadrille_weather::clouds`.
const SKY_STEP: f32 = 10.0;
const SKY_GROWTH: f32 = 3.0;
const SKY_STEPS: u32 = 4u;
const SKY_LEAN: f32 = 1.0;

fn henyey_greenstein(cos_theta: f32, g: f32) -> f32 {
    let g2 = g * g;
    return (1.0 - g2) / (4.0 * PI * pow(1.0 + g2 - 2.0 * g * cos_theta, 1.5));
}

fn phase(cos_theta: f32) -> f32 {
    let backward = henyey_greenstein(cos_theta, BACKWARD_G);
    let forward = henyey_greenstein(cos_theta, FORWARD_G);
    return backward * (1.0 - FORWARD_WEIGHT) + forward * FORWARD_WEIGHT;
}

fn multiple(depth: f32) -> f32 {
    let surface = SURFACE_LEVEL * exp(-depth / SURFACE_DEPTH);
    let diffusion = DIFFUSE_LEVEL / (1.0 + DIFFUSE_FALL * depth);
    return (surface + diffusion) / (4.0 * PI);
}

// `sun_scattering` of the reference, the phase already computed for the ray.
fn sun_scattering(depth_to_sun: f32, single_phase: f32) -> f32 {
    return single_phase * exp(-depth_to_sun) + multiple(depth_to_sun);
}

fn ambient_share(depth: f32) -> f32 {
    return multiple(depth) / multiple(0.0);
}

fn ambient_scattering(sky: vec3<f32>, ground: vec3<f32>, depth_up: f32, depth_down: f32) -> vec3<f32> {
    return 0.5 * AMBIENT_LEVEL * (sky * ambient_share(depth_up) + ground * ambient_share(depth_down));
}

// Interleaved gradient noise (Jimenez, "Next Generation Post Processing in Call of Duty:
// Advanced Warfare", 2014), shifted each frame: the start of each ray in its first step.
fn jitter(pixel: vec2<f32>, frame: u32) -> f32 {
    let p = pixel + 5.588238 * f32(frame % 64u);
    return fract(52.9829189 * fract(dot(p, vec2<f32>(0.06711056, 0.00583715))));
}

// Optical depth of the clouds from a point towards the sun: the full density on the first
// steps, so that the buds of the detail shade one another, the coarse one beyond.
fn optical_depth_to_sun(position: vec3<f32>, to_sun: vec3<f32>) -> f32 {
    var depth = 0.0;
    var travelled = 0.0;
    var step = LIGHT_STEP;
    for (var i = 0u; i < cloud_pass.light_steps; i++) {
        let at = position + to_sun * (travelled + 0.5 * step);
        var density = 0.0;
        if i < DETAILED_LIGHT_STEPS {
            density = cloud_density(
                clouds, shape_noise, detail_noise, weather_map, cloud_sampler, at,
            );
        } else {
            density = cloud_coarse(clouds, shape_noise, weather_map, cloud_sampler, at).x;
        }
        depth += density * step;
        travelled += step;
        step *= LIGHT_GROWTH;
    }
    return depth * clouds.extinction;
}

// Steps of the air's transmittance between the eye and the clouds.
const AIR_STEPS: u32 = 12u;

// Transmittance of the air over `t` metres of a ray from the radius `r` at the cosine `mu`
// of its zenith angle (atmosphere frame): the extinction of Bevy's atmosphere summed
// along the segment. Bevy's `sample_transmittance_lut_segment` divides two values of its
// table, both tiny near the planet's horizon, and switches formula where the ray meets
// the ground: clamped to 1, the ratio left a band of distant clouds unveiled, a yellow
// line across the horizon of `cumulus_midi` (art director's review).
fn air_transmittance(r: f32, mu: f32, t: f32) -> vec3<f32> {
    let dt = t / f32(AIR_STEPS);
    var depth = vec3<f32>(0.0);
    for (var i = 0u; i < AIR_STEPS; i++) {
        let r_i = get_local_r(r, mu, (f32(i) + 0.5) * dt);
        depth += (sample_density_lut(r_i, ABSORPTION_DENSITY)
            + sample_density_lut(r_i, SCATTERING_DENSITY)) * dt;
    }
    return exp(-depth);
}

// Optical depth of the clouds from a sample towards the sky: up, leaning towards the eye's
// side (`toward_eye`, horizontal, as long as the eye's ray is level), which is open for a
// flank the eye sees; the full density on the first step, the coarse one beyond. Under an
// overhang or at the foot of a tower the sky is far; on a top or an open flank, near.
// Each step is read at `offset` of its length, the frame's jitter, which the temporal pass
// averages over the step: read at their middles, the steps printed the cells of the noise
// on the shaded faces, round pale blots all of one size (art director's review).
// Taken until 2026-11-09 to the top of the envelope, or back along the eye's ray on a
// flank, which is nothing at the surface: every sample the eye saw took the whole sky,
// and the shaded faces of the contre-jour were even (art director's review).
fn optical_depth_to_sky(position: vec3<f32>, toward_eye: vec3<f32>, offset: f32) -> f32 {
    let direction = normalize(vec3<f32>(0.0, 1.0, 0.0) + SKY_LEAN * toward_eye);
    var depth = 0.0;
    var travelled = 0.0;
    var step = SKY_STEP;
    for (var i = 0u; i < SKY_STEPS; i++) {
        let at = position + direction * (travelled + offset * step);
        var density = 0.0;
        if i == 0u {
            density = cloud_density(
                clouds, shape_noise, detail_noise, weather_map, cloud_sampler, at,
            );
        } else {
            density = cloud_coarse(clouds, shape_noise, weather_map, cloud_sampler, at).x;
        }
        depth += density * step;
        travelled += step;
        step *= SKY_GROWTH;
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

// What a ray finds in the clouds: the light they send back along it (not exposed, before
// the aerial perspective), their transmittance, and their mean distance from its origin,
// weighted by what each step hides (m; the ray's end when it met none).
struct Marched {
    light: vec3<f32>,
    transmittance: f32,
    distance: f32,
}

// The march of a ray from `eye` along `direction` (unit), up to `far` (m). `first` places
// the first look within its stride, between 0 and 1: the jitter.
fn march(eye: vec3<f32>, direction: vec3<f32>, far_away: f32, first: f32) -> Marched {
    let clear = Marched(vec3<f32>(0.0), 1.0, far_away);
    var far = far_away;

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
    let r_view = length(get_view_position());
    let sky = sky_radiance(r_view);
    var to_sun = vec3<f32>(0.0, 1.0, 0.0);
    var sun = vec3<f32>(0.0);
    if lights.n_directional_lights > 0u {
        to_sun = lights.directional_lights[0].direction_to_light;
        sun = lights.directional_lights[0].color.rgb;
    }
    let ground_r = atmosphere.inner_radius;
    let sun_on_ground = sun * sample_transmittance_lut(ground_r, to_sun.y) * max(to_sun.y, 0.0);
    let ground = GROUND_ALBEDO / PI * (sun_on_ground * (1.0 - clouds.cover) + PI * sky);
    let cos_theta = dot(direction, to_sun);
    let single_phase = phase(cos_theta);
    // The eye's side, horizontal: the sky's march leans towards it; its steps are read at a
    // second jitter.
    let toward_eye = -vec3<f32>(direction.x, 0.0, direction.z);
    let sky_offset = fract(first + SECOND_JITTER);

    var transmittance = 1.0;
    var light = vec3<f32>(0.0);
    var weighted_distance = 0.0;
    var weights = 0.0;
    // The first look is shifted within a stride by the jitter, the others follow it. The
    // stretch it leaves before it, up to a stride, holds the cloud around an eye inside it
    // and the aircraft in front: one sample there, at a second jitter, stands for all of it.
    var t = near + clamp(near * STEP_SHARE, cloud_pass.min_step, MAX_STEP) * EMPTY_STRIDE * first;
    var gap = min(t, far) - near;
    var steps = 0u;
    var strided = false;
    var plain_steps = 0u;
    for (var look = 0u; look < MAX_LOOKS && steps < cloud_pass.steps; look++) {
        var at = t;
        var stretch = 0.0;
        var coarse: vec3<f32>;
        if gap > 0.0 {
            at = near + gap * fract(first + SECOND_JITTER);
            stretch = gap;
            gap = 0.0;
            coarse = cloud_coarse(clouds, shape_noise, weather_map, cloud_sampler, eye + direction * at);
            if coarse.x <= 0.0 {
                continue;
            }
        } else {
            let step = clamp(t * STEP_SHARE, cloud_pass.min_step, MAX_STEP);
            if at >= far {
                break;
            }
            coarse = cloud_coarse(clouds, shape_noise, weather_map, cloud_sampler, eye + direction * at);
            if coarse.x <= 0.0 {
                if coarse.z > 0.0 {
                    // Inside an envelope the shape may come back at the next look: plain
                    // steps, and no step back on meeting it.
                    strided = false;
                    t += step;
                } else if plain_steps > 0u {
                    plain_steps--;
                    t += step;
                } else {
                    t += step * EMPTY_STRIDE;
                    strided = true;
                }
                continue;
            }
            if strided {
                t -= step * (EMPTY_STRIDE - 1.0);
                strided = false;
                plain_steps = u32(EMPTY_STRIDE);
                continue;
            }
            t += step;
            stretch = min(step, far - at);
        }
        steps++;
        let position = eye + direction * at;
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
        // Cloud on each side: towards the sun; for the sky, by a short march up and towards
        // the eye's side, open on a flank the eye sees (third review: flanks lost the sky's
        // blue), or up to the top of the envelope at the density of the sample, the nearer;
        // down to the base of the layer, for the ground, at the density of the sample.
        let depth_to_sun = optical_depth_to_sun(position, to_sun);
        let depth_up = min(
            extinction * coarse.z,
            optical_depth_to_sky(position, toward_eye, sky_offset),
        );
        let depth_down = extinction * (position.y - bottom);
        let scattered = sun_through_air * sun_scattering(depth_to_sun, single_phase)
            + ambient_scattering(sky, ground, depth_up, depth_down);

        // Scattering coefficient = extinction (the albedo of cloud droplets is about 1).
        let source = extinction * scattered;
        let through_step = exp(-extinction * stretch);
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
    return Marched(light, transmittance, weighted_distance / weights);
}
