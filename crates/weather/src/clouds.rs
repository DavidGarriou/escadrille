//! The cloud density field, after Schneider and Vos ("The Real-time Volumetric
//! Cloudscapes of Horizon Zero Dawn", SIGGRAPH 2015) and Schneider ("Nubis", SIGGRAPH
//! 2017):
//!
//! - a 2D weather map, carried by the wind, places the clouds one by one and varies their
//!   type from place to place;
//! - each type (stratus, stratocumulus, cumulus) has its thickness and its vertical
//!   profile; the cover of the weather map sets how high each cloud rises in its layer,
//!   which makes an envelope tall in its middle and low at its edges: a dome;
//! - a 3D shape noise (Perlin-Worley) erodes the envelope into billows, and a finer
//!   detail noise (Worley) erodes their edges: wisps at the bottom, curls above.
//!
//! Eroding an envelope rather than cutting the shape noise by the cover is Nubis's way:
//! the clouds are where the weather map puts them, and the 4 km repeat of the shape noise
//! does not show as a grid of identical clouds.
//!
//! [`CloudTextures::density`] is the reference of `cloud_density` in
//! `assets/shaders/clouds/density.wesl`: same textures, same steps, same order. Any change
//! here goes there too, and the game's parity test compares them.

use glam::{DVec3, Vec2, Vec3};

use crate::grid::{Plane, Volume, to_unorm8};
use crate::noise::{PerlinFbm, WorleyFbm, pcg3d};

/// Size of the shape noise, texels per side (128³ in Schneider and Vos).
pub const SHAPE_SIZE: u32 = 128;
/// Size of the detail noise, texels per side (32³ in Schneider and Vos).
pub const DETAIL_SIZE: u32 = 32;
/// Size of the weather map, texels per side.
pub const WEATHER_SIZE: u32 = 512;

/// Height fraction where each type of cloud reaches its full profile, and where it starts
/// to fall off towards its top: stratus, stratocumulus, cumulus. Cumulus have flat bases
/// and rounded tops; stratus are a thin, even sheet.
const RISE_END: [f32; 3] = [0.1, 0.15, 0.06];
const FALL_START: [f32; 3] = [0.4, 0.5, 0.65];

/// Height fraction over which the detail noise turns from wisps (at the base) to billows:
/// a thin band, since the base of a cumulus is flat and sharp.
const WISPS_HEIGHT: f32 = 0.03;

/// Gain of the Perlin noise of the shape around ½, which spreads it over [0, 1].
const PERLIN_GAIN: f32 = 1.5;

/// The noise textures of the clouds. Generated at start from a seed, the same on every
/// machine; uploaded as they are to the GPU.
#[derive(Debug, Clone, PartialEq)]
pub struct CloudTextures {
    /// Shape noise, equalised: its values are spread evenly over [0, 1], so that the
    /// share of an envelope it erodes does not depend on the seed.
    pub shape: Volume,
    /// Detail noise, billows of 1 at their centres.
    pub detail: Volume,
    /// Weather map: cover (red) and type of cloud (green) relative to the layer's, both
    /// equalised.
    pub weather: Plane,
}

impl CloudTextures {
    pub fn generate(seed: u32) -> Self {
        let seed_of = |k: u32| pcg3d(seed, k, 0xC10D)[0];
        Self {
            shape: shape_noise(seed_of(1)),
            detail: detail_noise(seed_of(2)),
            weather: weather_map(seed_of(3)),
        }
    }

    /// Density of the clouds before the detail noise, and the height fraction in the
    /// layer of the type of cloud there: the cheap first look of a ray march.
    pub fn coarse(&self, uniforms: &CloudUniforms, position: Vec3) -> Coarse {
        let u = uniforms;
        let weather = self
            .weather
            .sample(Vec2::new(position.x, position.z) * u.weather_inverse_scale + u.weather_offset);
        let cover = saturate((weather.x - (1.0 - u.cover)) / u.cover_softness);
        let cloud_type = saturate(u.cloud_type + (weather.y - 0.5) * u.type_variation);
        let thickness = blend_types(u.thickness.to_array(), cloud_type);
        let height = (position.y + u.altitude_offset - u.base_altitude) / thickness;
        if height <= 0.0 || height >= cover {
            return Coarse::CLEAR;
        }
        let shape = self
            .shape
            .sample(position * u.shape_inverse_scale + u.shape_offset);
        let envelope = cover * profile(height / cover, cloud_type);
        let eaten = (1.0 - shape) * u.shape_erosion;
        Coarse {
            density: ((envelope - eaten) / (1.0 - eaten)).max(0.0),
            height,
        }
    }

