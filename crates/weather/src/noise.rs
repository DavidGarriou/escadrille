//! Deterministic, tileable noises for the cloud textures: gradient noise (Perlin) and
//! cellular noise (Worley) on an integer lattice that wraps with a period, so that the
//! textures they fill repeat without a seam. The lattice is drawn from an integer hash
//! of the cell and a seed: the same seed gives the same textures on every machine.
//!
//! Positions are in lattice cells: the cell (i, j, k) is the cell (i mod period, …).

use glam::{IVec3, Vec3};

/// PCG3D hash of three integers (Jarzynski and Olano, "Hash Functions for GPU
/// Rendering", JCGT 9(3), 2020): good mixing of all three inputs, integers only.
pub fn pcg3d(x: u32, y: u32, z: u32) -> [u32; 3] {
    let mut x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
    let mut y = y.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
    let mut z = z.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
    x = x.wrapping_add(y.wrapping_mul(z));
    y = y.wrapping_add(z.wrapping_mul(x));
    z = z.wrapping_add(x.wrapping_mul(y));
    x ^= x >> 16;
    y ^= y >> 16;
    z ^= z >> 16;
    x = x.wrapping_add(y.wrapping_mul(z));
    y = y.wrapping_add(z.wrapping_mul(x));
    z = z.wrapping_add(x.wrapping_mul(y));
    [x, y, z]
}

/// A hash as a number in [0, 1): its top 24 bits, exact in `f32`.
fn unit(hash: u32) -> f32 {
    (hash >> 8) as f32 * (1.0 / 16_777_216.0)
}

/// Hash of a lattice cell under a seed.
fn cell_hash(cell: IVec3, seed: u32) -> [u32; 3] {
    pcg3d(
        (cell.x as u32).wrapping_add(seed.wrapping_mul(0x9E37_79B9)),
        cell.y as u32,
        cell.z as u32,
    )
}

/// The cells of a lattice of `period`³ cells, x fastest, then y, then z.
fn cells(period: i32) -> impl Iterator<Item = IVec3> {
    (0..period).flat_map(move |k| {
        (0..period).flat_map(move |j| (0..period).map(move |i| IVec3::new(i, j, k)))
    })
}

/// Index in a lattice of `period`³ cells of a cell anywhere: the lattice wraps.
fn wrapped_index(cell: IVec3, period: i32) -> usize {
    let c = cell.rem_euclid(IVec3::splat(period));
    (c.x + period * (c.y + period * c.z)) as usize
}

/// The twelve edge directions of a cube, gradients of Perlin's improved noise (Perlin,
/// "Improving Noise", SIGGRAPH 2002).
const GRADIENTS: [Vec3; 12] = [
    Vec3::new(1.0, 1.0, 0.0),
    Vec3::new(-1.0, 1.0, 0.0),
    Vec3::new(1.0, -1.0, 0.0),
    Vec3::new(-1.0, -1.0, 0.0),
    Vec3::new(1.0, 0.0, 1.0),
    Vec3::new(-1.0, 0.0, 1.0),
    Vec3::new(1.0, 0.0, -1.0),
    Vec3::new(-1.0, 0.0, -1.0),
    Vec3::new(0.0, 1.0, 1.0),
    Vec3::new(0.0, -1.0, 1.0),
    Vec3::new(0.0, 1.0, -1.0),
    Vec3::new(0.0, -1.0, -1.0),
];

/// Perlin's gradient noise on a lattice that wraps: zero at the lattice points, about
/// −1 to 1 in between, smooth (quintic fade).
pub struct Perlin {
    period: i32,
    gradients: Vec<Vec3>,
}

impl Perlin {
    pub fn new(period: u32, seed: u32) -> Self {
        let period = period as i32;
        let gradients = cells(period)
            .map(|cell| GRADIENTS[(cell_hash(cell, seed)[0] % 12) as usize])
            .collect();
        Self { period, gradients }
    }

