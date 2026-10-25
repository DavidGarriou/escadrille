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

fn still(_: &Strip) -> DVec3 {
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
    LiftingSurface::new("wing".into(), strips, section(0.01, 0.0), induced).unwrap()
}

fn wing(span: f64, chord: f64, strips: usize, incidence: f64, centre: DVec3) -> LiftingSurface {
    let induced = InducedModel::new(span / chord, 1.0);
    wing_with(span, chord, strips, incidence, 0.0, centre, induced)
}

/// Single-strip surface at the tail, normal `normal`.
fn tail_strip(polar: Polar, normal: DVec3, induced: Option<InducedModel>) -> LiftingSurface {
    let strip = Strip {
        position: DVec3::new(-4.5, 0.0, -0.5),
        chord_axis: DVec3::X,
        normal,
        area: 1.0,
        chord: 1.0,
        control: None,
    };
    LiftingSurface::new("tail".into(), vec![strip], polar, induced).unwrap()
}

fn fin(aspect_ratio: f64) -> LiftingSurface {
    tail_strip(
        section(0.01, 0.0),
        DVec3::Y,
        InducedModel::new(aspect_ratio, 0.9),
    )
}

/// Synthetic cambered section with a curved lift curve and asymmetric stalls (not
/// historical data; aerodynamicist review): the thin section is exactly linear between
/// its stall points, so a single secant step solves it and hides solver errors.
fn cambered() -> Polar {
    let peak = |a: f64| 0.25 + 5.6 * a - 6.0 * a * a.abs();
    let points = (-16..=20)
        .map(|deg: i32| {
            let alpha = f64::from(deg).to_radians();
            let cl = if deg < -10 {
                -0.75 + 0.03 * f64::from(-10 - deg)
            } else if deg <= 14 {
                peak(alpha)
            } else {
                peak(14f64.to_radians()) - 0.06 * f64::from(deg - 14)
            };
            PolarPoint {
                alpha,
                cl,
                cd: 0.01 + 0.3 * alpha * alpha,
                cm: -0.05,
            }
        })
        .collect();
    Polar::new(points, viterna_cd_max(6.0)).unwrap()
}

fn cambered_strip(aspect_ratio: f64) -> LiftingSurface {
    tail_strip(
        cambered(),
        DVec3::NEG_Z,
        InducedModel::new(aspect_ratio, 0.9),
    )
}

