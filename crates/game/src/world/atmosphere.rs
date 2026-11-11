//! The air of the plain (plan `m3-atmosphere.md`): Bevy's Earth atmosphere, with its
//! aerosols set by the visibility at the ground.
//!
//! Bevy 0.20's `ScatteringMedium::earth` (and 0.20.1's) has two faults that change the
//! sky, fixed upstream after 0.20.1 (bevyengine/bevy#26143, merged on 2026-10-26):
//! - its aerosols absorb 3.996e-6 m⁻¹ and scatter 0.444e-6 m⁻¹, the reverse of Bevy 0.16
//!   and of Hillaire ("A Scalable and Production Ready Sky and Atmosphere Rendering
//!   Technique", EGSR 2020): the haze took most of the light it met;
//! - its scale heights are written for a 60 km atmosphere when `Atmosphere::earth` is
//!   100 km thick, 13.3 km for the air and 2 km for the aerosols instead of 8 and 1.2.
//!
//! [`medium`] is that fixed medium, built here until the next version of Bevy brings it.
//! The fix's third point stays a rule: Bevy indexes the scattering table by the wrong
//! resolution, so both resolutions stay equal ([`RESOLUTION`]).

use bevy::curve::{FunctionCurve, Interval};
use bevy::light::atmosphere::{Falloff, PhaseFunction, ScatteringMedium, ScatteringTerm};
use bevy::prelude::*;

/// Thickness of `Atmosphere::earth`, m.
pub const ATMOSPHERE_HEIGHT_M: f32 = 100_000.0;
/// Scale heights of the air and of the aerosols, m (Hillaire 2020).
pub const AIR_SCALE_HEIGHT_M: f32 = 8_000.0;
pub const AEROSOL_SCALE_HEIGHT_M: f32 = 1_200.0;
/// Scattering of the air at sea level, red, green and blue, m⁻¹ (Hillaire 2020).
pub const AIR_SCATTERING: Vec3 = Vec3::new(5.802e-6, 13.558e-6, 33.100e-6);
/// The aerosols of Hillaire 2020 at sea level, m⁻¹: scattering and absorption, a single
/// scattering albedo of 0.9. A very clear air: about 200 km of visibility.
pub const CLEAR_AEROSOL_SCATTERING: f32 = 3.996e-6;
pub const CLEAR_AEROSOL_ABSORPTION: f32 = 0.444e-6;
/// Contrast threshold of the eye in Koschmieder's law, 2 %: a dark object vanishes into
/// the haze at the visibility V where the extinction β gives e^(−β V) = 0.02, V = 3.912 / β.
pub const KOSCHMIEDER: f32 = 3.912;
/// Resolution of the medium's tables, the same for its heights and its phase: about 200 m
/// a step over the atmosphere, and the phase within 0.3° of the sun.
pub const RESOLUTION: u32 = 512;

/// Visibility at the ground on a fine summer day of 1917, m, until the research note
/// gives it (`ESTIMÉ`: plan `m3-atmosphere.md`, decision 1, 40 to 60 km).
pub const SUMMER_VISIBILITY_M: f32 = 50_000.0;

/// Phase of the aerosols, two Henyey-Greenstein lobes: a narrow peak, its share, and the
/// body, their mean cosine kept at Hillaire's 0.8. Bevy's single lobe (g = 0.8) spread the
/// sun's aureole over tens of degrees: under the summer haze, the sky was only 1.4 times
/// brighter at 1° from the sun than at 10°. The aureole of Buie, Monger and Dey
/// ("Sunshape distributions for terrestrial solar simulations", Solar Energy 74, 2003;
/// formulas as in Tonatiuh's `SunshapeBuie.cpp`) falls as θ^γ beyond the sun's disc, with
/// γ = −2.3 for a circumsolar ratio of 0.05. Fitted to it from 0.5 to 2.5° (within 7 %),
/// thin single scattering, a ground observer under [`SUMMER_VISIBILITY_M`] and a sun 30°
/// high (`ESTIMÉ`: the ratio of 0.05 for that haze). Beyond 30° from the sun, the sky is
/// as with one lobe.
pub const AEROSOL_PEAK_G: f32 = 0.991;
pub const AEROSOL_PEAK_SHARE: f32 = 0.33;
pub const AEROSOL_BODY_G: f32 = 0.706;

fn henyey_greenstein(cos_theta: f32, g: f32) -> f32 {
    let g2 = g * g;
    (1.0 - g2) / (4.0 * std::f32::consts::PI * (1.0 + g2 - 2.0 * g * cos_theta).powf(1.5))
}

/// Phase of the aerosols per steradian, `cos_theta` the cosine of the angle from the sun.
pub fn aerosol_phase(cos_theta: f32) -> f32 {
    AEROSOL_PEAK_SHARE * henyey_greenstein(cos_theta, AEROSOL_PEAK_G)
        + (1.0 - AEROSOL_PEAK_SHARE) * henyey_greenstein(cos_theta, AEROSOL_BODY_G)
}

/// How many times the aerosols of Hillaire's clear air give `visibility_m` at the ground,
/// in green light, with the air's own scattering: none below the air's own visibility.
pub fn aerosol_factor(visibility_m: f32) -> f32 {
    let extinction = KOSCHMIEDER / visibility_m;
    let aerosols = extinction - AIR_SCATTERING.y;
    (aerosols / (CLEAR_AEROSOL_SCATTERING + CLEAR_AEROSOL_ABSORPTION)).max(0.0)
}

