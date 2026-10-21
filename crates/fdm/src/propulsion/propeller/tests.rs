use super::*;

const RHO: f64 = 1.225;

/// Synthetic textbook-like propeller (not historical data): C_T = 0.11 (1 − (J/1.05)²),
/// C_P = 0.07 (1 − ½ (J/1.25)²), peak efficiency about 0.73 near J = 0.7.
pub(crate) fn table() -> Vec<PropellerPoint> {
    (0..=12)
        .map(|i| {
            let j = f64::from(i) * 0.1;
            PropellerPoint {
                j,
                ct: 0.11 * (1.0 - (j / 1.05).powi(2)),
                cp: 0.07 * (1.0 - 0.5 * (j / 1.25).powi(2)),
            }
        })
        .collect()
}

pub(crate) fn anchors() -> QuadrantAnchors {
    QuadrantAnchors {
        stopped: QuadrantPoint {
            ct: -0.09,
            cq: -0.01,
        },
        reversed_flow: QuadrantPoint { ct: 0.09, cq: 0.01 },
        reversed_rotation: QuadrantPoint {
            ct: -0.03,
            cq: -0.006,
        },
    }
}

pub(crate) fn geometry(right_handed: bool, p_factor: f64) -> PropellerGeometry {
    PropellerGeometry {
        diameter: 2.5,
        hub: DVec3::new(1.8, 0.0, 0.0),
        thrust_axis: DVec3::X,
        right_handed,
        p_factor,
    }
}

pub(crate) fn propeller() -> Propeller {
    Propeller::new(table(), anchors(), geometry(true, 0.0)).unwrap()
}

/// Table coefficients interpolated linearly in J (the reference the propeller must match).
fn table_at(j: f64) -> (f64, f64) {
    let t = table();
    let i = t.partition_point(|p| p.j <= j).clamp(1, t.len() - 1);
    let (a, b) = (t[i - 1], t[i]);
    let s = (j - a.j) / (b.j - a.j);
    (a.ct + s * (b.ct - a.ct), a.cp + s * (b.cp - a.cp))
}

fn close(a: f64, b: f64, rel: f64) -> bool {
    (a - b).abs() <= rel * a.abs().max(b.abs()).max(1e-12)
}

#[test]
fn table_gives_the_classical_thrust_torque_and_power() {
    let p = propeller();
    let d = 2.5;
    let n = 30.0;
    for i in 0..=24 {
        let j = f64::from(i) * 0.05;
        let w = p.loads(
            DVec3::new(j * n * d, 0.0, 0.0),
            DVec3::ZERO,
            2.0 * PI * n,
            RHO,
        );
        let (ct, cp) = table_at(j);
        assert!(
            close(w.thrust, ct * RHO * n * n * d.powi(4), 1e-12),
            "J {j}"
        );
        let power = w.torque * 2.0 * PI * n;
        assert!(
            close(power, cp * RHO * n.powi(3) * d.powi(5), 1e-12),
            "J {j}"
        );
        assert!((w.wrench.force - DVec3::new(w.thrust, 0.0, 0.0)).length() < 1e-9);
        if j > 0.0 && cp > 0.0 {
            let efficiency = w.thrust * j * n * d / power;
            assert!(close(efficiency, j * ct / cp, 1e-12), "J {j}");
        }
    }
}

#[test]
fn static_propeller_pulls_and_absorbs_power() {
    let w = propeller().loads(DVec3::ZERO, DVec3::ZERO, 2.0 * PI * 25.0, RHO);
    assert!(w.thrust > 0.0 && w.torque > 0.0, "{w:?}");
    assert_eq!(w.advance_angle, 0.0);
}

#[test]
fn stopped_propeller_drags_and_windmills() {
    let p = propeller();
    let v = 50.0;
    let w = p.loads(DVec3::new(v, 0.0, 0.0), DVec3::ZERO, 0.0, RHO);
    let q = 0.5 * RHO * v * v * p.disc_area();
    assert!(close(w.thrust, anchors().stopped.ct * q, 1e-12), "{w:?}");
    assert!(
        close(w.torque, anchors().stopped.cq * q * 2.5, 1e-12),
        "{w:?}"
    );
    assert!(w.thrust < 0.0 && w.torque < 0.0);
}

#[test]
fn fast_dive_at_low_rpm_windmills_the_propeller() {
    // Beyond the table (J = 2.5): drag, and the flow drives the propeller.
    let w = propeller().loads(
        DVec3::new(75.0, 0.0, 0.0),
        DVec3::ZERO,
        2.0 * PI * 12.0,
        RHO,
    );
    assert!(w.thrust < 0.0 && w.torque < 0.0, "{w:?}");
}

