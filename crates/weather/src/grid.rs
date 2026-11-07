//! Grids of 8-bit values that become textures on the GPU, and their sampling as the GPU
//! does it: texel centres at (i + ½) / size, linear filtering, repeat addressing. The
//! values are stored as the texture stores them (`R8Unorm`, `Rg8Unorm`), so the CPU
//! and the GPU read the same numbers; only the filter weights may differ by the GPU's
//! sub-texel precision.

use glam::{Vec2, Vec3};

/// A value in [0, 1] as an 8-bit unorm texel.
pub fn to_unorm8(value: f32) -> u8 {
    (value.clamp(0.0, 1.0) * 255.0).round() as u8
}

fn from_unorm8(texel: u8) -> f32 {
    texel as f32 / 255.0
}

/// One-channel cube of `size`³ texels (`R8Unorm`), x fastest, then y, then z: the
/// layout of a 3D texture's data.
#[derive(Debug, Clone, PartialEq)]
pub struct Volume {
    pub size: u32,
    pub data: Vec<u8>,
}

impl Volume {
    /// The texel at (x, y, z), wrapped into the cube.
    pub fn texel(&self, x: i32, y: i32, z: i32) -> f32 {
        let n = self.size as i32;
        let (x, y, z) = (x.rem_euclid(n), y.rem_euclid(n), z.rem_euclid(n));
        from_unorm8(self.data[(x + n * (y + n * z)) as usize])
    }

    /// Trilinear sample at texture coordinates `uvw` (1 = one size of the cube), with
    /// repeat addressing.
    pub fn sample(&self, uvw: Vec3) -> f32 {
        let s = uvw * self.size as f32 - 0.5;
        let floor = s.floor();
        let f = s - floor;
        let i = floor.as_ivec3();
        let lerp = |a: f32, b: f32, t: f32| a + (b - a) * t;
        let row = |y: i32, z: i32| lerp(self.texel(i.x, y, z), self.texel(i.x + 1, y, z), f.x);
        let plane = |z: i32| lerp(row(i.y, z), row(i.y + 1, z), f.y);
        lerp(plane(i.z), plane(i.z + 1), f.z)
    }
}

/// Two-channel square of `size`² texels (`Rg8Unorm`), x fastest, the two channels of a
/// texel side by side: the layout of a 2D texture's data.
#[derive(Debug, Clone, PartialEq)]
pub struct Plane {
    pub size: u32,
    pub data: Vec<u8>,
}

impl Plane {
    /// The texel at (x, y), wrapped into the square.
    pub fn texel(&self, x: i32, y: i32) -> Vec2 {
        let n = self.size as i32;
        let index = 2 * (x.rem_euclid(n) + n * y.rem_euclid(n)) as usize;
        Vec2::new(
            from_unorm8(self.data[index]),
            from_unorm8(self.data[index + 1]),
        )
    }

    /// Bilinear sample at texture coordinates `uv`, with repeat addressing.
    pub fn sample(&self, uv: Vec2) -> Vec2 {
        let s = uv * self.size as f32 - 0.5;
        let floor = s.floor();
        let f = s - floor;
        let (x, y) = (floor.x as i32, floor.y as i32);
        let bottom = self.texel(x, y).lerp(self.texel(x + 1, y), f.x);
        let top = self.texel(x, y + 1).lerp(self.texel(x + 1, y + 1), f.x);
        bottom.lerp(top, f.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A 4³ cube whose texel (x, y, z) holds x + 4 y + 16 z (as 8-bit numbers).
    fn ramp() -> Volume {
        Volume {
            size: 4,
            data: (0..64).collect(),
        }
    }

    #[test]
    fn unorm8_rounds_and_clamps() {
        assert_eq!(to_unorm8(0.0), 0);
        assert_eq!(to_unorm8(1.0), 255);
        assert_eq!(to_unorm8(0.5), 128);
        assert_eq!(to_unorm8(-0.2), 0);
        assert_eq!(to_unorm8(1.7), 255);
    }

    #[test]
    fn a_texel_centre_reads_the_texel() {
        let volume = ramp();
        let centre = (Vec3::new(1.0, 2.0, 3.0) + 0.5) / 4.0;
        assert!((volume.sample(centre) - (1 + 8 + 48) as f32 / 255.0).abs() < 1e-6);
    }

    #[test]
    fn between_centres_the_sample_is_linear() {
        let volume = ramp();
        // Halfway between texels (1, 2, 3) and (2, 2, 3).
        let uvw = (Vec3::new(1.5, 2.0, 3.0) + 0.5) / 4.0;
        assert!((volume.sample(uvw) - 57.5 / 255.0).abs() < 1e-6);
    }

    #[test]
    fn sampling_repeats() {
        let volume = ramp();
        let uvw = Vec3::new(0.1, 0.62, 0.93);
        for shift in [Vec3::X, -Vec3::Y, 3.0 * Vec3::Z] {
            assert!((volume.sample(uvw) - volume.sample(uvw + shift)).abs() < 1e-5);
        }
        // Across the edge, texel 3 blends into texel 0 of the next repeat.
        let edge = Vec3::new(1.0, 0.125, 0.125);
        assert!((volume.sample(edge) - 1.5 / 255.0).abs() < 1e-6);
    }

    #[test]
    fn a_plane_samples_both_channels() {
        let plane = Plane {
            size: 2,
            data: vec![0, 255, 255, 0, 0, 255, 255, 0],
        };
        let centre = plane.sample(Vec2::new(0.25, 0.75));
        assert!((centre - Vec2::new(0.0, 1.0)).length() < 1e-6);
        let middle = plane.sample(Vec2::new(0.5, 0.25));
        assert!((middle - Vec2::new(0.5, 0.5)).length() < 1e-6);
        assert!(
            (plane.sample(Vec2::new(0.1, 0.3)) - plane.sample(Vec2::new(1.1, -0.7))).length()
                < 1e-5
        );
    }
}