    pub fn sample(&self, p: Vec3) -> f32 {
        let floor = p.floor();
        let f = p - floor;
        let cell = floor.as_ivec3();
        let u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
        let corner = |dx: i32, dy: i32, dz: i32| {
            let offset = IVec3::new(dx, dy, dz);
            self.gradients[wrapped_index(cell + offset, self.period)].dot(f - offset.as_vec3())
        };
        let lerp = |a: f32, b: f32, t: f32| a + (b - a) * t;
        let x00 = lerp(corner(0, 0, 0), corner(1, 0, 0), u.x);
        let x10 = lerp(corner(0, 1, 0), corner(1, 1, 0), u.x);
        let x01 = lerp(corner(0, 0, 1), corner(1, 0, 1), u.x);
        let x11 = lerp(corner(0, 1, 1), corner(1, 1, 1), u.x);
        lerp(lerp(x00, x10, u.y), lerp(x01, x11, u.y), u.z)
    }
}

/// Worley's cellular noise on a lattice that wraps: one feature point per cell, and the
/// distance from a position to the nearest one (F1), in cells, at most 1.
pub struct Worley {
    period: i32,
    points: Vec<Vec3>,
}

impl Worley {
    pub fn new(period: u32, seed: u32) -> Self {
        let period = period as i32;
        let points = cells(period)
            .map(|cell| {
                let [x, y, z] = cell_hash(cell, seed);
                Vec3::new(unit(x), unit(y), unit(z))
            })
            .collect();
        Self { period, points }
    }

    pub fn sample(&self, p: Vec3) -> f32 {
        let floor = p.floor();
        let f = p - floor;
        let cell = floor.as_ivec3();
        let mut nearest = f32::MAX;
        for dz in -1..=1 {
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let offset = IVec3::new(dx, dy, dz);
                    let point =
                        offset.as_vec3() + self.points[wrapped_index(cell + offset, self.period)];
                    nearest = nearest.min((point - f).length_squared());
                }
            }
        }
        nearest.sqrt().min(1.0)
    }
}

/// Fractal sum of Perlin noises over a tile: octaves of period `period`, 2 `period`…,
/// each half the weight of the one before, divided by the sum of the weights. Positions
/// in tiles: the sum repeats every tile.
pub struct PerlinFbm {
    octaves: Vec<(f32, Perlin)>,
}

impl PerlinFbm {
    pub fn new(period: u32, octaves: u32, seed: u32) -> Self {
        let octaves = (0..octaves)
            .map(|o| {
                (
                    (period << o) as f32,
                    Perlin::new(period << o, seed.wrapping_add(o)),
                )
            })
            .collect();
        Self { octaves }
    }

    pub fn sample(&self, tile: Vec3) -> f32 {
        let (mut sum, mut weights, mut weight) = (0.0, 0.0, 1.0);
        for (frequency, octave) in &self.octaves {
            sum += weight * octave.sample(tile * *frequency);
            weights += weight;
            weight *= 0.5;
        }
        sum / weights
    }
}

/// Fractal sum of three inverted Worley noises (1 − F1) over a tile, of period `period`,
/// 2 `period` and 4 `period`, weighted 0.625, 0.25 and 0.125 (Schneider and Vos, "The
/// Real-time Volumetric Cloudscapes of Horizon Zero Dawn", SIGGRAPH 2015): billows, 1 at
/// the feature points. Positions in tiles: the sum repeats every tile.
pub struct WorleyFbm {
    octaves: [(f32, Worley); 3],
}

impl WorleyFbm {
    const WEIGHTS: [f32; 3] = [0.625, 0.25, 0.125];

    pub fn new(period: u32, seed: u32) -> Self {
        let octave = |o: u32| {
            (
                (period << o) as f32,
                Worley::new(period << o, seed.wrapping_add(o)),
            )
        };
        Self {
            octaves: [octave(0), octave(1), octave(2)],
        }
    }

