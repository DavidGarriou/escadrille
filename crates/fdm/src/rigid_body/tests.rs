//! Tests of the rigid body and its integrator. Several of them come from the
//! `aerodynamicien` review of 2026-10-20, which showed with deliberately injected
//! errors which mistakes the first test suite could not catch.

use super::*;
use glam::EulerRot;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_6};

const DT: f64 = 1.0 / 240.0;
const MASS: f64 = 700.0;

fn inertia(ixz: f64) -> Inertia {
    Inertia {
        ixx: 1_000.0,
        iyy: 1_500.0,
        izz: 2_200.0,
        ixz,
    }
}

fn body(ixz: f64) -> MassProperties {
    MassProperties::new(MASS, inertia(ixz), None).unwrap()
}

fn tumbler() -> MassProperties {
    let i = Inertia {
        ixx: 1.0,
        iyy: 2.0,
        izz: 3.0,
        ixz: 0.0,
    };
    MassProperties::new(1.0, i, None).unwrap()
}

/// Only gravity (added by the integrator).
fn no_loads(_: &RigidBodyState) -> Loads {
    Loads::default()
}

fn moment(m: DVec3) -> impl Fn(&RigidBodyState) -> Loads {
    move |_| Loads {
        wrench: Wrench {
            force: DVec3::ZERO,
            moment: m,
        },
        rotor: RotorTorques::default(),
    }
}

/// Cancels gravity, so that a body floats freely.
fn weightless(mass: f64) -> impl Fn(&RigidBodyState) -> Loads {
    move |s| Loads {
        wrench: Wrench {
            force: -gravity_body(s.attitude, mass),
            moment: DVec3::ZERO,
        },
        rotor: RotorTorques::default(),
    }
}

fn simulate(
    mut state: RigidBodyState,
    mass: &MassProperties,
    dt: f64,
    duration: f64,
    loads: impl Fn(&RigidBodyState) -> Loads,
) -> RigidBodyState {
    for _ in 0..(duration / dt).round() as usize {
        state = step_rk4(&state, mass, dt, &loads);
    }
    state
}

// --- Mass properties -------------------------------------------------------

#[test]
fn rejects_unphysical_mass_properties() {
    let ok = inertia(100.0);
    assert_eq!(
        MassProperties::new(0.0, ok, None),
        Err(MassError::NonPositiveMass)
    );
    let flat = Inertia { izz: 4_000.0, ..ok }; // ixx + iyy < izz
    assert_eq!(
        MassProperties::new(1.0, flat, None),
        Err(MassError::InvalidInertia)
    );
    // Diagonal triangle inequalities hold, but ixz is too large for any real body.
    assert_eq!(
        MassProperties::new(1.0, inertia(1_000.0), None),
        Err(MassError::InvalidInertia)
    );
    let skewed_axis = Rotor {
        axis: DVec3::new(1.0, 0.1, 0.0),
        inertia: 5.0,
    };
    assert_eq!(
        MassProperties::new(1.0, ok, Some(skewed_axis)),
        Err(MassError::InvalidRotor)
    );
    assert!(MassProperties::new(1.0, ok, None).is_ok());
}

#[test]
fn rotor_axial_inertia_is_removed_from_the_airframe() {
    let rotor = Rotor {
        axis: DVec3::X,
        inertia: 5.0,
    };
    let m = MassProperties::new(MASS, inertia(0.0), Some(rotor)).unwrap();
    let i = m.airframe_inertia();
    assert_eq!(i.x_axis.x, 995.0);
    assert_eq!(i.y_axis.y, 1_500.0);
    assert_eq!(i.z_axis.z, 2_200.0);
}

#[test]
fn force_at_a_point_creates_the_lever_arm_moment() {
    // Lift up (−z) on the right wing (+y) rolls left (negative roll moment).
    let w = Wrench::force_at(DVec3::new(0.0, 0.0, -100.0), DVec3::new(0.0, 3.0, 0.0));
    assert_eq!(w.moment, DVec3::new(-300.0, 0.0, 0.0));
}

