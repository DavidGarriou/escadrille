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
        normal_force: 0.0,
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
    // Shift = p_factor × V⊥ V_blade / V_r².
    let blade = 0.7 * PI * 25.0 * 2.5;
    let expected = 0.3 * 6.0 * blade / (30.0 * 30.0 + blade * blade) * r.thrust;
    assert!(close(-r.wrench.moment.z, expected, 1e-12));

    // It vanishes at low speed instead of saturating: crosswind of 3 m/s at rest.
    let crosswind = right.loads(DVec3::new(0.0, 3.0, 0.0), DVec3::ZERO, omega, RHO);
    let shift = crosswind.wrench.moment.y.abs() / crosswind.thrust;
    assert!(shift < 0.3 * 3.0 / blade + 1e-12, "{crosswind:?}");

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
fn a_windmilling_propeller_slows_and_swirls_the_air() {
    let p = propeller();
    let windmilling = p.loads(
        DVec3::new(75.0, 0.0, 0.0),
        DVec3::ZERO,
        2.0 * PI * 12.0,
        RHO,
    );
    assert!(
        windmilling.thrust < 0.0 && windmilling.torque < 0.0,
        "{windmilling:?}"
    );
    let s = p.slipstream(&windmilling, RHO);
    // Froude on the braking side: T = 2 ρ A (V + v_i) v_i with v_i < 0.
    let vi = s.induced_speed();
    assert!(vi < 0.0 && vi > -37.5, "{vi}");
    assert!(close(
        windmilling.thrust,
        2.0 * RHO * p.disc_area() * (75.0 + vi) * vi,
        1e-9
    ));
    // Driven by the air, it turns the air against its rotation: K = Q / ṁ < 0.
    let mass_flow = RHO * p.disc_area() * (75.0 + vi);
    assert!(close(s.swirl() * mass_flow, windmilling.torque, 1e-12));
    let hub = p.geometry().hub;
    let behind = s.local_flow(hub + DVec3::new(-2.0, 0.0, -0.8));
    // Slowed air: it moves forwards relative to the free stream.
    assert!(behind.x > 0.0 && behind.y < 0.0, "{behind:?}");
}

#[test]
fn a_hard_braking_propeller_stops_at_half_the_speed_and_a_bounded_tube() {
    // Far beyond the turbulent wake state: v_i held at −V/2, the tube at most three times
    // its area.
    let p = propeller();
    let mut braking = p.loads(DVec3::new(30.0, 0.0, 0.0), DVec3::ZERO, 0.0, RHO);
    braking.thrust = -5_000.0;
    let s = p.slipstream(&braking, RHO);
    assert_eq!(s.induced_speed(), -15.0);
    for x in [0.0, 1.0, 5.0, 100.0] {
        let r = s.tube_radius(x);
        assert!(
            r.is_finite() && r <= 1.25 * 3f64.sqrt() + 1e-12,
            "x {x}: {r}"
        );
    }
    let behind = s.local_flow(p.geometry().hub + DVec3::new(-50.0, 0.0, 0.3));
    assert!(behind.is_finite() && behind.x > 25.0, "{behind:?}");
}

#[test]
fn the_slipstream_is_continuous_when_the_thrust_crosses_zero() {
    // An idle glide sits near T = 0: the swirl used to vanish with the thrust, a jump
    // of the rolling moment on the wings that left no equilibrium (review of 2026-10-24).
    let p = propeller();
    let hub = p.geometry().hub;
    let at = |thrust: f64| {
        let loads = PropellerLoads {
            wrench: Wrench::default(),
            thrust,
            torque: 24.5,
            axial_speed: 35.0,
            advance_angle: 1.2,
        };
        let s = p.slipstream(&loads, RHO);
        (s.swirl(), s.local_flow(hub + DVec3::new(-1.5, 0.3, -0.8)))
    };
    let (minus, plus) = (at(-1e-6), at(1e-6));
    assert!(
        (minus.0 - plus.0).abs() < 1e-9 * plus.0.abs(),
        "{minus:?} {plus:?}"
    );
    assert!((minus.1 - plus.1).length() < 1e-6, "{minus:?} {plus:?}");
    assert!(plus.0 > 0.0);
    // No flow through the disc: the swirl fades out instead of blowing up.
    let still = PropellerLoads {
        wrench: Wrench::default(),
        thrust: -1.0,
        torque: 24.5,
        axial_speed: 0.0,
        advance_angle: 0.0,
    };
    let s = p.slipstream(&still, RHO);
    assert_eq!(s.swirl(), 0.0);
    assert_eq!(s.local_flow(hub + DVec3::new(-1.5, 0.0, -0.8)), DVec3::ZERO);
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
    for flip in 0..6 {
        let mut a = anchors();
        let value = match flip {
            0 => &mut a.stopped.ct,
            1 => &mut a.stopped.cq,
            2 => &mut a.reversed_flow.ct,
            3 => &mut a.reversed_flow.cq,
            4 => &mut a.reversed_rotation.ct,
            _ => &mut a.reversed_rotation.cq,
        };
        *value = -*value;
        assert_eq!(reject(table(), a, g), PropellerError::BadAnchors, "{flip}");
    }
    // Right signs, but a perpetual motion machine near β = 53° (aerodynamicist review).
    let mut a = anchors();
    a.stopped = QuadrantPoint {
        ct: -0.02,
        cq: -0.03,
    };
    assert_eq!(reject(table(), a, g), PropellerError::CreatesEnergy);
    // η > 1 inside the table.
    let mut t = table();
    t[8].cp = 0.01;
    assert_eq!(reject(t, anchors(), g), PropellerError::CreatesEnergy);
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
        PropellerGeometry {
            normal_force: f64::NAN,
            ..g
        },
    ] {
        assert_eq!(reject(table(), anchors(), bad), PropellerError::BadGeometry);
    }
}

