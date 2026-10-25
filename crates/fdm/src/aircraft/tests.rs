//! A synthetic 1917-like biplane (not historical data: round numbers of the right order).

use std::f64::consts::PI;

use glam::{DQuat, DVec3, EulerRot};

use super::*;
use crate::aero::body::SlenderBody;
use crate::aero::polar::{Polar, PolarPoint, viterna_cd_max};
use crate::aero::surface::{
    ControlChannel, InducedModel, LiftingSurface, Strip, StripControl, flap_effectiveness,
};
use crate::aero::system::MutualInduction;
use crate::environment::StandardEnvironment;
use crate::propulsion::engine::tests::engine;
use crate::propulsion::propeller::tests::{anchors, table};
use crate::propulsion::propeller::{Propeller, PropellerGeometry};
use crate::rigid_body::{Inertia, Rotor, Wrench};

const SEA_LEVEL: StandardEnvironment = StandardEnvironment {
    wind_ned: DVec3::ZERO,
    ground_altitude: 0.0,
};

fn section(cd0: f64, cm: f64) -> Polar {
    let peak = 2.0 * PI * 14f64.to_radians();
    let points = (-20..=20)
        .map(|deg: i32| {
            let alpha = f64::from(deg).to_radians();
            let a = deg.unsigned_abs();
            let cl = if a <= 14 {
                2.0 * PI * alpha
            } else {
                alpha.signum() * peak * (1.0 - 0.2 * f64::from(a - 14) / 6.0)
            };
            PolarPoint {
                alpha,
                cl,
                cd: cd0 + 0.5 * alpha * alpha,
                cm,
            }
        })
        .collect();
    Polar::new(points, viterna_cd_max(6.0)).unwrap()
}

/// Horizontal plane of `strips` strips; `control(y)` gives each strip's control.
fn plane(
    span: f64,
    chord: f64,
    centre: DVec3,
    incidence: f64,
    strips: u32,
    control: impl Fn(f64) -> Option<StripControl>,
) -> Vec<Strip> {
    let width = span / f64::from(strips);
    let chord_axis = DVec3::new(incidence.cos(), 0.0, -incidence.sin());
    let normal = DVec3::new(-incidence.sin(), 0.0, -incidence.cos());
    (0..strips)
        .map(|i| {
            let y = -0.5 * span + (f64::from(i) + 0.5) * width;
            Strip {
                position: centre + DVec3::new(0.0, y, 0.0),
                chord_axis,
                normal,
                area: width * chord,
                chord,
                control: control(y),
            }
        })
        .collect()
}