// --- Translation, frames, gravity -----------------------------------------

#[test]
fn free_fall_matches_exact_solution_in_any_attitude() {
    // RK4 integrates a constant acceleration exactly (up to rounding).
    let m = body(0.0);
    let attitude = DQuat::from_euler(EulerRot::ZYX, 0.7, FRAC_PI_6, -0.4);
    let start = RigidBodyState {
        attitude,
        ..Default::default()
    };
    let end = simulate(start, &m, DT, 3.0, no_loads);
    assert!(
        (end.position_ned.z - 0.5 * G0 * 9.0).abs() < 1e-9,
        "{end:?}"
    );
    assert!(end.position_ned.truncate().length() < 1e-9);
    assert!((end.velocity_ned() - DVec3::new(0.0, 0.0, G0 * 3.0)).length() < 1e-9);
    // Components, not `angle_between`: acos cannot resolve angles below ~1e-8.
    assert!((end.attitude - attitude).length() < 1e-12, "{end:?}");
}

#[test]
fn spinning_body_in_free_fall_follows_a_parabola() {
    // Catches a wrong or missing ω × v, and body/world mix-ups on the position.
    let m = body(80.0);
    let start = RigidBodyState {
        attitude: DQuat::from_euler(EulerRot::ZYX, 0.7, 0.4, -0.4),
        velocity_body: DVec3::new(50.0, 1.0, -2.0),
        angular_velocity_body: DVec3::new(1.5, 0.8, -0.6),
        ..Default::default()
    };
    let v0 = start.velocity_ned();
    let t = 5.0;
    let end = simulate(start, &m, DT, t, no_loads);
    let g = DVec3::new(0.0, 0.0, G0);
    assert!((end.velocity_ned() - (v0 + g * t)).length() < 1e-6);
    assert!((end.position_ned - (v0 * t + 0.5 * g * t * t)).length() < 1e-5);
}

#[test]
fn coordinated_right_turn_is_a_clockwise_circle() {
    // Yaw rate r > 0 with a centripetal force m V r along +y: circle of radius V / r
    // to the right (towards the east when starting north).
    let (v, r) = (40.0, 0.25);
    let m = body(0.0);
    let start = RigidBodyState {
        velocity_body: DVec3::new(v, 0.0, 0.0),
        angular_velocity_body: DVec3::new(0.0, 0.0, r),
        ..Default::default()
    };
    let t = (FRAC_PI_2 / r / DT).round() * DT;
    let centripetal = move |s: &RigidBodyState| Loads {
        wrench: Wrench {
            force: DVec3::new(0.0, MASS * v * r, 0.0) - gravity_body(s.attitude, MASS),
            moment: DVec3::ZERO,
        },
        rotor: RotorTorques::default(),
    };
    let end = simulate(start, &m, DT, t, centripetal);
    let (radius, psi) = (v / r, r * t);
    let expected = DVec3::new(radius * psi.sin(), radius * (1.0 - psi.cos()), 0.0);
    assert!((end.position_ned - expected).length() < 1e-6, "{end:?}");
    assert!((end.velocity_ned() - DVec3::new(v * psi.cos(), v * psi.sin(), 0.0)).length() < 1e-8);
}

#[test]
fn gravity_points_down_in_body_axes_when_pitched_up() {
    // Nose 90° up: "down" is −x in body axes.
    let g = gravity_body(DQuat::from_rotation_y(FRAC_PI_2), 1.0);
    assert!((g - DVec3::new(-G0, 0.0, 0.0)).length() < 1e-12, "{g}");
}

