// The clouds in the environment map (ADR 0009, complement of 2026-11-08). Bevy's
// atmosphere draws the sky into the cube of the camera's `GeneratedEnvironmentMapLight`
// (`bevy_pbr/src/atmosphere/environment.wesl`), then filters it into the diffuse and
// specular light of every lit surface. This pass, after it, draws the sky again and the
// clouds over it: the aircraft then takes the grey of a cloud base from above, the ground
// from below, the white of a sea of clouds under it, and the clouds in its reflections.
//
// Bevy filters the cube before the frame's passes, so the clouds drawn here light the next
// frame. Each texel marches one ray from the eye, without jitter from frame to frame (a
// fixed noise per texel instead of bands), at the step budget of the environment.
//
// Aerial perspective: the transmittance of the air up to the clouds, and as haze the sky's
// own radiance in that direction times the light the air takes away. It stands in for
// Bevy's aerial-view table, which is laid out along the camera's screen.
//
// The ground: below the horizon, the atmosphere's table holds its planet, of the grey
// albedo of the whole Earth (which its multiple scattering needs: the fields' green turned
// the haze olive), under the full sun (`raymarch_atmosphere`, `functions.wesl`). That
// planet is taken out again, and the fields put in: their albedo, under the sun dimmed by
// the clouds' mean shadow and under the sky. This is the light the undersides of the
// aircraft take from below.

import bevy_render::maths::PI;
import bevy_pbr::atmosphere::{
    bindings::{atmosphere, lights, view},
    functions::{
        sample_sky_view_lut, sample_transmittance_lut, direction_world_to_atmosphere,
        get_view_position,
    },
    bruneton_functions::{
        sample_transmittance_lut_segment, ray_intersects_ground,
        distance_to_bottom_atmosphere_boundary,
    },
};
import bevy_pbr::render::utils::sample_cube_dir;
import super::march::{air_transmittance, cloud_pass, jitter, march, sky_radiance};

// The atmosphere's view along `direction` (in its own frame, from the eye at
// `view_position`), with the fields in place of its planet's grey ground.
fn sky_over_fields(view_position: vec3<f32>, direction: vec3<f32>) -> vec3<f32> {
    let r = length(view_position);
    let mu = dot(direction, view_position / r);
    let sky = sample_sky_view_lut(r, direction);
    if !ray_intersects_ground(r, mu) || lights.n_directional_lights == 0u {
        return sky;
    }
    let t = distance_to_bottom_atmosphere_boundary(r, mu);
    let through = sample_transmittance_lut_segment(r, mu, t);
    let normal = normalize(view_position + direction * t);
    let sun = lights.directional_lights[0];
    let mu_sun = dot(sun.direction_to_light, normal);
    let sunlight = sample_transmittance_lut(atmosphere.inner_radius, mu_sun) * max(mu_sun, 0.0)
        * sun.color.rgb;
    let planet = through * atmosphere.ground_albedo / PI * sunlight;
    let fields = through * cloud_pass.ground_albedo / PI
        * (sunlight * cloud_pass.ground_shade + PI * sky_radiance(r));
    return max(sky - planet, vec3<f32>(0.0)) + fields;
}

// The cube, as Bevy's atmosphere writes it.
@group(0) @binding(13) var output: texture_storage_2d_array<rgba16float, write>;

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    let face = id.z;
    if id.x >= size.x || id.y >= size.y || face >= 6u {
        return;
    }
    let uv = (vec2<f32>(id.xy) + 0.5) / vec2<f32>(size);
    // Cubemaps are left-handed: as Bevy's atmosphere does.
    var direction = sample_cube_dir(uv, face);
    direction.z = -direction.z;

    let view_position = get_view_position();
    let r_view = length(view_position);
    let sky = sky_over_fields(view_position, direction_world_to_atmosphere(direction));
    let noise = jitter(vec2<f32>(id.xy) + vec2<f32>(37.0 * f32(face), 0.0), 0u);
    let marched = march(view.world_position, direction, cloud_pass.max_distance, noise);

    var color = sky;
    if marched.transmittance < 1.0 {
        let mu_view = dot(direction, view_position / r_view);
        let air = air_transmittance(r_view, mu_view, marched.distance);
        let haze = sky * (1.0 - air);
        color = sky * marched.transmittance + marched.light * air
            + haze * (1.0 - marched.transmittance);
    }
    textureStore(output, vec2<i32>(id.xy), i32(face), vec4<f32>(color, 1.0));
}
