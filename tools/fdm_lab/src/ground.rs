//! Ground tests (plan `docs/plans/m2-train-sol.md`, step 4): the aircraft at rest, at full
//! power held back, starting to roll, taking off, stopping, landing hard, turned too hard
//! or pushed over its nose. Flown by a simple scripted pilot with the player's commands.
//!
//! No source gives these figures for the S.VII (`spad7.md` § 5.7.4): they are measured and
//! reported, not compared with a target. What matters is their order, and the behaviours
//! the testimonies describe.

use std::f64::consts::PI;

use escadrille_fdm::aircraft::{Aircraft, PilotControls};
use escadrille_fdm::environment::StandardEnvironment;
use escadrille_fdm::ground::{Contact, ContactPoint};
use escadrille_fdm::rigid_body::RigidBodyState;
use glam::{DQuat, DVec3, EulerRot};

use crate::performance::{PerformanceError, stall_speed};
use crate::qualitative::{Observed, TracePoint};

/// Level ground at sea level, in still air: the wheels stand on it.
pub const FIELD: StandardEnvironment = StandardEnvironment {
    wind_ned: DVec3::ZERO,
    ground_altitude: 0.0,
};

/// Integration step, s (ADR 0005: 120 Hz with two substeps).
const DT: f64 = 1.0 / 240.0;

const DEG: f64 = PI / 180.0;

/// Shaft speed the engine is started from, rad/s: its idle, near enough for it to settle.
const IDLE_ROTOR: f64 = 52.0;

/// Rate at which the throttle is opened for the start of the roll, per second.
const SLOW_OPENING: f64 = 0.02;
/// Speed that counts as rolling, m/s.
const MOVING: f64 = 0.5;
/// Time to open the throttle fully for the take-off, s (E:, a smooth opening).
const OPENING: f64 = 2.0;

/// The pilot's rudder (E:): full rudder for 0.4 rad off the heading, damped by the yaw rate.
const HEADING_GAIN: f64 = 2.5;
const YAW_DAMPING: f64 = 1.5;
/// The pilot's ailerons (E:): as in the qualitative scenarios.
const BANK_GAIN: f64 = 2.0;
const ROLL_DAMPING: f64 = 0.5;
/// The pilot's elevator on the attitude (E:): full travel for 0.5 rad, damped by the pitch
/// rate.
const PITCH_GAIN: f64 = 2.0;
const PITCH_DAMPING: f64 = 1.0;
/// Stick pushed to raise the tail (E:: « pousser un peu le manche », T8, but early: on the
/// S.VII, the tail « comes up almost at once », T9).
const TAIL_UP_STICK: f64 = -1.0;
/// Attitude held on the wheels, tail up, rad (E:: fuselage about level).
const TAIL_UP_PITCH: f64 = 2.0 * DEG;
/// Attitude of the initial climb, rad (E:).
const CLIMB_PITCH: f64 = 7.0 * DEG;
/// Speed of the pull, over the idle stall speed (E:).
const ROTATION: f64 = 1.15;
/// Height that ends a take-off, m (E:: the classic screen of 50 ft).
const SCREEN: f64 = 15.24;

/// Speed of a landing's roll, over the idle stall speed (E:: on the stall, three points).
const TOUCHDOWN: f64 = 1.0;
/// Speed that ends a roll, m/s.
const STOPPED: f64 = 0.5;

/// Time for which nothing should touch for the aircraft to count as flying, s.
const AIRBORNE: f64 = 0.5;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Loads {
    /// Heaviest wheel, N.
    wheel: f64,
    /// N.
    skid: f64,
    /// Heaviest hard point, N.
    hard_point: f64,
    /// Longest travel of a wheel's axle, m.
    travel: f64,
}

fn loads_of(contacts: &[Contact]) -> Loads {
    let mut out = Loads::default();
    for c in contacts {
        match c.point {
            ContactPoint::Wheel(_) => {
                out.wheel = out.wheel.max(c.load);
                out.travel = out.travel.max(c.cord_travel);
            }
            ContactPoint::Skid(_) => out.skid = out.skid.max(c.load),
            ContactPoint::HardPoint(_) => out.hard_point = out.hard_point.max(c.load),
        }
    }
    out
}

