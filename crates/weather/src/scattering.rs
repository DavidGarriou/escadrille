//! Light scattered by clouds: the lighting model of the cloud shader
//! (`assets/shaders/clouds/march.wesl`, functions and constants of the same names),
//! written here to test its energy against two-stream theory.
//!
//! The light of the sun that a cloud sends towards the eye, per unit scattering
//! coefficient and per unit solar irradiance, at an optical depth τ from the sun along the
//! sun's direction:
//! - single scattering: the phase function times e^−τ;
//! - multiple scattering, in two parts:
//!   - near the lit surface, the first few scatterings, falling fast as e^−τ (the first
//!     octaves of Wrenninge, "Oz: The Great and Volumetric", SIGGRAPH 2013 Talks): a bud in
//!     the shadow of its neighbour, a few optical depths away from the sun, turns two to
//!     three times darker than its lit face (test `a_bud_in_its_own_shadow_is_two_to_three_times_darker`);
//!   - deep in the cloud, the diffusion approximation: the light that goes on through it
//!     falls slowly, as 1 / (1 + k τ), not as e^−τ.
//!
//!   Their levels and falls are set so that a thick layer returns from its top and lets
//!   through its base what two-stream theory gives (test
//!   `a_thick_layer_reflects_and_transmits_as_two_stream_theory`). With the diffusion
//!   alone (art director's second review), a lit cumulus had no shade between its buds.
//!
//! Limit: the model is local. A thin wisp lit by the sun returns as much as the top of a
//! thick cloud, where two-stream theory would return less (2.3 times less at τ = 5): its
//! edges may glow a little too much.

use std::f32::consts::PI;

/// Two-lobe Henyey-Greenstein phase function: forward lobe, backward lobe, weight of the
/// forward one (the silver lining against the sun, the glow away from it).
pub const FORWARD_G: f32 = 0.8;
pub const BACKWARD_G: f32 = -0.3;
pub const FORWARD_WEIGHT: f32 = 0.7;
/// The droplets' narrow forward peak, which makes the rim of a cloud blaze when the sun is
/// just behind it, and its share, taken from the two lobes. Jendersie and d'Eon ("An
/// Approximate Mie Scattering Function for Fog and Cloud Rendering", SIGGRAPH 2023 Talks)
/// fit Mie scattering by water droplets of diameter d (5 to 50 µm) with a
/// Henyey-Greenstein peak of g = e^(−0.0990567 / (d − 1.67154)) and a Draine body of share
/// w_D = e^(−0.599085 / (d − 0.641583) − 0.665888). For d = 15 µm (`ESTIMÉ`: fair-weather
/// cumulus, droplets of 10 to 20 µm), g = 0.9926 and the peak holds 1 − w_D = 0.51 of the
/// light. The peak, 0.3° wide, is narrower than the sun's disc (0.53°): broadened by it,
/// g = 0.99. The two lobes stand for the Draine body; scaled by half, they take 0.4 % of
/// the light of the lit domes of `cumulus_midi` and 1.5 % of those of `cumulus_contre_jour`
/// (2026-11-11).
pub const PEAK_G: f32 = 0.99;
pub const PEAK_WEIGHT: f32 = 0.5;

/// Asymmetry of the phase function of cloud droplets (Mie, droplets of about 10 µm in
/// visible light).
pub const DROPLET_G: f32 = 0.85;

/// Multiple scattering near the lit surface: its level, against isotropic scattering of
/// the full sunlight, and the optical depth over which it falls by e.
pub const SURFACE_LEVEL: f32 = 1.8;
pub const SURFACE_DEPTH: f32 = 1.0;
/// Multiple scattering deep in the cloud: its level, and its fall per unit optical depth.
/// The four constants are a compromise, found by a sweep: the top and the base of a layer
/// of τ = 50 within 8 %, the base of layers from τ = 5 to 100 within 20 %, a bud in its
/// own shadow twice as dark.
pub const DIFFUSE_LEVEL: f32 = 2.1;
pub const DIFFUSE_FALL: f32 = 0.055;

pub fn henyey_greenstein(cos_theta: f32, g: f32) -> f32 {
    let g2 = g * g;
    (1.0 - g2) / (4.0 * PI * (1.0 + g2 - 2.0 * g * cos_theta).powf(1.5))
}

/// Phase function of single scattering, sr⁻¹, for the cosine of the angle between the
/// eye's ray and the direction towards the sun.
pub fn phase(cos_theta: f32) -> f32 {
    let backward = henyey_greenstein(cos_theta, BACKWARD_G);
    let forward = henyey_greenstein(cos_theta, FORWARD_G);
    let lobes = backward * (1.0 - FORWARD_WEIGHT) + forward * FORWARD_WEIGHT;
    lobes * (1.0 - PEAK_WEIGHT) + henyey_greenstein(cos_theta, PEAK_G) * PEAK_WEIGHT
}