/// Force and moment over the dynamic pressure, and diagnostics, at angle of attack `alpha`.
fn coefficients_at(s: &LiftingSurface, alpha: f64) -> (DVec3, DVec3, StripState) {
    let (w, state) = s.strip_loads(&s.strips[0], flight(alpha), RHO, &NO_CONTROLS, 0.0);
    let q = state.dynamic_pressure;
    (w.force / q, w.moment / q, state)
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
        let x = s.effective_alpha(alpha, k, 0.0);
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
        let mut previous = s.effective_alpha(-PI + 1e-9, k, 0.0);
        for i in 1..3_600 {
            let alpha = -PI + f64::from(i) * PI / 1_800.0;
            let x = s.effective_alpha(alpha, k, 0.0);
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
fn induced_solver_is_exact_on_a_curved_asymmetric_polar() {
    for ar in [1.0, 1.5, 2.0, 3.5, 6.0, 10.0] {
        let s = cambered_strip(ar);
        let k = s.induced.unwrap().factor();
        let (neg, pos) = s.polar.stall_points();
        let (lo, hi) = (neg.alpha + k * neg.cl, pos.alpha + k * pos.cl);
        for i in 0..=400 {
            let alpha = lo + (hi - lo) * f64::from(i) / 400.0;
            let state = coefficients_at(&s, alpha).2;
            let x = state.alpha - state.induced_alpha;
            let residual = state.induced_alpha - k * s.polar.coefficients(x).cl;
            assert!(residual.abs() < 1e-12, "AR {ar}, α {alpha}: {residual:e}");
        }
    }
}

#[test]
fn loads_are_continuous_over_the_whole_circle_with_a_cambered_section() {
    for ar in [1.0, 1.5, 6.0] {
        let s = cambered_strip(ar);
        let n = 72_000;
        let first = coefficients_at(&s, -PI + 1e-12);
        let mut previous = first;
        for i in 1..=n {
            let alpha = -PI + 2.0 * PI * f64::from(i) / f64::from(n);
            let c = coefficients_at(&s, alpha);
            let jump = (c.0 - previous.0).length().max((c.1 - previous.1).length());
            assert!(
                jump < 0.02,
                "AR {ar}, {:.3}°: jump {jump}",
                alpha.to_degrees()
            );
            previous = c;
        }
        let jump = (previous.0 - first.0)
            .length()
            .max((previous.1 - first.1).length());
        assert!(jump < 1e-6, "AR {ar}: jump across 180°: {jump}");
    }
}

#[test]
fn induction_vanishes_in_separated_and_reversed_flow() {
    // The diagnostic itself must be zero, not ±2π, even at exactly −180°.
    for ar in [1.0, 1.5, 6.0] {
        let s = cambered_strip(ar);
        for deg in (60..=180).step_by(5) {
            for alpha in [f64::from(deg).to_radians(), -f64::from(deg).to_radians()] {
                let state = coefficients_at(&s, alpha).2;
                assert!(
                    state.induced_alpha.abs() < 1e-12,
                    "AR {ar}, {deg}°: {state:?}"
                );
            }
        }
    }
}

#[test]
fn force_is_the_section_force_tilted_by_the_induced_angle() {
    let s = cambered_strip(1.0);
    for deg in [10.0, 25.0, 35.0, 45.0] {
        let alpha: f64 = f64::to_radians(deg);
        let (force, _, state) = coefficients_at(&s, alpha);
        let c = state.coefficients;
        let flow = DVec3::new(alpha.cos(), 0.0, alpha.sin());
        let up = DVec3::new(alpha.sin(), 0.0, -alpha.cos());
        let ai = state.induced_alpha;
        let lift = c.cl * ai.cos() - c.cd * ai.sin();
        let drag = c.cd * ai.cos() + c.cl * ai.sin();
        assert!(
            (force.dot(up) - lift).abs() < 1e-9 && (-force.dot(flow) - drag).abs() < 1e-9,
            "{deg}°: {force:?} {state:?}"
        );
    }
}

#[test]
fn a_surface_knows_its_section_slope() {
    let w = wing(10.0, 1.5, 4, 0.0, DVec3::ZERO);
    assert!((w.section_slope() - A0).abs() < 1e-9 * A0);
    assert_eq!(w.section_slope(), w.polar().zero_lift_slope().unwrap());
}

#[test]
fn rejects_unphysical_induced_models() {
    assert!(InducedModel::new(0.0, 0.8).is_none());
    assert!(InducedModel::new(6.0, 1.2).is_none());
    assert!(InducedModel::new(0.5, 0.9).is_none());
    assert!(InducedModel::new(0.6, 0.9).is_some());
    assert!(InducedModel::new(6.0, 0.8).is_some());
}

#[test]
fn rejects_unphysical_surfaces() {
    let strip = wing(1.0, 1.0, 1, 0.0, DVec3::ZERO).strips()[0];
    let polar = || section(0.01, 0.0);
    let new = |strips: Vec<Strip>, polar: Polar, ar: f64| {
        LiftingSurface::new("s".into(), strips, polar, InducedModel::new(ar, 0.9))
    };
    assert_eq!(new(vec![], polar(), 6.0), Err(SurfaceError::NoStrips));
    for bad in [
        Strip { area: 0.0, ..strip },
        Strip {
            chord: -1.0,
            ..strip
        },
        Strip {
            normal: DVec3::new(0.1, 0.0, -1.0).normalize(),
            ..strip
        },
        Strip {
            chord_axis: DVec3::new(2.0, 0.0, 0.0),
            ..strip
        },
        Strip {
            position: DVec3::new(f64::NAN, 0.0, 0.0),
            ..strip
        },
    ] {
        assert_eq!(new(vec![bad], polar(), 6.0), Err(SurfaceError::BadStrip));
    }
    // A dip of the lift curve before the stall (slope −11.5 /rad): several induced
    // solutions when −1/k = −π e AR is above it (AR 1), unique at AR 6.
    let dip = |alpha: f64, cl: f64| PolarPoint {
        alpha: alpha.to_radians(),
        cl,
        cd: 0.01,
        cm: 0.0,
    };
    let dipped = || {
        let points = vec![
            dip(-14.0, -0.9),
            dip(-12.0, -1.0),
            dip(0.0, 0.0),
            dip(10.0, 1.0),
            dip(11.0, 0.8),
            dip(14.0, 1.2),
            dip(18.0, 1.0),
        ];
        Polar::new(points, viterna_cd_max(6.0)).unwrap()
    };
    assert_eq!(
        new(vec![strip], dipped(), 1.0),
        Err(SurfaceError::InductionNotMonotone)
    );
    assert!(new(vec![strip], dipped(), 6.0).is_ok());
    assert!(LiftingSurface::new("s".into(), vec![strip], dipped(), None).is_ok());
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
    let (loads, state) = w.strip_loads(&strip, v, RHO, &NO_CONTROLS, 0.0);
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

/// A strip in a swirl that raises its incidence: what the strip feels with a tilt fraction
/// τ, and strip theory's loads.
fn swirled(tau: f64) -> (LiftingSurface, Wrench, Wrench, f64, f64) {
    let induced = InducedModel::new(6.0, 0.9).unwrap();
    let s = tail_strip(section(0.02, 0.0), DVec3::NEG_Z, Some(induced));
    let v = flight(0.03);
    let swirl = DVec3::new(0.0, 0.0, 3.0);
    let flow = StripFlow {
        velocity: swirl,
        swirl,
        swirl_tilt: tau,
    };
    let felt = s.loads(v, DVec3::ZERO, RHO, &NO_CONTROLS, |_| flow);
    let strip_theory = s.loads(v, DVec3::ZERO, RHO, &NO_CONTROLS, |_| swirl);
    let ka = induced.factor() * s.section_slope();
    (s, felt, strip_theory, ka / (1.0 + ka), (v - swirl).length())
}

#[test]
fn the_wing_takes_back_as_drag_the_swirl_thrust_its_induced_flow_cancels() {
    let (s, felt, strip_theory, kappa, speed) = swirled(0.15);
    // The swirl raises the incidence by δ_s = 3 / speed (the flow from below, here along
    // the normal's opposite): the lift it adds, ΔC_L, is that of δ_s / (1 + k a₀).
    let strip = &s.strips[0];
    let v = flight(0.03) - DVec3::new(0.0, 0.0, 3.0);
    let (_, state) = s.strip_loads(strip, v, RHO, &NO_CONTROLS, 0.0);
    let delta = 3.0 * v.x / (speed * speed);
    let x = state.alpha - state.induced_alpha;
    let added = s.polar().coefficients(x).cl
        - s.polar()
            .coefficients(x - delta / (1.0 + s.induced().unwrap().factor() * s.section_slope()))
            .cl;
    assert!(added > 0.1 && delta > 0.03, "{added} {delta}");
    let drag = (1.0 - 0.15 - kappa) * added * delta * state.dynamic_pressure * strip.area;
    // The extra drag lies along the strip's drag direction, nothing else changes.
    let difference = felt.force - strip_theory.force;
    assert!(
        (difference.length() - drag).abs() < 1e-9 * drag,
        "{difference:?} {drag}"
    );
    let flow_dir = v.normalize();
    assert!((difference.normalize().dot(flow_dir) + (state.induced_alpha).cos()).abs() < 1e-9);
}

#[test]
fn strip_theory_is_kept_when_the_strip_straightens_nothing_more() {
    // τ = 1, or τ + κ ≥ 1: no extra drag; a vector flow is a swirl-free strip flow.
    for tau in [1.0, 0.95] {
        let (_, felt, strip_theory, kappa, _) = swirled(tau);
        assert!(tau + kappa >= 1.0, "{kappa}");
        assert_eq!(felt, strip_theory);
    }
    let (s, ..) = swirled(1.0);
    let v = flight(0.1);
    assert_eq!(
        s.loads(v, DVec3::ZERO, RHO, &NO_CONTROLS, |_| DVec3::ZERO),
        s.loads(v, DVec3::ZERO, RHO, &NO_CONTROLS, |_| StripFlow::default())
    );
}

#[test]
fn the_swirl_drag_fades_with_the_induction_past_the_stall() {
    // Deep in the stall (60°) the induction has faded out: strip theory again.
    let induced = InducedModel::new(6.0, 0.9).unwrap();
    let s = tail_strip(section(0.02, 0.0), DVec3::NEG_Z, Some(induced));
    let swirl = DVec3::new(0.0, 0.0, 3.0);
    let flow = StripFlow {
        velocity: swirl,
        swirl,
        swirl_tilt: 0.0,
    };
    let v = flight(60f64.to_radians()) + swirl;
    let felt = s.loads(v, DVec3::ZERO, RHO, &NO_CONTROLS, |_| flow);
    let strip_theory = s.loads(v, DVec3::ZERO, RHO, &NO_CONTROLS, |_| swirl);
    assert_eq!(felt, strip_theory);
}
