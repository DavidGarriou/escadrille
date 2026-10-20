use std::f64::consts::PI;

use super::*;
use crate::aero::polar::{Polar, PolarPoint, viterna_cd_max};
use crate::aero::surface::{InducedModel, Strip};

const RHO: f64 = 1.225;
const V: f64 = 50.0;
const A0: f64 = 2.0 * PI;

const NO_CONTROLS: ControlDeflections = ControlDeflections {
    aileron: 0.0,
    elevator: 0.0,
    rudder: 0.0,
};

/// Thin-aerofoil textbook section (not historical data): C_L = 2πα up to ±14°, then a
/// stall to 80 % of the peak at ±20°; no profile drag, so that all drag is induced.
fn section() -> Polar {
    let peak = A0 * 14f64.to_radians();
    let points = (-20..=20)
        .map(|deg: i32| {
            let alpha = f64::from(deg).to_radians();
            let a = deg.unsigned_abs();
            let cl = if a <= 14 {
                A0 * alpha
            } else {
                alpha.signum() * peak * (1.0 - 0.2 * f64::from(a - 14) / 6.0)
            };
            PolarPoint {
                alpha,
                cl,
                cd: 0.0,
                cm: 0.0,
            }
        })
        .collect();
    Polar::new(points, viterna_cd_max(6.0)).unwrap()
}

/// Rectangular plane of span `span` and chord `chord`, centred at `centre`, e = 1.
fn plane(span: f64, chord: f64, centre: DVec3) -> LiftingSurface {
    let strips = 4;
    let width = span / f64::from(strips);
    let strips = (0..strips)
        .map(|i| Strip {
            position: centre + DVec3::new(0.0, -0.5 * span + (f64::from(i) + 0.5) * width, 0.0),
            chord_axis: DVec3::X,
            normal: DVec3::NEG_Z,
            area: width * chord,
            chord,
            control: None,
        })
        .collect();
    let induced = InducedModel::new(span / chord, 1.0);
    LiftingSurface::new("plane".into(), strips, section(), induced).unwrap()
}

/// Prandtl's mutual factors for planes of spans b and areas S, interference σ.
fn biplane(upper: (f64, f64), lower: (f64, f64), sigma: f64) -> LiftingSystem {
    let ((b1, c1), (b2, c2)) = (upper, lower);
    let (s1, s2) = (b1 * c1, b2 * c2);
    let surfaces = vec![
        plane(b1, c1, DVec3::new(0.3, 0.0, -0.8)),
        plane(b2, c2, DVec3::new(0.0, 0.0, 0.6)),
    ];
    let mutual = vec![
        MutualInduction {
            on: 0,
            from: 1,
            factor: sigma * s2 / (PI * b1 * b2),
        },
        MutualInduction {
            on: 1,
            from: 0,
            factor: sigma * s1 / (PI * b1 * b2),
        },
    ];
    LiftingSystem::new(surfaces, mutual).unwrap()
}

fn flight(alpha: f64) -> DVec3 {
    DVec3::new(V * alpha.cos(), 0.0, V * alpha.sin())
}

fn still(_: usize, _: DVec3) -> DVec3 {
    DVec3::ZERO
}

#[test]
fn equal_planes_follow_prandtl_closed_form() {
    // Linear range: x + k (1 + σ) a₀ x = α, so C_L = a₀ α / (1 + a₀ k (1 + σ)).
    let sigma = 0.5;
    let system = biplane((8.0, 1.0), (8.0, 1.0), sigma);
    let k = 1.0 / (PI * 8.0);
    for deg in [-6.0, 2.0, 4.0, 8.0] {
        let alpha = f64::to_radians(deg);
        let loads = system.loads(flight(alpha), DVec3::ZERO, RHO, &NO_CONTROLS, still);
        let expected = A0 * alpha / (1.0 + A0 * k * (1.0 + sigma));
        for (cl, angle) in loads.lift_coefficients.iter().zip(&loads.mutual_angles) {
            assert!((cl - expected).abs() < 1e-12, "{deg}°: {cl} vs {expected}");
            assert!((angle - sigma * k * expected).abs() < 1e-14);
        }
    }
}

#[test]
fn induced_drag_of_unequal_planes_matches_prandtl() {
    let sigma = 0.45;
    let (upper, lower) = ((8.0, 1.2), (7.0, 1.0));
    let system = biplane(upper, lower, sigma);
    let alpha = 5f64.to_radians();
    let loads = system.loads(flight(alpha), DVec3::ZERO, RHO, &NO_CONTROLS, still);
    let (c1, c2) = (loads.lift_coefficients[0], loads.lift_coefficients[1]);
    let q = 0.5 * RHO * V * V;
    let (b1, b2) = (upper.0, lower.0);
    let (l1, l2) = (q * b1 * upper.1 * c1, q * b2 * lower.1 * c2);

    // Each plane solves x + k_ii C_L(x) + k_ij C_L,j = α exactly (linear range: x = C_L / a₀).
    let k11 = 1.0 / (PI * b1 / upper.1);
    let k22 = 1.0 / (PI * b2 / lower.1);
    let m = system.mutual();
    assert!((c1 / A0 + k11 * c1 + m[0].factor * c2 - alpha).abs() < 1e-12);
    assert!((c2 / A0 + k22 * c2 + m[1].factor * c1 - alpha).abs() < 1e-12);

    // Drag in wind axes against Prandtl's formula (small-angle theory: 0.1 %).
    let d = flight(alpha).normalize();
    let drag = -loads.wrench.force.dot(d);
    let prandtl =
        (l1 * l1 / (b1 * b1) + 2.0 * sigma * l1 * l2 / (b1 * b2) + l2 * l2 / (b2 * b2)) / (PI * q);
    assert!(
        (drag - prandtl).abs() < 1e-3 * prandtl,
        "{drag} vs {prandtl}"
    );
    // The interference costs drag compared with two isolated planes.
    let isolated = (l1 * l1 / (b1 * b1) + l2 * l2 / (b2 * b2)) / (PI * q);
    assert!(drag > isolated);
}

