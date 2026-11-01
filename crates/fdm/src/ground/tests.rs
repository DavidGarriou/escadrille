//! A taildragger of round numbers, of the order of a 1917 fighter (not historical data):
//! ground forces alone, gravity added by the integrator.

use glam::{DQuat, EulerRot};

use super::*;
use crate::constants::G0;
use crate::environment::StandardEnvironment;
use crate::rigid_body::{Inertia, Loads, MassProperties, step_rk4};

const MASS: f64 = 700.0;
const DT: f64 = 1.0 / 240.0;
const FLAT: StandardEnvironment = StandardEnvironment {
    wind_ned: DVec3::ZERO,
    ground_altitude: 0.0,
};

const WHEEL_SPRING: Spring = Spring {
    contact_stiffness: 250_000.0,
    preload: 500.0,
    stiffness: 75_000.0,
    travel: 0.125,
    stop_stiffness: 2_000_000.0,
    damping: 4_500.0,
};

const SKID_SPRING: Spring = Spring {
    contact_stiffness: 100_000.0,
    preload: 300.0,
    stiffness: 20_000.0,
    travel: 0.10,
    stop_stiffness: 2_000_000.0,
    damping: 1_500.0,
};

fn mass() -> MassProperties {
    let inertia = Inertia {
        ixx: 800.0,
        iyy: 800.0,
        izz: 1_400.0,
        ixz: 0.0,
    };
    MassProperties::new(MASS, inertia, None).unwrap()
}

fn wheel(y: f64) -> Wheel {
    Wheel {
        axle: DVec3::new(0.6, y, 1.2),
        radius: 0.3,
        guide: DVec3::new(-0.3f64.sin(), 0.0, -0.3f64.cos()),
        spring: WHEEL_SPRING,
        rolling: 0.07,
        grip: 0.6,
        cornering: 0.2,
    }
}

fn gear() -> Undercarriage {
    Undercarriage::new(
        vec![wheel(0.77), wheel(-0.77)],
        vec![Skid {
            shoe: DVec3::new(-3.9, 0.0, 0.35),
            spring: SKID_SPRING,
            friction: 0.5,
        }],
        vec![HardPoint {
            position: DVec3::new(0.0, 3.9, -0.8),
            stiffness: 200_000.0,
            damping: 10_000.0,
            friction: 0.5,
        }],
    )
    .unwrap()
}

fn run(
    gear: &Undercarriage,
    env: &dyn Environment,
    mut state: RigidBodyState,
    seconds: f64,
    mut each: impl FnMut(&RigidBodyState),
) -> RigidBodyState {
    let mass = mass();
    for _ in 0..(seconds / DT) as usize {
        state = step_rk4(&state, &mass, DT, |s| Loads {
            wrench: gear.loads(s, env).wrench,
            ..Loads::default()
        });
        each(&state);
    }
    state
}

/// Pitched up by the three-point attitude of the gear, wheels and skid a few centimetres
/// above the ground: it settles from there.
fn just_above_the_ground() -> RigidBodyState {
    let pitch = ((1.2f64 + 0.3 - 0.35) / 4.5).atan();
    let attitude = DQuat::from_euler(EulerRot::ZYX, 0.0, pitch, 0.0);
    let lowest = attitude * DVec3::new(0.6, 0.77, 1.2);
    RigidBodyState {
        position_ned: DVec3::new(0.0, 0.0, -lowest.z - 0.33),
        attitude,
        ..RigidBodyState::default()
    }
}

fn energy(state: &RigidBodyState) -> f64 {
    state.kinetic_energy(&mass()) - MASS * G0 * state.position_ned.z
}

