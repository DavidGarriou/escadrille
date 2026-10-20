//! Several tests come from the `aerodynamicien` reviews of 2026-10-20, which injected
//! deliberate errors to find what the suite could not catch.

use super::*;
use crate::aero::polar::{PolarPoint, viterna_cd_max};
use std::f64::consts::PI;

const RHO: f64 = 1.225;
const V: f64 = 50.0;
const A0: f64 = 2.0 * PI;

/// Thin-aerofoil textbook section (not historical data): C_L = 2πα up to ±14°, then a
/// stall to 80 % of the peak at ±20°; constant `cm`.
fn section(cd0: f64, cm: f64) -> Polar {
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
                cd: cd0,
                cm,
            }
        })
        .collect();
    Polar::new(points, viterna_cd_max(6.0)).unwrap()
}

fn still(_: DVec3) -> DVec3 {
    DVec3::ZERO
}

const NO_CONTROLS: ControlDeflections = ControlDeflections {
    aileron: 0.0,
    elevator: 0.0,
    rudder: 0.0,
};

/// Rectangular surface centred at `centre`, panels rising outboard by `dihedral`.
fn wing_with(
    span: f64,
    chord: f64,
    strips: usize,
    incidence: f64,
    dihedral: f64,
    centre: DVec3,
    induced: Option<InducedModel>,
) -> LiftingSurface {
    let chord_axis = DVec3::new(incidence.cos(), 0.0, -incidence.sin());
    let width = span / strips as f64;
    let strips = (0..strips)
        .map(|i| {
            let y = -0.5 * span + (i as f64 + 0.5) * width;
            let side = y.signum();
            // Outboard span axis rises with the dihedral; the normal is up and inboard.
            let span_axis = DVec3::new(0.0, side * dihedral.cos(), -dihedral.sin());
            let normal = (span_axis * side).cross(chord_axis).normalize();
            Strip {
                position: centre + DVec3::new(0.0, y, -y.abs() * dihedral.tan()),
                chord_axis,
                normal,
                area: width * chord,
                chord,
                control: None,
            }
        })
        .collect();
    LiftingSurface {
        name: "wing".into(),
        strips,
        polar: section(0.01, 0.0),
        induced,
    }
}

fn wing(span: f64, chord: f64, strips: usize, incidence: f64, centre: DVec3) -> LiftingSurface {
    let induced = InducedModel::new(span / chord, 1.0);
    wing_with(span, chord, strips, incidence, 0.0, centre, induced)
}

fn fin(aspect_ratio: f64) -> LiftingSurface {
    LiftingSurface {
        name: "fin".into(),
        strips: vec![Strip {
            position: DVec3::new(-4.5, 0.0, -0.5),
            chord_axis: DVec3::X,
            normal: DVec3::Y,
            area: 1.0,
            chord: 1.0,
            control: None,
        }],
        polar: section(0.01, 0.0),
        induced: InducedModel::new(aspect_ratio, 0.9),
    }
}

fn flight(alpha: f64) -> DVec3 {
    DVec3::new(V * alpha.cos(), 0.0, V * alpha.sin())
}

/// Lift and drag in wind axes (x–z plane flight).
fn lift_drag(force: DVec3, velocity: DVec3) -> (f64, f64) {
    let d = velocity.normalize();
    let up = DVec3::new(d.z, 0.0, -d.x);
    (force.dot(up), -force.dot(d))
}

// --- Induced angle -----------------------------------------------------------

#[test]
fn induced_solver_is_exact_for_any_aspect_ratio() {
    // Linear range: x + k a₀ x = α, so C_L = a₀ α / (1 + k a₀), down to fins of AR 1
    // where a fixed-point iteration diverges (k a₀ = 2 / (e AR) > 1).
    let alpha = 4f64.to_radians();
    for ar in [1.0, 1.5, 2.0, 3.5, 6.0, 10.0] {
        let mut s = wing(1.0, 1.0, 1, 0.0, DVec3::ZERO);
        s.induced = InducedModel::new(ar, 1.0);
        let k = s.induced.unwrap().factor();
        let x = s.effective_alpha(alpha, k);
        let residual = x + k * s.polar.coefficients(x).cl - alpha;
        assert!(residual.abs() < 1e-12, "AR {ar}: residual {residual:e}");
        let cl = s.polar.coefficients(x).cl;
        let expected = A0 * alpha / (1.0 + k * A0);
        assert!((cl - expected).abs() < 1e-9, "AR {ar}: {cl} vs {expected}");
    }
}