#[test]
fn normal_force_opposes_the_cross_flow_and_destabilises() {
    let g = PropellerGeometry {
        normal_force: 0.4,
        ..geometry(true, 0.0)
    };
    let p = Propeller::new(table(), anchors(), g).unwrap();
    let omega = 2.0 * PI * 25.0;
    // Nose up: air from below. Upward in-plane force ahead of the centre of gravity:
    // nose-up moment, the destabilising sense.
    let up = p.loads(DVec3::new(30.0, 0.0, 6.0), DVec3::ZERO, omega, RHO);
    let blade = 0.7 * PI * 25.0 * 2.5;
    let v_r = (30.0f64 * 30.0 + blade * blade).sqrt();
    let expected = 0.5 * RHO * v_r * p.disc_area() * 0.4 * 6.0;
    assert!(close(-up.wrench.force.z, expected, 1e-12), "{up:?}");
    assert!(up.wrench.moment.y > 0.0);
    // A yaw rate moves the hub sideways: cross flow there too.
    let yawing = p.loads(
        DVec3::new(30.0, 0.0, 0.0),
        DVec3::new(0.0, 0.0, 0.5),
        omega,
        RHO,
    );
    assert!(yawing.wrench.force.y < 0.0, "{yawing:?}");
}

#[test]
fn energy_balance_holds_over_the_whole_circle() {
    let p = propeller();
    for i in 0..3_600 {
        let beta = -PI + f64::from(i) * PI / 1_800.0;
        let c = p.coefficients(beta);
        assert!(
            c.cq * beta.cos() / 0.35 - c.ct * beta.sin() >= -1e-12,
            "β = {beta}: {c:?}"
        );
    }
}

#[test]
fn slipstream_swirls_with_the_propeller() {
    let p = propeller();
    let w = p.loads(
        DVec3::new(30.0, 0.0, 0.0),
        DVec3::ZERO,
        2.0 * PI * 25.0,
        RHO,
    );
    let s = p.slipstream(&w, RHO);
    let mass_flow = RHO * p.disc_area() * (30.0 + s.induced_speed());
    assert!(close(s.swirl() * mass_flow, w.torque, 1e-12));
    // Clockwise seen from the cockpit: above the axis the air goes to the right.
    let hub = p.geometry().hub;
    let far = 1e4;
    let above = s.local_flow(hub + DVec3::new(-far, 0.0, -0.8));
    assert!(above.y > 0.0 && above.z.abs() < 1e-12, "{above:?}");
    assert!(close(above.y, s.swirl() / 0.8, 1e-6));
    // Half of it at the disc, continuous across the core, nothing on the axis.
    let at_disc = s.local_flow(hub + DVec3::new(0.0, 0.0, -0.8));
    assert!(close(at_disc.y, 0.5 * s.swirl() / 0.8, 1e-12));
    // r v_θ is conserved along a streamline: all of it just behind the disc (on the
    // wing roots), none ahead of it.
    let behind = s.local_flow(hub + DVec3::new(-0.5 * 1.25, 0.0, -0.8));
    assert!(close(behind.y, s.swirl() / 0.8, 1e-12), "{behind:?}");
    let ahead = s.local_flow(hub + DVec3::new(0.5 * 1.25, 0.0, -0.8));
    assert_eq!(ahead.y, 0.0);
    // Solid-body core: continuous at its edge, proportional to the radius inside.
    let core = 0.25 * s.tube_radius(far);
    let at = |r: f64| s.local_flow(hub + DVec3::new(-far, 0.0, -r)).y;
    assert!((at(core - 1e-9) - at(core + 1e-9)).abs() < 1e-6 * at(core));
    assert!(close(at(0.5 * core), 0.5 * at(core), 1e-9));
    assert_eq!(s.local_flow(hub - DVec3::X * 3.0).y, 0.0);
}