/// One instant on the ground.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rolled {
    pub time: f64,
    pub state: RigidBodyState,
    pub controls: PilotControls,
    /// Distance from the start, m.
    pub distance: f64,
    /// Speed over the ground, m/s.
    pub ground_speed: f64,
    /// Airspeed, m/s.
    pub airspeed: f64,
    pub alpha: f64,
    pub pitch: f64,
    pub bank: f64,
    pub heading: f64,
    /// Load factor of the air and the ground, over the weight.
    pub load_factor: f64,
    /// Height of the propeller's disc above the ground, m.
    pub clearance: f64,
    loads: Loads,
    touching: bool,
}

impl Rolled {
    fn of(
        aircraft: &Aircraft,
        time: f64,
        start: &RigidBodyState,
        state: RigidBodyState,
        controls: PilotControls,
    ) -> Self {
        let d = aircraft.evaluate(&state, &controls, &FIELD);
        let velocity = state.velocity_ned();
        let (yaw, pitch, bank) = state.attitude.to_euler(EulerRot::ZYX);
        let weight = aircraft.mass().mass() * escadrille_fdm::constants::G0;
        let travelled = state.position_ned - start.position_ned;
        Self {
            time,
            state,
            controls,
            distance: travelled.truncate().length(),
            ground_speed: velocity.truncate().length(),
            airspeed: d.air_velocity.length(),
            alpha: d.air_velocity.z.atan2(d.air_velocity.x),
            pitch,
            bank,
            heading: yaw,
            load_factor: -d.loads.wrench.force.z / weight,
            clearance: aircraft
                .propeller_clearance(&state, &FIELD)
                .unwrap_or(f64::INFINITY),
            loads: loads_of(&d.ground.contacts),
            touching: !d.ground.contacts.is_empty(),
        }
    }
}

/// What a scripted run left behind.
#[derive(Debug, Clone, PartialEq)]
pub struct Run {
    pub samples: Vec<Rolled>,
    /// Every sample was finite.
    pub finite: bool,
}

impl Run {
    fn last(&self) -> &Rolled {
        &self.samples[self.samples.len() - 1]
    }

    /// Largest loads of the run.
    fn peak(&self) -> Loads {
        let mut out = Loads::default();
        for s in &self.samples {
            out.wheel = out.wheel.max(s.loads.wheel);
            out.skid = out.skid.max(s.loads.skid);
            out.hard_point = out.hard_point.max(s.loads.hard_point);
            out.travel = out.travel.max(s.loads.travel);
        }
        out
    }

    fn min(&self, f: impl Fn(&Rolled) -> f64) -> f64 {
        self.samples.iter().map(f).fold(f64::MAX, f64::min)
    }

    fn max(&self, f: impl Fn(&Rolled) -> f64) -> f64 {
        self.samples.iter().map(f).fold(f64::MIN, f64::max)
    }

    /// Time history for the plots.
    pub fn trace(&self) -> Vec<TracePoint> {
        const EVERY: usize = 12;
        self.samples
            .iter()
            .step_by(EVERY)
            .map(|s| TracePoint {
                time: s.time,
                speed: s.ground_speed,
                alpha: s.alpha,
                pitch: s.pitch,
                bank: s.bank,
                load_factor: s.load_factor,
                controls: s.controls,
            })
            .collect()
    }
}

/// Runs from `start` until the pilot returns `None` or `seconds` have passed. `pilot` sees
/// the last sample and may keep what it needs.
fn run(
    aircraft: &Aircraft,
    start: RigidBodyState,
    seconds: f64,
    pilot: impl FnMut(&Rolled) -> Option<PilotControls>,
) -> Run {
    run_at(aircraft, start, seconds, DT, pilot)
}

