//! The plain of the test flight (plan `m2-vol-essai.md`, step 5): flat ground covered
//! with fields and hedges, the atmosphere and the afternoon sun. The fields' texture is
//! made here at start, no image file: a tile of [`TILE_M`], the step of the floating
//! origin's grid, so that the ground does not move when the origin jumps. No relief,
//! trees or villages yet: the real ground of Flanders comes with `tools/terrain_bake`.

use bevy::asset::RenderAssetUsages;
use bevy::image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy::light::atmosphere::ScatteringMedium;
use bevy::light::{Atmosphere, CascadeShadowConfigBuilder, SunDisk, light_consts::lux};
use bevy::math::{Affine2, DVec3};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

use crate::flight::bridge::to_bevy;
use crate::flight::simulation::ORIGIN_GRID_M;

/// Side of the fields' texture tile, m.
pub const TILE_M: f32 = ORIGIN_GRID_M as f32;
/// Texels of the tile's side: about a metre each.
pub const TEXTURE_SIZE: usize = 2048;
/// Blocks of fields across the tile, hedged around: about 500 m each.
pub const BLOCKS: usize = 4;
/// Width of the strip fields inside a block, m: long narrow parcels, as on the maps of
/// Flanders (a first look, to be checked on the maps of 1917).
pub const STRIP_M: (f32, f32) = (25.0, 70.0);
/// Width of the hedges around the blocks, texels.
pub const HEDGE_TEXELS: f32 = 3.0;
/// Side of the ground, m: to the haze of the horizon from the test flight's height.
pub const GROUND_M: f32 = 200_000.0;
/// The afternoon sun, south-west, 40° high (plan `m2-vol-essai.md`).
pub const SUN_AZIMUTH_DEG: f64 = 225.0;
pub const SUN_ELEVATION_DEG: f64 = 40.0;

/// Colours of the fields, sRGB, with their share: pasture, meadow, wheat, stubble,
/// ploughed earth, beet. A first palette for the eye, not a survey of 1917 crops.
const FIELD_COLOURS: [([f32; 3], f32); 6] = [
    ([0.36, 0.43, 0.24], 0.35),
    ([0.42, 0.47, 0.27], 0.25),
    ([0.58, 0.55, 0.36], 0.10),
    ([0.53, 0.51, 0.37], 0.08),
    ([0.45, 0.39, 0.30], 0.12),
    ([0.31, 0.38, 0.22], 0.10),
];
/// A field is one to [`MAX_RUN`] strips of the same crop.
const MAX_RUN: f32 = 4.0;
const HEDGE_COLOUR: [f32; 3] = [0.17, 0.23, 0.11];

/// The plain: ground, atmosphere and sun.
pub fn spawn_plain(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    images: &mut Assets<Image>,
    media: &mut Assets<ScatteringMedium>,
) {
    let fields = images.add(fields_texture(TEXTURE_SIZE, BLOCKS, 1917));
    commands.spawn((
        Name::new("Plain"),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(GROUND_M, GROUND_M))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(fields),
            perceptual_roughness: 0.95,
            uv_transform: Affine2::from_scale(Vec2::splat(GROUND_M / TILE_M)),
            ..default()
        })),
    ));
    commands.spawn(Atmosphere::earth(
        media.add(ScatteringMedium::earth(256, 256)),
    ));
    let (az, el) = (SUN_AZIMUTH_DEG.to_radians(), SUN_ELEVATION_DEG.to_radians());
    let towards_sun = DVec3::new(el.cos() * az.cos(), el.cos() * az.sin(), -el.sin());
    commands.spawn((
        Name::new("Sun"),
        DirectionalLight {
            illuminance: lux::RAW_SUNLIGHT,
            shadow_maps_enabled: true,
            ..default()
        },
        SunDisk::EARTH,
        // The aircraft in the first cascade from the chase view, its shadow on the ground
        // within the last when it flies low.
        CascadeShadowConfigBuilder {
            num_cascades: 4,
            minimum_distance: 0.1,
            first_cascade_far_bound: 20.0,
            maximum_distance: 1500.0,
            ..default()
        }
        .build(),
        Transform::default().looking_to(-to_bevy(towards_sun).as_vec3(), Vec3::Y),
    ));
}

/// SplitMix64: a well-mixed hash, enough for fields.
fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn unit(h: u64) -> f32 {
    (h >> 40) as f32 / (1u64 << 24) as f32
}