#[test]
fn total_energy_is_conserved_under_gravity() {
    let m = body(80.0);
    let start = RigidBodyState {
        position_ned: DVec3::new(0.0, 0.0, -1_000.0),
        velocity_body: DVec3::new(50.0, 3.0, -2.0),
        angular_velocity_body: DVec3::new(1.0, 0.5, -0.8),
        ..Default::default()
    };
    // Potential energy m g h with h = −z.
    let energy = |s: &RigidBodyState| s.kinetic_energy(&m) - MASS * G0 * s.position_ned.z;
    let end = simulate(start, &m, DT, 20.0, no_loads);
    let rel = (energy(&end) - energy(&start)) / energy(&start);
    assert!(rel.abs() < 1e-9, "relative energy drift {rel:e}");
}

// --- Rotation ---------------------------------------------------------------

#[test]
fn angular_velocity_is_in_body_axes() {
    // Rolled 90° right, a positive pitch rate swings the nose to the right (east).
    let m = body(0.0);
    let q = 0.5;
    let start = RigidBodyState {
        attitude: DQuat::from_rotation_x(FRAC_PI_2),
        angular_velocity_body: DVec3::new(0.0, q, 0.0),
        ..Default::default()
    };
    let end = simulate(start, &m, DT, 1.0, weightless(MASS));
    let nose = end.attitude * DVec3::X;
    assert!(
        (nose - DVec3::new(q.cos(), q.sin(), 0.0)).length() < 1e-9,
        "{nose}"
    );
}

#[test]
fn torque_free_motion_conserves_energy_and_angular_momentum() {
    // Spinning near the unstable intermediate axis: the body tumbles violently,
    // but energy and world-frame angular momentum are conserved.
    let m = tumbler();
    let start = RigidBodyState {
        angular_velocity_body: DVec3::new(0.05, 2.0, 0.05),
        ..Default::default()
    };
    let (e0, l0) = (start.kinetic_energy(&m), start.angular_momentum_ned(&m));
    let end = simulate(start, &m, DT, 20.0, weightless(1.0));
    let (e1, l1) = (end.kinetic_energy(&m), end.angular_momentum_ned(&m));
    assert!(((e1 - e0) / e0).abs() < 1e-7, "energy {e0} -> {e1}");
    assert!(
        (l1 - l0).length() / l0.length() < 1e-6,
        "momentum {l0} -> {l1}"
    );
    assert!((end.angular_velocity_body - start.angular_velocity_body).length() > 0.5);
}

#[test]
fn symmetric_top_precesses_at_the_analytic_rate() {
    // Ixx = Iyy: ω⊥ turns about the body z axis at λ = (Izz − Ixx) / Ixx · ωz.
    let (ixx, izz, wz, a) = (2.0, 3.0, 4.0, 0.3);
    let i = Inertia {
        ixx,
        iyy: ixx,
        izz,
        ixz: 0.0,
    };
    let m = MassProperties::new(1.0, i, None).unwrap();
    let start = RigidBodyState {
        angular_velocity_body: DVec3::new(a, 0.0, wz),
        ..Default::default()
    };
    let t = 1.5;
    let end = simulate(start, &m, DT, t, weightless(1.0));
    let lambda = (izz - ixx) / ixx * wz;
    let expected = DVec3::new(a * (lambda * t).cos(), a * (lambda * t).sin(), wz);
    assert!((end.angular_velocity_body - expected).length() < 1e-8);
}

#[test]
fn product_of_inertia_couples_roll_into_yaw_at_the_exact_ratio() {
    // From rest, a pure roll moment gives ṙ / ṗ = ixz / izz.
    let m = body(100.0);
    let end = step_rk4(
        &RigidBodyState::default(),
        &m,
        1e-4,
        moment(DVec3::new(500.0, 0.0, 0.0)),
    );
    let ratio = end.angular_velocity_body.z / end.angular_velocity_body.x;
    assert!((ratio - 100.0 / 2_200.0).abs() < 1e-9, "r/p = {ratio}");
}