    pub fn sample(&self, tile: Vec3) -> f32 {
        Self::WEIGHTS
            .iter()
            .zip(&self.octaves)
            .map(|(weight, (frequency, octave))| weight * (1.0 - octave.sample(tile * *frequency)))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Positions spread over a few cells, deterministic.
    fn positions(count: u32, extent: f32) -> impl Iterator<Item = Vec3> {
        (0..count).map(move |n| {
            let [x, y, z] = pcg3d(n, 7, 11);
            Vec3::new(unit(x), unit(y), unit(z)) * extent
        })
    }

    #[test]
    fn the_hash_is_deterministic_and_mixes_every_input() {
        assert_eq!(pcg3d(1, 2, 3), pcg3d(1, 2, 3));
        assert_ne!(pcg3d(1, 2, 3), pcg3d(2, 2, 3));
        assert_ne!(pcg3d(1, 2, 3), pcg3d(1, 3, 3));
        assert_ne!(pcg3d(1, 2, 3), pcg3d(1, 2, 4));
    }

    #[test]
    fn unit_numbers_spread_over_zero_one() {
        let values: Vec<f32> = (0..10_000).map(|n| unit(pcg3d(n, 0, 0)[0])).collect();
        assert!(values.iter().all(|v| (0.0..1.0).contains(v)));
        let mean = values.iter().sum::<f32>() / values.len() as f32;
        assert!((mean - 0.5).abs() < 0.01, "mean {mean}");
    }

    #[test]
    fn perlin_is_zero_on_the_lattice_and_bounded_between() {
        let noise = Perlin::new(4, 1);
        for p in [
            Vec3::ZERO,
            Vec3::new(1.0, 2.0, 3.0),
            Vec3::new(-5.0, 7.0, 0.0),
        ] {
            assert!(noise.sample(p).abs() < 1e-6);
        }
        let values: Vec<f32> = positions(20_000, 4.0).map(|p| noise.sample(p)).collect();
        assert!(values.iter().all(|v| v.abs() <= 1.1), "out of range");
        let mean = values.iter().sum::<f32>() / values.len() as f32;
        assert!(mean.abs() < 0.05, "mean {mean}");
        assert!(values.iter().any(|v| *v > 0.5) && values.iter().any(|v| *v < -0.5));
    }

    #[test]
    fn noises_repeat_with_their_period() {
        let perlin = Perlin::new(4, 3);
        let worley = Worley::new(4, 3);
        for p in positions(500, 4.0) {
            for shift in [Vec3::X, Vec3::Y, Vec3::Z, -Vec3::ONE] {
                let q = p + 4.0 * shift;
                assert!((perlin.sample(p) - perlin.sample(q)).abs() < 1e-5);
                assert!((worley.sample(p) - worley.sample(q)).abs() < 1e-5);
            }
        }
    }

    #[test]
    fn noises_are_continuous_across_cells() {
        let perlin = Perlin::new(8, 5);
        let worley = Worley::new(8, 5);
        for p in positions(500, 8.0) {
            for axis in [Vec3::X, Vec3::Y, Vec3::Z] {
                // Just before and just after the cell boundary along the axis.
                let boundary = p - axis * p.dot(axis).fract() + axis;
                let (a, b) = (boundary - 1e-4 * axis, boundary + 1e-4 * axis);
                assert!((perlin.sample(a) - perlin.sample(b)).abs() < 1e-3);
                assert!((worley.sample(a) - worley.sample(b)).abs() < 1e-3);
            }
        }
    }

    #[test]
    fn worley_is_a_distance_to_points_one_per_cell() {
        let noise = Worley::new(4, 9);
        for (n, point) in noise.points.iter().enumerate() {
            let cell = IVec3::new(n as i32 % 4, n as i32 / 4 % 4, n as i32 / 16);
            assert!(noise.sample(cell.as_vec3() + *point) < 1e-6);
        }
        assert!(positions(20_000, 4.0).all(|p| (0.0..=1.0).contains(&noise.sample(p))));
    }

    #[test]
    fn seeds_give_different_noises() {
        let (a, b) = (Perlin::new(4, 1), Perlin::new(4, 2));
        let p = Vec3::new(0.3, 1.7, 2.2);
        assert_ne!(a.sample(p), b.sample(p));
    }

    #[test]
    fn fractal_sums_stay_in_range_and_repeat_every_tile() {
        let perlin = PerlinFbm::new(4, 4, 1);
        let worley = WorleyFbm::new(4, 1);
        for p in positions(2_000, 1.0) {
            let (a, b) = (perlin.sample(p), worley.sample(p));
            assert!(a.abs() <= 1.1 && (0.0..=1.0).contains(&b));
            assert!((a - perlin.sample(p + Vec3::X)).abs() < 1e-4);
            assert!((b - worley.sample(p - Vec3::Z)).abs() < 1e-4);
        }
    }
}