    /// Density of the clouds at a position relative to the origin of the uniforms, 0 to
    /// 1: the share of the full extinction ([`CloudUniforms::extinction`]).
    pub fn density(&self, uniforms: &CloudUniforms, position: Vec3) -> f32 {
        let coarse = self.coarse(uniforms, position);
        if coarse.density <= 0.0 {
            return 0.0;
        }
        let detail = self
            .detail
            .sample(position * uniforms.detail_inverse_scale + uniforms.detail_offset);
        let billows = mix(detail, 1.0 - detail, saturate(coarse.height / WISPS_HEIGHT));
        let erosion = billows * uniforms.detail_erosion;
        saturate((coarse.density - erosion) / (1.0 - erosion))
    }

    /// Extinction coefficient of the clouds at a position, 1/m.
    pub fn extinction(&self, uniforms: &CloudUniforms, position: Vec3) -> f32 {
        self.density(uniforms, position) * uniforms.extinction
    }
}

/// Density before the detail noise, and height fraction in the layer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coarse {
    pub density: f32,
    pub height: f32,
}

impl Coarse {
    const CLEAR: Self = Self {
        density: 0.0,
        height: 0.0,
    };
}

/// Vertical profile of a type of cloud (0 stratus, ½ stratocumulus, 1 cumulus) at a
/// height fraction of its layer: the share of the shape noise kept there.
pub fn profile(height: f32, cloud_type: f32) -> f32 {
    let of_type = |n: usize| {
        smoothstep(0.0, RISE_END[n], height) * (1.0 - smoothstep(FALL_START[n], 1.0, height))
    };
    blend_types([of_type(0), of_type(1), of_type(2)], cloud_type)
}

/// A quantity of the three types of cloud, blended for a type between them: stratus at
/// 0, stratocumulus at ½, cumulus at 1.
fn blend_types(values: [f32; 3], cloud_type: f32) -> f32 {
    mix(
        mix(values[0], values[1], saturate(2.0 * cloud_type)),
        values[2],
        saturate(2.0 * cloud_type - 1.0),
    )
}

/// WGSL's `mix`, `saturate` and `smoothstep`, with the same formulas.
fn mix(a: f32, b: f32, t: f32) -> f32 {
    a * (1.0 - t) + b * t
}

fn saturate(x: f32) -> f32 {
    x.clamp(0.0, 1.0)
}

fn smoothstep(low: f32, high: f32, x: f32) -> f32 {
    let t = saturate((x - low) / (high - low));
    t * t * (3.0 - 2.0 * t)
}

/// A layer of clouds over the map, as a mission's weather sets it, with the scales of
/// its noises.
#[derive(Debug, Clone, PartialEq)]
pub struct CloudLayer {
    /// Cloud base, m above mean sea level.
    pub base_altitude: f64,
    /// Thickness of a layer of stratus, of stratocumulus and of cumulus, m; a type
    /// between them blends them.
    pub thickness: [f64; 3],
    /// Share of the weather map where clouds stand, 0 (clear) to 1 (overcast). Seen from
    /// below, the share of the sky covered is less, since the shape noise erodes the
    /// clouds' edges: about 40 % at 0.55 (test `a_summer_afternoon_has_scattered_cumulus`).
    pub cover: f64,
    /// Range of the weather map over which a cloud goes from its edge to its full
    /// envelope, 0 to 1: small, sharp clouds of even height; large, varied clouds, the
    /// smallest of them low and thin.
    pub cover_softness: f64,
    /// Type of cloud: 0 stratus, ½ stratocumulus, 1 cumulus.
    pub cloud_type: f64,
    /// How much the weather map changes the type from place to place, ± half of it.
    pub type_variation: f64,
    /// Wind that carries the clouds, m/s, cloud frame (x east, z south); the vertical
    /// part is ignored.
    pub wind: DVec3,
    /// Extinction coefficient of the clouds at full density, 1/m.
    pub extinction: f64,
    /// Size of one repeat of the shape noise, of the detail noise and of the weather map, m.
    pub shape_scale: f64,
    pub detail_scale: f64,
    pub weather_scale: f64,
    /// How deep the shape noise and the detail noise eat into the clouds, 0 to less
    /// than 1.
    pub shape_erosion: f64,
    pub detail_erosion: f64,
}