/// A seamless tile of fields: a jittered grid of `blocks` × `blocks` block centres, each
/// texel in the block of the nearest (across the tile's edges too), hedges where two
/// blocks meet; inside a block, parallel strips of one width and direction, each its own
/// crop. Rows from the top, sRGB with alpha, repeating, with mipmaps to come
/// (`crate::render::mipmaps`).
/// Colour of the texel (`x`, `y`) of the tile (sRGB), any integer coordinates: the tile
/// repeats exactly every `size` texels.
pub fn field_texel(x: i64, y: i64, size: usize, blocks: usize, seed: u64) -> [f32; 3] {
    let n = size as i64;
    let cell = size as f32 / blocks as f32;
    let texels_per_m = size as f32 / TILE_M;
    let centre = |i: usize, j: usize| {
        let h = mix(seed ^ (i as u64) << 32 ^ j as u64);
        let (jx, jy) = (0.15 + 0.7 * unit(h), 0.15 + 0.7 * unit(mix(h)));
        (
            (i as f32 + jx) * cell,
            (j as f32 + jy) * cell,
            mix(h ^ 0xf1e1d),
        )
    };
    let colour_of = |h: u64| {
        let mut r = unit(h);
        for (c, share) in FIELD_COLOURS {
            if r < share {
                return c;
            }
            r -= share;
        }
        FIELD_COLOURS[0].0
    };
    let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
    let (ci, cj) = ((px / cell).floor() as isize, (py / cell).floor() as isize);
    let (mut d1, mut d2) = (f32::MAX, f32::MAX);
    let mut block = (0.0, 0.0, 0u64);
    for dj in -1..=1 {
        for di in -1..=1 {
            let (i, j) = (ci + di, cj + dj);
            let wrap = |v: isize| v.rem_euclid(blocks as isize) as usize;
            let (cx, cy, h) = centre(wrap(i), wrap(j));
            // The neighbour across an edge of the tile, at its place beside it.
            let cx = cx + (i - wrap(i) as isize) as f32 * cell;
            let cy = cy + (j - wrap(j) as isize) as f32 * cell;
            let d = (cx - px).hypot(cy - py);
            if d < d1 {
                (d2, d1, block) = (d1, d, (cx, cy, h));
            } else if d < d2 {
                d2 = d;
            }
        }
    }
    // Strips across the block, measured from its centre: the same on both sides
    // of the tile's edges.
    let (cx, cy, h) = block;
    let angle = std::f32::consts::PI * unit(mix(h ^ 1));
    let width = (STRIP_M.0 + (STRIP_M.1 - STRIP_M.0) * unit(mix(h ^ 2))) * texels_per_m;
    let across = ((px - cx) * angle.cos() + (py - cy) * angle.sin()) / width;
    let strip = across.floor();
    let run = 1.0 + (MAX_RUN * unit(mix(h ^ 3))).floor();
    let field = mix(h ^ ((strip / run).floor() as i64 as u64));
    let base = colour_of(field);
    // Each field a little lighter or darker; furrows along it, faint.
    let along = (px - cx) * -angle.sin() + (py - cy) * angle.cos();
    let shade = 1.0
        + 0.08 * (unit(mix(field ^ 7)) - 0.5)
        + 0.03 * (std::f32::consts::TAU * along / (1.5 * texels_per_m)).sin()
        + 0.03 * (unit(mix((y.rem_euclid(n) * n + x.rem_euclid(n)) as u64)) - 0.5);
    // A darker line between strips, a hedge where two blocks meet: half the
    // difference of the distances is the distance to the blocks' border.
    let edge = (across - strip).min(1.0 - (across - strip)) * width;
    let line = (1.0 - edge).clamp(0.0, 1.0) * 0.10;
    let hedge = (1.0 - ((0.5 * (d2 - d1)) / HEDGE_TEXELS)).clamp(0.0, 1.0);
    let mut colour = [0.0; 3];
    for (c, value) in colour.iter_mut().enumerate() {
        let field = base[c] * shade * (1.0 - line);
        *value = field * (1.0 - hedge) + HEDGE_COLOUR[c] * hedge;
    }
    colour
}

pub fn fields_texture(size: usize, blocks: usize, seed: u64) -> Image {
    let mut data = vec![255u8; size * size * 4];
    for y in 0..size {
        for x in 0..size {
            let colour = field_texel(x as i64, y as i64, size, blocks, seed);
            let texel = &mut data[(y * size + x) * 4..][..3];
            for c in 0..3 {
                texel[c] = (colour[c].clamp(0.0, 1.0) * 255.0).round() as u8;
            }
        }
    }
    let mut image = Image::new(
        Extent3d {
            width: size as u32,
            height: size as u32,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::default(),
    );
    image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Linear,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        ..default()
    });
    image
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_fields_tile_without_seams() {
        // The tile repeats exactly: a texel and its copies a tile away, either way.
        let (size, blocks) = (512, 2);
        for k in (0..size as i64).step_by(7) {
            for edge in [0, 1, size as i64 - 2, size as i64 - 1] {
                let here = field_texel(edge, k, size, blocks, 1917);
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let n = size as i64;
                    let there = field_texel(edge + dx * n, k + dy * n, size, blocks, 1917);
                    assert!(
                        here.iter().zip(there).all(|(a, b)| (a - b).abs() < 1e-4),
                        "({edge}, {k}) and a tile away: {here:?} {there:?}"
                    );
                }
            }
        }
        // And it is the same every time.
        assert_eq!(
            fields_texture(64, 2, 1917).data,
            fields_texture(64, 2, 1917).data
        );
    }
}
