//! Drag polar of the airframe from steady glides at idle, in the convention of the
//! full-scale glide tests of NACA Report 304: the propeller's own forces are taken out
//! ("true drag"), so that the lift-dependent drag of the model can be compared with a
//! measured aeroplane through its Oswald factor (plan `docs/plans/m1-ecarts-spad7.md`).

use escadrille_fdm::aircraft::Aircraft;
use escadrille_fdm::constants::G0;
use escadrille_fdm::environment::Environment;
use glam::DVec3;

use crate::performance::{PerformanceError, glide, stall_speed};

/// Lift coefficients of the fit, as in Report 304 (attached flow, below the stall and
/// above the steep dives).
pub const FIT_RANGE: (f64, f64) = (0.2, 0.8);

/// Number of glides between 1.1 times the stall speed and the speed of the lowest lift
/// coefficient fitted.
const GLIDES: u32 = 24;

/// The fastest glide aims at this fraction of the lowest fitted lift coefficient, so that
/// the fit range is covered whatever the engine: Report 304 reached it in dives, beyond
/// the maximum level speed.
const FASTEST_GLIDE_CL: f64 = 0.9;

/// Reference of the coefficients: wing area and span, and the biplane factor
/// κ = (1 + σ) / 2 of Prandtl (1 for a monoplane), giving the equivalent aspect ratio
/// b² / (κ S).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Reference {
    pub area: f64,
    pub span: f64,
    pub kappa: f64,
}

impl Reference {
    pub fn equivalent_aspect_ratio(&self) -> f64 {
        self.span * self.span / (self.kappa * self.area)
    }
}

/// One glide: airspeed (m/s) and the airframe's coefficients.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PolarPoint {
    pub speed: f64,
    pub cl: f64,
    pub cd: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AirframePolar {
    pub altitude: f64,
    pub reference: Reference,
    pub points: Vec<PolarPoint>,
    /// C_D = cd0 + k C_L², fitted over [`FIT_RANGE`].
    pub cd0: f64,
    pub k: f64,
    /// Points used by the fit.
    pub fitted: usize,
}

impl AirframePolar {
    /// Oswald factor of the whole airframe: 1 / (π A_eq k).
    pub fn oswald(&self) -> f64 {
        1.0 / (std::f64::consts::PI * self.reference.equivalent_aspect_ratio() * self.k)
    }
}

/// Least squares of y = a + b x; `None` with fewer than two distinct x.
pub fn line_fit(points: &[(f64, f64)]) -> Option<(f64, f64)> {
    let n = points.len() as f64;
    let (sx, sy) = points
        .iter()
        .fold((0.0, 0.0), |(sx, sy), (x, y)| (sx + x, sy + y));
    let (sxx, sxy) = points
        .iter()
        .fold((0.0, 0.0), |(a, b), (x, y)| (a + x * x, b + x * y));
    let det = n * sxx - sx * sx;
    if points.len() < 2 || det.abs() < 1e-12 {
        return None;
    }
    let slope = (n * sxy - sx * sy) / det;
    Some(((sy - slope * sx) / n, slope))
}

/// Lift and drag coefficients of the airframe in a steady glide: aerodynamic force less
/// the propeller's, in wind axes.
fn coefficients(
    aircraft: &Aircraft,
    env: &dyn Environment,
    trim: &crate::trim::Trim,
    reference: &Reference,
) -> PolarPoint {
    let state = trim.flight.state();
    let controls = trim.commands(aircraft);
    let d = aircraft.evaluate_deflected(&state, &trim.flight.deflections, controls.throttle, env);
    let propeller = d.propeller.map_or(DVec3::ZERO, |p| p.wrench.force);
    let force = d.loads.wrench.force - propeller;
    let air = d.air_velocity;
    let speed = air.length();
    let along = air / speed;
    // Lift direction: perpendicular to the airspeed in the plane of symmetry, upwards.
    let alpha = air.z.atan2(air.x);
    let up = DVec3::new(alpha.sin(), 0.0, -alpha.cos());
    let q = 0.5 * d.density * speed * speed * reference.area;
    PolarPoint {
        speed,
        cl: force.dot(up) / q,
        cd: -force.dot(along) / q,
    }
}