/// Level of the light of the sky and the ground scattered by a cloud, each from its half
/// of the sphere, against isotropic scattering.
pub const AMBIENT_LEVEL: f32 = 2.3;

/// Albedo of the ground under the clouds: fields and meadows. ESTIMÉ: 0.16 to 0.26 for
/// grass and crops (Oke, "Boundary Layer Climates", 2nd ed., 1987, table 1.1), the low side
/// for the dark crops and woods of Flanders.
pub const GROUND_ALBEDO: f32 = 0.18;

/// Multiple scattering of a light at an optical depth towards it, per unit scattering
/// coefficient and unit irradiance, sr⁻¹.
pub fn multiple(depth: f32) -> f32 {
    let surface = SURFACE_LEVEL * (-depth / SURFACE_DEPTH).exp();
    let diffusion = DIFFUSE_LEVEL / (1.0 + DIFFUSE_FALL * depth);
    (surface + diffusion) / (4.0 * PI)
}

/// Sunlight scattered towards the eye, per unit scattering coefficient and unit solar
/// irradiance, sr⁻¹: single scattering and multiple scattering.
pub fn sun_scattering(depth_to_sun: f32, cos_theta: f32) -> f32 {
    phase(cos_theta) * (-depth_to_sun).exp() + multiple(depth_to_sun)
}

/// Share of a diffuse light (the sky's, the ground's) left at an optical depth towards it:
/// the multiple scattering's fall, 1 at the surface.
pub fn ambient_share(depth: f32) -> f32 {
    multiple(depth) / multiple(0.0)
}

