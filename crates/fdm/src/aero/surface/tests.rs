use super::*;
use crate::aero::polar::{PolarPoint, viterna_cd_max};
use std::f64::consts::PI;

const RHO: f64 = 1.225;
const V: f64 = 50.0;

/// Thin-aerofoil textbook section (not historical data).
fn thin_aerofoil(cd0: f64) -> Polar {
    let points = (-15..=15)
        .map(|deg| {
            let alpha = f64::from(deg).to_radians();
            PolarPoint {
                alpha,
                cl: 2.0 * PI * alpha,
                cd: cd0,
                cm: 0.0,
            }
        })
        .collect();
    Polar::new(points, viterna_cd_max(6.0)).unwrap()
}

/// Rectangular surface in the x–y plane, centred at `centre`, strips across the span.
fn wing(span: f64, chord: f64, strips: usize, incidence: f64, centre: DVec3) -> LiftingSurface {
    let chord_axis = DVec3::new(incidence.cos(), 0.0, -incidence.sin());
    let normal = DVec3::new(-incidence.sin(), 0.0, -incidence.cos());
    let width = span / strips as f64;
    let strips = (0..strips)
        .map(|i| Strip {
            position: centre + DVec3::new(0.0, -0.5 * span + (i as f64 + 0.5) * width, 0.0),
            chord_axis,
            normal,
            area: width * chord,
            chord,
            control: None,
        })
        .collect();
    LiftingSurface {
        name: "wing".into(),
        strips,
        polar: thin_aerofoil(0.01),
        induced: Some(InducedModel {
            aspect_ratio: span / chord,
            efficiency: 1.0,
        }),
    }
}

fn flight(alpha: f64) -> DVec3 {
    DVec3::new(V * alpha.cos(), 0.0, V * alpha.sin())
}

/// Lift and drag in wind axes (x–z plane flight).
fn lift_drag(force: DVec3, velocity: DVec3) -> (f64, f64) {
    let d = velocity.normalize();
    let up = DVec3::new(d.z, 0.0, -d.x); // perpendicular to d, pointing up for small α
    (force.dot(up), -force.dot(d))
}

const NO_CONTROLS: ControlDeflections = ControlDeflections {
    aileron: 0.0,
    elevator: 0.0,
    rudder: 0.0,
};

#[test]
fn symmetric_flight_gives_no_lateral_loads() {
    let w = wing(9.0, 1.5, 10, 0.0, DVec3::ZERO);
    let loads = w.loads(flight(0.08), DVec3::ZERO, RHO, &NO_CONTROLS, 0.0);
    assert!(loads.force.y.abs() < 1e-9, "{loads:?}");
    assert!(
        loads.moment.x.abs() < 1e-9 && loads.moment.z.abs() < 1e-9,
        "{loads:?}"
    );
}

#[test]
fn positive_angle_of_attack_lifts_up_and_drags_back() {
    let w = wing(9.0, 1.5, 10, 0.0, DVec3::ZERO);
    let loads = w.loads(flight(0.08), DVec3::ZERO, RHO, &NO_CONTROLS, 0.0);
    assert!(loads.force.z < 0.0, "lift must point up (−z): {loads:?}");
    let (lift, drag) = lift_drag(loads.force, flight(0.08));
    assert!(lift > 0.0 && drag > 0.0);
}

#[test]
fn incidence_lifts_at_zero_body_angle() {
    let w = wing(9.0, 1.5, 10, 2f64.to_radians(), DVec3::ZERO);
    let loads = w.loads(flight(0.0), DVec3::ZERO, RHO, &NO_CONTROLS, 0.0);
    assert!(loads.force.z < 0.0, "{loads:?}");
}

#[test]
fn finite_span_matches_lifting_line_theory() {
    // Elliptic-equivalent wing (e = 1), linear range:
    // C_L = a₀ α / (1 + a₀ / (π AR)),  C_Di = C_L² / (π AR).
    let (span, chord) = (9.0, 1.5);
    let ar = span / chord;
    let w = wing(span, chord, 10, 0.0, DVec3::ZERO);
    let alpha = 4f64.to_radians();
    let loads = w.loads(flight(alpha), DVec3::ZERO, RHO, &NO_CONTROLS, 0.0);
    let (lift, drag) = lift_drag(loads.force, flight(alpha));
    let qs = 0.5 * RHO * V * V * span * chord;
    let a0 = 2.0 * PI;
    let cl_expected = a0 * alpha / (1.0 + a0 / (PI * ar));
    let cl = lift / qs;
    assert!(
        (cl - cl_expected).abs() < 1e-3 * cl_expected,
        "{cl} vs {cl_expected}"
    );
    let cdi = drag / qs - 0.01;
    let cdi_expected = cl * cl / (PI * ar);
    assert!(
        (cdi - cdi_expected).abs() < 2e-3 * cdi_expected,
        "{cdi} vs {cdi_expected}"
    );
}

