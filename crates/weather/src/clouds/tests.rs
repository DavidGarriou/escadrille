use std::sync::OnceLock;

use super::*;
use crate::noise::pcg3d;

/// The textures of seed 1, generated once per test process.
fn textures() -> &'static CloudTextures {
    static TEXTURES: OnceLock<CloudTextures> = OnceLock::new();
    TEXTURES.get_or_init(|| CloudTextures::generate(1))
}

/// Deterministic points in [0, 1)², spread evenly.
fn points(count: u32) -> impl Iterator<Item = Vec2> {
    (0..count).map(|n| {
        let [x, y, _] = pcg3d(n, 3, 5);
        Vec2::new(x as f32, y as f32) / 4_294_967_296.0
    })
}

/// Positions at an altitude over a square of `extent` metres around the origin.
fn positions(count: u32, extent: f32, altitude: f32) -> impl Iterator<Item = Vec3> {
    points(count).map(move |p| Vec3::new((p.x - 0.5) * extent, altitude, (p.y - 0.5) * extent))
}

/// A layer of cumulus of a cover and a softness, of the same type everywhere.
fn even_layer(cover: f64, cover_softness: f64) -> CloudLayer {
    CloudLayer {
        cover,
        cover_softness,
        type_variation: 0.0,
        ..CloudLayer::default()
    }
}

/// Share of positions with clouds before the detail noise.
fn cloudy_share(uniforms: &CloudUniforms, positions: impl Iterator<Item = Vec3>) -> f32 {
    let (mut cloudy, mut all) = (0, 0);
    for p in positions {
        all += 1;
        if textures().coarse(uniforms, p).density > 0.0 {
            cloudy += 1;
        }
    }
    cloudy as f32 / all as f32
}

#[test]
fn the_textures_depend_only_on_the_seed() {
    assert_eq!(&CloudTextures::generate(1), textures());
    let other = CloudTextures::generate(2);
    assert_ne!(other.shape, textures().shape);
    assert_ne!(other.detail, textures().detail);
    assert_ne!(other.weather, textures().weather);
}

#[test]
fn the_textures_have_their_sizes() {
    let t = textures();
    assert_eq!(t.shape.data.len(), 128 * 128 * 128);
    assert_eq!(t.detail.data.len(), 32 * 32 * 32);
    assert_eq!(t.weather.data.len(), 2 * 512 * 512);
}

#[test]
fn the_shape_noise_is_spread_evenly() {
    let mut bins = [0usize; 10];
    for &texel in &textures().shape.data {
        bins[(texel as usize * 10 / 256).min(9)] += 1;
    }
    let total = textures().shape.data.len() as f32;
    for (n, count) in bins.iter().enumerate() {
        let share = *count as f32 / total;
        assert!((share - 0.1).abs() < 0.005, "bin {n}: {share}");
    }
}

/// Mean difference between a texel and its neighbour along x, inside the cube and
/// across its edge, where the texture meets its next repeat.
fn neighbour_differences(volume: &Volume) -> (f32, f32) {
    let n = volume.size as i32;
    let (mut inside, mut across) = (0.0, 0.0);
    for z in 0..n {
        for y in 0..n {
            inside += (volume.texel(n / 2, y, z) - volume.texel(n / 2 - 1, y, z)).abs();
            across += (volume.texel(0, y, z) - volume.texel(n - 1, y, z)).abs();
        }
    }
    (inside, across)
}

#[test]
fn the_noises_repeat_without_a_seam() {
    for volume in [&textures().shape, &textures().detail] {
        let (inside, across) = neighbour_differences(volume);
        assert!(across < 1.2 * inside, "seam: {across} against {inside}");
    }
}

#[test]
fn profiles_are_zero_at_base_and_top_and_full_between() {
    for cloud_type in [0.0, 0.25, 0.5, 0.75, 1.0] {
        assert_eq!(profile(0.0, cloud_type), 0.0);
        assert_eq!(profile(1.0, cloud_type), 0.0);
        assert!((profile(0.25, cloud_type) - 1.0).abs() < 1e-6);
    }
    // Cumulus have flatter bases than stratocumulus; they round off from half their height.
    assert!(profile(0.05, 1.0) > profile(0.05, 0.5));
    assert!(profile(0.6, 1.0) < 1.0);
}

#[test]
fn there_is_no_cloud_outside_the_layer() {
    let layer = even_layer(1.0, 0.1);
    let uniforms = layer.uniforms(0.0, DVec3::ZERO);
    let top = (layer.base_altitude + layer.thickness[2]) as f32;
    for altitude in [0.0, 1_000.0, 1_199.0, top + 1.0, 3_000.0] {
        for p in positions(200, 20_000.0, altitude) {
            assert_eq!(textures().density(&uniforms, p), 0.0, "at {p}");
        }
    }
}

