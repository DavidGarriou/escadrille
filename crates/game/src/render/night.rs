//! The night sky (plan `m3-heure-du-jour.md`, step 4): the moon with its phase, and the
//! stars, both drawn far away around the camera, behind the clouds; the atmosphere veils
//! them as it veils the ground (the sky is marched for each pixel, `camera`).
//!
//! - **Moon**: a sphere the size of its disc ([`MOON_ANGULAR_RADIUS_DEG`]), lit on the
//!   sun's side by its own shader (Bevy's sun disc has no phase). Its surface radiance
//!   gives the moon's illuminance by Allen's phase law (`world::sky`).
//! - **Stars**: [`STARS`] small squares on the celestial sphere, at random places from a
//!   fixed seed (`ESTIMÉ`: a sky of the right richness, not the real constellations;
//!   plan decision 3), their magnitudes spread as the sky's, N(< m) = 10^(0.5 m + 0.7):
//!   about 16 brighter than the first magnitude, 5 000 to the sixth. One rotation, from
//!   the sidereal time and the latitude, sets them in the sky. They show at night only.

use bevy::asset::RenderAssetUsages;
use bevy::light::{NotShadowCaster, NotShadowReceiver};
use bevy::math::{DMat3, DVec3};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

use crate::render::exposure::FollowsDaylight;
use crate::world::sky::{MAGNITUDE_ZERO_LUX, Sky, towards};

const MOON_SHADER: &str = "shaders/night/moon.wesl";
const STARS_SHADER: &str = "shaders/night/stars.wesl";

/// How far the moon and the stars are drawn, m: beyond the clouds of a scene.
pub const SKY_DISTANCE_M: f32 = 60_000.0;
/// The moon's angular radius, degrees (its mean, 15.5′).
pub const MOON_ANGULAR_RADIUS_DEG: f32 = 0.259;
/// Colour of the moon's surface, linear (`ESTIMÉ`: a warm grey).
const MOON_TINT: Vec3 = Vec3::new(1.0, 0.95, 0.88);
/// Number of stars, and the half-side of each square, degrees: about a pixel and a half in
/// 1440p at the chase camera's field.
pub const STARS: usize = 5_000;
const STAR_HALF_SIDE_DEG: f64 = 0.03;
/// The stars fade in as the sun sinks between these heights, degrees: gone in daylight,
/// all there at the end of nautical twilight.
const STARS_APPEAR_DEG: f64 = -4.0;
const STARS_FULL_DEG: f64 = -12.0;

/// The moon's material: the sun's direction and the radiance of its lit surface.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct MoonMaterial {
    /// Direction towards the sun, Bevy axes (xyz; w unused).
    #[uniform(0)]
    pub to_sun: Vec4,
    /// Radiance of the lit surface, cd/m² (rgb; a unused).
    #[uniform(1)]
    pub radiance: Vec4,
}

impl Material for MoonMaterial {
    fn fragment_shader() -> ShaderRef {
        MOON_SHADER.into()
    }

    fn enable_shadows() -> bool {
        false
    }
}

/// The stars' material: their radiance is in the vertex colours, scaled by `fade`.
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct StarsMaterial {
    /// x: how much of the stars shows, 0 by day, 1 at night.
    #[uniform(0)]
    pub fade: Vec4,
}

impl Material for StarsMaterial {
    fn fragment_shader() -> ShaderRef {
        STARS_SHADER.into()
    }