#[test]
fn rolling_wing_is_damped() {
    let w = wing(9.0, 1.5, 10, 0.0, DVec3::ZERO);
    let roll_right = DVec3::new(0.5, 0.0, 0.0);
    let loads = w.loads(flight(0.05), roll_right, RHO, &NO_CONTROLS, 0.0);
    assert!(loads.moment.x < 0.0, "{loads:?}");
}

#[test]
fn tail_behind_the_centre_of_gravity_damps_pitch_and_stabilises() {
    let tail = wing(3.0, 1.0, 4, 0.0, DVec3::new(-4.0, 0.0, 0.0));
    let pitching_up = DVec3::new(0.0, 0.5, 0.0);
    let damped = tail.loads(flight(0.0), pitching_up, RHO, &NO_CONTROLS, 0.0);
    assert!(damped.moment.y < 0.0, "pitch damping: {damped:?}");
    // More angle of attack → nose-down moment (static stability).
    let stable = tail.loads(flight(0.05), DVec3::ZERO, RHO, &NO_CONTROLS, 0.0);
    assert!(stable.moment.y < 0.0, "static stability: {stable:?}");
}

#[test]
fn downwash_reduces_the_tail_lift() {
    let tail = wing(3.0, 1.0, 4, 0.0, DVec3::new(-4.0, 0.0, 0.0));
    let clean = tail.loads(flight(0.05), DVec3::ZERO, RHO, &NO_CONTROLS, 0.0);
    let washed = tail.loads(flight(0.05), DVec3::ZERO, RHO, &NO_CONTROLS, 0.02);
    assert!(
        washed.force.z > clean.force.z,
        "less upward lift with downwash"
    );
}

#[test]
fn positive_aileron_rolls_right() {
    // Right aileron up (less lift) and left aileron down (more lift) for a positive
    // command, through opposite signs on each side.
    let mut w = wing(9.0, 1.5, 10, 0.0, DVec3::ZERO);
    let tau = flap_effectiveness(0.25);
    for strip in &mut w.strips {
        let side = strip.position.y.signum();
        if strip.position.y.abs() > 2.5 {
            strip.control = Some(StripControl {
                channel: ControlChannel::Aileron,
                alpha_per_rad: -side * tau,
            });
        }
    }
    let controls = ControlDeflections {
        aileron: 0.2,
        ..NO_CONTROLS
    };
    let loads = w.loads(flight(0.05), DVec3::ZERO, RHO, &controls, 0.0);
    assert!(loads.moment.x > 0.0, "{loads:?}");
}

#[test]
fn fin_turns_the_nose_into_a_sideslip() {
    // Air coming from the right (v > 0): a fin behind the CG yaws the nose right.
    let fin = LiftingSurface {
        name: "fin".into(),
        strips: vec![Strip {
            position: DVec3::new(-4.5, 0.0, -0.5),
            chord_axis: DVec3::X,
            normal: DVec3::Y,
            area: 1.0,
            chord: 1.0,
            control: None,
        }],
        polar: thin_aerofoil(0.01),
        induced: Some(InducedModel {
            aspect_ratio: 1.5,
            efficiency: 0.9,
        }),
    };
    let sideslip = DVec3::new(V, 3.0, 0.0);
    let loads = fin.loads(sideslip, DVec3::ZERO, RHO, &NO_CONTROLS, 0.0);
    assert!(loads.moment.z > 0.0, "{loads:?}");
}

#[test]
fn no_force_and_no_nan_at_rest() {
    let w = wing(9.0, 1.5, 10, 0.0, DVec3::ZERO);
    let loads = w.loads(DVec3::ZERO, DVec3::ZERO, RHO, &NO_CONTROLS, 0.0);
    assert_eq!(loads, Wrench::default());
}

#[test]
fn finite_in_any_flight_condition() {
    // Tail first, upside down, broadside: forces stay finite.
    let w = wing(9.0, 1.5, 10, 0.03, DVec3::new(0.2, 0.0, 0.0));
    for i in 0..72 {
        let a = f64::from(i) * 5f64.to_radians();
        let v = DVec3::new(a.cos(), 0.3 * a.sin(), a.sin()) * 40.0;
        let loads = w.loads(v, DVec3::new(0.3, -0.2, 0.1), RHO, &NO_CONTROLS, 0.0);
        assert!(
            loads.force.is_finite() && loads.moment.is_finite(),
            "{a}: {loads:?}"
        );
    }
}

#[test]
fn flap_effectiveness_follows_thin_aerofoil_theory() {
    assert!(flap_effectiveness(0.0).abs() < 1e-12);
    assert!((flap_effectiveness(1.0) - 1.0).abs() < 1e-12);
    let mut previous = 0.0;
    for i in 1..=10 {
        let tau = flap_effectiveness(f64::from(i) / 10.0);
        assert!(tau > previous);
        previous = tau;
    }
    // Quarter-chord flap: θ_f = 120°, τ = 1 − (2π/3 − √3/2) / π ≈ 0.609.
    assert!((flap_effectiveness(0.25) - 0.609).abs() < 1e-3);
}