fn biplane() -> Aircraft {
    let aileron = flap_effectiveness(0.25);
    let elevator = flap_effectiveness(0.4);
    let rudder = flap_effectiveness(0.5);
    let wing = |name: &str, span: f64, z: f64, x: f64, ailerons: bool| {
        let strips = plane(span, 1.4, DVec3::new(x, 0.0, z), 0.03, 8, |y| {
            (ailerons && y.abs() > 2.5).then_some(StripControl {
                channel: ControlChannel::Aileron,
                // Positive roll: right wing lift down, left wing lift up.
                alpha_per_rad: -y.signum() * aileron,
            })
        });
        let induced = InducedModel::new(span / 1.4, 0.85);
        LiftingSurface::new(name.into(), strips, section(0.012, -0.05), induced).unwrap()
    };
    let tailplane = LiftingSurface::new(
        "tailplane".into(),
        plane(3.0, 0.8, DVec3::new(-4.6, 0.0, -0.2), -0.01, 2, |_| {
            Some(StripControl {
                channel: ControlChannel::Elevator,
                alpha_per_rad: -elevator,
            })
        }),
        section(0.012, 0.0),
        InducedModel::new(3.75, 0.85),
    )
    .unwrap();
    let fin = LiftingSurface::new(
        "fin".into(),
        vec![Strip {
            position: DVec3::new(-4.9, 0.0, -0.6),
            chord_axis: DVec3::X,
            normal: DVec3::Y,
            area: 1.0,
            chord: 0.9,
            control: Some(StripControl {
                channel: ControlChannel::Rudder,
                alpha_per_rad: -rudder,
            }),
        }],
        section(0.012, 0.0),
        InducedModel::new(1.6, 0.85),
    )
    .unwrap();
    let (b1, b2, c) = (8.0, 7.6, 1.4);
    let sigma = 0.5;
    let lifting = LiftingSystem::new(
        vec![
            wing("upper", b1, -1.0, 0.2, true),
            wing("lower", b2, 0.3, 0.0, false),
            tailplane,
            fin,
        ],
        vec![
            MutualInduction {
                on: 0,
                from: 1,
                factor: sigma * b2 * c / (PI * b1 * b2),
            },
            MutualInduction {
                on: 1,
                from: 0,
                factor: sigma * b1 * c / (PI * b1 * b2),
            },
        ],
    )
    .unwrap();
    let drag = |name: &str, drag_area: f64, position: DVec3| DragItem {
        name: name.into(),
        drag_area,
        position,
    };
    let propeller = Propeller::new(
        table(),
        anchors(),
        PropellerGeometry {
            diameter: 2.5,
            hub: DVec3::new(1.8, 0.0, 0.0),
            thrust_axis: DVec3::X,
            right_handed: true,
            p_factor: 0.7,
            normal_force: 0.3,
        },
    )
    .unwrap();
    let rotor = Rotor {
        axis: propeller.rotation_axis(),
        inertia: 1.2,
    };
    let inertia = Inertia {
        ixx: 1_100.0,
        iyy: 1_300.0,
        izz: 2_200.0,
        ixz: 40.0,
    };
    Aircraft::new(
        MassProperties::new(700.0, inertia, Some(rotor)).unwrap(),
        lifting,
        vec![
            drag("fuselage", 0.35, DVec3::new(0.0, 0.0, 0.0)),
            drag("struts_right", 0.07, DVec3::new(0.1, 2.5, -0.35)),
            drag("struts_left", 0.07, DVec3::new(0.1, -2.5, -0.35)),
            drag("undercarriage", 0.1, DVec3::new(0.6, 0.0, 1.0)),
        ],
        vec![SlenderBody {
            name: "fuselage".into(),
            position: DVec3::new(-0.8, 0.0, 0.0),
            volume: 3.0,
            apparent_mass: 0.9,
            plan_area: 4.0,
            side_area: 5.0,
            crossflow_drag: 0.8,
        }],
        Some(Propulsion {
            propeller,
            engine: engine(),
        }),
        Some(Downwash {
            wings: vec![0, 1],
            tail: vec![2],
            factor: 0.11,
            tail_arm: 4.7,
            swirl_left: 0.5,
        }),
        ControlTravel {
            aileron: 0.35,
            elevator: 0.45,
            rudder: 0.5,
        },
    )
    .unwrap()
}

/// Flying at `speed` with angle of attack `alpha`, rotor at `rotor` rad/s.
fn flying(speed: f64, alpha: f64, rotor: f64) -> RigidBodyState {
    RigidBodyState {
        position_ned: DVec3::new(0.0, 0.0, -1_000.0),
        attitude: DQuat::from_euler(EulerRot::ZYX, 0.0, alpha, 0.0),
        velocity_body: DVec3::new(speed * alpha.cos(), 0.0, speed * alpha.sin()),
        rotor_speed: rotor,
        ..Default::default()
    }
}

const GLIDE: PilotControls = PilotControls {
    roll: 0.0,
    pitch: 0.0,
    yaw: 0.0,
    throttle: 0.0,
};

#[test]
fn at_rest_there_is_nothing_but_a_finite_zero() {
    let d = biplane().evaluate(&RigidBodyState::default(), &GLIDE, &SEA_LEVEL);
    assert_eq!(d.loads.wrench, Wrench::default());
    assert_eq!(d.loads.rotor, RotorTorques::default());
    assert_eq!(d.loads.aux_rates, [0.0; AUX_STATES]);
}