#[test]
fn the_cover_sets_the_share_of_cloud() {
    // At a quarter of the layer, where a cumulus's profile is full.
    let level: Vec<Vec3> = positions(20_000, 30_000.0, 1_200.0 + 0.25 * 1_200.0).collect();
    let share = |cover, softness| {
        let uniforms = even_layer(cover, softness).uniforms(0.0, DVec3::ZERO);
        cloudy_share(&uniforms, level.iter().copied())
    };
    assert_eq!(share(0.0, 0.4), 0.0);
    let shares: Vec<f32> = [0.1, 0.25, 0.4, 0.6, 0.8].map(|c| share(c, 0.4)).to_vec();
    assert!(shares.windows(2).all(|w| w[0] < w[1]), "{shares:?}");
    // A cloud stands only where the weather map is above 1 − cover.
    assert!(
        shares
            .iter()
            .zip([0.1, 0.25, 0.4, 0.6, 0.8])
            .all(|(s, c)| *s < c + 0.02)
    );
    // Overcast: a cover of 1 with sharp edges leaves no gap.
    assert!(share(1.0, 0.1) > 0.95, "overcast: {}", share(1.0, 0.1));
}

#[test]
fn densities_are_bounded_and_the_detail_only_erodes() {
    let uniforms = CloudLayer::default().uniforms(0.0, DVec3::ZERO);
    for altitude in [1_250.0, 1_500.0, 1_900.0, 2_300.0] {
        for p in positions(5_000, 30_000.0, altitude) {
            let coarse = textures().coarse(&uniforms, p);
            let density = textures().density(&uniforms, p);
            assert!((0.0..=1.0).contains(&density));
            // The detail erodes the coarse density before its gain (the sharpness).
            assert!(density <= saturate(SHARPNESS * coarse.density) + 1e-6);
            assert!(coarse.density > 0.0 || density == 0.0);
            assert_eq!(textures().extinction(&uniforms, p), density * 0.09);
        }
    }
}

#[test]
fn a_summer_afternoon_has_scattered_cumulus() {
    // Columns over 30 km: the share of sky with a cloud above, scattered (3 to 4 eighths),
    // and the depth of cloud in the cloudy columns, cumulus humilis to mediocris: summits
    // (the deepest tenth of the columns) of 600 m to 1.2 km. A dome is about half as deep
    // on average over its columns as at its summit: the mean, 350 to 700 m, was 500 to
    // 850 m when the cumulus stood full to 65 % of their height (before 2026-11-08).
    let layer = CloudLayer::default();
    let uniforms = layer.uniforms(0.0, DVec3::ZERO);
    let mut depths = Vec::new();
    let columns = 4_000;
    for p in positions(columns, 30_000.0, 0.0) {
        let depth = (0..60)
            .filter(|step| {
                let at = Vec3::new(p.x, 1_200.0 + 20.0 * (*step as f32 + 0.5), p.z);
                textures().density(&uniforms, at) > 0.0
            })
            .count();
        if depth > 0 {
            depths.push(20.0 * depth as f32);
        }
    }
    let share = depths.len() as f32 / columns as f32;
    let mean = depths.iter().sum::<f32>() / depths.len() as f32;
    depths.sort_by(f32::total_cmp);
    let summits = depths[depths.len() * 9 / 10];
    assert!((0.35..0.5).contains(&share), "sky covered: {share}");
    assert!((350.0..700.0).contains(&mean), "mean cloud depth: {mean} m");
    assert!((600.0..1_200.0).contains(&summits), "summits: {summits} m");
}

/// The summer afternoon's layer over 4.5 km, on a grid of 50 m: the height above the base
/// of the highest cloud (before the detail noise) in each column, every 10 m; `None`
/// over clear sky.
fn tops() -> (Vec<Option<f32>>, usize, f32) {
    let layer = CloudLayer::default();
    let uniforms = layer.uniforms(0.0, DVec3::ZERO);
    let (base, thickness) = (layer.base_altitude as f32, layer.thickness[2] as f32);
    let n = 90;
    let tops = (0..n * n)
        .map(|k| {
            let (x, z) = ((k % n) as f32 * 50.0, (k / n) as f32 * 50.0);
            (0..(thickness / 10.0) as u32)
                .rev()
                .map(|step| 10.0 * step as f32)
                .find(|h| {
                    let p = Vec3::new(x, base + h + 5.0, z);
                    textures().coarse(&uniforms, p).density > 0.0
                })
        })
        .collect();
    (tops, n, thickness)
}

#[test]
fn cumulus_rise_from_their_edges_to_summits_of_their_own() {
    let (tops, n, thickness) = tops();
    // The flanks: the top of the cloudy columns beside a clear one, at the visible edge.
    let mut walls: Vec<f32> = Vec::new();
    for j in 1..n - 1 {
        for i in 1..n - 1 {
            let Some(top) = tops[j * n + i] else { continue };
            let beside = [
                j * n + i - 1,
                j * n + i + 1,
                (j - 1) * n + i,
                (j + 1) * n + i,
            ];
            if beside.iter().any(|&k| tops[k].is_none()) {
                walls.push(top);
            }
        }
    }
    walls.sort_by(f32::total_cmp);
    let wall = walls[walls.len() / 2] / thickness;
    // The summits: cores whose cover saturated all reached the layer's top, flat at one
    // height. Measured with the profile before 2026-11-08: 7.6 % of the cloudy columns
    // above 90 % of the layer, a median wall of 0.15 of it (on the scenes' layers, 0.21).
    let cloudy: Vec<f32> = tops.iter().flatten().copied().collect();
    let high = cloudy.iter().filter(|&&t| t > 0.9 * thickness).count() as f32;
    let high = high / cloudy.len() as f32;
    assert!(high < 0.01, "share of columns near the layer's top: {high}");
    assert!(
        wall < 0.14,
        "median wall at the visible edge: {wall} of the layer"
    );
}