impl Default for CloudLayer {
    /// Fair-weather cumulus of a summer afternoon, no wind. ESTIMÉ, to be replaced by a
    /// mission's weather:
    /// - base 1 200 m, cumulus 1 200 m thick (humilis to mediocris are a few hundred
    ///   metres to 1.5 km deep), stratocumulus 500 m, stratus 300 m;
    /// - extinction 0.09/m, from σ ≈ 3 LWC / (2 ρw re) with a liquid water content of
    ///   0.5 g/m³ and droplets of 8 µm: a visibility of about 45 m inside at full density;
    /// - weather map repeat 32 km: clumps of about 2 km, one cumulus each, with a cover
    ///   that goes from clear to almost full from one clump to the next;
    /// - shape repeat 4 km, billows of about 700 m and finer; detail repeat 250 m.
    fn default() -> Self {
        Self {
            base_altitude: 1_200.0,
            thickness: [300.0, 500.0, 1_200.0],
            cover: 0.55,
            cover_softness: 0.4,
            cloud_type: 1.0,
            type_variation: 0.2,
            wind: DVec3::ZERO,
            extinction: 0.09,
            shape_scale: 4_000.0,
            detail_scale: 250.0,
            weather_scale: 32_000.0,
            shape_erosion: 0.7,
            detail_erosion: 0.35,
        }
    }
}

impl CloudLayer {
    /// The uniforms of the density at a time (s) for positions relative to an origin
    /// (cloud frame, m), such as the render origin. The clouds move with the wind: the
    /// density at p and time t is the one at p − wind t at time 0. Offsets are wrapped
    /// into one repeat of their texture in `f64` before going to `f32`.
    pub fn uniforms(&self, time: f64, origin: DVec3) -> CloudUniforms {
        let drift = DVec3::new(self.wind.x, 0.0, self.wind.z) * time;
        let anchor = origin - drift;
        let wrapped = |scale: f64| {
            let tiles = anchor / scale;
            (tiles - tiles.floor()).as_vec3()
        };
        let weather = wrapped(self.weather_scale);
        CloudUniforms {
            shape_offset: wrapped(self.shape_scale),
            shape_inverse_scale: (1.0 / self.shape_scale) as f32,
            detail_offset: wrapped(self.detail_scale),
            detail_inverse_scale: (1.0 / self.detail_scale) as f32,
            weather_offset: Vec2::new(weather.x, weather.z),
            weather_inverse_scale: (1.0 / self.weather_scale) as f32,
            altitude_offset: origin.y as f32,
            thickness: DVec3::from_array(self.thickness).as_vec3(),
            base_altitude: self.base_altitude as f32,
            cover: self.cover as f32,
            cover_softness: self.cover_softness as f32,
            cloud_type: self.cloud_type as f32,
            type_variation: self.type_variation as f32,
            shape_erosion: self.shape_erosion as f32,
            detail_erosion: self.detail_erosion as f32,
            extinction: self.extinction as f32,
        }
    }
}

/// What the density needs of a [`CloudLayer`] at a time and for an origin, in `f32`: the
/// uniforms of the shader, field for field and in the same order.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CloudUniforms {
    /// Offset of the shape noise, in repeats, and its repeats per metre.
    pub shape_offset: Vec3,
    pub shape_inverse_scale: f32,
    pub detail_offset: Vec3,
    pub detail_inverse_scale: f32,
    /// Offset of the weather map (x east, z south), in repeats, and its repeats per metre.
    pub weather_offset: Vec2,
    pub weather_inverse_scale: f32,
    /// Altitude of the origin, m.
    pub altitude_offset: f32,
    pub thickness: Vec3,
    pub base_altitude: f32,
    pub cover: f32,
    pub cover_softness: f32,
    pub cloud_type: f32,
    pub type_variation: f32,
    pub shape_erosion: f32,
    pub detail_erosion: f32,
    pub extinction: f32,
}

/// Values computed for every texel of a grid, `count` of them, over all the cores: each
/// texel depends only on its index, so the result does not depend on the threads.
fn fill(count: usize, value: impl Fn(usize) -> f32 + Sync) -> Vec<f32> {
    let threads = std::thread::available_parallelism().map_or(1, |n| n.get());
    let chunk = count.div_ceil(threads).max(1);
    let mut values = vec![0.0; count];
    std::thread::scope(|scope| {
        for (n, part) in values.chunks_mut(chunk).enumerate() {
            let value = &value;
            scope.spawn(move || {
                for (i, v) in part.iter_mut().enumerate() {
                    *v = value(n * chunk + i);
                }
            });
        }
    });
    values
}