#[test]
fn angular_impulse_equals_the_change_of_world_momentum() {
    // A moment fixed in world axes, applied to a spinning body.
    let m = body(80.0);
    let world_moment = DVec3::new(0.0, 300.0, 0.0);
    let start = RigidBodyState {
        angular_velocity_body: DVec3::new(6.0, 0.0, 0.0),
        ..Default::default()
    };
    let l0 = start.angular_momentum_ned(&m);
    let t = 2.0;
    let loads = move |s: &RigidBodyState| Loads {
        wrench: Wrench {
            force: -gravity_body(s.attitude, MASS),
            moment: s.attitude.conjugate() * world_moment,
        },
        rotor: RotorTorques::default(),
    };
    let end = simulate(start, &m, DT, t, loads);
    let err = (end.angular_momentum_ned(&m) - (l0 + world_moment * t)).length() / l0.length();
    assert!(err < 1e-8, "relative error {err:e}");
}

// --- Rotor -------------------------------------------------------------------

fn with_rotor(inertia_rotor: f64) -> MassProperties {
    let rotor = Rotor {
        axis: DVec3::X,
        inertia: inertia_rotor,
    };
    MassProperties::new(MASS, inertia(0.0), Some(rotor)).unwrap()
}

fn rotor_loads(engine: f64, propeller: f64) -> impl Fn(&RigidBodyState) -> Loads {
    move |s| Loads {
        wrench: Wrench {
            force: -gravity_body(s.attitude, MASS),
            moment: DVec3::ZERO,
        },
        rotor: RotorTorques {
            engine,
            propeller,
            friction: 0.0,
        },
    }
}

#[test]
fn engine_torque_spins_the_rotor_and_counter_rotates_the_airframe() {
    // In a vacuum: the engine torque is internal, so total angular momentum stays zero.
    let m = with_rotor(5.0);
    let end = simulate(
        RigidBodyState::default(),
        &m,
        DT,
        1.0,
        rotor_loads(100.0, 0.0),
    );
    assert!((end.rotor_speed - 20.0).abs() < 1e-9, "{end:?}");
    assert!((end.angular_velocity_body.x + 100.0 / 995.0).abs() < 1e-9);
    assert!(end.angular_momentum_ned(&m).length() < 1e-9);
    assert!((end.rotor_relative_speed(&m) - (20.0 + 100.0 / 995.0)).abs() < 1e-9);
}

#[test]
fn propeller_torque_brakes_the_rotor_without_turning_the_airframe() {
    // The propeller's aerodynamic torque is external and acts on the rotor only.
    let m = with_rotor(5.0);
    let start = RigidBodyState {
        rotor_speed: 200.0,
        ..Default::default()
    };
    let end = simulate(start, &m, DT, 1.0, rotor_loads(0.0, 50.0));
    assert!((end.rotor_speed - 190.0).abs() < 1e-9);
    assert!(end.angular_velocity_body.length() < 1e-12, "{end:?}");
}

#[test]
fn spinning_rotor_makes_a_right_yaw_pitch_the_nose_down() {
    // Rotor momentum along +x (as on a Camel): yawing right gives a pitch-down
    // gyroscopic moment −r·h, emerging from ω × h.
    let m = with_rotor(5.0);
    let start = RigidBodyState {
        angular_velocity_body: DVec3::new(0.0, 0.0, 0.5),
        rotor_speed: 120.0,
        ..Default::default()
    };
    let end = step_rk4(&start, &m, 1e-4, rotor_loads(0.0, 0.0));
    let q_dot = end.angular_velocity_body.y / 1e-4;
    let expected = -0.5 * 5.0 * 120.0 / 1_500.0;
    assert!(
        (q_dot - expected).abs() < 1e-3 * expected.abs(),
        "q̇ = {q_dot}"
    );
}

