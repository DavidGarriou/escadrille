//! Aerofoil files (`assets/airfoils/<id>.ron`): wind-tunnel tables as the source prints
//! them, converted on loading to the section polar the flight model expects (absolute
//! coefficients, two-dimensional, moment about the quarter chord).
//!
//! Period tables need up to three conversions (aerodynamicist review of 2026-10-20):
//! - **halved coefficients**: British reports (k_L, k_D, k_m) and some early NACA reports
//!   give half the absolute coefficients;
//! - **moment about the leading edge**: C_m,c/4 = C_m,LE + C_N / 4, with
//!   C_N = C_L cos α + C_D sin α;
//! - **finite-span model**: tables measured on a wing of aspect ratio A are 3D. The
//!   flight model adds its own induced angle, so they are brought back to infinite span
//!   with Glauert's corrections (*The Elements of Aerofoil and Airscrew Theory*, 1926):
//!   α∞ = α − C_L (1 + τ) / (π A), C_D∞ = C_D − C_L² (1 + δ) / (π A). Applied to every
//!   point, past the stall too (an approximation, E:).

use escadrille_fdm::aero::polar::{Polar, PolarPoint};
use serde::Deserialize;

use crate::units::Angle;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Convention {
    Absolute,
    /// Half the absolute coefficients (British k_L, k_D, k_m).
    Halved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum MomentReference {
    QuarterChord,
    LeadingEdge,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct AirfoilPoint {
    pub alpha: Angle,
    pub cl: f64,
    pub cd: f64,
    /// Pitching moment, positive nose up.
    pub cm: f64,
}

/// Wind-tunnel model of finite span.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub struct TestWing {
    pub aspect_ratio: f64,
    /// Glauert's correction on the induced angle (C: from the planform).
    pub tau: f64,
    /// Glauert's correction on the induced drag (C: from the planform).
    pub delta: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename = "Airfoil")]
pub struct AirfoilFile {
    pub name: String,
    /// Where the table comes from: research sheet and source number.
    pub source: String,
    pub convention: Convention,
    pub moment_reference: MomentReference,
    /// `None` for section (two-dimensional) data.
    pub test_wing: Option<TestWing>,
    pub points: Vec<AirfoilPoint>,
}

impl AirfoilFile {
    /// Section points: absolute, two-dimensional, moment about the quarter chord.
    pub fn section_points(&self) -> Vec<PolarPoint> {
        let scale = match self.convention {
            Convention::Absolute => 1.0,
            Convention::Halved => 2.0,
        };
        self.points
            .iter()
            .map(|p| {
                let alpha = p.alpha.si();
                let (cl, cd) = (p.cl * scale, p.cd * scale);
                let mut cm = p.cm * scale;
                if self.moment_reference == MomentReference::LeadingEdge {
                    cm += 0.25 * (cl * alpha.cos() + cd * alpha.sin());
                }
                match self.test_wing {
                    Some(w) => {
                        let k = 1.0 / (std::f64::consts::PI * w.aspect_ratio);
                        PolarPoint {
                            alpha: alpha - cl * (1.0 + w.tau) * k,
                            cl,
                            cd: cd - cl * cl * (1.0 + w.delta) * k,
                            cm,
                        }
                    }
                    None => PolarPoint { alpha, cl, cd, cm },
                }
            })
            .collect()
    }

    /// Section polar with the given broadside drag coefficient.
    pub fn polar(&self, cd_max: f64) -> Result<Polar, String> {
        Polar::new(self.section_points(), cd_max)
            .map_err(|e| format!("aerofoil {}: {e:?}", self.name))
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;

    /// Lifting-line 3D table of a 2π section on an AR 6 model (synthetic).
    fn measured(convention: Convention, reference: MomentReference) -> AirfoilFile {
        let (a, tau) = (6.0, 0.17);
        let slope = 2.0 * PI / (1.0 + 2.0 * PI * (1.0 + tau) / (PI * a));
        let scale = if convention == Convention::Halved {
            0.5
        } else {
            1.0
        };
        let points = (-4..=12)
            .map(|deg: i32| {
                let alpha = f64::from(deg).to_radians();
                let cl = slope * alpha;
                let cd = 0.01 + cl * cl * (1.0 + 0.05) / (PI * a);
                let cm_c4 = -0.05;
                let cm = match reference {
                    MomentReference::QuarterChord => cm_c4,
                    MomentReference::LeadingEdge => {
                        cm_c4 - 0.25 * (cl * alpha.cos() + cd * alpha.sin())
                    }
                };
                AirfoilPoint {
                    alpha: Angle::Deg(f64::from(deg)),
                    cl: cl * scale,
                    cd: cd * scale,
                    cm: cm * scale,
                }
            })
            .collect();
        AirfoilFile {
            name: "synthetic".into(),
            source: "test".into(),
            convention,
            moment_reference: reference,
            test_wing: Some(TestWing {
                aspect_ratio: a,
                tau,
                delta: 0.05,
            }),
            points,
        }
    }

    #[test]
    fn three_dimensional_tables_come_back_to_the_section() {
        for convention in [Convention::Absolute, Convention::Halved] {
            for reference in [MomentReference::QuarterChord, MomentReference::LeadingEdge] {
                for p in measured(convention, reference).section_points() {
                    assert!((p.cl - 2.0 * PI * p.alpha).abs() < 1e-12, "{p:?}");
                    assert!((p.cd - 0.01).abs() < 1e-12, "{p:?}");
                    assert!((p.cm + 0.05).abs() < 1e-12, "{p:?}");
                }
            }
        }
    }

    #[test]
    fn section_data_is_kept_as_is() {
        let mut file = measured(Convention::Absolute, MomentReference::QuarterChord);
        file.test_wing = None;
        let p = file.section_points()[4];
        assert_eq!(p.alpha, 0.0);
        assert_eq!(p.cd, 0.01);
    }

    #[test]
    fn a_table_without_stall_is_reported() {
        let file = measured(Convention::Absolute, MomentReference::QuarterChord);
        let err = file.polar(1.2).unwrap_err();
        assert!(err.contains("NoStallInTable"), "{err}");
    }
}