#[test]
fn without_mutual_induction_the_surfaces_are_independent() {
    let surfaces = vec![
        plane(8.0, 1.2, DVec3::new(0.3, 0.0, -0.8)),
        plane(7.0, 1.0, DVec3::new(0.0, 0.0, 0.6)),
    ];
    let system = LiftingSystem::new(surfaces.clone(), vec![]).unwrap();
    let (v, omega) = (DVec3::new(48.0, 3.0, 4.0), DVec3::new(0.2, -0.1, 0.3));
    let loads = system.loads(v, omega, RHO, &NO_CONTROLS, still);
    let sum = surfaces
        .iter()
        .map(|s| s.loads(v, omega, RHO, &NO_CONTROLS, |_| DVec3::ZERO))
        .fold(Wrench::default(), |a, b| a + b);
    assert_eq!(loads.wrench, sum);
    assert_eq!(loads.mutual_angles, vec![0.0, 0.0]);
}

#[test]
fn biplane_loads_are_continuous_over_the_whole_circle() {
    let system = biplane((8.0, 1.2), (7.0, 1.0), 0.5);
    let n = 36_000;
    let at = |alpha: f64| {
        let loads = system.loads(flight(alpha), DVec3::ZERO, RHO, &NO_CONTROLS, still);
        let q = 0.5 * RHO * V * V;
        (loads.wrench.force / q, loads.wrench.moment / q)
    };
    let first = at(-PI + 1e-12);
    let mut previous = first;
    for i in 1..=n {
        let alpha = -PI + 2.0 * PI * f64::from(i) / f64::from(n);
        let c = at(alpha);
        assert!(c.0.is_finite() && c.1.is_finite());
        let jump = (c.0 - previous.0).length().max((c.1 - previous.1).length());
        assert!(jump < 0.05, "{:.3}°: jump {jump}", alpha.to_degrees());
        previous = c;
    }
    let jump = (previous.0 - first.0)
        .length()
        .max((previous.1 - first.1).length());
    assert!(jump < 1e-6, "jump across 180°: {jump}");
}

#[test]
fn mirrored_flight_mirrors_the_lateral_loads() {
    let system = biplane((8.0, 1.2), (7.0, 1.0), 0.5);
    let flip = |v: DVec3| DVec3::new(v.x, -v.y, v.z);
    let flip_rates = |w: DVec3| DVec3::new(-w.x, w.y, -w.z);
    let (v, omega) = (DVec3::new(48.0, 5.0, 4.0), DVec3::new(0.4, 0.1, -0.2));
    let a = system.loads(v, omega, RHO, &NO_CONTROLS, still).wrench;
    let b = system
        .loads(flip(v), flip_rates(omega), RHO, &NO_CONTROLS, still)
        .wrench;
    let tolerance = 1e-9 * a.force.length();
    assert!(
        (a.force - flip(b.force)).length() < tolerance,
        "{a:?} {b:?}"
    );
    assert!(
        (a.moment - flip_rates(b.moment)).length() < tolerance,
        "{a:?} {b:?}"
    );
}

#[test]
fn rejects_inconsistent_pairs() {
    let surfaces = || {
        vec![
            plane(8.0, 1.2, DVec3::new(0.3, 0.0, -0.8)),
            plane(7.0, 1.0, DVec3::new(0.0, 0.0, 0.6)),
        ]
    };
    let pair = |on, from, factor| MutualInduction { on, from, factor };
    let check = |m: Vec<MutualInduction>| LiftingSystem::new(surfaces(), m).unwrap_err();
    assert_eq!(check(vec![pair(0, 2, 0.01)]), SystemError::BadPair);
    assert_eq!(check(vec![pair(1, 1, 0.01)]), SystemError::BadPair);
    assert_eq!(
        check(vec![pair(0, 1, 0.01), pair(0, 1, 0.02)]),
        SystemError::BadPair
    );
    assert_eq!(check(vec![pair(0, 1, f64::NAN)]), SystemError::BadFactor);
    assert_eq!(check(vec![pair(0, 1, -0.01)]), SystemError::BadFactor);

    let mut no_induction = surfaces();
    let strips = no_induction[1].strips().to_vec();
    no_induction[1] = LiftingSurface::new("tail".into(), strips, section(), None).unwrap();
    assert_eq!(
        LiftingSystem::new(no_induction, vec![pair(0, 1, 0.01)]),
        Err(SystemError::NoInducedModel)
    );
}