/// Same as [`run`] with a given integration step.
fn run_at(
    aircraft: &Aircraft,
    start: RigidBodyState,
    seconds: f64,
    dt: f64,
    mut pilot: impl FnMut(&Rolled) -> Option<PilotControls>,
) -> Run {
    let mut last = Rolled::of(aircraft, 0.0, &start, start, PilotControls::default());
    let mut samples = vec![last];
    for i in 1..=(seconds / dt).round() as u32 {
        let Some(controls) = pilot(&last) else { break };
        let state = aircraft.step(&last.state, &controls, &FIELD, dt);
        let finite = state.position_ned.is_finite()
            && state.velocity_body.is_finite()
            && state.angular_velocity_body.is_finite()
            && state.rotor_speed.is_finite();
        if !finite {
            return Run {
                samples,
                finite: false,
            };
        }
        last = Rolled::of(aircraft, f64::from(i) * dt, &start, state, controls);
        samples.push(last);
    }
    Run {
        samples,
        finite: true,
    }
}

fn wings_level(s: &Rolled) -> f64 {
    -BANK_GAIN * s.bank - ROLL_DAMPING * s.state.angular_velocity_body.x
}

fn hold_heading(s: &Rolled, heading: f64) -> f64 {
    let error = escadrille_fdm::aero::polar::wrap_angle(heading - s.heading);
    (HEADING_GAIN * error - YAW_DAMPING * s.state.angular_velocity_body.z).clamp(-1.0, 1.0)
}

fn hold_pitch(s: &Rolled, pitch: f64) -> f64 {
    (PITCH_GAIN * (pitch - s.pitch) - PITCH_DAMPING * s.state.angular_velocity_body.y)
        .clamp(-1.0, 1.0)
}

/// The aircraft at rest on its wheels and skid, engine at idle.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rest {
    pub state: RigidBodyState,
    /// Nose above the horizon, rad.
    pub pitch: f64,
    /// N.
    pub wheel_load: f64,
    pub skid_load: f64,
    /// Travel of a wheel's axle, m.
    pub travel: f64,
    /// Speed still left after settling, m/s: the slow creep of the regularised friction.
    pub creep: f64,
    pub rpm: f64,
}

/// Lets the aircraft settle on the ground from just above it, engine at idle.
pub fn rest(aircraft: &Aircraft) -> Result<Rest, String> {
    let gear = aircraft
        .undercarriage()
        .ok_or("no undercarriage in aircraft.ron")?;
    let wheel = gear.wheels().first().ok_or("no wheel")?;
    let skid = gear.skids().first().ok_or("no skid")?;
    let foot = wheel.axle + DVec3::Z * wheel.radius;
    let pitch = (foot.z - skid.shoe.z).atan2(foot.x - skid.shoe.x);
    let attitude = DQuat::from_euler(EulerRot::ZYX, 0.0, pitch, 0.0);
    const DROP: f64 = 0.02;
    let start = RigidBodyState {
        position_ned: DVec3::new(0.0, 0.0, -(attitude * wheel.axle).z - wheel.radius - DROP),
        attitude,
        rotor_speed: IDLE_ROTOR,
        ..RigidBodyState::default()
    };
    let settled = run(aircraft, start, 30.0, |_| Some(PilotControls::default()));
    if !settled.finite {
        return Err("non-finite values while settling".to_owned());
    }
    let s = settled.last();
    let shaft = s.state.rotor_relative_speed(aircraft.mass());
    Ok(Rest {
        state: s.state,
        pitch: s.pitch,
        wheel_load: s.loads.wheel,
        skid_load: s.loads.skid,
        travel: s.loads.travel,
        creep: s.ground_speed,
        rpm: shaft * 60.0 / (2.0 * PI),
    })
}

/// A state of the aircraft at rest, stopped where it stands.
fn stopped(state: &RigidBodyState) -> RigidBodyState {
    RigidBodyState {
        velocity_body: DVec3::ZERO,
        angular_velocity_body: DVec3::ZERO,
        ..*state
    }
}

