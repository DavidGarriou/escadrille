//! The meadow of the field (plan `m2-train-sol.md`, decision 2): grass with no hedge across
//! it, a little lighter than the plain, its long side east to west. It lies on the plain
//! rather than in its texture, whose tile repeats every 2 km and would draw a field every
//! 2 km. The size is an estimate: no source gives the size of a French squadron's field
//! in 1917 (`spad7.md` § 6.6.5), nor a canvas T by day, so it has none.
//!
//! Over it, a repeating tile of tufts: without them, the ground near the aircraft is a flat
//! colour, and nothing tells the speed of a roll or the height of a flare. Seen from the
//! cockpit, 2 m above the grass, a pixel covers 12 cm of ground at 15 m and 5 m at 100 m:
//! the tufts' strength lies in metres, not in centimetres, stretched east to west, along
//! the roll, where the foreshortening keeps them and they draw lines towards the horizon.
//!
//! The meadow is masked rather than blended: the atmosphere's haze is drawn before the
//! transparent passes (`bevy_pbr`, `render_sky`), and a blended field escapes it. The tufts
//! cannot be masked and stay blended: they fade out as the aircraft leaves the field.

use bevy::asset::RenderAssetUsages;
use bevy::image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy::math::{Affine2, DVec3};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use super::plain::{mix, unit};
use crate::flight::bridge::to_bevy;
use crate::flight::simulation::{Player, RenderOrigin};

/// East to west and north to south, m (E:, plan `m2-train-sol.md`, decision 2).
pub const MEADOW_M: Vec2 = Vec2::new(800.0, 300.0);
/// Height above the plain, m: enough to stay above it at the end of the depth range.
pub const LIFT_M: f32 = 0.03;
/// Texels of the texture per metre: about what the plain has.
const TEXELS_PER_M: f32 = 1.0;
/// Over how far the grass fades into the plain at its edges, m.
const EDGE_M: f32 = 12.0;
/// The grass, sRGB: between the plain's pasture and meadow, cropped short.
const GRASS: [f32; 3] = [0.50, 0.56, 0.32];
const SEED: u64 = 1917;
/// Side of the tile of tufts, m, and its texels: 2 cm each. Its noise has whole numbers of
/// cells across the tile, so that it repeats without a seam: east to west, north to
/// south, and its weight. Patches of 6 by 2 m, 1.8 by 0.6 m, 0.6 by 0.2 m. The largest
/// stay out of the tile, in the meadow's own texture: its period would show from above.
const TUFT_TILE_M: f32 = 40.0;
const TUFT_TEXELS: usize = 2048;
const TUFT_CELLS: [(f32, f32, f32); 3] = [(7.0, 21.0, 0.8), (22.0, 66.0, 0.6), (67.0, 201.0, 0.4)];
/// A tuft darker or lighter than the grass, sRGB, and its strongest opacity.
const TUFT_DARK: [f32; 3] = [0.20, 0.27, 0.11];
const TUFT_LIGHT: [f32; 3] = [0.56, 0.60, 0.34];
const TUFT_ALPHA: f32 = 0.55;
/// Noise at which a tuft starts to show and is at its full opacity: the patches have
/// edges, rather than melting into the grass.
const TUFT_EDGES: (f32, f32) = (0.15, 0.30);
/// Shift of the noise: three dark tufts for two light ones.
const TUFT_DARKER: f32 = 0.04;
/// Distance from the aircraft to the meadow over which the tufts fade out, m.
const TUFT_FADE_M: (f32, f32) = (300.0, 600.0);
/// The tufts stop this far inside the meadow, m, where the grass fades out.
const TUFT_MARGIN_M: f32 = 20.0;
/// Height of the tufts above the meadow, m.
const TUFT_LIFT_M: f32 = 0.01;

/// The meadow and its tufts, on the render origin's grid, at their height.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct Meadow {
    lift: f32,
}