#[test]
fn nothing_at_rest_and_finite_in_all_four_quadrants() {
    let p = propeller();
    let rest = p.loads(DVec3::ZERO, DVec3::ZERO, 0.0, RHO);
    assert_eq!((rest.thrust, rest.torque), (0.0, 0.0));
    assert_eq!(rest.wrench, Wrench::default());
    for v in [-60.0, -5.0, -0.0, 0.0, 1e-9, 5.0, 80.0] {
        for omega in [-300.0, -1.0, -0.0, 0.0, 1e-9, 1.0, 300.0] {
            let w = p.loads(
                DVec3::new(v, 3.0, -2.0),
                DVec3::new(1.0, -0.5, 0.2),
                omega,
                RHO,
            );
            assert!(
                w.thrust.is_finite() && w.torque.is_finite() && w.wrench.moment.is_finite(),
                "v {v}, Ω {omega}: {w:?}"
            );
        }
    }
}

#[test]
fn coefficients_are_continuous_over_the_whole_circle() {
    let p = propeller();
    let step = 1e-4;
    let steps = (2.0 * PI / step) as i32;
    let mut previous = p.coefficients(-PI);
    for i in 1..=steps {
        let c = p.coefficients(-PI + f64::from(i) * step);
        assert!(
            (c.ct - previous.ct).abs() < 1e-4 && (c.cq - previous.cq).abs() < 1e-4,
            "jump at β = {}",
            -PI + f64::from(i) * step
        );
        previous = c;
    }
    let (a, b) = (p.coefficients(-PI), p.coefficients(PI));
    assert!(close(a.ct, b.ct, 1e-12) && close(a.cq, b.cq, 1e-12));
    // The anchors are reached exactly.
    assert_eq!(p.coefficients(FRAC_PI_2), anchors().stopped);
    assert_eq!(p.coefficients(-FRAC_PI_2), anchors().reversed_flow);
    assert_eq!(p.coefficients(PI), anchors().reversed_rotation);
}

#[test]
fn rotation_axis_follows_the_hand_of_the_propeller() {
    let right = Propeller::new(table(), anchors(), geometry(true, 0.0)).unwrap();
    let left = Propeller::new(table(), anchors(), geometry(false, 0.0)).unwrap();
    assert_eq!(right.rotation_axis(), DVec3::X);
    assert_eq!(left.rotation_axis(), -DVec3::X);
}

#[test]
fn p_factor_yaws_a_right_handed_tractor_to_the_left_at_high_incidence() {
    // Nose up: the air comes from below (w > 0); the descending blade, on the right
    // of a propeller turning clockwise seen from the cockpit, works harder.
    let air = DVec3::new(30.0, 0.0, 6.0);
    let omega = 2.0 * PI * 25.0;
    let right = Propeller::new(table(), anchors(), geometry(true, 0.3)).unwrap();
    let left = Propeller::new(table(), anchors(), geometry(false, 0.3)).unwrap();
    let r = right.loads(air, DVec3::ZERO, omega, RHO);
    let l = left.loads(air, DVec3::ZERO, omega, RHO);
    assert!(r.thrust > 0.0);
    assert!(r.wrench.moment.z < 0.0, "{r:?}");
    assert!((l.wrench.moment + r.wrench.moment).length() < 1e-9, "{l:?}");
    // Shift = p_factor × sin(inflow angle) × cos β.
    let blade = 0.7 * PI * 25.0 * 2.5;
    let cos_beta = blade / (30.0f64 * 30.0 + blade * blade).sqrt();
    let expected = 0.3 * 6.0 / air.length() * cos_beta * r.thrust;
    assert!(close(-r.wrench.moment.z, expected, 1e-12));

    // A stopped propeller has no P-factor; turning backwards reverses it.
    let stopped = right.loads(air, DVec3::ZERO, 0.0, RHO);
    assert!(stopped.wrench.moment.length() < 1e-12, "{stopped:?}");
    let backwards = right.loads(air, DVec3::ZERO, -omega, RHO);
    let shift = |w: &PropellerLoads| -w.wrench.moment.z / w.thrust;
    assert!(shift(&backwards) * shift(&r) < 0.0, "{backwards:?}");

    // Sideslip to the right (air from the right): the top blade works harder, the thrust
    // rises above the axis and pitches the nose down.
    let s = right.loads(DVec3::new(30.0, 6.0, 0.0), DVec3::ZERO, omega, RHO);
    assert!(
        s.wrench.moment.y < 0.0 && s.wrench.moment.z.abs() < 1e-9,
        "{s:?}"
    );

    // No inflow angle, hub on the thrust axis: no moment at all.
    let straight = right.loads(DVec3::new(30.0, 0.0, 0.0), DVec3::ZERO, omega, RHO);
    assert!(straight.wrench.moment.length() < 1e-12);
}