    fn enable_shadows() -> bool {
        false
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Moon;

#[derive(Component, Debug, Clone, Copy)]
pub struct Stars;

/// The radiance of the moon's lit surface, cd/m²: its illuminance spread over the lit part
/// of its disc.
pub fn moon_radiance(sky: &Sky) -> f32 {
    let Some(moon) = sky.moon else {
        return 0.0;
    };
    let disc = std::f64::consts::PI * (MOON_ANGULAR_RADIUS_DEG as f64).to_radians().powi(2);
    (sky.moonlight_lux() / (disc * moon.illuminated.max(0.01))) as f32
}

/// How much of the stars shows with the sun at `elevation_deg`.
pub fn stars_fade(elevation_deg: f64) -> f32 {
    ((STARS_APPEAR_DEG - elevation_deg) / (STARS_APPEAR_DEG - STARS_FULL_DEG)).clamp(0.0, 1.0)
        as f32
}

/// The rotation that sets the stars' sphere in the local sky: from equatorial coordinates
/// (x towards the vernal point, z towards the north pole) to Bevy axes (x east, y up,
/// z south), at the local sidereal angle and the latitude.
pub fn celestial_rotation(sidereal_deg: f64, latitude_deg: f64) -> Quat {
    let (s, c) = latitude_deg.to_radians().sin_cos();
    // Hour-angle frame (x towards the meridian, y towards the west, z the pole) to Bevy:
    // east = −y, up = x cos φ + z sin φ, south = x sin φ − z cos φ.
    let to_bevy = DMat3::from_cols(
        DVec3::new(0.0, c, s),
        DVec3::new(-1.0, 0.0, 0.0),
        DVec3::new(0.0, s, -c),
    );
    // Equatorial to hour angle: H = sidereal − right ascension, a turn and a mirror.
    let turn = DMat3::from_rotation_z(sidereal_deg.to_radians());
    let mirror = DMat3::from_diagonal(DVec3::new(1.0, -1.0, 1.0));
    Quat::from_mat3(&(to_bevy * turn * mirror).as_mat3())
}

/// The stars' mesh: a square per star on the unit sphere, in equatorial coordinates, its
/// radiance in the vertex colour.
pub fn stars_mesh(seed: u64) -> Mesh {
    let mut state = seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1;
    let mut next = move || {
        // xorshift64*, in [0, 1).
        state ^= state >> 12;
        state ^= state << 25;
        state ^= state >> 27;
        (state.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 11) as f64 / (1u64 << 53) as f64
    };
    let half = STAR_HALF_SIDE_DEG.to_radians();
    let solid_angle = (2.0 * half).powi(2);
    let mut positions = Vec::with_capacity(STARS * 4);
    let mut colours = Vec::with_capacity(STARS * 4);
    let mut indices = Vec::with_capacity(STARS * 6);
    for i in 0..STARS {
        // Evenly over the sphere.
        let z = 2.0 * next() - 1.0;
        let a = std::f64::consts::TAU * next();
        let r = (1.0 - z * z).sqrt();
        let centre = DVec3::new(r * a.cos(), r * a.sin(), z);
        // The i-th brightest of the sky: N(< m) = 10^(0.5 m + 0.7).
        let magnitude = (((i + 1) as f64).log10() - 0.7) / 0.5;
        let radiance = MAGNITUDE_ZERO_LUX * 10f64.powf(-0.4 * magnitude) / solid_angle;
        // From bluish to orange (`ESTIMÉ`).
        let warmth = next();
        let tint = Vec3::new(
            0.85 + 0.3 * warmth as f32,
            0.95,
            1.15 - 0.35 * warmth as f32,
        );
        let colour = (tint * radiance as f32).extend(1.0);
        let east = DVec3::Z.cross(centre).normalize_or(DVec3::X);
        let north = centre.cross(east);
        let base = positions.len() as u32;
        for (u, v) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
            let p = centre + (east * u + north * v) * half;
            positions.push(p.normalize().as_vec3().to_array());
            colours.push(colour.to_array());
        }
        indices.extend([base, base + 2, base + 1, base, base + 3, base + 2]);
    }
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colours)
    .with_inserted_indices(Indices::U32(indices))
}

/// The night sky's bodies already there.
type NightBodies = Or<(With<Moon>, With<Stars>)>;
/// Where the moon and the stars are, and whether they show.
type Placed = (&'static mut Transform, &'static mut Visibility);

/// The moon and the stars, once a scene with a clock has its sky.
pub fn spawn_night_sky(
    mut commands: Commands,
    sky: Option<Res<Sky>>,
    existing: Query<(), NightBodies>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut moons: ResMut<Assets<MoonMaterial>>,
    mut stars: ResMut<Assets<StarsMaterial>>,
) {
    let Some(sky) = sky else {
        return;
    };
    if sky.when.is_none() || !existing.is_empty() {
        return;
    }
    let radius = SKY_DISTANCE_M * MOON_ANGULAR_RADIUS_DEG.to_radians().tan();
    commands.spawn((
        Name::new("Moon"),
        Moon,
        Mesh3d(meshes.add(Sphere::new(radius).mesh().uv(48, 24))),
        MeshMaterial3d(moons.add(MoonMaterial {
            to_sun: towards(sky.sun).extend(0.0),
            radiance: (MOON_TINT * moon_radiance(&sky)).extend(0.0),
        })),
        Transform::default(),
        NotShadowCaster,
        NotShadowReceiver,
    ));
    commands.spawn((
        Name::new("Stars"),
        Stars,
        Mesh3d(meshes.add(stars_mesh(1917))),
        MeshMaterial3d(stars.add(StarsMaterial {
            fade: Vec4::new(stars_fade(sky.sun.elevation_deg), 0.0, 0.0, 0.0),
        })),
        Transform::default(),
        NotShadowCaster,
        NotShadowReceiver,
    ));
}