#[test]
fn the_jet_width_factor_is_table_2_of_r_and_m_2368() {
    // Nodes of the table: (V/v, 1/μ, ΔL / ΔL_strip).
    for (ratio, narrowness, eta) in [
        (0.0, 1.0, 0.1257),
        (0.0, 0.2, 0.4081),
        (0.5, 0.4, 0.4880),
        (0.5, 0.2, 0.6225),
        (1.0 / 1.2, 1.0, 0.5685),
        (1.0 / 1.2, 0.4, 0.7861),
    ] {
        assert!(
            (jet_width_factor(ratio, narrowness) - eta).abs() < 1e-12,
            "{ratio} {narrowness}"
        );
    }
    // Strip theory when the jet barely differs from the stream (Koning), or is infinitely
    // wide.
    for x in [0.0, 0.3, 1.0, 5.0] {
        assert_eq!(jet_width_factor(1.0, x), 1.0);
        assert_eq!(jet_width_factor(x.min(1.0), 0.0), 1.0);
    }
}

#[test]
fn the_jet_width_factor_is_monotone_continuous_and_bounded() {
    let grid = |n: i32, top: f64| (0..=n).map(move |i| f64::from(i) * top / f64::from(n));
    for x in grid(200, 1.5) {
        let mut previous = jet_width_factor(0.0, x);
        for r in grid(200, 1.0).skip(1) {
            let eta = jet_width_factor(r, x);
            assert!(eta >= previous && eta - previous < 0.02, "{r} {x}");
            previous = eta;
        }
    }
    for r in grid(100, 1.0) {
        let mut previous = jet_width_factor(r, 0.0);
        for x in grid(300, 1.5).skip(1) {
            let eta = jet_width_factor(r, x);
            assert!(eta <= previous && previous - eta < 0.02, "{r} {x}");
            assert!(eta > 0.0 && eta <= 1.0);
            previous = eta;
        }
        // Narrower than μ = 1: the factor of μ = 1.
        assert_eq!(jet_width_factor(r, 3.0), jet_width_factor(r, 1.0));
        // A jet slower than the stream: the factor of the inverse ratio.
        if r > 0.0 {
            assert!((jet_width_factor(1.0 / r, 0.3) - jet_width_factor(r, 0.3)).abs() < 1e-12);
        }
    }
    assert_eq!(jet_width_factor(f64::NAN, 0.3), 1.0);
}

/// Slipstream of the synthetic propeller at 20 m/s and 1 500 rpm, and a point inside its
/// tube, 1.5 m behind the disc and 0.6 m to the right of its axis.
fn blowing() -> (Slipstream, DVec3) {
    let p = propeller();
    let w = p.loads(
        DVec3::new(20.0, 0.0, 0.0),
        DVec3::ZERO,
        2.0 * PI * 25.0,
        RHO,
    );
    (
        p.slipstream(&w, RHO),
        p.geometry().hub + DVec3::new(-1.5, 0.6, 0.0),
    )
}

#[test]
fn a_strip_feels_eta_times_the_lift_the_slipstream_adds() {
    let (s, point) = blowing();
    let (chord, slope) = (1.5, 5.5);
    let full = s.local_flow(point);
    let felt = s.lifting_flow(point, chord, slope);
    // Axial parts (the air moves backwards: −x) and swirl parts.
    let (free, jet, effective) = (20.0, 20.0 - full.x, 20.0 - felt.x);
    let (swirl, swirl_felt) = (full.z, felt.z);
    assert!(jet > free + 1.0 && swirl.abs() > 0.5, "{full:?}");
    let narrowness = slope * chord / (8.0 * PI * s.tube_radius(1.5));
    let eta = jet_width_factor(free / jet, narrowness);
    assert!(eta > 0.3 && eta < 0.9, "{eta}");
    // Dynamic pressure: v_e² − V² = η (v² − V²).
    assert!(close(
        effective * effective - free * free,
        eta * (jet * jet - free * free),
        1e-12
    ));
    // Swirl incidence: the lift it adds, ∝ speed × tangential velocity, is η times.
    assert!(close(effective * swirl_felt, eta * jet * swirl, 1e-12));
    assert_eq!(felt.y, full.y);
}