/// Values replaced by their rank, spread evenly over [0, 1]: the order is kept, the
/// distribution becomes uniform. Ties break by index, so the result is deterministic.
fn equalised(values: &[f32]) -> Vec<f32> {
    let mut order: Vec<usize> = (0..values.len()).collect();
    order.sort_unstable_by(|&a, &b| values[a].total_cmp(&values[b]).then(a.cmp(&b)));
    let mut ranks = vec![0.0; values.len()];
    for (rank, &index) in order.iter().enumerate() {
        ranks[index] = (rank as f32 + 0.5) / values.len() as f32;
    }
    ranks
}

/// Centre of the texel of index `index` in a cube of `size`³, in repeats.
fn texel_centre(index: usize, size: u32) -> Vec3 {
    let size = size as usize;
    let (x, y, z) = (index % size, index / size % size, index / (size * size));
    (Vec3::new(x as f32, y as f32, z as f32) + 0.5) / size as f32
}

/// Perlin-Worley shape noise: Perlin's smooth noise, its hollows filled with Worley
/// billows, then eroded by finer billows (Schneider and Vos 2015), equalised.
fn shape_noise(seed: u32) -> Volume {
    let perlin = PerlinFbm::new(4, 4, seed);
    let billows = WorleyFbm::new(6, seed.wrapping_add(16));
    let erosion = WorleyFbm::new(10, seed.wrapping_add(32));
    let raw = fill((SHAPE_SIZE as usize).pow(3), |index| {
        let tile = texel_centre(index, SHAPE_SIZE);
        let smooth = saturate(0.5 + PERLIN_GAIN * perlin.sample(tile));
        let billow = billows.sample(tile);
        let perlin_worley = billow + smooth * (1.0 - billow);
        let eroded_by = erosion.sample(tile);
        (perlin_worley + 1.0 - eroded_by) / (2.0 - eroded_by)
    });
    Volume {
        size: SHAPE_SIZE,
        data: equalised(&raw).into_iter().map(to_unorm8).collect(),
    }
}

/// Worley detail noise: billows of three sizes, the largest a quarter of the repeat.
fn detail_noise(seed: u32) -> Volume {
    let billows = WorleyFbm::new(2, seed);
    let data = fill((DETAIL_SIZE as usize).pow(3), |index| {
        billows.sample(texel_centre(index, DETAIL_SIZE))
    });
    Volume {
        size: DETAIL_SIZE,
        data: data.into_iter().map(to_unorm8).collect(),
    }
}

/// Weather map, equalised: where the clouds stand (red), Worley clumps of a sixteenth of
/// the repeat on a smooth noise of a third of it, and the type of cloud (green), smooth
/// over half of it. The clumps place the clouds one by one, over a repeat far larger than
/// the shape noise's, which only sculpts them: its own repeat no longer shows as a grid.
fn weather_map(seed: u32) -> Plane {
    let clumps = WorleyFbm::new(16, seed);
    let smooth = PerlinFbm::new(3, 4, seed.wrapping_add(8));
    let cloud_type = PerlinFbm::new(2, 3, seed.wrapping_add(16));
    let count = (WEATHER_SIZE as usize).pow(2);
    // A plane of the 3D noises off their lattice, so that it does not cut through the
    // lattice points, where they are all zero.
    let at = |index: usize| {
        let size = WEATHER_SIZE as usize;
        let (x, y) = (index % size, index / size);
        Vec3::new(
            (x as f32 + 0.5) / size as f32,
            (y as f32 + 0.5) / size as f32,
            0.37,
        )
    };
    let covers = equalised(&fill(count, |index| {
        let tile = at(index);
        0.65 * clumps.sample(tile) + 0.35 * saturate(0.5 + 0.5 * smooth.sample(tile))
    }));
    let types = equalised(&fill(count, |index| cloud_type.sample(at(index))));
    Plane {
        size: WEATHER_SIZE,
        data: covers
            .into_iter()
            .zip(types)
            .flat_map(|(c, t)| [to_unorm8(c), to_unorm8(t)])
            .collect(),
    }
}

#[cfg(test)]
mod tests;