/// The moon and the stars around the camera, where the sky sets them.
pub fn place_night_sky(
    sky: Option<Res<Sky>>,
    cameras: Query<&GlobalTransform, With<FollowsDaylight>>,
    mut moons: Query<Placed, (With<Moon>, Without<Stars>)>,
    mut stars: Query<Placed, (With<Stars>, Without<Moon>)>,
) {
    let (Some(sky), Some(camera)) = (sky, cameras.iter().next()) else {
        return;
    };
    let eye = camera.translation();
    if let (Some(moon), Ok((mut transform, mut visibility))) = (sky.moon, moons.single_mut()) {
        transform.translation = eye + towards(moon.sky) * SKY_DISTANCE_M;
        visibility.set_if_neq(if moon.sky.elevation_deg > -1.0 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        });
    }
    if let (Some(sidereal), Ok((mut transform, mut visibility))) =
        (sky.sidereal_deg(), stars.single_mut())
    {
        *transform = Transform {
            translation: eye,
            rotation: celestial_rotation(sidereal, sky.place.latitude_deg),
            scale: Vec3::splat(SKY_DISTANCE_M),
        };
        visibility.set_if_neq(if stars_fade(sky.sun.elevation_deg) > 0.0 {
            Visibility::Inherited
        } else {
            Visibility::Hidden
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_pole_star_stands_at_the_latitude_due_north() {
        // The celestial pole, whatever the hour, stands due north at the latitude's height.
        for sidereal in [0.0, 90.0, 237.0] {
            let pole = celestial_rotation(sidereal, 50.85) * Vec3::Z;
            let height = pole.y.asin().to_degrees();
            assert!((height - 50.85).abs() < 1e-3, "{height}");
            // North is −z in Bevy.
            assert!(pole.z < 0.0 && pole.x.abs() < 1e-5, "{pole}");
        }
    }

    #[test]
    fn a_star_on_the_meridian_culminates_south() {
        // A star of the equator whose right ascension is the sidereal time stands due south,
        // at 90° minus the latitude.
        let sidereal = 120.0_f64;
        let ra = sidereal.to_radians();
        let star = Vec3::new(ra.cos() as f32, ra.sin() as f32, 0.0);
        let seen = celestial_rotation(sidereal, 50.85) * star;
        assert!(
            (seen.y.asin().to_degrees() - (90.0 - 50.85)).abs() < 1e-3,
            "{seen}"
        );
        assert!(seen.z > 0.0 && seen.x.abs() < 1e-4, "{seen}");
    }

    #[test]
    fn the_stars_rise_in_the_east() {
        // Six sidereal hours later, the same star stands in the west: it rose in the east.
        let ra = 120.0_f64.to_radians();
        let star = Vec3::new(ra.cos() as f32, ra.sin() as f32, 0.0);
        let earlier = celestial_rotation(120.0 - 90.0, 50.85) * star;
        let later = celestial_rotation(120.0 + 90.0, 50.85) * star;
        assert!(earlier.x > 0.9 && later.x < -0.9, "{earlier} {later}");
    }

    #[test]
    fn the_sky_has_its_bright_stars_and_its_faint_ones() {
        let mesh = stars_mesh(1917);
        assert_eq!(mesh.count_vertices(), STARS * 4);
        // Sirius-like at the top, sixth magnitude at the end: twelve magnitudes apart,
        // radiances about 10^5 apart.
        let first = (
            ((1.0f64).log10() - 0.7) / 0.5,
            ((STARS as f64).log10() - 0.7) / 0.5,
        );
        assert!(first.0 < -1.0 && (5.5..6.5).contains(&first.1), "{first:?}");
    }

    #[test]
    fn the_stars_come_out_with_the_dusk() {
        assert_eq!(stars_fade(10.0), 0.0);
        assert_eq!(stars_fade(-4.0), 0.0);
        assert_eq!(stars_fade(-12.0), 1.0);
        assert!((stars_fade(-8.0) - 0.5).abs() < 1e-6);
    }
}