#[test]
fn a_strip_much_narrower_than_the_jet_feels_all_of_it() {
    let (s, point) = blowing();
    let full = s.local_flow(point);
    assert!((s.lifting_flow(point, 1e-9, 5.5) - full).length() < 1e-6 * full.length());
}

#[test]
fn a_strip_feels_the_slipstream_continuously_up_to_its_edge() {
    let (s, point) = blowing();
    let hub = point + DVec3::new(1.5, -0.6, 0.0);
    let tube = s.tube_radius(1.5);
    let at = |y: f64| s.lifting_flow(hub + DVec3::new(-1.5, y, 0.0), 1.5, 5.5);
    assert_eq!(at(1.11 * tube), DVec3::ZERO);
    // Steps of 1 mm: less than 1 % of the jet's axial speed each, as `local_flow`.
    let full = s.axial_speed_at(1.5);
    let mut previous = at(0.3);
    for i in 1..=3_000 {
        let value = at(0.3 + f64::from(i) * 1e-3);
        assert!(
            (value - previous).length() < 0.01 * full,
            "y {}",
            0.3 + f64::from(i) * 1e-3
        );
        previous = value;
    }
}

#[test]
fn at_rest_a_strip_feels_the_root_of_eta_of_the_jet_speed() {
    let p = propeller();
    let w = p.loads(DVec3::ZERO, DVec3::ZERO, 2.0 * PI * 25.0, RHO);
    let s = p.slipstream(&w, RHO);
    let point = p.geometry().hub + DVec3::new(-1.5, 0.6, 0.0);
    let jet = -s.local_flow(point).x;
    let felt = -s.lifting_flow(point, 1.5, 5.5).x;
    let eta = jet_width_factor(0.0, 5.5 * 1.5 / (8.0 * PI * s.tube_radius(1.5)));
    assert!(jet > 5.0);
    assert!(close(felt, eta.sqrt() * jet, 1e-12), "{felt} {jet} {eta}");
}

#[test]
fn the_wings_leave_the_fraction_of_the_swirl_they_do_not_straighten() {
    let (s, point) = blowing();
    let hub = point + DVec3::new(1.5, -0.6, 0.0);
    let wake = WingWake {
        start: 1.0,
        length: 0.5,
        swirl_left: 0.4,
    };
    let behind = s.behind_wings(wake);
    let at = |s: &Slipstream, distance: f64| s.local_flow(hub + DVec3::new(-distance, 0.6, 0.0));
    // Ahead of the trailing edge, nothing changes; past the wake, 40 % of the swirl.
    assert_eq!(at(&behind, 0.9), at(&s, 0.9));
    let (free, left) = (at(&s, 2.0), at(&behind, 2.0));
    assert_eq!(left.x, free.x);
    assert!(
        (left.z - 0.4 * free.z).abs() < 1e-12 && free.z.abs() > 0.5,
        "{free:?}"
    );
    // Smoothly in between.
    let mut previous = at(&behind, 0.9).z;
    for i in 1..=800 {
        let value = at(&behind, 0.9 + f64::from(i) * 1e-3).z;
        assert!((value - previous).abs() < 0.01 * free.z.abs(), "{i}");
        previous = value;
    }
    assert!(at(&s.without_swirl(), 2.0).z.abs() < 1e-15);
}

#[test]
fn the_rotational_power_is_the_swirl_energy_through_the_disc() {
    // ½ ρ ∫ v_θ² v_x dA just behind the disc, by quadrature over the tube.
    let (s, point) = blowing();
    let hub = point + DVec3::new(1.5, -0.6, 0.0);
    let distance = 0.1 * 1.25;
    let (tube, axial) = (s.tube_radius(distance), 20.0 + s.axial_speed_at(distance));
    let n = 20_000;
    let mut energy = 0.0;
    for i in 0..n {
        let r = (f64::from(i) + 0.5) / f64::from(n) * 1.2 * tube;
        let flow = s.local_flow(hub + DVec3::new(-distance, r, 0.0));
        let dr = 1.2 * tube / f64::from(n);
        energy += 0.5 * RHO * flow.z * flow.z * (20.0 - flow.x) * 2.0 * PI * r * dr;
    }
    let expected = s.rotational_power(RHO) * axial / (20.0 + s.induced_speed());
    // The formula leaves out the blend at the tube's edge: within a few per cent.
    assert!(close(energy, expected, 0.05), "{energy} {expected}");
    assert!(expected > 0.0);
}