#[test]
fn the_envelope_tells_its_top_and_clouds_lie_within_it() {
    let layer = CloudLayer::default();
    let uniforms = layer.uniforms(0.0, DVec3::ZERO);
    for altitude in [1_250.0, 1_500.0, 1_900.0] {
        for p in positions(5_000, 30_000.0, altitude) {
            let coarse = textures().coarse(&uniforms, p);
            assert!((0.0..=layer.thickness[2] as f32).contains(&coarse.above));
            // No cloud outside an envelope: a march strides only there.
            assert!(coarse.above > 0.0 || coarse.density == 0.0);
            // Above the top of its envelope, nothing.
            let over = textures().coarse(&uniforms, p + Vec3::Y * (coarse.above + 1.0));
            assert!(coarse.above == 0.0 || over.density == 0.0);
        }
    }
}

#[test]
fn the_sky_is_farther_at_the_foot_of_a_tower_than_at_its_top() {
    // The tower of traversee_nuage, seen from the west: on its flank, the first dense
    // sample east of the eye at each height.
    let textures = CloudTextures::generate(85);
    let layer = CloudLayer {
        base_altitude: 600.0,
        cover: 0.45,
        ..CloudLayer::default()
    };
    let uniforms = layer.uniforms(0.0, DVec3::ZERO);
    let toward_eye = Vec3::NEG_X;
    let sky = |height: f32| {
        let x = (0..400)
            .map(|i| 200.0 + 5.0 * i as f32)
            .find(|&x| textures.density(&uniforms, Vec3::new(x, height, 0.0)) > 0.5)
            .expect("the flank");
        textures.optical_depth_to_sky(&uniforms, Vec3::new(x, height, 0.0), toward_eye, 0.5)
    };
    // 21, 6 and 0.8 on 2026-11-09: under the cloud above, the sky is far; on the top, near.
    let (foot, middle, top) = (sky(660.0), sky(870.0), sky(1_080.0));
    assert!(
        foot > 2.0 * middle && middle > 2.0 * top,
        "{foot} {middle} {top}"
    );
    assert!(top < 2.0 && foot > 10.0, "{foot} {middle} {top}");
    // Above the layer, nothing.
    let above = Vec3::new(1_000.0, 2_500.0, 0.0);
    assert_eq!(
        textures.optical_depth_to_sky(&uniforms, above, toward_eye, 0.5),
        0.0
    );
}

#[test]
fn the_wind_carries_the_clouds() {
    let layer = CloudLayer {
        wind: DVec3::new(12.0, 3.0, -5.0),
        ..CloudLayer::default()
    };
    let (now, later) = (
        layer.uniforms(0.0, DVec3::ZERO),
        layer.uniforms(60.0, DVec3::ZERO),
    );
    let moved = Vec3::new(12.0, 0.0, -5.0) * 60.0;
    for p in positions(5_000, 20_000.0, 1_500.0) {
        let (before, after) = (
            textures().density(&now, p),
            textures().density(&later, p + moved),
        );
        assert!(
            (before - after).abs() < 2e-3,
            "at {p}: {before} then {after}"
        );
    }
}

#[test]
fn the_origin_does_not_move_the_clouds() {
    let layer = CloudLayer::default();
    let origin = DVec3::new(31_250.0, 400.0, -18_750.0);
    let (at_zero, at_origin) = (
        layer.uniforms(0.0, DVec3::ZERO),
        layer.uniforms(0.0, origin),
    );
    for p in positions(5_000, 4_000.0, 1_100.0) {
        let world = p + origin.as_vec3();
        let (a, b) = (
            textures().density(&at_zero, world),
            textures().density(&at_origin, p),
        );
        assert!((a - b).abs() < 2e-3, "at {p}: {a} against {b}");
    }
}

#[test]
fn wrapped_offsets_stay_within_one_repeat() {
    let layer = CloudLayer {
        wind: DVec3::new(-30.0, 0.0, 17.0),
        ..CloudLayer::default()
    };
    let u = layer.uniforms(86_400.0, DVec3::new(-45_000.0, 250.0, 72_000.0));
    for offset in [
        u.shape_offset,
        u.detail_offset,
        u.weather_offset.extend(0.5),
    ] {
        assert!(
            offset.cmpge(Vec3::ZERO).all() && offset.cmplt(Vec3::ONE).all(),
            "{offset}"
        );
    }
    assert_eq!(u.altitude_offset, 250.0);
}
