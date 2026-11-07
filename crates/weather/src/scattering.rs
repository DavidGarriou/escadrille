//! Light scattered by clouds: the lighting model of the cloud shader
//! (`assets/shaders/clouds/render.wesl`, functions and constants of the same names),
//! written here to test its energy against two-stream theory.
//!
//! The light of the sun that a cloud sends towards the eye, per unit scattering
//! coefficient and per unit solar irradiance, at an optical depth τ from the sun along the
//! sun's direction:
//! - single scattering: the phase function times e^−τ;
//! - multiple scattering, by the diffusion approximation: in a thick cloud the light that
//!   goes on through it falls slowly, as 1 / (1 + k τ), not as e^−τ. Its level `A` and its
//!   fall `k` are set so that a thick layer returns from its top and lets through its base
//!   what two-stream theory gives (test `a_thick_layer_reflects_and_transmits_as_two_stream_theory`).
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

/// Asymmetry of the phase function of cloud droplets (Mie, droplets of about 10 µm in
/// visible light).
pub const DROPLET_G: f32 = 0.85;

/// Level of the multiple scattering, against isotropic scattering of the full sunlight.
pub const DIFFUSE_LEVEL: f32 = 2.8;
/// Fall of the multiple scattering per unit optical depth towards the sun.
pub const DIFFUSE_FALL: f32 = 0.08;

pub fn henyey_greenstein(cos_theta: f32, g: f32) -> f32 {
    let g2 = g * g;
    (1.0 - g2) / (4.0 * PI * (1.0 + g2 - 2.0 * g * cos_theta).powf(1.5))
}

/// Phase function of single scattering, sr⁻¹, for the cosine of the angle between the
/// eye's ray and the direction towards the sun.
pub fn phase(cos_theta: f32) -> f32 {
    let backward = henyey_greenstein(cos_theta, BACKWARD_G);
    let forward = henyey_greenstein(cos_theta, FORWARD_G);
    backward * (1.0 - FORWARD_WEIGHT) + forward * FORWARD_WEIGHT
}

/// Level of the light of the sky and the ground scattered by a cloud, each from its half
/// of the sphere, against isotropic scattering.
pub const AMBIENT_LEVEL: f32 = 1.8;

/// Albedo of the ground under the clouds: fields and meadows. ESTIMÉ: 0.16 to 0.26 for
/// grass and crops (Oke, "Boundary Layer Climates", 2nd ed., 1987, table 1.1), the low side
/// for the dark crops and woods of Flanders.
pub const GROUND_ALBEDO: f32 = 0.18;

/// Diffused share of a light at an optical depth towards it: 1 at the lit surface, falling
/// slowly through the cloud.
pub fn diffused(depth_to_sun: f32) -> f32 {
    1.0 / (1.0 + DIFFUSE_FALL * depth_to_sun)
}

/// Sunlight scattered towards the eye, per unit scattering coefficient and unit solar
/// irradiance, sr⁻¹: single scattering and multiple scattering.
pub fn sun_scattering(depth_to_sun: f32, cos_theta: f32) -> f32 {
    phase(cos_theta) * (-depth_to_sun).exp() + DIFFUSE_LEVEL * diffused(depth_to_sun) / (4.0 * PI)
}

/// Light of the sky (radiance `sky`, from above) and of the ground (radiance `ground`,
/// from below) scattered towards the eye, per unit scattering coefficient, each dimmed by
/// diffusion through the optical depth of cloud on its side.
pub fn ambient_scattering(sky: f32, ground: f32, depth_up: f32, depth_down: f32) -> f32 {
    0.5 * AMBIENT_LEVEL * (sky * diffused(depth_up) + ground * diffused(depth_down))
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
        assert!((layer_radiance(50.0, noon(), true) / top - 1.0).abs() < 0.05);
        assert!((layer_radiance(50.0, noon(), false) / base - 1.0).abs() < 0.05);
        // Through other thicknesses the base holds; the top, which the shader cannot
        // know is thin, does not fall with the thickness.
        for thickness in [5.0, 20.0, 100.0] {
            let (top, base) = two_stream(thickness, noon());
            let below = layer_radiance(thickness, noon(), false);
            assert!(
                (below / base - 1.0).abs() < 0.1,
                "τ {thickness}: base {below} against {base}"
            );
            if thickness >= 20.0 {
                let above = layer_radiance(thickness, noon(), true);
                assert!(
                    (above / top - 1.0).abs() < 0.25,
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
        // ∫ p dΩ = 2π ∫ p(μ) dμ over μ from −1 to 1, by the midpoint rule.
        let n = 20_000;
        let sum: f32 = (0..n)
            .map(|i| phase(-1.0 + (i as f32 + 0.5) * 2.0 / n as f32) * 2.0 / n as f32)
            .sum();
        assert!((2.0 * PI * sum - 1.0).abs() < 1e-3, "{}", 2.0 * PI * sum);
    }

    #[test]
    fn against_the_sun_the_edges_shine() {
        // The silver lining: looking towards the sun, single scattering outshines the rest.
        assert!(phase(0.98) > 10.0 * phase(-0.5));
    }
}