#[test]
fn a_spring_is_continuous_and_stiffens_at_its_stop() {
    let s = WHEEL_SPRING;
    let (mut last_load, mut last_travel) = (0.0, 0.0);
    for i in 1..=5_000 {
        let (load, travel) = s.load(f64::from(i) * 1e-4);
        assert!(load > last_load, "load falls at step {i}");
        assert!(load - last_load < 600.0, "load jumps at step {i}");
        assert!(travel >= last_travel, "travel falls at step {i}");
        (last_load, last_travel) = (load, travel);
    }
    assert_eq!(s.load(0.5 * s.preload / s.contact_stiffness).1, 0.0);
    // At the end of the travel: the cord's load, tyre and cord in series.
    let stop_load = s.preload + s.stiffness * s.travel;
    let (load, travel) = s.load(stop_load / s.contact_stiffness + s.travel);
    assert!((load - stop_load).abs() < 1e-6, "stop load {load}");
    assert!((travel - s.travel).abs() < 1e-9, "stop travel {travel}");
    let beyond = s.load(stop_load / s.contact_stiffness + s.travel + 0.01).0 - load;
    assert!(beyond > 2.0 * 0.01 * s.stiffness, "stop slope {beyond}");
}

#[test]
fn nothing_touches_in_the_air() {
    let state = RigidBodyState {
        position_ned: DVec3::new(0.0, 0.0, -2.0),
        velocity_body: DVec3::new(40.0, 0.0, 2.0),
        ..RigidBodyState::default()
    };
    assert_eq!(gear().loads(&state, &FLAT), GroundLoads::default());
    let high = RigidBodyState {
        position_ned: DVec3::new(0.0, 0.0, -1_000.0),
        ..state
    };
    assert_eq!(gear().loads(&high, &FLAT), GroundLoads::default());
}

#[test]
fn at_rest_the_loads_balance_the_weight_on_the_wheels_and_skid() {
    let gear = gear();
    let state = run(&gear, &FLAT, just_above_the_ground(), 20.0, |_| {});
    assert!(state.velocity_body.length() < 1e-3, "{:?}", state);
    assert!(state.angular_velocity_body.length() < 1e-3, "{:?}", state);
    let loads = gear.loads(&state, &FLAT);
    assert_eq!(loads.contacts.len(), 3, "{:?}", loads.contacts);
    assert!(loads.on_undercarriage());
    let sum: f64 = loads.contacts.iter().map(|c| c.load).sum();
    assert!((sum - MASS * G0).abs() < 1.0, "sum of the loads {sum}");
    // Moments about the wheels' line: the skid carries a fifth or so of the weight.
    let skid = loads.contacts[2].load / sum;
    assert!((0.05..0.3).contains(&skid), "skid's share {skid}");
    let [left, right] = [loads.contacts[0].load, loads.contacts[1].load];
    assert!((left - right).abs() < 1e-6, "{left} and {right}");
    assert!(loads.contacts[0].cord_travel > 0.0, "the cord gives");
}

#[test]
fn a_drop_bounces_lower_and_never_gains_energy() {
    let gear = gear();
    let start = RigidBodyState {
        position_ned: DVec3::new(0.0, 0.0, -1.55 - 0.30),
        ..RigidBodyState::default()
    };
    // The springs hold energy while they touch: it is compared in the air only.
    let (mut last, mut highest, mut touched) = (energy(&start), f64::MIN, false);
    run(&gear, &FLAT, start, 6.0, |s| {
        let touching = !gear.loads(s, &FLAT).contacts.is_empty();
        touched |= touching;
        if touching {
            return;
        }
        let e = energy(s);
        assert!(e <= last + 1e-6, "energy rises from {last} to {e}");
        last = e;
        if touched {
            highest = highest.max(-s.position_ned.z);
        }
    });
    assert!(touched);
    assert!(highest < 1.55 + 0.30, "bounced to {highest}");
}

#[test]
fn rolling_slows_down_at_the_rolling_resistance() {
    // On the wheels alone, tail up: one state, to read the resistance over the load.
    let gear = Undercarriage::new(vec![wheel(0.77), wheel(-0.77)], Vec::new(), Vec::new()).unwrap();
    let state = RigidBodyState {
        position_ned: DVec3::new(0.0, 0.0, -1.46),
        velocity_body: DVec3::new(10.0, 0.0, 0.0),
        ..RigidBodyState::default()
    };
    let loads = gear.loads(&state, &FLAT);
    let normal: f64 = loads.contacts.iter().map(|c| c.load).sum();
    assert!(normal > 0.0);
    assert!(
        (loads.wrench.force.x + 0.07 * normal).abs() < 1e-3 * normal,
        "{:?}",
        loads
    );
    assert!(
        loads.contacts[0].rolling_speed > 9.9,
        "{:?}",
        loads.contacts
    );
}