/// Full power held back: the engine settles, the airframe does not move.
pub fn run_up(aircraft: &Aircraft, rest: &Rest) -> (f64, f64) {
    let full = PilotControls {
        throttle: 1.0,
        ..PilotControls::default()
    };
    let mut state = stopped(&rest.state);
    for _ in 0..(15.0 / DT) as u32 {
        let next = aircraft.step(&state, &full, &FIELD, DT);
        state = RigidBodyState {
            rotor_speed: next.rotor_speed,
            aux: next.aux,
            ..state
        };
    }
    let d = aircraft.evaluate(&state, &full, &FIELD);
    let thrust = d.propeller.map_or(0.0, |p| p.wrench.force.x);
    (
        state.rotor_relative_speed(aircraft.mass()) * 60.0 / (2.0 * PI),
        thrust,
    )
}

/// The throttle opened slowly from rest: throttle and shaft speed when it starts to roll.
pub fn start_of_the_roll(aircraft: &Aircraft, rest: &Rest) -> Option<(f64, f64)> {
    let mut opening = 0.0;
    let moving = run(aircraft, stopped(&rest.state), 45.0, |s| {
        opening += SLOW_OPENING * DT;
        (s.ground_speed < MOVING).then_some(PilotControls {
            throttle: opening,
            ..PilotControls::default()
        })
    });
    let last = moving.last();
    (last.ground_speed >= MOVING).then(|| {
        (
            last.controls.throttle,
            last.state.rotor_relative_speed(aircraft.mass()) * 60.0 / (2.0 * PI),
        )
    })
}

/// What a take-off showed.
#[derive(Debug, Clone, PartialEq)]
pub struct TakeOff {
    pub run: Run,
    /// When the skid left the ground, s and km/h.
    pub tail_up: Option<(f64, f64)>,
    /// When nothing touched any more: s, m and km/h.
    pub lift_off: Option<(f64, f64, f64)>,
    /// Over the screen: s, m.
    pub screen: Option<(f64, f64)>,
    /// Largest heading away from the start, rad.
    pub swing: f64,
    /// Largest rudder the pilot needed.
    pub rudder: f64,
}

/// Full throttle from rest, the tail raised early, pulled at [`ROTATION`] times the stall.
/// With `rudder`, the pilot holds the heading; without, the swing shows what he contains.
pub fn take_off(aircraft: &Aircraft, rest: &Rest, rudder: bool) -> Result<TakeOff, String> {
    take_off_at(aircraft, rest, rudder, DT)
}

fn take_off_at(aircraft: &Aircraft, rest: &Rest, rudder: bool, dt: f64) -> Result<TakeOff, String> {
    let stall = stall_speed(aircraft, &free_air(), 500.0).map_err(error)?;
    let rotation = ROTATION * stall.speed;
    let (mut skid_off, mut lift_off, mut screen) = (None, None, None);
    let mut airborne_since = None;
    let mut tail_is_up = false;
    let run = run_at(aircraft, stopped(&rest.state), 45.0, dt, |s| {
        if skid_off.is_none() && s.time > 0.1 && s.loads.skid == 0.0 {
            skid_off = Some((s.time, s.airspeed * 3.6));
        }
        if s.touching {
            airborne_since = None;
        } else if s.time > 0.1 {
            let since = *airborne_since.get_or_insert(s.time);
            if lift_off.is_none() && s.time - since >= AIRBORNE {
                lift_off = Some((since, s.distance, s.airspeed * 3.6));
            }
        }
        let height = -s.state.position_ned.z;
        if screen.is_none() && height > SCREEN {
            screen = Some((s.time, s.distance));
            return None;
        }
        tail_is_up |= s.pitch < TAIL_UP_PITCH + 2.0 * DEG;
        let pitch = if s.airspeed > rotation {
            hold_pitch(s, CLIMB_PITCH)
        } else if tail_is_up {
            hold_pitch(s, TAIL_UP_PITCH)
        } else {
            TAIL_UP_STICK
        };
        Some(PilotControls {
            roll: wings_level(s),
            pitch,
            yaw: if rudder { hold_heading(s, 0.0) } else { 0.0 },
            throttle: (s.time / OPENING).min(1.0),
        })
    });
    let swing = run.max(|s| s.heading.abs());
    let used = run.max(|s| s.controls.yaw.abs());
    Ok(TakeOff {
        tail_up: skid_off,
        lift_off,
        screen,
        swing,
        rudder: used,
        run,
    })
}