#[test]
fn air_moving_with_the_aircraft_gives_no_aerodynamic_load() {
    let wind = StandardEnvironment {
        wind_ned: DVec3::new(20.0, -5.0, 1.0),
        ground_altitude: 0.0,
    };
    let state = RigidBodyState {
        attitude: DQuat::from_euler(EulerRot::ZYX, 0.6, 0.1, -0.2),
        ..Default::default()
    };
    let state = RigidBodyState {
        velocity_body: state.attitude.conjugate() * wind.wind_ned,
        ..state
    };
    let d = biplane().evaluate(&state, &GLIDE, &wind);
    assert!(d.air_velocity.length() < 1e-12);
    assert!(d.loads.wrench.force.length() < 1e-9, "{:?}", d.loads);
}

#[test]
fn symmetric_glide_has_no_lateral_loads() {
    // Propeller windmilling freely (zero aerodynamic torque): no swirl and no induced
    // speed (negative thrust), hence no side force and no rolling moment. (The pitch
    // cross flow gives the turning propeller a P-factor yaw.)
    let a = biplane();
    let torque = |rotor: f64| {
        a.evaluate(&flying(40.0, 0.06, rotor), &GLIDE, &SEA_LEVEL)
            .propeller
            .unwrap()
            .torque
    };
    let (mut lo, mut hi) = (0.0, 300.0);
    assert!(torque(lo) < 0.0 && torque(hi) > 0.0);
    for _ in 0..200 {
        let mid = 0.5 * (lo + hi);
        if torque(mid) < 0.0 {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    let d = a.evaluate(&flying(40.0, 0.06, lo), &GLIDE, &SEA_LEVEL);
    let w = d.loads.wrench;
    assert!(w.force.y.abs() < 1e-6 && w.moment.x.abs() < 1e-6, "{w:?}");
    assert!(w.force.z < 0.0, "lift: {w:?}");
    assert!(d.propeller.unwrap().thrust < 0.0);
}

#[test]
fn a_stopped_propeller_in_the_flow_turns_the_air() {
    // Rotor at rest: the flow drives the twisted blades (it would windmill), and the
    // reaction swirls the slipstream against the normal rotation (review of 2026-10-24:
    // the swirl no longer depends on the sign of the thrust).
    let a = biplane();
    let stopped = a.evaluate(&flying(40.0, 0.06, 0.0), &GLIDE, &SEA_LEVEL);
    let p = stopped.propeller.unwrap();
    assert!(p.thrust < 0.0 && stopped.loads.rotor.propeller < 0.0);
    // Absorbing power, the propeller swirls the air the other way: the rolling moment
    // of the swirl on the wings changes sign with the propeller torque.
    let absorbing = a.evaluate(&flying(40.0, 0.06, 250.0), &GLIDE, &SEA_LEVEL);
    assert!(absorbing.propeller.unwrap().torque > 0.0);
    let (roll_stopped, roll_absorbing) = (
        stopped.loads.wrench.moment.x,
        absorbing.loads.wrench.moment.x,
    );
    assert!(
        roll_stopped * roll_absorbing < 0.0,
        "{roll_stopped} {roll_absorbing}"
    );
}

#[test]
fn the_airframe_alone_has_no_lateral_load_in_a_symmetric_glide() {
    // Without propulsion (no torque, swirl nor P-factor), the mirrored airframe gives
    // no side force, rolling or yawing moment.
    let a = biplane();
    let glider = Aircraft::new(
        a.mass,
        a.lifting.clone(),
        a.drag.clone(),
        a.bodies.clone(),
        None,
        a.downwash.clone(),
        a.travel,
    )
    .unwrap();
    let w = glider
        .evaluate(&flying(40.0, 0.06, 0.0), &GLIDE, &SEA_LEVEL)
        .loads
        .wrench;
    let scale = w.force.length();
    assert!(
        w.force.y.abs() < 1e-9 * scale
            && w.moment.x.abs() < 1e-9 * scale
            && w.moment.z.abs() < 1e-9 * scale,
        "{w:?}"
    );
}

#[test]
fn engine_torque_rolls_the_airframe_against_the_propeller() {
    // Right-handed propeller (clockwise from the cockpit): reaction rolls to the left.
    let a = biplane();
    let state = flying(40.0, 0.06, 200.0);
    let full = PilotControls {
        throttle: 1.0,
        ..GLIDE
    };
    let d = a.evaluate(&state, &full, &SEA_LEVEL);
    let t = d.loads.rotor;
    assert!(t.engine > 0.0 && t.propeller > 0.0);
    assert!(d.propeller.unwrap().thrust > 0.0);
    // What the integrator applies to the airframe: −(Q_engine − Q_friction) about +x.
    let reaction = -(t.engine - t.friction);
    assert!(reaction < 0.0);
}

#[test]
fn controls_move_the_aircraft_the_right_way() {
    let a = biplane();
    let state = flying(45.0, 0.05, 0.0);
    let base = a.evaluate(&state, &GLIDE, &SEA_LEVEL).loads.wrench.moment;
    let with = |controls: PilotControls| {
        a.evaluate(&state, &controls, &SEA_LEVEL)
            .loads
            .wrench
            .moment
            - base
    };
    let roll = with(PilotControls { roll: 0.5, ..GLIDE });
    let pitch = with(PilotControls {
        pitch: 0.5,
        ..GLIDE
    });
    let yaw = with(PilotControls { yaw: 0.5, ..GLIDE });
    assert!(roll.x > 0.0, "{roll:?}");
    assert!(pitch.y > 0.0, "{pitch:?}");
    assert!(yaw.z > 0.0, "{yaw:?}");
    // Commands are clamped.
    assert_eq!(
        with(PilotControls {
            pitch: 3.0,
            ..GLIDE
        }),
        with(PilotControls {
            pitch: 1.0,
            ..GLIDE
        })
    );
}

#[test]
fn downwash_lags_towards_its_target_and_lifts_the_nose() {
    let a = biplane();
    let state = flying(45.0, 0.08, 0.0);
    let d = a.evaluate(&state, &GLIDE, &SEA_LEVEL);
    let speed = 45.0;
    let wings = (d.lift_coefficients[0] * 8.0 + d.lift_coefficients[1] * 7.6) / 15.6;
    let target = 0.11 * wings;
    assert!(target > 0.0);
    let rate = d.loads.aux_rates[AUX_DOWNWASH];
    assert!((rate - target * speed / 4.7).abs() < 1e-12 * rate.abs());

    let mut settled = state;
    settled.aux[AUX_DOWNWASH] = target;
    let e = a.evaluate(&settled, &GLIDE, &SEA_LEVEL);
    assert!(e.loads.aux_rates[AUX_DOWNWASH].abs() < 1e-12);
    // Downwash lowers the tail's angle of attack: less tail lift, nose-up moment.
    assert!(e.lift_coefficients[2] < d.lift_coefficients[2]);
    assert!(e.loads.wrench.moment.y > d.loads.wrench.moment.y);
}

#[test]
fn slipstream_makes_the_elevator_more_effective_at_low_speed() {
    let a = biplane();
    let full = PilotControls {
        throttle: 1.0,
        ..GLIDE
    };
    let elevator_effect = |rotor: f64| {
        let state = flying(25.0, 0.1, rotor);
        let up = PilotControls { pitch: 0.5, ..full };
        a.evaluate(&state, &up, &SEA_LEVEL).loads.wrench.moment.y
            - a.evaluate(&state, &full, &SEA_LEVEL).loads.wrench.moment.y
    };
    assert!(elevator_effect(200.0) > 1.2 * elevator_effect(0.0));
}

#[test]
fn rejects_inconsistent_aircraft() {
    let a = biplane();
    let wrong_rotor = MassProperties::new(
        700.0,
        Inertia {
            ixx: 1_100.0,
            iyy: 1_300.0,
            izz: 2_200.0,
            ixz: 0.0,
        },
        Some(Rotor {
            axis: -DVec3::X,
            inertia: 1.2,
        }),
    )
    .unwrap();
    let travel = a.travel;
    let rebuild = |mass: MassProperties, downwash: Option<Downwash>, travel: ControlTravel| {
        Aircraft::new(
            mass,
            a.lifting.clone(),
            vec![],
            vec![],
            a.propulsion.clone(),
            downwash,
            travel,
        )
    };
    assert_eq!(
        rebuild(wrong_rotor, None, travel),
        Err(AircraftError::RotorMismatch)
    );
    let downwash = |wings: Vec<usize>, tail: Vec<usize>| {
        Some(Downwash {
            wings,
            tail,
            factor: 0.1,
            tail_arm: 4.0,
            swirl_left: 0.5,
        })
    };
    let swirl_left =
        |swirl_left: f64| downwash(vec![0, 1], vec![2]).map(|d| Downwash { swirl_left, ..d });
    assert!(rebuild(a.mass, swirl_left(0.0), travel).is_ok());
    assert!(rebuild(a.mass, swirl_left(1.0), travel).is_ok());
    for bad in [
        downwash(vec![], vec![2]),
        downwash(vec![0], vec![7]),
        downwash(vec![0, 2], vec![2]),
        swirl_left(-0.1),
        swirl_left(1.1),
        swirl_left(f64::NAN),
    ] {
        assert_eq!(
            rebuild(a.mass, bad, travel),
            Err(AircraftError::BadDownwash)
        );
    }
    let bad_travel = ControlTravel {
        aileron: f64::NAN,
        ..travel
    };
    assert_eq!(
        rebuild(a.mass, None, bad_travel),
        Err(AircraftError::BadTravel)
    );
}

#[test]
fn flight_stays_finite_and_deterministic() {
    // Twenty seconds of untrimmed flight, then a stall and a tail slide: no NaN, and two
    // runs are identical to the bit.
    let a = biplane();
    let run = || {
        let mut s = flying(45.0, 0.05, 180.0);
        let dt = 1.0 / 240.0;
        for i in 0..4_800 {
            let controls = PilotControls {
                roll: (f64::from(i) * 0.01).sin() * 0.3,
                pitch: if i < 2_400 { 0.1 } else { 1.0 },
                yaw: 0.0,
                throttle: if i < 2_400 { 0.8 } else { 0.0 },
            };
            s = a.step(&s, &controls, &SEA_LEVEL, dt);
        }
        // Tail slide: falling backwards, nose up.
        let mut t = RigidBodyState {
            velocity_body: DVec3::new(-15.0, 0.0, 2.0),
            ..flying(0.0, 1.4, 150.0)
        };
        for _ in 0..1_200 {
            t = a.step(&t, &GLIDE, &SEA_LEVEL, dt);
        }
        (s, t)
    };
    let (s, t) = run();
    for state in [s, t] {
        let values = [
            state.position_ned.to_array(),
            state.velocity_body.to_array(),
            state.angular_velocity_body.to_array(),
        ];
        assert!(
            values.iter().flatten().all(|v| v.is_finite())
                && state.rotor_speed.is_finite()
                && state.aux.iter().all(|v| v.is_finite()),
            "{state:?}"
        );
    }
    assert_eq!(run(), (s, t));
}

// --- Aerodynamicist review of 2026-10-21: what the assembly must not lose ---------------

const FULL: PilotControls = PilotControls {
    roll: 0.0,
    pitch: 0.0,
    yaw: 0.0,
    throttle: 1.0,
};

#[test]
fn total_loads_are_the_sum_of_the_components() {
    // Engine running, sideslip and rates: every component, thrust included, must be in
    // the total, with the slipstream and the downwash where they belong.
    let a = biplane();
    let mut state = flying(40.0, 0.08, 200.0);
    state.velocity_body.y = 3.0;
    state.angular_velocity_body = DVec3::new(0.2, -0.1, 0.15);
    state.aux[AUX_DOWNWASH] = 0.02;
    let d = a.evaluate(&state, &FULL, &SEA_LEVEL);
    let (v, w, rho) = (d.air_velocity, state.angular_velocity_body, d.density);
    let p = a.propulsion().unwrap();
    let prop = p.propeller.loads(v, w, state.rotor_speed, rho);
    let wake = a.wing_wake().unwrap();
    let slip = p.propeller.slipstream(&prop, rho).behind_wings(wake);
    let blown = |r: DVec3| slip.local_flow(r);
    let downwash = downwash_velocity(v, 0.02);
    let surfaces = a.lifting.surfaces();
    let lifting = a
        .lifting
        .loads(v, w, rho, &ControlDeflections::default(), |i, strip| {
            let mut felt = slip.lifting_flow(
                strip.position,
                strip.chord_axis.cross(strip.normal),
                strip.chord,
                surfaces[i].section_slope(),
            );
            if i == 2 {
                felt.velocity += downwash;
            }
            felt
        });
    let mut expected = lifting.wrench + prop.wrench;
    for item in &a.drag {
        expected += item.loads(v, w, rho, blown);
    }
    for body in &a.bodies {
        expected += body.loads(v, w, rho, blown);
    }
    assert!(
        (d.loads.wrench.force - expected.force).length() < 1e-9,
        "{:?} {expected:?}",
        d.loads
    );
    assert!((d.loads.wrench.moment - expected.moment).length() < 1e-9);
    assert!(prop.thrust > 0.0);
    // The same pieces through the aircraft's own accessors.
    assert_eq!(a.slipstream(&prop, rho), Some(slip));
    let own = a.lifting_loads(
        &state,
        &ControlDeflections::default(),
        &SEA_LEVEL,
        Some(&slip),
    );
    assert_eq!(own.wrench, lifting.wrench);
    let sum = own
        .surface_wrenches
        .iter()
        .fold(Wrench::default(), |acc, &w| acc + w);
    assert!((sum.force - own.wrench.force).length() < 1e-9);
}

#[test]
fn the_wings_straighten_the_swirl_from_their_trailing_edge() {
    let a = biplane();
    let wake = a.wing_wake().unwrap();
    let hub = a.propulsion().unwrap().propeller.geometry().hub;
    let wings = &a.downwash().unwrap().wings;
    let (mut rearmost, mut chords, mut count) = (f64::NEG_INFINITY, 0.0, 0.0);
    for &i in wings {
        for s in a.lifting.surfaces()[i].strips() {
            // Trailing edge, the chord tilted by the incidence; thrust along +x.
            let trailing_edge = s.position - s.chord_axis * (0.75 * s.chord);
            rearmost = rearmost.max(hub.x - trailing_edge.x);
            chords += s.chord * s.area;
            count += s.area;
        }
    }
    assert!((wake.start - rearmost).abs() < 1e-9, "{wake:?} {rearmost}");
    assert!((wake.length - chords / count).abs() < 1e-9, "{wake:?}");
    assert_eq!(wake.swirl_left, 0.5);
    // The aircraft's slipstream carries it: half the swirl at the tail, all at the wings.
    let state = flying(40.0, 0.05, 200.0);
    let d = a.evaluate(&state, &FULL, &SEA_LEVEL);
    let prop = d.propeller.unwrap();
    let free = a
        .propulsion()
        .unwrap()
        .propeller
        .slipstream(&prop, d.density);
    let ours = a.slipstream(&prop, d.density).unwrap();
    let swirl =
        |s: &Slipstream, distance: f64| s.local_flow(hub + DVec3::new(-distance, 0.6, 0.0)).z;
    let at_wings = wake.start - wake.length;
    assert_eq!(swirl(&ours, at_wings), swirl(&free, at_wings));
    let at_tail = wake.start + 2.0 * wake.length;
    assert!((swirl(&ours, at_tail) - 0.5 * swirl(&free, at_tail)).abs() < 1e-12);
}

#[test]
fn propeller_sees_the_absolute_rotor_speed_and_the_engine_the_relative_one() {
    let a = biplane();
    let mut state = flying(40.0, 0.05, 200.0);
    state.angular_velocity_body = DVec3::new(1.5, 0.0, 0.0);
    let d = a.evaluate(&state, &FULL, &SEA_LEVEL);
    assert!((d.shaft_speed - (200.0 - 1.5)).abs() < 1e-12);
    let p = a.propulsion().unwrap();
    let absolute = p.propeller.loads(
        d.air_velocity,
        state.angular_velocity_body,
        200.0,
        d.density,
    );
    assert_eq!(d.propeller.unwrap(), absolute);
    assert_eq!(
        d.engine,
        p.engine
            .torques(200.0 - 1.5, 1.0, d.density / crate::constants::RHO0)
    );
}

#[test]
fn closing_the_throttle_slows_the_rotor_and_altitude_reduces_the_power() {
    let a = biplane();
    let state = RigidBodyState {
        rotor_speed: 220.0,
        ..RigidBodyState::default()
    };
    let idle = a.evaluate(&state, &GLIDE, &SEA_LEVEL).loads.rotor;
    assert!(
        idle.engine - idle.propeller - idle.friction < 0.0,
        "{idle:?}"
    );

    let low = a.evaluate(&flying(45.0, 0.05, 200.0), &FULL, &SEA_LEVEL);
    let mut high_state = flying(45.0, 0.05, 200.0);
    high_state.position_ned.z = -3_000.0;
    let high = a.evaluate(&high_state, &FULL, &SEA_LEVEL);
    let sigma = high.density / low.density;
    let low_altitude_density = crate::atmosphere::isa(1_000.0).density;
    assert!((low.density - low_altitude_density).abs() < 1e-12);
    // Full throttle: the indicated torque follows the density.
    let ratio = high.engine.combustion / low.engine.combustion;
    assert!((ratio - sigma).abs() < 1e-12, "{ratio} vs {sigma}");
}

#[test]
fn slipstream_blows_on_the_drag_items_and_the_wings() {
    // With the engine running, the loads differ from the same state computed without
    // slipstream, and a drag item inside the tube drags more (aerodynamicist review:
    // removing the slipstream from either went unnoticed).
    let a = biplane();
    let state = flying(30.0, 0.08, 220.0);
    let d = a.evaluate(&state, &FULL, &SEA_LEVEL);
    let (v, w, rho) = (d.air_velocity, state.angular_velocity_body, d.density);
    let p = a.propulsion().unwrap();
    let prop = p.propeller.loads(v, w, state.rotor_speed, rho);
    let slip = p.propeller.slipstream(&prop, rho);
    let fuselage = &a.drag[0];
    let blown = fuselage
        .loads(v, w, rho, |r| slip.local_flow(r))
        .force
        .length();
    let free = fuselage.loads(v, w, rho, |_| DVec3::ZERO).force.length();
    assert!(blown > 1.2 * free, "{blown} vs {free}");
    let unblown = a
        .lifting
        .loads(v, w, rho, &ControlDeflections::default(), |_, _| {
            DVec3::ZERO
        })
        .wrench;
    let blown_lifting = a
        .lifting
        .loads(v, w, rho, &ControlDeflections::default(), |_, strip| {
            slip.local_flow(strip.position)
        })
        .wrench;
    assert!((blown_lifting.force - unblown.force).length() > 100.0);
}

#[test]
fn downwash_stays_in_the_plane_of_symmetry_and_fades_backwards() {
    for v in [
        DVec3::new(40.0, 0.0, 3.0),
        DVec3::new(40.0, 8.0, 3.0),
        DVec3::new(30.0, -6.0, -2.0),
    ] {
        let d = downwash_velocity(v, 0.05);
        assert_eq!(d.y, 0.0);
        let symmetric = DVec3::new(v.x, 0.0, v.z);
        let forward = v.x / v.length();
        assert!((d.length() - 0.05 * symmetric.length() * forward).abs() < 1e-12);
        assert!(d.dot(symmetric).abs() < 1e-9, "perpendicular to the flow");
        assert!(d.z > 0.0, "downwards for positive downwash");
    }
    assert_eq!(
        downwash_velocity(DVec3::new(-30.0, 0.0, 5.0), 0.05),
        DVec3::ZERO
    );
    assert_eq!(downwash_velocity(DVec3::ZERO, 0.05), DVec3::ZERO);
}

#[test]
fn a_pusher_leaves_the_swirl_to_the_tail() {
    // Wings ahead of the disc: no wake of the wings in the slipstream.
    let a = biplane();
    let geometry = PropellerGeometry {
        hub: DVec3::new(-2.5, 0.0, 0.0),
        ..*a.propulsion().unwrap().propeller.geometry()
    };
    let pusher = Aircraft::new(
        a.mass,
        a.lifting.clone(),
        vec![],
        vec![],
        Some(Propulsion {
            propeller: Propeller::new(table(), anchors(), geometry).unwrap(),
            engine: engine(),
        }),
        a.downwash.clone(),
        a.travel,
    )
    .unwrap();
    assert!(a.wing_wake().is_some());
    assert_eq!(pusher.wing_wake(), None);
}