#[test]
fn the_side_force_opposes_the_slip_and_stops_at_the_grip() {
    let gear = Undercarriage::new(vec![wheel(0.77), wheel(-0.77)], Vec::new(), Vec::new()).unwrap();
    let at = |velocity: DVec3| {
        let state = RigidBodyState {
            position_ned: DVec3::new(0.0, 0.0, -1.46),
            velocity_body: velocity,
            ..RigidBodyState::default()
        };
        let l = gear.loads(&state, &FLAT);
        (
            l.wrench.force.y,
            l.contacts.iter().map(|c| c.load).sum::<f64>(),
        )
    };
    let (small, normal) = at(DVec3::new(20.0, 0.2, 0.0));
    assert!(small < 0.0, "side force {small}");
    // tan α = 0.01 of a saturation at 0.2: on the initial slope.
    let expected = 0.6 * normal * 0.01 / 0.2;
    assert!(
        (small + expected).abs() < 0.02 * expected,
        "{small} {expected}"
    );
    let (sliding, normal) = at(DVec3::new(0.0, 15.0, 0.0));
    assert!((sliding + 0.6 * normal).abs() < 1e-2 * normal, "{sliding}");
    assert!(at(DVec3::new(20.0, -0.2, 0.0)).0 > 0.0);
}

#[test]
fn the_skid_rubs_against_the_slide_in_every_direction() {
    let gear = Undercarriage::new(
        Vec::new(),
        vec![Skid {
            shoe: DVec3::ZERO,
            spring: SKID_SPRING,
            friction: 0.5,
        }],
        Vec::new(),
    )
    .unwrap();
    for velocity in [DVec3::new(10.0, 0.0, 0.0), DVec3::new(-3.0, 4.0, 0.0)] {
        let state = RigidBodyState {
            position_ned: DVec3::new(0.0, 0.0, 0.05),
            velocity_body: velocity,
            ..RigidBodyState::default()
        };
        let l = gear.loads(&state, &FLAT);
        let friction = DVec3::new(l.wrench.force.x, l.wrench.force.y, 0.0);
        let expected = 0.5 * l.contacts[0].load;
        assert!((friction.length() - expected).abs() < 1e-2 * expected);
        assert!(friction.normalize().dot(velocity.normalize()) < -0.999);
    }
}

#[test]
fn a_wing_tip_on_the_ground_pushes_it_up() {
    let gear = gear();
    let state = RigidBodyState {
        position_ned: DVec3::new(0.0, 0.0, -0.75),
        attitude: DQuat::from_euler(EulerRot::ZYX, 0.0, 0.0, 0.5),
        velocity_body: DVec3::new(15.0, 0.0, 0.0),
        ..RigidBodyState::default()
    };
    let loads = gear.loads(&state, &FLAT);
    let tip = loads
        .contacts
        .iter()
        .find(|c| c.point == ContactPoint::HardPoint(0))
        .expect("the tip touches");
    assert!(tip.load > 0.0, "{tip:?}");
    assert!(
        loads.wrench.moment.x < 0.0,
        "rolled back: {:?}",
        loads.wrench
    );
    assert!(
        !GroundLoads {
            contacts: vec![*tip],
            ..loads
        }
        .on_undercarriage()
    );
}

#[test]
fn the_ground_only_pushes() {
    let spring = WHEEL_SPRING;
    assert_eq!(spring.normal(spring.load(0.01).0, 0.01, -5.0), 0.0);
    // The damping comes in with the compression: no load in the instant of the touch.
    assert_eq!(spring.normal(spring.load(0.0).0, 0.0, 5.0), 0.0);
}