/// Light of the sky (radiance `sky`, from above) and of the ground (radiance `ground`,
/// from below) scattered towards the eye, per unit scattering coefficient, each dimmed
/// through the optical depth of cloud on its side.
pub fn ambient_scattering(sky: f32, ground: f32, depth_up: f32, depth_down: f32) -> f32 {
    0.5 * AMBIENT_LEVEL * (sky * ambient_share(depth_up) + ground * ambient_share(depth_down))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Radiance (per unit solar irradiance) leaving a homogeneous layer of optical
    /// thickness `thickness`, lit by the sun at `mu_sun` (cosine of its zenith angle), seen
    /// straight down from above (`from_above`) or straight up from below, marched as the
    /// shader marches: each step integrated exactly over its length.
    fn layer_radiance(thickness: f32, mu_sun: f32, from_above: bool) -> f32 {
        let step = 0.01;
        let cos_theta = if from_above { -mu_sun } else { mu_sun };
        let (mut light, mut transmittance, mut depth) = (0.0, 1.0, 0.0);
        while depth < thickness && transmittance > 1e-4 {
            let from_top = if from_above {
                depth + 0.5 * step
            } else {
                thickness - depth - 0.5 * step
            };
            let source = sun_scattering(from_top / mu_sun, cos_theta);
            let through = (-step).exp();
            light += transmittance * source * (1.0 - through);
            transmittance *= through;
            depth += step;
        }
        light
    }

    /// Two-stream theory of a thick layer of conservative scatterers (Bohren, "Multiple
    /// scattering of light and some of its observable consequences", Am. J. Phys. 55,
    /// 1987): it lets through T = 1 / (1 + ¾ (1 − g) τ) and returns R = 1 − T; seen as a
    /// Lambertian surface lit at μ, the radiance is R μ / π above and T μ / π below.
    fn two_stream(thickness: f32, mu_sun: f32) -> (f32, f32) {
        let through = 1.0 / (1.0 + 0.75 * (1.0 - DROPLET_G) * thickness);
        ((1.0 - through) * mu_sun / PI, through * mu_sun / PI)
    }

    fn noon() -> f32 {
        58f32.to_radians().sin()
    }

    #[test]
    fn a_thick_layer_reflects_and_transmits_as_two_stream_theory() {
        // Calibrated on τ = 50: a cumulus a few hundred metres thick.
        let (top, base) = two_stream(50.0, noon());
        assert!((layer_radiance(50.0, noon(), true) / top - 1.0).abs() < 0.1);
        assert!((layer_radiance(50.0, noon(), false) / base - 1.0).abs() < 0.05);
        // Through other thicknesses the base holds; the top, which the shader cannot
        // know is thin, does not fall with the thickness.
        for thickness in [5.0, 20.0, 100.0] {
            let (top, base) = two_stream(thickness, noon());
            let below = layer_radiance(thickness, noon(), false);
            assert!(
                (below / base - 1.0).abs() < 0.25,
                "τ {thickness}: base {below} against {base}"
            );
            if thickness >= 20.0 {
                let above = layer_radiance(thickness, noon(), true);
                assert!(
                    (above / top - 1.0).abs() < 0.35,
                    "τ {thickness}: top {above} against {top}"
                );
            }
        }
    }

    #[test]
    fn under_an_even_sky_a_thick_layer_returns_its_two_stream_share() {
        // Under a sky of even radiance 1, without sun or ground, seen from above: two-stream
        // theory returns R = 1 − T of the incoming light.
        let thickness = 50.0;
        let (mut light, mut transmittance, mut depth) = (0.0, 1.0, 0.0);
        let step = 0.01;
        while depth < thickness && transmittance > 1e-4 {
            let source = ambient_scattering(1.0, 0.0, depth + 0.5 * step, 0.0);
            let through = (-step).exp();
            light += transmittance * source * (1.0 - through);
            transmittance *= through;
            depth += step;
        }
        let reflected = 1.0 - 1.0 / (1.0 + 0.75 * (1.0 - DROPLET_G) * thickness);
        assert!(
            (light / reflected - 1.0).abs() < 0.05,
            "{light} against {reflected}"
        );
    }

    #[test]
    fn the_lit_top_is_four_to_six_times_brighter_than_the_base() {
        let ratio = layer_radiance(50.0, noon(), true) / layer_radiance(50.0, noon(), false);
        assert!((4.0..6.5).contains(&ratio), "ratio {ratio}");
    }

    #[test]
    fn a_bud_in_its_own_shadow_is_two_to_three_times_darker() {
        // A lit face is about 0.3 optical depth from the sun; a bud of 50 m behind its
        // neighbour, about 5 at the cloud's extinction (art director's second review:
        // hollows two to three times darker than the lit buds).
        let ratio = multiple(0.3) / multiple(5.0);
        assert!((2.0..3.0).contains(&ratio), "ratio {ratio}");
    }

    #[test]
    fn the_light_falls_through_the_cloud() {
        for cos_theta in [-0.9, -0.3, 0.0, 0.5, 0.95] {
            let depths = [0.0, 1.0, 3.0, 10.0, 30.0, 100.0];
            let light: Vec<f32> = depths
                .iter()
                .map(|d| sun_scattering(*d, cos_theta))
                .collect();
            assert!(light.windows(2).all(|w| w[0] > w[1]), "{light:?}");
        }
    }

    #[test]
    fn the_phase_function_integrates_to_one() {
        // ∫ p dΩ = 2π ∫ p(μ) dμ over μ from −1 to 1, by the midpoint rule, fine enough for
        // the narrow peak (0.3°, 1 − μ ≈ 1.4e-5).
        let n = 2_000_000;
        let sum: f32 = (0..n)
            .map(|i| phase(-1.0 + (i as f32 + 0.5) * 2.0 / n as f32) * 2.0 / n as f32)
            .sum();
        assert!((2.0 * PI * sum - 1.0).abs() < 1e-3, "{}", 2.0 * PI * sum);
    }

    #[test]
    fn the_peak_is_jendersie_and_d_eon_s_for_cumulus_droplets() {
        let d: f32 = 15.0;
        let g = (-0.0990567 / (d - 1.67154)).exp();
        let share = 1.0 - (-0.599085 / (d - 0.641583) - 0.665888).exp();
        assert!(g > PEAK_G && g - PEAK_G < 0.005, "{g}");
        assert!((share - PEAK_WEIGHT).abs() < 0.02, "{share}");
        // Within 2° of the sun, the peak outshines the two lobes more than ten times; at 20°,
        // it adds less than a tenth.
        let lobes = |deg: f32| {
            let c = deg.to_radians().cos();
            henyey_greenstein(c, BACKWARD_G) * (1.0 - FORWARD_WEIGHT)
                + henyey_greenstein(c, FORWARD_G) * FORWARD_WEIGHT
        };
        let peak = |deg: f32| henyey_greenstein(deg.to_radians().cos(), PEAK_G);
        assert!(peak(2.0) > 10.0 * lobes(2.0));
        assert!(peak(20.0) < 0.1 * lobes(20.0));
    }

    #[test]
    fn against_the_sun_the_edges_shine() {
        // The silver lining: looking towards the sun, single scattering outshines the rest.
        assert!(phase(0.98) > 10.0 * phase(-0.5));
    }
}