fn error(e: PerformanceError) -> String {
    format!("{e:?}")
}

/// Still air and no ground, for the speeds the ground tests start from.
pub fn free_air() -> StandardEnvironment {
    StandardEnvironment {
        wind_ned: DVec3::ZERO,
        ground_altitude: escadrille_fdm::environment::NO_GROUND,
    }
}

/// What a landing's roll showed: distance and time to stop, km/h at the touch.
#[derive(Debug, Clone, PartialEq)]
pub struct Stop {
    pub run: Run,
    pub speed_kmh: f64,
    pub distance: f64,
    pub seconds: f64,
    /// Nose-down attitude reached, rad: the aircraft stays on its wheels and skid.
    pub lowest_pitch: f64,
}

/// Rolling on three points at [`TOUCHDOWN`] times the stall, throttle closed, stick back.
pub fn stop(aircraft: &Aircraft, rest: &Rest) -> Result<Stop, String> {
    let stall = stall_speed(aircraft, &free_air(), 500.0).map_err(error)?;
    let speed = TOUCHDOWN * stall.speed;
    let forward = rest.state.attitude.conjugate() * DVec3::X * speed;
    let start = RigidBodyState {
        velocity_body: forward,
        ..stopped(&rest.state)
    };
    let run = run(aircraft, start, 60.0, |s| {
        (s.time < 0.1 || s.ground_speed > STOPPED).then_some(PilotControls {
            roll: wings_level(s),
            pitch: 1.0,
            yaw: hold_heading(s, 0.0),
            throttle: 0.0,
        })
    });
    let last = *run.last();
    Ok(Stop {
        speed_kmh: speed * 3.6,
        distance: last.distance,
        seconds: last.time,
        lowest_pitch: run.min(|s| s.pitch),
        run,
    })
}

/// What a landing at a given sink showed.
#[derive(Debug, Clone, PartialEq)]
pub struct Landing {
    /// m/s.
    pub sink: f64,
    pub run: Run,
    /// Heaviest wheel and skid, N.
    pub wheel_load: f64,
    pub skid_load: f64,
    /// Longest travel of an axle, m.
    pub travel: f64,
    /// Highest rebound of the centre of gravity above its height at rest, m.
    pub rebound: f64,
    /// Heaviest load on a part that should not touch, N.
    pub hard_point: f64,
    /// Lowest height of the propeller's disc, m.
    pub clearance: f64,
    pub finite: bool,
}

/// From just above the ground, three-point attitude, sinking at `sink`, at the speed of the
/// touch, throttle closed. The attitude is held: with the stick hard back, the aircraft
/// would fly off again and the test would measure that second drop.
pub fn landing(aircraft: &Aircraft, rest: &Rest, sink: f64) -> Result<Landing, String> {
    landing_at(aircraft, rest, sink, DT)
}

fn landing_at(aircraft: &Aircraft, rest: &Rest, sink: f64, dt: f64) -> Result<Landing, String> {
    let stall = stall_speed(aircraft, &free_air(), 500.0).map_err(error)?;
    let speed = TOUCHDOWN * stall.speed;
    const ABOVE: f64 = 0.15;
    let ned = DVec3::new(speed, 0.0, sink);
    let start = RigidBodyState {
        position_ned: rest.state.position_ned - DVec3::Z * ABOVE,
        velocity_body: rest.state.attitude.conjugate() * ned,
        ..stopped(&rest.state)
    };
    let run = run_at(aircraft, start, 4.0, dt, |s| {
        Some(PilotControls {
            roll: wings_level(s),
            pitch: hold_pitch(s, rest.pitch),
            yaw: hold_heading(s, 0.0),
            throttle: 0.0,
        })
    });
    let peak = run.peak();
    let settled = -rest.state.position_ned.z;
    let touched = run.samples.iter().position(|s| s.touching).unwrap_or(0);
    let rebound = run.samples[touched..]
        .iter()
        .map(|s| -s.state.position_ned.z - settled)
        .fold(f64::MIN, f64::max);
    Ok(Landing {
        sink,
        wheel_load: peak.wheel,
        skid_load: peak.skid,
        travel: peak.travel,
        rebound,
        hard_point: peak.hard_point,
        clearance: run.min(|s| s.clearance),
        finite: run.finite,
        run,
    })
}