/// Smooth noise of `cells` wide and high, in cells: the hash of a grid, blended between
/// its corners. The grid wraps after `period` cells each way, a tile's worth, or never
/// with `i64::MAX`.
fn blotch_wrapped(gx: f32, gy: f32, period: (i64, i64), seed: u64) -> f32 {
    let (ix, iy) = (gx.floor(), gy.floor());
    let (fx, fy) = (gx - ix, gy - iy);
    let (sx, sy) = (fx * fx * (3.0 - 2.0 * fx), fy * fy * (3.0 - 2.0 * fy));
    let corner = |dx: f32, dy: f32| {
        let wrapped = |v: f32, p: i64| (v as i64).rem_euclid(p) as u64;
        let (i, j) = (wrapped(ix + dx, period.0), wrapped(iy + dy, period.1));
        let hash = seed ^ i.wrapping_mul(73_856_093) ^ j.wrapping_mul(19_349_663);
        unit(mix(hash))
    };
    let top = corner(0.0, 0.0) + (corner(1.0, 0.0) - corner(0.0, 0.0)) * sx;
    let bottom = corner(0.0, 1.0) + (corner(1.0, 1.0) - corner(0.0, 1.0)) * sx;
    top + (bottom - top) * sy - 0.5
}

fn blotch(x: f32, y: f32, cell: f32, seed: u64) -> f32 {
    blotch_wrapped(x / cell, y / cell, (i64::MAX, i64::MAX), seed)
}

/// One texel of the meadow: its colour (sRGB) and its opacity.
fn texel(x: usize, y: usize, width: usize, height: usize) -> ([f32; 3], f32) {
    let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
    let shade = 1.0
        + 0.16 * blotch(px, py, 90.0 * TEXELS_PER_M, SEED)
        + 0.20 * blotch(px, py, 18.0 * TEXELS_PER_M, SEED ^ 1)
        + 0.18 * blotch(px, py, 6.0 * TEXELS_PER_M, SEED ^ 3)
        + 0.04 * (unit(mix((y * width + x) as u64)) - 0.5);
    let colour = GRASS.map(|c| c * shade);
    // The distance to the edge, a little uneven: nobody mowed along a ruler.
    let edge = px.min(py).min(width as f32 - px).min(height as f32 - py) / TEXELS_PER_M;
    let uneven = edge - 8.0 * (blotch(px, py, 40.0 * TEXELS_PER_M, SEED ^ 2) + 0.5);
    (colour, (uneven / EDGE_M).clamp(0.0, 1.0))
}

/// One texel of the tufts: their colour (sRGB) and opacity, repeating every tile.
fn tuft_texel(x: usize, y: usize) -> ([f32; 3], f32) {
    let (u, v) = (x as f32 + 0.5, y as f32 + 0.5);
    let size = TUFT_TEXELS as f32;
    let value: f32 = TUFT_CELLS
        .iter()
        .enumerate()
        .map(|(i, (wide, high, weight))| {
            let period = (*wide as i64, *high as i64);
            let seed = SEED ^ (i as u64 + 16);
            weight * blotch_wrapped(u / size * wide, v / size * high, period, seed)
        })
        .sum::<f32>()
        - TUFT_DARKER;
    let colour = if value < 0.0 { TUFT_DARK } else { TUFT_LIGHT };
    let (from, to) = TUFT_EDGES;
    let edge = ((value.abs() - from) / (to - from)).clamp(0.0, 1.0);
    (colour, edge * edge * (3.0 - 2.0 * edge) * TUFT_ALPHA)
}

/// An sRGB image of `texel`, its sampler repeating or not.
fn image_of(
    width: usize,
    height: usize,
    repeat: bool,
    texel: impl Fn(usize, usize) -> ([f32; 3], f32),
) -> Image {
    let mut data = vec![255u8; width * height * 4];
    for y in 0..height {
        for x in 0..width {
            let (colour, alpha) = texel(x, y);
            let out = &mut data[(y * width + x) * 4..][..4];
            for c in 0..3 {
                out[c] = (colour[c].clamp(0.0, 1.0) * 255.0).round() as u8;
            }
            out[3] = (alpha * 255.0).round() as u8;
        }
    }
    let mut image = Image::new(
        Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    let address = if repeat {
        ImageAddressMode::Repeat
    } else {
        ImageAddressMode::ClampToEdge
    };
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: address,
        address_mode_v: address,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        ..default()
    });
    image
}