#[test]
fn friction_stays_finite_at_rest() {
    let gear = gear();
    let start = just_above_the_ground();
    let settled = run(&gear, &FLAT, start, 5.0, |s| {
        assert!(s.position_ned.is_finite() && s.velocity_body.is_finite());
    });
    let exactly_still = RigidBodyState {
        velocity_body: DVec3::ZERO,
        angular_velocity_body: DVec3::ZERO,
        ..settled
    };
    let force = gear.loads(&exactly_still, &FLAT).wrench.force;
    assert!(force.is_finite());
    assert!(force.y.abs() < 1e-6, "no side force at rest: {force}");
}

#[test]
fn the_same_run_gives_the_same_bits() {
    let gear = gear();
    let start = RigidBodyState {
        velocity_body: DVec3::new(20.0, 1.0, 0.5),
        ..just_above_the_ground()
    };
    let a = run(&gear, &FLAT, start, 3.0, |_| {});
    let b = run(&gear, &FLAT, start, 3.0, |_| {});
    assert_eq!(a, b);
}

#[test]
fn a_mirrored_run_stays_mirrored() {
    let gear = gear();
    let start = RigidBodyState {
        velocity_body: DVec3::new(20.0, 0.0, 0.0),
        ..just_above_the_ground()
    };
    let end = run(&gear, &FLAT, start, 3.0, |_| {});
    assert!(end.position_ned.y.abs() < 1e-6, "{end:?}");
    assert!(end.angular_velocity_body.x.abs() < 1e-6, "{end:?}");
    assert!(end.angular_velocity_body.z.abs() < 1e-6, "{end:?}");
}

#[test]
fn the_ground_s_height_moves_the_contact_with_it() {
    let high = StandardEnvironment {
        wind_ned: DVec3::ZERO,
        ground_altitude: 178.0,
    };
    let gear = gear();
    let state = just_above_the_ground();
    let raised = RigidBodyState {
        position_ned: state.position_ned - DVec3::Z * 178.0,
        ..state
    };
    let (a, b) = (
        run(&gear, &FLAT, state, 2.0, |_| {}),
        run(&gear, &high, raised, 2.0, |_| {}),
    );
    assert!((a.position_ned.z - (b.position_ned.z + 178.0)).abs() < 1e-9);
}

#[test]
fn a_bad_undercarriage_is_refused() {
    let mut bad = wheel(0.77);
    bad.radius = 0.0;
    assert_eq!(
        Undercarriage::new(vec![bad], Vec::new(), Vec::new()),
        Err(GroundError::BadContact)
    );
    let mut bad = wheel(0.77);
    bad.guide = DVec3::new(0.0, 0.0, -2.0);
    assert_eq!(
        Undercarriage::new(vec![bad], Vec::new(), Vec::new()),
        Err(GroundError::BadGuide)
    );
    let mut bad = wheel(0.77);
    bad.spring.travel = f64::NAN;
    assert_eq!(
        Undercarriage::new(vec![bad], Vec::new(), Vec::new()),
        Err(GroundError::BadContact)
    );
}

#[test]
fn the_propeller_s_disc_reaches_lowest_towards_the_ground() {
    let (hub, axis, radius) = (DVec3::new(1.2, 0.0, 0.0), DVec3::X, 1.2);
    let level = RigidBodyState {
        position_ned: DVec3::new(0.0, 0.0, -1.5),
        ..RigidBodyState::default()
    };
    let clearance = disc_clearance(&level, hub, axis, radius, &FLAT);
    assert!((clearance - 0.3).abs() < 1e-12, "{clearance}");
    // Nose down, the hub sinks; the disc keeps its radius below it, along its own plane.
    let pitch = 0.3f64;
    let nose_down = RigidBodyState {
        attitude: DQuat::from_euler(EulerRot::ZYX, 0.0, -pitch, 0.0),
        ..level
    };
    let expected = 1.5 - 1.2 * pitch.sin() - 1.2 * pitch.cos();
    let clearance = disc_clearance(&nose_down, hub, axis, radius, &FLAT);
    assert!(
        (clearance - expected).abs() < 1e-12,
        "{clearance} {expected}"
    );
}