/// What a rough handling showed.
#[derive(Debug, Clone, PartialEq)]
pub struct Rough {
    pub run: Run,
    /// Largest change of heading, rad.
    pub heading: f64,
    /// Largest bank, rad.
    pub bank: f64,
    /// Lowest nose, rad.
    pub lowest_pitch: f64,
    pub clearance: f64,
    /// Heaviest load on a part that should not touch, N.
    pub hard_point: f64,
}

fn rough(run: Run) -> Rough {
    Rough {
        heading: run.max(|s| s.heading.abs()),
        bank: run.max(|s| s.bank.abs()),
        lowest_pitch: run.min(|s| s.pitch),
        clearance: run.min(|s| s.clearance),
        hard_point: run.peak().hard_point,
        run,
    }
}

/// Rolling on the wheels at `speed`, tail up, wheels a little into the ground.
fn tail_up(aircraft: &Aircraft, rest: &Rest, speed: f64) -> RigidBodyState {
    let attitude = DQuat::from_euler(EulerRot::ZYX, 0.0, TAIL_UP_PITCH, 0.0);
    let height = aircraft.undercarriage().map_or(0.0, |g| {
        g.wheels()
            .first()
            .map_or(0.0, |w| (attitude * w.axle).z + w.radius - 0.03)
    });
    RigidBodyState {
        position_ned: DVec3::new(0.0, 0.0, -height),
        attitude,
        velocity_body: attitude.conjugate() * DVec3::X * speed,
        rotor_speed: rest.state.rotor_speed,
        ..RigidBodyState::default()
    }
}

/// Full rudder for two seconds, rolling at `speed` tail up, throttle closed: the ground loop.
pub fn ground_loop(aircraft: &Aircraft, rest: &Rest, speed: f64) -> Rough {
    let start = tail_up(aircraft, rest, speed);
    rough(run(aircraft, start, 12.0, |s| {
        Some(PilotControls {
            yaw: if s.time < 2.0 { 1.0 } else { 0.0 },
            pitch: hold_pitch(s, TAIL_UP_PITCH),
            ..PilotControls::default()
        })
    }))
}

/// A turn while taxiing: full rudder at a throttle, stick forward to lighten the skid. The
/// mean rate of turn, rad/s, once it is established, and the speed reached, m/s.
pub fn taxi_turn(aircraft: &Aircraft, rest: &Rest, throttle: f64, stick: f64) -> (f64, f64) {
    const SETTLING: f64 = 6.0;
    const SECONDS: f64 = 16.0;
    let turning = run(aircraft, stopped(&rest.state), SECONDS, |s| {
        Some(PilotControls {
            yaw: if s.time > SETTLING { 1.0 } else { 0.0 },
            pitch: stick,
            throttle,
            ..PilotControls::default()
        })
    });
    let last = turning.last();
    let index = (SETTLING / DT) as usize;
    let mut turned = 0.0;
    for pair in turning.samples[index..].windows(2) {
        turned += escadrille_fdm::aero::polar::wrap_angle(pair[1].heading - pair[0].heading);
    }
    (turned / (SECONDS - SETTLING), last.ground_speed)
}

/// Full forward stick at full throttle from a roll at `speed`, tail up: over the nose?
pub fn nose_over(aircraft: &Aircraft, rest: &Rest, speed: f64) -> Rough {
    let start = tail_up(aircraft, rest, speed);
    rough(run(aircraft, start, 6.0, |s| {
        (s.clearance > -0.5).then_some(PilotControls {
            pitch: -1.0,
            throttle: 1.0,
            roll: wings_level(s),
            yaw: hold_heading(s, 0.0),
        })
    }))
}