pub fn spawn_meadow(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    images: &mut Assets<Image>,
) {
    let (width, height) = (
        (MEADOW_M.x * TEXELS_PER_M) as usize,
        (MEADOW_M.y * TEXELS_PER_M) as usize,
    );
    let grass = image_of(width, height, false, |x, y| texel(x, y, width, height));
    commands.spawn((
        Name::new("Meadow"),
        Meadow { lift: LIFT_M },
        Mesh3d(meshes.add(Plane3d::default().mesh().size(MEADOW_M.x, MEADOW_M.y))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(images.add(grass)),
            perceptual_roughness: 0.95,
            alpha_mode: AlphaMode::Mask(0.5),
            ..default()
        })),
        Transform::from_translation(Vec3::Y * LIFT_M),
    ));
    let side = TUFT_TEXELS;
    let tufts = MEADOW_M - Vec2::splat(2.0 * TUFT_MARGIN_M);
    let lift = LIFT_M + TUFT_LIFT_M;
    commands.spawn((
        Name::new("Tufts"),
        Meadow { lift },
        Mesh3d(meshes.add(Plane3d::default().mesh().size(tufts.x, tufts.y))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(images.add(image_of(side, side, true, tuft_texel))),
            perceptual_roughness: 0.95,
            alpha_mode: AlphaMode::Blend,
            // Drawn after the meadow, which shares its place in the depth sort.
            depth_bias: 1.0,
            uv_transform: Affine2::from_scale(tufts / TUFT_TILE_M),
            ..default()
        })),
        Transform::from_translation(Vec3::Y * lift),
    ));
}

/// The tufts fade as the aircraft leaves the meadow: they are blended, and would stand
/// out of the haze.
pub fn fade_tufts(
    aircraft: Query<&Transform, With<Player>>,
    meadow: Query<(&Meadow, &Transform, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Ok(aircraft) = aircraft.single() else {
        return;
    };
    for (part, transform, material) in &meadow {
        if part.lift == LIFT_M {
            continue;
        }
        let off = (aircraft.translation - transform.translation).abs();
        let half = MEADOW_M / 2.0;
        let beside = Vec2::new((off.x - half.x).max(0.0), (off.z - half.y).max(0.0));
        let distance = beside.length().hypot(off.y);
        let (near, far) = TUFT_FADE_M;
        let fade = 1.0 - ((distance - near) / (far - near)).clamp(0.0, 1.0);
        let Some(current) = materials.get(&material.0).map(|m| m.base_color.alpha()) else {
            continue;
        };
        if (current - fade).abs() > 0.01
            && let Some(mut m) = materials.get_mut(&material.0)
        {
            m.base_color = m.base_color.with_alpha(fade);
        }
    }
}

/// The meadow where the map has it: over the origin of the map, whatever the render
/// origin.
pub fn place_meadow(origin: Res<RenderOrigin>, mut meadow: Query<(&Meadow, &mut Transform)>) {
    for (part, mut transform) in &mut meadow {
        transform.translation =
            (to_bevy(DVec3::ZERO) - to_bevy(origin.0)).as_vec3() + Vec3::Y * part.lift;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_grass_fades_into_the_plain_at_its_edges() {
        let (w, h) = (MEADOW_M.x as usize, MEADOW_M.y as usize);
        assert_eq!(texel(0, h / 2, w, h).1, 0.0);
        assert_eq!(texel(w - 1, h / 2, w, h).1, 0.0);
        assert_eq!(texel(w / 2, h / 2, w, h).1, 1.0);
    }

    #[test]
    fn the_tufts_tile_without_seams() {
        // The noise wraps exactly: a texel beyond the tile's edge is the one at its start.
        for (wide, high, _) in TUFT_CELLS {
            let period = (wide as i64, high as i64);
            for k in [0.25, 3.5, 5.75] {
                let at = |x: f32, y: f32| blotch_wrapped(x, y, period, SEED);
                assert!((at(0.5, k) - at(wide + 0.5, k)).abs() < 1e-4, "{wide}");
                assert!((at(k, 0.5) - at(k, high + 0.5)).abs() < 1e-4, "{high}");
            }
        }
    }

    #[test]
    fn the_meadow_is_the_same_every_time() {
        let grass = || image_of(64, 32, false, |x, y| texel(x, y, 64, 32)).data;
        assert_eq!(grass(), grass());
    }

    #[test]
    fn the_start_on_the_field_stands_on_the_meadow() {
        use std::path::Path;

        use escadrille_data::scenario::load_scenario;

        let s = load_scenario(Path::new(crate::ASSETS_DIR), crate::flight::ON_THE_FIELD).unwrap();
        let [north, east] = s.position_m;
        assert!(north.abs() < f64::from(MEADOW_M.y) / 2.0);
        assert!(east.abs() < f64::from(MEADOW_M.x) / 2.0);
        // And it has the length of the take-off and of a stop ahead of it.
        assert!(f64::from(MEADOW_M.x) / 2.0 - east > 400.0, "{east}");
    }
}