#[test]
fn slipstream_obeys_momentum_theory() {
    let p = propeller();
    let w = p.loads(
        DVec3::new(30.0, 0.0, 0.0),
        DVec3::ZERO,
        2.0 * PI * 25.0,
        RHO,
    );
    let s = p.slipstream(&w, RHO);
    let (v, vi) = (30.0, s.induced_speed());
    assert!(vi > 0.0);
    assert!(close(
        w.thrust,
        2.0 * RHO * p.disc_area() * (v + vi) * vi,
        1e-12
    ));
    // Air pushed backwards: induced speed at the disc, twice that far downstream.
    let hub = p.geometry().hub;
    assert!(close(s.local_flow(hub).x, -vi, 1e-12));
    assert!(close(s.local_flow(hub - DVec3::X * 1e4).x, -2.0 * vi, 1e-6));
    assert!(s.local_flow(hub + DVec3::X * 1e4).length() < 1e-6 * vi);
    // Mass flow conserved along the contracting tube.
    let flow0 = (v + vi) * 1.25f64.powi(2);
    for x in [-3.0, 0.5, 2.0, 6.0, 50.0] {
        let r = s.tube_radius(x);
        assert!(
            close((v + s.axial_speed_at(x)) * r * r, flow0, 1e-12),
            "x {x}"
        );
        assert!(x <= 0.0 || r < 1.25);
    }
}

#[test]
fn slipstream_fades_smoothly_at_its_edge() {
    let p = propeller();
    let w = p.loads(
        DVec3::new(20.0, 0.0, 0.0),
        DVec3::ZERO,
        2.0 * PI * 25.0,
        RHO,
    );
    let s = p.slipstream(&w, RHO);
    let x = 4.0;
    let tube = s.tube_radius(x);
    let at = |r: f64| {
        s.local_flow(p.geometry().hub + DVec3::new(-x, 0.0, r))
            .x
            .abs()
    };
    let full = s.axial_speed_at(x);
    assert!(close(at(0.85 * tube), full, 1e-12));
    assert_eq!(at(1.11 * tube), 0.0);
    let mut previous = at(0.0);
    for i in 1..=2_000 {
        let value = at(f64::from(i) * 1e-3);
        assert!(value <= previous + 1e-12 && previous - value < 0.01 * full);
        previous = value;
    }
}

#[test]
fn no_slipstream_without_thrust() {
    let p = propeller();
    let windmilling = p.loads(
        DVec3::new(75.0, 0.0, 0.0),
        DVec3::ZERO,
        2.0 * PI * 12.0,
        RHO,
    );
    let s = p.slipstream(&windmilling, RHO);
    assert_eq!(s.induced_speed(), 0.0);
    assert_eq!(s.local_flow(DVec3::new(-4.0, 0.0, 0.0)), DVec3::ZERO);
}

#[test]
fn rejects_unphysical_propellers() {
    let reject = |t: Vec<PropellerPoint>, a: QuadrantAnchors, g: PropellerGeometry| {
        Propeller::new(t, a, g).unwrap_err()
    };
    let g = geometry(true, 0.0);
    assert_eq!(
        reject(table()[..1].to_vec(), anchors(), g),
        PropellerError::TooFewPoints
    );
    let mut t = table();
    t[3].cp = f64::NAN;
    assert_eq!(reject(t, anchors(), g), PropellerError::NonFinite);
    let mut t = table();
    t.swap(2, 3);
    assert_eq!(
        reject(t, anchors(), g),
        PropellerError::NotStrictlyIncreasing
    );
    assert_eq!(
        reject(table()[1..].to_vec(), anchors(), g),
        PropellerError::BadRange
    );
    let mut t = table();
    t[0].ct = -0.01;
    assert_eq!(reject(t, anchors(), g), PropellerError::NoStaticThrust);
    let mut a = anchors();
    a.stopped.ct = 0.05;
    assert_eq!(reject(table(), a, g), PropellerError::BadAnchors);
    let mut a = anchors();
    a.reversed_rotation.cq = 0.01;
    assert_eq!(reject(table(), a, g), PropellerError::BadAnchors);
    for bad in [
        PropellerGeometry { diameter: 0.0, ..g },
        PropellerGeometry {
            thrust_axis: DVec3::new(1.0, 0.1, 0.0),
            ..g
        },
        PropellerGeometry {
            p_factor: -0.1,
            ..g
        },
    ] {
        assert_eq!(reject(table(), anchors(), bad), PropellerError::BadGeometry);
    }
}