/// `essais.ron`: the take-off, with and without the rudder, in French.
pub fn observed_take_off(aircraft: &Aircraft) -> Result<Observed, String> {
    let r = rest(aircraft)?;
    let held = take_off(aircraft, &r, true)?;
    let free = take_off(aircraft, &r, false)?;
    let early = free
        .run
        .samples
        .iter()
        .find(|s| s.heading.abs() > 2.0 * DEG)
        .map_or(0.0, |s| s.heading);
    let side = if early < 0.0 { "gauche" } else { "droite" };
    let lift_off = held.lift_off.map_or_else(
        || "pas d'envol".to_owned(),
        |(t, d, v)| format!("envol en {t:.1} s, {d:.0} m, {v:.0} km/h"),
    );
    let screen = held.screen.map_or_else(
        || "15 m non atteints".to_owned(),
        |(t, d)| format!("15 m en {t:.1} s, {d:.0} m"),
    );
    Ok(Observed {
        text: format!(
            "sans palonnier, l'avion part à {side} ; au pied, palonnier jusqu'à {:.0} %, écart de cap {:.1}° ; {lift_off} ; {screen}",
            held.rudder * 100.0,
            held.swing.to_degrees(),
        ),
        finite: held.run.finite && free.run.finite,
        completed: held.screen.is_some(),
        trace: held.run.trace(),
        ..Observed::default()
    })
}