#[test]
fn induced_angle_is_continuous_and_monotone_over_the_whole_circle() {
    // Low aspect ratio fin: past the stall the equation has several roots; the
    // effective incidence must still vary continuously, without jumps or hysteresis.
    for ar in [1.0, 1.5, 6.0] {
        let mut s = wing(1.0, 1.0, 1, 0.0, DVec3::ZERO);
        s.induced = InducedModel::new(ar, 0.9);
        let k = s.induced.unwrap().factor();
        let mut previous = s.effective_alpha(-PI + 1e-9, k);
        for i in 1..3_600 {
            let alpha = -PI + f64::from(i) * PI / 1_800.0;
            let x = s.effective_alpha(alpha, k);
            assert!(x.is_finite(), "AR {ar}, {alpha}");
            let step = x - previous;
            assert!(
                (0.0..0.01).contains(&step),
                "AR {ar}, {alpha}: {previous} -> {x}"
            );
            previous = x;
        }
    }
}

#[test]
fn rejects_unphysical_induced_models() {
    assert!(InducedModel::new(0.0, 0.8).is_none());
    assert!(InducedModel::new(6.0, 1.2).is_none());
    assert!(InducedModel::new(6.0, 0.8).is_some());
}

#[test]
fn finite_span_matches_lifting_line_theory() {
    // e = 1, linear range: C_L = a₀ α / (1 + a₀ / (π AR)),  C_Di = C_L² / (π AR).
    let (span, chord) = (9.0, 1.5);
    let ar = span / chord;
    let w = wing(span, chord, 10, 0.0, DVec3::ZERO);
    let alpha = 4f64.to_radians();
    let loads = w.loads(flight(alpha), DVec3::ZERO, RHO, &NO_CONTROLS, still);
    let (lift, drag) = lift_drag(loads.force, flight(alpha));
    let qs = 0.5 * RHO * V * V * span * chord;
    let cl_expected = A0 * alpha / (1.0 + A0 / (PI * ar));
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

// --- Signs, symmetry, flow angles ---------------------------------------------

#[test]
fn positive_angle_of_attack_lifts_up_and_drags_back() {
    let w = wing(9.0, 1.5, 10, 0.0, DVec3::ZERO);
    let loads = w.loads(flight(0.08), DVec3::ZERO, RHO, &NO_CONTROLS, still);
    assert!(loads.force.z < 0.0, "lift must point up (−z): {loads:?}");
    let (lift, drag) = lift_drag(loads.force, flight(0.08));
    assert!(lift > 0.0 && drag > 0.0);
}

#[test]
fn incidence_lifts_at_zero_body_angle() {
    let w = wing(9.0, 1.5, 10, 2f64.to_radians(), DVec3::ZERO);
    let loads = w.loads(flight(0.0), DVec3::ZERO, RHO, &NO_CONTROLS, still);
    assert!(loads.force.z < 0.0, "{loads:?}");
}

#[test]
fn mirrored_flight_mirrors_the_lateral_loads() {
    // Wing with dihedral plus a fin: flipping β, p and r flips F_y, L and N only.
    let w = wing_with(
        9.0,
        1.5,
        10,
        0.02,
        3f64.to_radians(),
        DVec3::ZERO,
        InducedModel::new(6.0, 0.85),
    );
    let f = fin(1.5);
    let total = |v: DVec3, omega: DVec3| {
        w.loads(v, omega, RHO, &NO_CONTROLS, still) + f.loads(v, omega, RHO, &NO_CONTROLS, still)
    };
    let (v, omega) = (DVec3::new(48.0, 6.0, 3.0), DVec3::new(0.4, 0.1, -0.3));
    let a = total(v, omega);
    let b = total(
        v * DVec3::new(1.0, -1.0, 1.0),
        omega * DVec3::new(-1.0, 1.0, -1.0),
    );
    let mirror = |x: DVec3| x * DVec3::new(1.0, -1.0, 1.0);
    let mirror_moment = |x: DVec3| x * DVec3::new(-1.0, 1.0, -1.0);
    assert!((b.force - mirror(a.force)).length() < 1e-9, "{a:?} {b:?}");
    assert!(
        (b.moment - mirror_moment(a.moment)).length() < 1e-9,
        "{a:?} {b:?}"
    );
}

#[test]
fn sideslip_reduces_the_dynamic_pressure_by_cos_squared() {
    // On a straight wing the spanwise component is ignored: q ∝ cos²β at fixed α.
    let w = wing(9.0, 1.5, 10, 0.0, DVec3::ZERO);
    let alpha: f64 = 4f64.to_radians();
    let lift_at = |beta: f64| {
        let v = DVec3::new(beta.cos(), beta.sin(), beta.cos() * alpha.tan()) * V;
        -w.loads(v, DVec3::ZERO, RHO, &NO_CONTROLS, still).force.z
    };
    let ratio = lift_at(30f64.to_radians()) / lift_at(0.0);
    let expected = 30f64.to_radians().cos().powi(2);
    assert!((ratio - expected).abs() < 1e-9, "{ratio} vs {expected}");
}

#[test]
fn dihedral_rolls_away_from_the_sideslip() {
    // Air from the right (v > 0): the right wing sees more incidence, so the aircraft
    // rolls left (negative rolling moment).
    let w = wing_with(
        9.0,
        1.5,
        10,
        0.0,
        5f64.to_radians(),
        DVec3::ZERO,
        InducedModel::new(6.0, 0.85),
    );
    let v = DVec3::new(V, 4.0, V * 0.05);
    let loads = w.loads(v, DVec3::ZERO, RHO, &NO_CONTROLS, still);
    assert!(loads.moment.x < 0.0, "{loads:?}");
}

#[test]
fn reversed_flow_force_is_the_flat_plate_normal_force() {
    let w = wing_with(9.0, 1.5, 1, 0.0, 0.0, DVec3::ZERO, None);
    let strip = w.strips[0];
    let alpha = 175f64.to_radians();
    let v = DVec3::new(V * alpha.cos(), 0.0, V * alpha.sin());
    let (loads, state) = w.strip_loads(&strip, v, RHO, &NO_CONTROLS);
    let c = state.coefficients;
    let normal = c.cl * alpha.cos() + c.cd * alpha.sin();
    let expected = 0.5 * RHO * V * V * strip.area * normal;
    assert!((loads.force.dot(strip.normal) - expected).abs() < 1e-9 * expected.abs().max(1.0));
}

#[test]
fn section_moment_acts_about_the_surface_pitch_axis() {
    // cm alone (no lift at α = 0): q S c cm about +y for a wing, +z for a fin.
    let cm = -0.05;
    let mut w = wing_with(2.0, 1.0, 1, 0.0, 0.0, DVec3::ZERO, None);
    w.polar = section(0.0, cm);
    let loads = w.loads(flight(0.0), DVec3::ZERO, RHO, &NO_CONTROLS, still);
    let expected = 0.5 * RHO * V * V * 2.0 * 1.0 * cm;
    assert!((loads.moment.y - expected).abs() < 1e-9, "{loads:?}");
    let mut f = fin(1.5);
    f.strips[0].position = DVec3::ZERO;
    f.polar = section(0.0, cm);
    let loads = f.loads(flight(0.0), DVec3::ZERO, RHO, &NO_CONTROLS, still);
    assert!((loads.moment.z - expected / 2.0).abs() < 1e-9, "{loads:?}");
}

// --- Damping and stability ----------------------------------------------------

#[test]
fn roll_damping_matches_strip_theory_closed_form() {
    // Small roll rate p: each strip gains Δα = p y / V, so with the uniform induced
    // factor L_p = −q (a_eff + C_D0) Σ S y² / V, a_eff = a₀ / (1 + k a₀); the profile
    // drag, tilted with the local flow, also damps (C_D0 = 0.01 here).
    let (span, chord, n) = (9.0, 1.5, 40);
    let w = wing(span, chord, n, 0.0, DVec3::ZERO);
    let p = 0.01;
    let loads = w.loads(
        flight(0.0),
        DVec3::new(p, 0.0, 0.0),
        RHO,
        &NO_CONTROLS,
        still,
    );
    let k = w.induced.unwrap().factor();
    let a_eff = A0 / (1.0 + k * A0);
    let q = 0.5 * RHO * V * V;
    let sum: f64 = w.strips.iter().map(|s| s.area * s.position.y.powi(2)).sum();
    let expected = -q * (a_eff + 0.01) * sum * p / V;
    assert!(
        (loads.moment.x - expected).abs() < 1e-3 * expected.abs(),
        "{} vs {expected}",
        loads.moment.x
    );
}

#[test]
fn tail_behind_the_centre_of_gravity_damps_pitch_and_stabilises() {
    let tail = wing(3.0, 1.0, 4, 0.0, DVec3::new(-4.0, 0.0, 0.0));
    let damped = tail.loads(
        flight(0.0),
        DVec3::new(0.0, 0.5, 0.0),
        RHO,
        &NO_CONTROLS,
        still,
    );
    assert!(damped.moment.y < 0.0, "pitch damping: {damped:?}");
    let stable = tail.loads(flight(0.05), DVec3::ZERO, RHO, &NO_CONTROLS, still);
    assert!(stable.moment.y < 0.0, "static stability: {stable:?}");
}

#[test]
fn downwash_reduces_the_tail_lift() {
    let tail = wing(3.0, 1.0, 4, 0.0, DVec3::new(-4.0, 0.0, 0.0));
    let clean = tail.loads(flight(0.05), DVec3::ZERO, RHO, &NO_CONTROLS, still);
    // Air moving down (+z) at the tail.
    let washed = tail.loads(flight(0.05), DVec3::ZERO, RHO, &NO_CONTROLS, |_| {
        DVec3::new(0.0, 0.0, 1.0)
    });
    assert!(
        washed.force.z > clean.force.z,
        "less upward lift with downwash"
    );
}

#[test]
fn fin_turns_the_nose_into_a_sideslip_at_any_aspect_ratio() {
    // Air from the right (v > 0): a fin behind the CG yaws the nose right.
    for ar in [1.0, 1.5, 2.0, 3.5] {
        let loads = fin(ar).loads(
            DVec3::new(V, 3.0, 0.0),
            DVec3::ZERO,
            RHO,
            &NO_CONTROLS,
            still,
        );
        assert!(loads.moment.z > 0.0, "AR {ar}: {loads:?}");
    }
}

// --- Controls -----------------------------------------------------------------

#[test]
fn positive_elevator_pitches_the_nose_up() {
    let mut tail = wing(3.0, 1.0, 4, 0.0, DVec3::new(-4.0, 0.0, 0.0));
    let tau = flap_effectiveness(0.4);
    for strip in &mut tail.strips {
        // Stick back: trailing edge up, less tail lift.
        strip.control = Some(StripControl {
            channel: ControlChannel::Elevator,
            alpha_per_rad: -tau,
        });
    }
    let controls = ControlDeflections {
        elevator: 0.2,
        ..NO_CONTROLS
    };
    let loads = tail.loads(flight(0.02), DVec3::ZERO, RHO, &controls, still);
    let neutral = tail.loads(flight(0.02), DVec3::ZERO, RHO, &NO_CONTROLS, still);
    assert!(loads.moment.y > neutral.moment.y, "{loads:?}");
}

#[test]
fn positive_aileron_rolls_right_with_adverse_yaw() {
    // Right aileron up, left aileron down. More lift on the left also means more
    // induced drag on the left: the nose yaws left (N < 0), without being scripted.
    let mut w = wing(9.0, 1.5, 10, 0.0, DVec3::ZERO);
    let tau = flap_effectiveness(0.25);
    for strip in &mut w.strips {
        if strip.position.y.abs() > 2.5 {
            strip.control = Some(StripControl {
                channel: ControlChannel::Aileron,
                alpha_per_rad: -strip.position.y.signum() * tau,
            });
        }
    }
    let controls = ControlDeflections {
        aileron: 0.2,
        ..NO_CONTROLS
    };
    let alpha: f64 = 0.08;
    let loads = w.loads(flight(alpha), DVec3::ZERO, RHO, &controls, still);
    assert!(loads.moment.x > 0.0, "roll right: {loads:?}");
    // Yaw in stability axes: body N also contains L sin α from the roll moment.
    let yaw = loads.moment.z * alpha.cos() - loads.moment.x * alpha.sin();
    assert!(yaw < 0.0, "adverse yaw {yaw}: {loads:?}");
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

// --- Robustness ------------------------------------------------------------------

#[test]
fn no_force_and_no_nan_at_rest() {
    let w = wing(9.0, 1.5, 10, 0.0, DVec3::ZERO);
    let loads = w.loads(DVec3::ZERO, DVec3::ZERO, RHO, &NO_CONTROLS, still);
    assert_eq!(loads, Wrench::default());
}

#[test]
fn finite_in_any_flight_condition() {
    // Tail first, upside down, broadside, low aspect ratio: forces stay finite.
    let w = wing(9.0, 1.5, 10, 0.03, DVec3::new(0.2, 0.0, 0.0));
    let f = fin(1.0);
    for i in 0..72 {
        let a = f64::from(i) * 5f64.to_radians();
        let v = DVec3::new(a.cos(), 0.3 * a.sin(), a.sin()) * 40.0;
        let omega = DVec3::new(0.3, -0.2, 0.1);
        for loads in [
            w.loads(v, omega, RHO, &NO_CONTROLS, still),
            f.loads(v, omega, RHO, &NO_CONTROLS, still),
        ] {
            assert!(
                loads.force.is_finite() && loads.moment.is_finite(),
                "{a}: {loads:?}"
            );
        }
    }
}