#[test]
fn rotor_motion_conserves_total_angular_momentum_while_tumbling() {
    let m = with_rotor(5.0);
    let start = RigidBodyState {
        angular_velocity_body: DVec3::new(0.3, 1.0, -0.6),
        rotor_speed: 150.0,
        ..Default::default()
    };
    let l0 = start.angular_momentum_ned(&m);
    let e0 = start.kinetic_energy(&m);
    let end = simulate(start, &m, DT, 10.0, rotor_loads(0.0, 0.0));
    assert!((end.angular_momentum_ned(&m) - l0).length() / l0.length() < 1e-8);
    assert!(((end.kinetic_energy(&m) - e0) / e0).abs() < 1e-8);
}

// --- Numerics -------------------------------------------------------------------

#[test]
fn error_shrinks_like_fourth_order() {
    // Halving the step divides the error by about 2⁴ = 16.
    let m = tumbler();
    let start = RigidBodyState {
        angular_velocity_body: DVec3::new(0.3, 1.0, 0.6),
        ..Default::default()
    };
    let reference = simulate(start, &m, 1.0 / 3_840.0, 2.0, weightless(1.0));
    let error = |dt: f64| {
        let s = simulate(start, &m, dt, 2.0, weightless(1.0));
        (s.angular_velocity_body - reference.angular_velocity_body).length()
    };
    let ratio = error(1.0 / 60.0) / error(1.0 / 120.0);
    assert!((10.0..24.0).contains(&ratio), "error ratio {ratio}");
}

#[test]
fn quaternion_stays_unit_after_a_long_tumble() {
    let m = tumbler();
    let start = RigidBodyState {
        angular_velocity_body: DVec3::new(0.05, 4.0, 0.05),
        ..Default::default()
    };
    let end = simulate(start, &m, 1.0 / 60.0, 60.0, weightless(1.0));
    assert!((end.attitude.length() - 1.0).abs() < 1e-14);
}

#[test]
fn load_model_always_sees_a_unit_quaternion() {
    let m = body(0.0);
    let start = RigidBodyState {
        angular_velocity_body: DVec3::new(10.0, 0.0, 0.0),
        ..Default::default()
    };
    let worst = std::cell::Cell::new(0.0_f64);
    step_rk4(&start, &m, 1.0 / 120.0, |s| {
        worst.set(worst.get().max((s.attitude.length() - 1.0).abs()));
        Loads::default()
    });
    assert!(worst.get() < 1e-15, "max | |q| − 1 | = {:e}", worst.get());
}

/// Golden trajectory frozen bit for bit: catches any change of arithmetic order,
/// of integrator or of platform behaviour that would break replays.
#[test]
fn trajectory_is_reproducible_bit_for_bit() {
    let m = MassProperties::new(
        MASS,
        inertia(80.0),
        Some(Rotor {
            axis: DVec3::X,
            inertia: 5.0,
        }),
    )
    .unwrap();
    let start = RigidBodyState {
        velocity_body: DVec3::new(50.0, 1.0, -2.0),
        angular_velocity_body: DVec3::new(0.3, 1.0, 0.6),
        rotor_speed: 150.0,
        ..Default::default()
    };
    let end = simulate(start, &m, DT, 5.0, rotor_loads(20.0, 15.0));
    let bits: Vec<u64> = [
        end.position_ned.to_array(),
        end.velocity_body.to_array(),
        end.angular_velocity_body.to_array(),
    ]
    .iter()
    .flatten()
    .chain(end.attitude.to_array().iter())
    .chain(std::iter::once(&end.rotor_speed))
    .map(|v| v.to_bits())
    .collect();
    assert_eq!(bits, GOLDEN, "values: {end:?}");
}

// Recorded on 2026-10-20 (x86_64, Rust 1.99.1). Update only for a deliberate change
// of the integrator, and say so in the commit message.
const GOLDEN: [u64; 14] = [
    4643000109586379871,
    4617315517964024657,
    13845191154444474800,
    4631499741830481134,
    4622273110157196362,
    4626198387796587693,
    4601993944194938805,
    13831177834952618701,
    4599502237317354833,
    4606196127353382555,
    4581090097305902862,
    4596620250780539323,
    13824336082511140949,
    4639657594238009184,
];