/// Bevy's Earth medium, fixed, with the aerosols of `visibility_m` at the ground.
pub fn medium(visibility_m: f32) -> ScatteringMedium {
    let factor = aerosol_factor(visibility_m);
    ScatteringMedium::new(
        RESOLUTION,
        RESOLUTION,
        [
            ScatteringTerm {
                absorption: Vec3::ZERO,
                scattering: AIR_SCATTERING,
                falloff: Falloff::Exponential {
                    scale: AIR_SCALE_HEIGHT_M / ATMOSPHERE_HEIGHT_M,
                },
                phase: PhaseFunction::Rayleigh,
            },
            ScatteringTerm {
                absorption: Vec3::splat(CLEAR_AEROSOL_ABSORPTION * factor),
                scattering: Vec3::splat(CLEAR_AEROSOL_SCATTERING * factor),
                falloff: Falloff::Exponential {
                    scale: AEROSOL_SCALE_HEIGHT_M / ATMOSPHERE_HEIGHT_M,
                },
                phase: PhaseFunction::from_curve(FunctionCurve::new(
                    Interval::EVERYWHERE,
                    aerosol_phase,
                )),
            },
            // Ozone, as Bevy's: its layer was already written for 100 km.
            ScatteringTerm {
                absorption: Vec3::new(0.650e-6, 1.881e-6, 0.085e-6),
                scattering: Vec3::ZERO,
                falloff: Falloff::Tent {
                    center: 0.75,
                    width: 0.3,
                },
                phase: PhaseFunction::Isotropic,
            },
        ],
    )
    .with_label("escadrille_atmosphere")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Extinction at the ground in green light, m⁻¹.
    fn ground_extinction(m: &ScatteringMedium) -> f32 {
        m.terms
            .iter()
            .map(|t| (t.scattering.y + t.absorption.y) * t.falloff.sample(1.0))
            .sum()
    }

    #[test]
    fn the_haze_gives_the_visibility_asked() {
        for visibility in [20_000.0, SUMMER_VISIBILITY_M, 100_000.0] {
            let m = medium(visibility);
            let seen = KOSCHMIEDER / ground_extinction(&m);
            assert!(
                (seen / visibility - 1.0).abs() < 1e-3,
                "{visibility}: {seen}"
            );
        }
        // Clearer than the air alone allows, no aerosols at all.
        assert_eq!(aerosol_factor(1e6), 0.0);
    }

    #[test]
    fn the_aerosols_scatter_nine_tenths_of_what_they_meet() {
        let m = medium(SUMMER_VISIBILITY_M);
        let aerosols = &m.terms[1];
        let albedo = aerosols.scattering.x / (aerosols.scattering.x + aerosols.absorption.x);
        assert!((albedo - 0.9).abs() < 1e-3, "{albedo}");
    }

    #[test]
    fn air_and_haze_thin_out_over_their_scale_heights() {
        let m = medium(SUMMER_VISIBILITY_M);
        // At a scale height above the ground, the density has fallen by e.
        for (term, height) in [
            (&m.terms[0], AIR_SCALE_HEIGHT_M),
            (&m.terms[1], AEROSOL_SCALE_HEIGHT_M),
        ] {
            let p = 1.0 - height / ATMOSPHERE_HEIGHT_M;
            let ratio = term.falloff.sample(p) / term.falloff.sample(1.0);
            assert!((ratio - (-1.0f32).exp()).abs() < 2e-3, "{height}: {ratio}");
        }
    }

    #[test]
    fn the_aerosols_scatter_all_they_take_once_with_hillaire_s_mean_cosine() {
        let n = 200_000;
        let du = 2.0 / n as f32;
        let (mut total, mut mean_cosine) = (0.0, 0.0);
        for i in 0..n {
            let u = -1.0 + (i as f32 + 0.5) * du;
            let p = aerosol_phase(u) * 2.0 * std::f32::consts::PI * du;
            total += p;
            mean_cosine += u * p;
        }
        assert!((total - 1.0).abs() < 2e-3, "{total}");
        assert!((mean_cosine - 0.8).abs() < 2e-3, "{mean_cosine}");
    }

    #[test]
    fn the_aureole_falls_as_buie_s_sunshape() {
        // Log slope of the phase from 0.5 to 2.5° from the sun, against Buie's γ for a
        // circumsolar ratio of 0.05: γ = 2.2 ln(0.52 × 0.05) × 0.05^0.43 − 0.1.
        let csr: f32 = 0.05;
        let gamma = 2.2 * (0.52 * csr).ln() * csr.powf(0.43) - 0.1;
        let at = |deg: f32| aerosol_phase(deg.to_radians().cos());
        let slope = (at(2.5) / at(0.5)).ln() / 5f32.ln();
        assert!((slope - gamma).abs() < 0.25, "{slope} against {gamma}");
        // Away from the sun, the sky of a single lobe of 0.8.
        let single = henyey_greenstein(30f32.to_radians().cos(), 0.8);
        assert!(
            (at(30.0) / single - 1.0).abs() < 0.2,
            "{} against {single}",
            at(30.0)
        );
    }

    #[test]
    fn the_tables_are_square() {
        let m = medium(SUMMER_VISIBILITY_M);
        assert_eq!(m.falloff_resolution, m.phase_resolution);
    }
}