/// `essais.ron`: the stop, the turns and the start of the roll, in French.
pub fn observed_handling(aircraft: &Aircraft) -> Result<Observed, String> {
    let r = rest(aircraft)?;
    let s = stop(aircraft, &r)?;
    let start = start_of_the_roll(aircraft, &r).map_or_else(
        || "ne se met pas en mouvement".to_owned(),
        |(throttle, _)| format!("démarre à {:.0} % de gaz", throttle * 100.0),
    );
    let (gentle, _) = taxi_turn(aircraft, &r, 0.3, 0.0);
    let (hard, _) = taxi_turn(aircraft, &r, 0.5, -1.0);
    Ok(Observed {
        text: format!(
            "{start} ; arrêt de {:.0} km/h en {:.0} m et {:.1} s, cap tenu à {:.1}° ; virage de roulage {:.1} °/s à 30 % de gaz, {:.0} °/s à 50 % manche poussé",
            s.speed_kmh,
            s.distance,
            s.seconds,
            s.run.max(|x| x.heading.abs()).to_degrees(),
            gentle.to_degrees(),
            hard.to_degrees(),
        ),
        finite: s.run.finite,
        completed: true,
        trace: s.run.trace(),
        ..Observed::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    use escadrille_data::aircraft::{Loading, load_aircraft};

    fn spad() -> Aircraft {
        let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../assets");
        load_aircraft(&assets, "spad7")
            .unwrap()
            .build(&Loading { fuel_fraction: 1.0 })
            .unwrap()
            .aircraft
    }

    #[test]
    fn at_rest_the_spad_sits_on_three_points() {
        let a = spad();
        let r = rest(&a).unwrap();
        let weight = a.mass().mass() * escadrille_fdm::constants::G0;
        let sum = 2.0 * r.wheel_load + r.skid_load;
        // A little less than the weight: at idle, the slipstream already lifts the wings.
        assert!((sum - weight).abs() < 0.03 * weight, "{r:?}");
        assert!((0.15..0.25).contains(&(r.skid_load / weight)), "{r:?}");
        assert!((12.0..16.0).contains(&r.pitch.to_degrees()), "{r:?}");
        assert!(r.creep < 0.02, "{r:?}");
    }

    #[test]
    fn at_full_power_held_back_it_pulls_as_the_calculation_says() {
        let a = spad();
        let (rpm, thrust) = run_up(&a, &rest(&a).unwrap());
        assert!((1_500.0..1_700.0).contains(&rpm), "{rpm}");
        assert!((3_500.0..4_500.0).contains(&thrust), "{thrust}");
    }

    #[test]
    fn at_idle_it_stays_put_and_it_takes_some_throttle_to_move() {
        let a = spad();
        let r = rest(&a).unwrap();
        let (throttle, _) = start_of_the_roll(&a, &r).expect("it rolls");
        assert!((0.05..0.5).contains(&throttle), "{throttle}");
    }

    #[test]
    fn the_take_off_swings_left_held_by_right_rudder() {
        let a = spad();
        let r = rest(&a).unwrap();
        let held = take_off(&a, &r, true).unwrap();
        assert!(held.run.finite);
        let (seconds, distance, speed) = held.lift_off.expect("it flies");
        assert!((6.0..20.0).contains(&seconds), "{seconds}");
        assert!((60.0..300.0).contains(&distance), "{distance}");
        assert!((85.0..125.0).contains(&speed), "{speed}");
        assert!(held.screen.is_some(), "{:?}", held.screen);
        assert!(held.swing < 25.0 * DEG, "{}", held.swing.to_degrees());
        let mean: f64 = held.run.samples.iter().map(|s| s.controls.yaw).sum();
        assert!(mean > 0.0, "right rudder on the whole: {mean}");
        let free = take_off(&a, &r, false).unwrap();
        let early = free
            .run
            .samples
            .iter()
            .find(|s| s.heading.abs() > 2.0 * DEG);
        assert!(early.expect("it swings").heading < 0.0, "to the left");
    }

    #[test]
    fn halving_the_step_does_not_change_the_take_off_or_the_landing() {
        let a = spad();
        let r = rest(&a).unwrap();
        let (coarse, fine) = (
            take_off_at(&a, &r, true, DT).unwrap(),
            take_off_at(&a, &r, true, 0.5 * DT).unwrap(),
        );
        let (d1, d2) = (coarse.lift_off.unwrap().1, fine.lift_off.unwrap().1);
        assert!((d1 - d2).abs() < 0.01 * d2, "{d1} and {d2}");
        let (coarse, fine) = (
            landing_at(&a, &r, 2.0, DT).unwrap(),
            landing_at(&a, &r, 2.0, 0.5 * DT).unwrap(),
        );
        let (w1, w2) = (coarse.wheel_load, fine.wheel_load);
        assert!((w1 - w2).abs() < 0.01 * w2, "{w1} and {w2}");
    }

    #[test]
    fn it_stops_straight_on_three_points() {
        let a = spad();
        let r = rest(&a).unwrap();
        let s = stop(&a, &r).unwrap();
        assert!((60.0..300.0).contains(&s.distance), "{}", s.distance);
        assert!(s.run.max(|x| x.heading.abs()) < 15.0 * DEG);
        assert!(s.lowest_pitch > 10.0 * DEG, "it stays on its tail");
    }

    #[test]
    fn a_harder_landing_loads_the_wheels_more_up_to_the_stop() {
        let a = spad();
        let r = rest(&a).unwrap();
        let loads: Vec<f64> = [0.5, 1.5, 2.5]
            .iter()
            .map(|sink| landing(&a, &r, *sink).unwrap().wheel_load)
            .collect();
        assert!(loads[0] < loads[1] && loads[1] < loads[2], "{loads:?}");
        let gentle = landing(&a, &r, 0.5).unwrap();
        assert!(
            gentle.hard_point == 0.0 && gentle.clearance > 0.0,
            "{gentle:?}"
        );
    }

    #[test]
    fn rudder_and_power_turn_it_on_the_ground() {
        let a = spad();
        let r = rest(&a).unwrap();
        let (gentle, _) = taxi_turn(&a, &r, 0.3, 0.0);
        let (hard, _) = taxi_turn(&a, &r, 0.5, -1.0);
        assert!(gentle > 0.5 * DEG, "{}", gentle.to_degrees());
        assert!(hard > 3.0 * gentle, "{} {}", gentle, hard);
    }

    #[test]
    fn full_rudder_in_a_fast_roll_does_not_spin_it_round() {
        let a = spad();
        let r = rest(&a).unwrap();
        let g = ground_loop(&a, &r, 60.0 / 3.6);
        assert!(g.run.finite);
        assert!(g.heading < 90.0 * DEG, "{}", g.heading.to_degrees());
    }

    #[test]
    fn pushed_hard_at_full_power_the_propeller_strikes_before_it_goes_over() {
        let a = spad();
        let r = rest(&a).unwrap();
        let n = nose_over(&a, &r, 70.0 / 3.6);
        assert!(n.run.finite);
        assert!(n.clearance < 0.0, "{}", n.clearance);
    }
}