/// The airframe polar at `altitude` from glides at idle.
pub fn airframe_polar(
    aircraft: &Aircraft,
    env: &dyn Environment,
    altitude: f64,
    reference: Reference,
) -> Result<AirframePolar, PerformanceError> {
    let low = 1.1 * stall_speed(aircraft, env, altitude)?.speed;
    let density = env.atmosphere(DVec3::new(0.0, 0.0, -altitude)).density;
    let weight = aircraft.mass().mass() * G0;
    let high = (2.0 * weight / (density * reference.area * FASTEST_GLIDE_CL * FIT_RANGE.0))
        .sqrt()
        .max(low);
    let mut points = Vec::new();
    for i in 0..GLIDES {
        let speed = low + (high - low) * f64::from(i) / f64::from(GLIDES - 1);
        if let Ok(trim) = glide(aircraft, env, altitude, speed) {
            points.push(coefficients(aircraft, env, &trim, &reference));
        }
    }
    let fit: Vec<(f64, f64)> = points
        .iter()
        .filter(|p| (FIT_RANGE.0..=FIT_RANGE.1).contains(&p.cl))
        .map(|p| (p.cl * p.cl, p.cd))
        .collect();
    let (cd0, k) = line_fit(&fit).ok_or(PerformanceError::NoStall)?;
    Ok(AirframePolar {
        altitude,
        reference,
        points,
        cd0,
        k,
        fitted: fit.len(),
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use escadrille_data::aircraft::{Loading, load_aircraft};

    use super::*;
    use crate::trim::tests::{STILL_AIR, biplane};

    #[test]
    fn the_line_fit_is_exact_on_a_parabolic_polar() {
        let polar: Vec<(f64, f64)> = (0..10)
            .map(|i| {
                let cl = 0.2 + 0.06 * f64::from(i);
                (cl * cl, 0.05 + 0.1 * cl * cl)
            })
            .collect();
        let (a, b) = line_fit(&polar).unwrap();
        assert!((a - 0.05).abs() < 1e-12 && (b - 0.1).abs() < 1e-12);
        assert!(line_fit(&polar[..1]).is_none());
    }

    #[test]
    fn report_304_gives_an_oswald_factor_of_0_73() {
        // The 21 glides of the Sperry Messenger with RAF 15 wings between C_L 0.2 and
        // 0.8 (Report 304, table II, read on the page image), fitted as the model is:
        // span 20 ft, area 148.3 sq ft (from L / (q C_L)), gap 3 ft 10 in.
        let glides = [
            (0.0605, 0.234),
            (0.0608, 0.233),
            (0.0608, 0.267),
            (0.0642, 0.270),
            (0.0673, 0.327),
            (0.0640, 0.315),
            (0.1137, 0.726),
            (0.0768, 0.380),
            (0.0764, 0.376),
            (0.0818, 0.485),
            (0.0850, 0.505),
            (0.1083, 0.658),
            (0.0986, 0.608),
            (0.0588, 0.240),
            (0.0653, 0.254),
            (0.0679, 0.317),
            (0.0692, 0.366),
            (0.0888, 0.502),
            (0.0956, 0.533),
            (0.1093, 0.675),
            (0.1348, 0.810),
        ];
        let fit: Vec<(f64, f64)> = glides.iter().map(|&(cd, cl)| (cl * cl, cd)).collect();
        let (cd0, k) = line_fit(&fit).unwrap();
        let ft = 0.3048;
        let (span, gap) = (20.0 * ft, (3.0 + 10.0 / 12.0) * ft);
        let g = gap / span;
        // Prandtl, NACA TN 182: σ ≈ (1 − 0.66 G/b) / (1.05 + 3.7 G/b).
        let sigma = (1.0 - 0.66 * g) / (1.05 + 3.7 * g);
        let polar = AirframePolar {
            altitude: 0.0,
            reference: Reference {
                area: 148.3 * ft * ft,
                span,
                kappa: 0.5 * (1.0 + sigma),
            },
            points: Vec::new(),
            cd0,
            k,
            fitted: fit.len(),
        };
        assert!(
            (cd0 - 0.055).abs() < 0.001 && (k - 0.120).abs() < 0.001,
            "{cd0} {k}"
        );
        assert!((polar.oswald() - 0.734).abs() < 0.005, "{}", polar.oswald());
    }

    fn comparison_assets() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("comparison")
    }

    fn game_assets() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets")
    }

    /// The Messenger flown on the RAF 15 table with the given fabric drag.
    fn messenger(fabric_drag_rise: f64) -> AirframePolar {
        let mut data = load_aircraft(&comparison_assets(), "messenger_raf15").unwrap();
        data.airfoils
            .get_mut("airfoils/raf15.ron")
            .unwrap()
            .fabric_drag_rise = fabric_drag_rise;
        let built = data.build(&Loading { fuel_fraction: 1.0 }).unwrap();
        let ft = 0.3048;
        let reference = Reference {
            area: 148.5 * ft * ft,
            span: 20.0 * ft,
            kappa: 0.75,
        };
        airframe_polar(&built.aircraft, &STILL_AIR, 0.0, reference).unwrap()
    }

    #[test]
    fn the_comparison_tables_are_those_of_the_spad() {
        let comparison = load_aircraft(&comparison_assets(), "messenger_raf15").unwrap();
        let spad = load_aircraft(&game_assets(), "spad7").unwrap();
        for (copy, original) in [
            ("airfoils/raf15.ron", "airfoils/spad.ron"),
            ("airfoils/empennage.ron", "airfoils/spad_empennage.ron"),
        ] {
            let (copy, original) = (&comparison.airfoils[copy], &spad.airfoils[original]);
            assert_eq!(copy.points, original.points);
            assert_eq!(copy.test_wing, original.test_wing);
            assert_eq!(copy.fabric_drag_rise, 0.0, "the table as measured");
        }
    }

    #[test]
    fn the_fabric_drag_of_the_spad_wing_reproduces_the_full_scale_messenger() {
        // Report 304: K = 0.120 ± 0.004 over C_L 0.2 to 0.8, C_D0 0.055.
        let polished = messenger(0.0);
        assert!(polished.fitted >= 10, "{polished:?}");
        assert!(polished.oswald() > 0.85, "{}", polished.oswald());
        let spad = load_aircraft(&game_assets(), "spad7").unwrap();
        let fabric = spad.airfoils["airfoils/spad.ron"].fabric_drag_rise;
        let built = messenger(fabric);
        assert!((built.k - 0.120).abs() < 0.002, "{built:?}");
        assert!((built.cd0 - 0.055).abs() < 0.003, "{built:?}");
    }

    #[test]
    fn the_biplane_glide_polar_is_parabolic_with_a_plausible_oswald_factor() {
        let a = biplane();
        let reference = Reference {
            area: 21.2,
            span: 8.0,
            kappa: 0.75,
        };
        let polar = airframe_polar(&a, &STILL_AIR, 1_000.0, reference).unwrap();
        assert!(polar.fitted >= 5, "{polar:?}");
        assert!(polar.cd0 > 0.0 && polar.k > 0.0, "{polar:?}");
        let e = polar.oswald();
        assert!(e > 0.4 && e < 1.2, "{e}");
        // The drag grows with the lift across the glides.
        let mut by_cl = polar.points.clone();
        by_cl.sort_by(|a, b| a.cl.total_cmp(&b.cl));
        assert!(by_cl.last().unwrap().cd > by_cl[0].cd, "{by_cl:?}");
    }
}
