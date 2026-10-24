//! Qualitative scenarios of `essais.ron` (M1 plan, step 6): behaviours from the pilots'
//! testimonies (fiche, section 6), flown by a simple scripted pilot with the player's
//! commands. The outcome is what the aircraft did, set beside the expected behaviour of
//! the scenario: testimonies are not measurements, so these scenarios never block the
//! validation, except when the simulation produces non-finite values (the shape of a
//! behaviour is for the player and the aerodynamicist to judge).
//!
//! Every manoeuvre starts heading north, wings level, in still air.

use std::f64::consts::{PI, TAU};

use escadrille_data::essais::{Manoeuvre, Scenario};
use escadrille_fdm::aero::polar::wrap_angle;
use escadrille_fdm::aircraft::{Aircraft, PilotControls};
use escadrille_fdm::atmosphere::isa;
use escadrille_fdm::constants::G0;
use escadrille_fdm::environment::Environment;
use escadrille_fdm::rigid_body::RigidBodyState;
use glam::{DVec3, EulerRot};

use crate::performance::{PerformanceError, glide, max_level_speed, stall_speed, stall_speed_at};
use crate::trim::{Flight, LEVEL_AT_SPEED, Trim, solve_robust};

const DEG: f64 = PI / 180.0;

/// Settings of the scripted pilot (E:, a period pilot's feel, not a measurement).
#[derive(Debug, Clone, Copy)]
struct Pilot {
    /// Integration step, s.
    dt: f64,
    /// How far ahead the pilot anticipates the incidence, s (feel of the controls).
    lead: f64,
    /// Margin kept below the stall incidence, rad (the buffet warns the pilot).
    margin: f64,
}

/// The pilot of the reports: 120 Hz with two substeps (ADR 0005), incidence anticipated
/// by 0.3 s, 2° kept below the stall.
const PILOT: Pilot = Pilot {
    dt: 1.0 / 240.0,
    lead: 0.3,
    margin: 2.0 * DEG,
};

/// Pilot's elevator (E:): proportional and integral gains on the normalised margin (a
/// brisk but smooth pull, reaching the load factor in about a second), load-factor error
/// that counts as a full margin, and incidence margin that counts as much.
const PULL_GAIN: f64 = 0.4;
const PULL_INTEGRAL: f64 = 1.5;
const LOAD_SCALE: f64 = 2.0;
const ALPHA_SCALE: f64 = 5.0 * DEG;

/// Time in which the pilot brings the flight path back to the wanted one, s (E:).
const PATH_TIME: f64 = 1.0;

/// Pilot's ailerons and rudder (E:): full aileron for half a radian of bank, damped by
/// the roll rate; full rudder for about 15° of sideslip, and an integral that removes a
/// steady sideslip in a couple of seconds.
const ROLL_GAIN: f64 = 2.0;
const ROLL_DAMPING: f64 = 0.5;
const YAW_GAIN: f64 = 4.0;
const YAW_INTEGRAL: f64 = 2.0;

/// Power-off stall (E:): entry speed over the idle stall speed (the usual 1.3), speed
/// reduction (a knot per second, the practice of modern certification tests), time in
/// which a speed error is corrected, nose-down pitch rate and bank that mark the break
/// (the bank also tells a wing drop from a nose drop), window after the stall incidence
/// over which the aircraft is left alone and measured, and the instants of it reported,
/// shortest time it is left alone after a late break, time at the stick's stop without a
/// break before giving up. One window for every departure (aerodynamicist review of
/// 2026-10-23: two stopping rules raced and a stop was reported as a measurement).
const STALL_ENTRY: f64 = 1.3;
const DECELERATION: f64 = 0.5144;
const SPEED_TIME: f64 = 2.0;
const BREAK_PITCH_RATE: f64 = 3.0 * DEG;
const DEPARTURE_BANK: f64 = 30.0 * DEG;
const DEPARTURE_WINDOW: f64 = 3.0;
const DEPARTURE_INSTANTS: [f64; 3] = [1.0, 2.0, 3.0];
const LATE_BREAK_HOLD: f64 = 0.5;
const STOP_TIME: f64 = 10.0;

/// Loop: bank out of the vertical plane beyond which it is no longer a loop (E:).
const LOOP_PLANE_BANK: f64 = 45.0 * DEG;

/// Dive: part of the dive taken as its entry, s; time the elevator is released to
/// neutral (E:, the fiche's test); change of flight path below which the dive is
/// neither stable nor divergent (E:).
const DIVE_ENTRY: f64 = 2.0;
const RELEASE_TIME: f64 = 3.0;
const RELEASE_THRESHOLD: f64 = 1.0 * DEG;

/// Longest time given to a manoeuvre, s.
const LOOP_TIME: f64 = 60.0;
const APPROACH_TIME: f64 = 120.0;
const RECOVERY_TIME: f64 = 30.0;

/// Search tolerance on the speed of best glide, m/s.
const GLIDE_TOLERANCE: f64 = 0.1;

#[derive(Debug, Clone, PartialEq)]
pub struct QualitativeOutcome {
    pub name: String,
    /// Expected behaviour, from the testimonies.
    pub description: String,
    pub source: String,
    /// `None` for a behaviour `fdm_lab` does not fly; an error when the manoeuvre could
    /// not be set up (no trim, no stall found).
    pub flown: Option<Result<Observed, String>>,
}

impl QualitativeOutcome {
    /// The simulation stayed finite.
    pub fn finite(&self) -> bool {
        !matches!(&self.flown, Some(Ok(o)) if !o.finite)
    }

    /// What the aircraft did, in French.
    pub fn observed(&self) -> String {
        match &self.flown {
            None => "non simulé par `fdm_lab`".to_owned(),
            Some(Ok(o)) if o.finite => o.text.clone(),
            Some(Ok(_)) => "**valeurs non finies (NaN)**".to_owned(),
            Some(Err(message)) => format!("non calculé : {message}"),
        }
    }
}

/// What a flown manoeuvre showed: the report's text and its main figures.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Observed {
    pub text: String,
    pub finite: bool,
    /// The manoeuvre went through: loop closed, stall broken then recovered, dive
    /// pulled out.
    pub completed: bool,
    /// Largest load factor of the pilot's pull and its target.
    pub load_factor: Option<(f64, f64)>,
    /// Largest incidence and the stall incidence, rad.
    pub alpha: Option<(f64, f64)>,
    /// Flight-path angles between which the dive was held, rad.
    pub held_path: Option<(f64, f64)>,
    /// After the stall, stick held, ailerons and rudder at the glide's trim: nose drop and the
    /// largest bank, signed (positive
    /// right wing down), rad.
    pub stall_drop: Option<(f64, f64)>,
    /// Mean speed reduction before the stall, m/s².
    pub deceleration: Option<f64>,
    /// Change of flight path with the elevator released in the held dive, rad
    /// (positive: the dive flattens by itself).
    pub release: Option<f64>,
    pub glide_ratio: Option<f64>,
    /// Time history of the manoeuvre, for the plots.
    pub trace: Vec<TracePoint>,
}

/// One instant of a manoeuvre's time history (angles in the frame that suits it: the
/// vertical plane for loops and dives, Euler angles otherwise).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct TracePoint {
    /// s.
    pub time: f64,
    /// Airspeed, m/s.
    pub speed: f64,
    pub alpha: f64,
    pub pitch: f64,
    pub bank: f64,
    pub load_factor: f64,
    pub controls: PilotControls,
}

/// Rate of the time histories, Hz.
const TRACE_RATE: f64 = 20.0;

/// One instant of a manoeuvre.
#[derive(Debug, Clone, Copy)]
struct Sample {
    time: f64,
    state: RigidBodyState,
    /// Airspeed, m/s.
    speed: f64,
    alpha: f64,
    beta: f64,
    altitude: f64,
    /// Flight-path angle, rad (−90° to 90°).
    flight_path: f64,
    /// Flight-path angle in the initial vertical plane, unwrapped: a loop adds 2π.
    plane_path: f64,
    /// Aerodynamic and propeller force along −z over the weight.
    load_factor: f64,
    /// Same force along the airspeed, over the mass, m/s².
    along: f64,
    /// Nose angle above the horizon in the initial vertical plane, unwrapped.
    pitch: f64,
    /// Bank out of that plane (positive right wing down): for loops and dives.
    roll: f64,
    /// Euler angles (heading unwrapped): for flight near the horizontal, whatever the
    /// heading.
    elevation: f64,
    bank: f64,
    heading: f64,
    /// Controls flown to reach this instant.
    controls: PilotControls,
}

fn unwrapped(raw: f64, previous: Option<f64>) -> f64 {
    previous.map_or(raw, |p| p + wrap_angle(raw - p))
}

impl Sample {
    fn new(
        aircraft: &Aircraft,
        env: &dyn Environment,
        time: f64,
        state: RigidBodyState,
        controls: &PilotControls,
        previous: Option<&Sample>,
    ) -> Self {
        let d = aircraft.evaluate(&state, controls, env);
        let air = d.air_velocity;
        let speed = air.length();
        let force = d.loads.wrench.force;
        let mass = aircraft.mass().mass();
        let velocity = state.attitude * state.velocity_body;
        let ground_speed = velocity.length();
        let nose = state.attitude * DVec3::X;
        let (wing, belly) = (state.attitude * DVec3::Y, state.attitude * DVec3::Z);
        let (yaw, elevation, bank) = state.attitude.to_euler(EulerRot::ZYX);
        Self {
            time,
            state,
            speed,
            alpha: air.z.atan2(air.x),
            beta: if speed > 0.0 {
                (air.y / speed).asin()
            } else {
                0.0
            },
            altitude: -state.position_ned.z,
            flight_path: if ground_speed > 0.0 {
                (-velocity.z / ground_speed).asin()
            } else {
                0.0
            },
            plane_path: unwrapped(
                (-velocity.z).atan2(velocity.x),
                previous.map(|p| p.plane_path),
            ),
            load_factor: -force.z / (mass * G0),
            along: if speed > 0.0 {
                force.dot(air) / (speed * mass)
            } else {
                0.0
            },
            pitch: unwrapped((-nose.z).atan2(nose.x), previous.map(|p| p.pitch)),
            roll: (-belly.y).atan2(wing.y),
            elevation,
            bank,
            heading: unwrapped(yaw, previous.map(|p| p.heading)),
            controls: *controls,
        }
    }
}

struct Flown {
    samples: Vec<Sample>,
    finite: bool,
}

impl Flown {
    fn first(&self) -> Sample {
        self.samples[0]
    }

    fn last(&self) -> Sample {
        self.samples[self.samples.len() - 1]
    }

    fn max(&self, f: impl Fn(&Sample) -> f64) -> f64 {
        self.samples.iter().map(f).fold(f64::MIN, f64::max)
    }

    fn min(&self, f: impl Fn(&Sample) -> f64) -> f64 {
        self.samples.iter().map(f).fold(f64::MAX, f64::min)
    }

    /// Time history at [`TRACE_RATE`], from `offset` seconds, angles in the vertical
    /// plane (`plane`) or Euler angles.
    fn trace(&self, offset: f64, plane: bool) -> Vec<TracePoint> {
        let mut next = 0.0;
        let mut out = Vec::new();
        for s in &self.samples {
            if s.time + 1e-9 < next {
                continue;
            }
            next = s.time + 1.0 / TRACE_RATE;
            out.push(TracePoint {
                time: offset + s.time,
                speed: s.speed,
                alpha: s.alpha,
                pitch: if plane { s.pitch } else { s.elevation },
                bank: if plane { s.roll } else { s.bank },
                load_factor: s.load_factor,
                controls: s.controls,
            });
        }
        out
    }
}

/// Flies from `start` until the pilot returns `None` or `seconds` have passed.
fn fly(
    aircraft: &Aircraft,
    env: &dyn Environment,
    pilot: &Pilot,
    start: RigidBodyState,
    controls: PilotControls,
    seconds: f64,
    mut command: impl FnMut(&Sample) -> Option<PilotControls>,
) -> Flown {
    let mut samples = vec![Sample::new(aircraft, env, 0.0, start, &controls, None)];
    let steps = (seconds / pilot.dt).round() as u32;
    for i in 1..=steps {
        let last = samples[samples.len() - 1];
        let Some(c) = command(&last) else { break };
        let state = aircraft.step(&last.state, &c, env, pilot.dt);
        let values = [
            state.velocity_body.to_array(),
            state.angular_velocity_body.to_array(),
            state.position_ned.to_array(),
        ];
        if !(values.iter().flatten().all(|v| v.is_finite())
            && state.rotor_speed.is_finite()
            && state.aux.iter().all(|v| v.is_finite()))
        {
            return Flown {
                samples,
                finite: false,
            };
        }
        let time = f64::from(i) * pilot.dt;
        samples.push(Sample::new(aircraft, env, time, state, &c, Some(&last)));
    }
    Flown {
        samples,
        finite: true,
    }
}

/// Elevator of a pilot flying a load factor without going past an angle of attack:
/// proportional and integral on the smaller of the two normalised margins, the incidence
/// anticipated by its rate. Faster than the entry speed, the pilot moves the stick less,
/// as the elevator grows stronger with the dynamic pressure (gains scaled by it).
#[derive(Debug, Clone)]
struct Pull {
    alpha_limit: f64,
    lead: f64,
    dt: f64,
    base: f64,
    reference_speed: f64,
    integral: f64,
    last_alpha: Option<f64>,
    /// Steps flown, and those on which the incidence limit was the tighter.
    steps: u32,
    limited: u32,
}

impl Pull {
    fn new(pilot: &Pilot, alpha_limit: f64, base: f64, reference_speed: f64) -> Self {
        Self {
            alpha_limit,
            lead: pilot.lead,
            dt: pilot.dt,
            base,
            reference_speed,
            integral: 0.0,
            last_alpha: None,
            steps: 0,
            limited: 0,
        }
    }

    fn command(&mut self, s: &Sample, target: f64) -> f64 {
        let rate = self.last_alpha.map_or(0.0, |a| (s.alpha - a) / self.dt);
        self.last_alpha = Some(s.alpha);
        let load = (target - s.load_factor) / LOAD_SCALE;
        let incidence = (self.alpha_limit - s.alpha - self.lead * rate) / ALPHA_SCALE;
        self.steps += 1;
        if incidence < load {
            self.limited += 1;
        }
        let error = load.min(incidence);
        let scale = (self.reference_speed / s.speed.max(1.0)).powi(2).min(1.0);
        // The integral stays within what the stick can add (no wind-up at the stops).
        self.integral = (self.integral + scale * PULL_INTEGRAL * error * self.dt)
            .clamp(-1.0 - self.base, 1.0 - self.base);
        (self.base + scale * PULL_GAIN * error + self.integral).clamp(-1.0, 1.0)
    }

    /// Part of the time spent on the incidence limit.
    fn limited_fraction(&self) -> f64 {
        f64::from(self.limited) / f64::from(self.steps.max(1))
    }
}

/// Load factor that brings the flight path to `wanted` in [`PATH_TIME`], wings level:
/// V dγ/dt = g (n − cos γ).
fn path_load_factor(s: &Sample, path: f64, wanted: f64) -> f64 {
    path.cos() + s.speed * (wanted - path) / (PATH_TIME * G0)
}

/// Ailerons levelling the wings (bank measured as suits the manoeuvre).
fn wings_level(bank: f64, s: &Sample) -> f64 {
    (-ROLL_GAIN * bank - ROLL_DAMPING * s.state.angular_velocity_body.x).clamp(-1.0, 1.0)
}

/// Rudder nulling the sideslip: proportional and integral around a trim value.
#[derive(Debug, Clone)]
struct Rudder {
    base: f64,
    dt: f64,
    integral: f64,
}

impl Rudder {
    fn new(pilot: &Pilot, base: f64) -> Self {
        Self {
            base,
            dt: pilot.dt,
            integral: 0.0,
        }
    }

    fn command(&mut self, s: &Sample) -> f64 {
        self.integral = (self.integral + YAW_INTEGRAL * s.beta * self.dt)
            .clamp(-1.0 - self.base, 1.0 - self.base);
        (self.base + YAW_GAIN * s.beta + self.integral).clamp(-1.0, 1.0)
    }
}

fn deg(angle: f64) -> f64 {
    angle.to_degrees()
}

fn kmh(speed: f64) -> f64 {
    speed * 3.6
}

fn rpm(rotor_speed: f64) -> f64 {
    rotor_speed * 30.0 / PI
}

/// Equivalent airspeed, km/h.
fn eas_kmh(speed: f64, altitude: f64) -> f64 {
    kmh(speed * isa(altitude).sigma().sqrt())
}

fn error(e: PerformanceError) -> String {
    format!("{e:?}")
}

/// Loop at full throttle from level flight.
fn flown_loop(
    aircraft: &Aircraft,
    env: &dyn Environment,
    pilot: &Pilot,
    altitude: f64,
    speed: Option<f64>,
    load_factor: f64,
) -> Result<Observed, String> {
    // The incidence limit of the manoeuvre's throttle setting.
    let stall = stall_speed_at(aircraft, env, altitude, 1.0).map_err(error)?;
    let trim = match speed {
        None => max_level_speed(aircraft, env, altitude).map_err(error)?,
        Some(v) => solve_robust(
            aircraft,
            env,
            Flight::guess(altitude, v, 0.6),
            LEVEL_AT_SPEED,
        )
        .map_err(|e| format!("pas de palier à {:.0} km/h : {e:?}", kmh(v)))?,
    };
    let c0 = trim.commands(aircraft);
    let limit = stall.alpha - pilot.margin;
    let mut pull = Pull::new(pilot, limit, c0.pitch, trim.flight.speed);
    let mut rudder = Rudder::new(pilot, c0.yaw);
    let mut start_path = None;
    let flown = fly(
        aircraft,
        env,
        pilot,
        trim.flight.state(),
        c0,
        LOOP_TIME,
        |s| {
            let p0 = *start_path.get_or_insert(s.plane_path);
            (s.plane_path - p0 < TAU && s.roll.abs() < LOOP_PLANE_BANK).then(|| PilotControls {
                pitch: pull.command(s, load_factor),
                roll: wings_level(s.roll, s),
                yaw: rudder.command(s),
                throttle: 1.0,
            })
        },
    );
    let (entry, last) = (flown.first(), flown.last());
    let p0 = entry.plane_path;
    let top = flown
        .samples
        .iter()
        .copied()
        .fold(entry, |a, b| if b.altitude > a.altitude { b } else { a });
    let out_of_plane = flown.max(|s| s.roll.abs()) >= LOOP_PLANE_BANK;
    let completed = last.plane_path - p0 >= TAU && !out_of_plane;
    let max_alpha = flown.max(|s| s.alpha);
    let stalled = flown
        .samples
        .iter()
        .find(|s| s.alpha > stall.alpha)
        .map(|s| s.plane_path - p0);
    let top_text = format!(
        "sommet à +{:.0} m, à {:.0} km/h, soit {:.2} V_s (décrochage pleins gaz à 1 g : {:.0} km/h)",
        top.altitude - entry.altitude,
        kmh(top.speed),
        top.speed / stall.speed,
        kmh(stall.speed)
    );
    let mut text = if completed {
        format!(
            "**bouclée** en {:.1} s ; {top_text} ; sortie à {:+.0} m de l'entrée, {:.0} km/h",
            last.time,
            last.altitude - entry.altitude,
            kmh(last.speed)
        )
    } else {
        format!(
            "**non bouclée** : trajectoire tournée de {:.0}° en {:.0} s{} ; {top_text}",
            deg(flown.max(|s| s.plane_path) - p0),
            last.time,
            if out_of_plane {
                " (sortie du plan vertical)"
            } else {
                ""
            }
        )
    };
    text += &format!(
        " ; entrée à {:.0} km/h, facteur de charge maximal {:.1} (consigne {load_factor:.1}), incidence maximale {:.1}° (limite du pilote {:.1}°, décrochage {:.1}°), sur la limite d'incidence {:.0} % du temps, inclinaison hors du plan jusqu'à {:.0}°",
        kmh(entry.speed),
        flown.max(|s| s.load_factor),
        deg(max_alpha),
        deg(limit),
        deg(stall.alpha),
        100.0 * pull.limited_fraction(),
        deg(flown.max(|s| s.roll.abs()))
    );
    text += &match stalled {
        Some(at) => format!(" ; **décroché** à {:.0}° de rotation", deg(at)),
        None => format!(
            " ; sans décrocher (marge {:.1}°)",
            deg(stall.alpha - max_alpha)
        ),
    };
    Ok(Observed {
        text,
        finite: flown.finite,
        completed,
        load_factor: Some((flown.max(|s| s.load_factor), load_factor)),
        alpha: Some((max_alpha, stall.alpha)),
        trace: flown.trace(0.0, true),
        ..Observed::default()
    })
}

/// Power-off stall from level flight at 1.3 times the idle stall speed.
fn flown_stall(
    aircraft: &Aircraft,
    env: &dyn Environment,
    pilot: &Pilot,
    altitude: f64,
) -> Result<Observed, String> {
    let stall = stall_speed(aircraft, env, altitude).map_err(error)?;
    let entry_speed = STALL_ENTRY * stall.speed;
    let level = solve_robust(
        aircraft,
        env,
        Flight::guess(altitude, entry_speed, 0.3),
        LEVEL_AT_SPEED,
    )
    .map_err(|e| format!("{e:?}"))?;
    let glide = glide(aircraft, env, altitude, entry_speed).map_err(error)?;
    let c0 = level.commands(aircraft);
    let recovery = glide.commands(aircraft);
    let gamma_glide = glide.flight.flight_path;

    // Approach: idle, wings held level, the speed reduced by a knot per second with the
    // elevator (no incidence limit) until the nose or a wing drops past the stall
    // incidence. From the stall incidence on, no aileron nor rudder (ailerons and rudder
    // at the glide's trim): a pilot fighting the roll would cross the controls and make
    // the departure himself.
    let mut pull = Pull::new(pilot, f64::INFINITY, c0.pitch, entry_speed);
    let mut rudder = Rudder::new(pilot, recovery.yaw);
    let mut beyond = false;
    let mut at_stop = None::<f64>;
    let mut broke = false;
    let mut applied = c0;
    let approach = fly(
        aircraft,
        env,
        pilot,
        level.flight.state(),
        c0,
        APPROACH_TIME,
        |s| {
            beyond |= s.alpha >= stall.alpha;
            if beyond
                && (s.state.angular_velocity_body.y < -BREAK_PITCH_RATE
                    || s.bank.abs() > DEPARTURE_BANK)
            {
                broke = true;
                return None;
            }
            if at_stop.is_some_and(|t| s.time - t > STOP_TIME) {
                return None;
            }
            let wanted_speed = entry_speed - DECELERATION * s.time;
            let acceleration = -DECELERATION + (wanted_speed - s.speed) / SPEED_TIME;
            let path = ((s.along - acceleration) / G0).clamp(-1.0, 1.0).asin();
            let stick = pull.command(s, path_load_factor(s, s.flight_path, path));
            if stick >= 1.0 && at_stop.is_none() {
                at_stop = Some(s.time);
            }
            applied = if beyond {
                PilotControls {
                    pitch: stick,
                    roll: recovery.roll,
                    yaw: recovery.yaw,
                    throttle: 0.0,
                }
            } else {
                PilotControls {
                    pitch: stick,
                    roll: wings_level(s.bank, s),
                    yaw: rudder.command(s),
                    throttle: 0.0,
                }
            };
            Some(applied)
        },
    );
    let (entry, at_break) = (approach.first(), approach.last());
    let max_alpha = approach.max(|s| s.alpha);
    // Mean speed reduction from the settling of the throttle cut to the stall
    // incidence (past it, the aircraft sinks rather than slows down).
    let settled = approach.samples.iter().find(|s| s.time >= 2.0);
    let stalling = approach
        .samples
        .iter()
        .find(|s| s.alpha >= stall.alpha)
        .unwrap_or(&at_break);
    let deceleration = settled
        .filter(|s| stalling.time > s.time)
        .map(|s| (s.speed - stalling.speed) / (stalling.time - s.time));
    let stall_text = format!(
        "décrochage stabilisé au ralenti : {:.0} km/h ({:.0} km/h en vitesse équivalente), {:.1}°",
        kmh(stall.speed),
        eas_kmh(stall.speed, altitude),
        deg(stall.alpha)
    );
    if !(broke && approach.finite) {
        let what = if at_stop.is_some() {
            format!(
                "**pas de décrochage franc** : manche en butée {STOP_TIME:.0} s, l'avion s'enfonce à {:.0} km/h",
                kmh(at_break.speed)
            )
        } else {
            format!("**pas de décrochage en {APPROACH_TIME:.0} s**")
        };
        let text = format!(
            "{what}, incidence maximale {:.1}° ({stall_text}){}",
            deg(max_alpha),
            if stall.elevator_limited {
                " ; la profondeur ne suffit pas à décrocher l'aile en vol stabilisé"
            } else {
                ""
            }
        );
        return Ok(Observed {
            text,
            finite: approach.finite,
            alpha: Some((max_alpha, stall.alpha)),
            deceleration,
            trace: approach.trace(0.0, false),
            ..Observed::default()
        });
    };

    // The aircraft alone: stick held as at the break, ailerons and rudder at the glide's
    // trim (as since the stall incidence), up to DEPARTURE_WINDOW after the stall
    // incidence, and at least LATE_BREAK_HOLD after the break. The departure is measured
    // over exactly DEPARTURE_WINDOW after the stall incidence, whichever way it broke;
    // the reference attitude is the highest of the last second before the stall
    // incidence.
    let held = PilotControls {
        pitch: applied.pitch,
        roll: recovery.roll,
        yaw: recovery.yaw,
        throttle: 0.0,
    };
    let hold_time = (stalling.time + DEPARTURE_WINDOW - at_break.time).max(LATE_BREAK_HOLD);
    let hold = fly(
        aircraft,
        env,
        pilot,
        at_break.state,
        held,
        hold_time,
        |_| Some(held),
    );
    let held_end = hold.last();
    let reference = approach
        .samples
        .iter()
        .filter(|s| s.time >= stalling.time - 1.0 && s.time <= stalling.time)
        .map(|s| s.elevation)
        .fold(f64::MIN, f64::max);
    // Samples of the window, with their time from the start of the approach.
    let window_end = stalling.time + DEPARTURE_WINDOW;
    let window: Vec<(f64, &Sample)> = approach
        .samples
        .iter()
        .map(|s| (s.time, s))
        .chain(
            hold.samples
                .iter()
                .skip(1)
                .map(|s| (at_break.time + s.time, s)),
        )
        .filter(|(t, _)| *t >= stalling.time && *t <= window_end + 0.5 * pilot.dt)
        .collect();
    let at = |after: f64| {
        window
            .iter()
            .min_by(|a, b| {
                (a.0 - stalling.time - after)
                    .abs()
                    .total_cmp(&(b.0 - stalling.time - after).abs())
            })
            .map_or(&at_break, |(_, s)| *s)
    };
    let instants = DEPARTURE_INSTANTS.map(at);
    let extreme = |f: &dyn Fn(&Sample) -> f64| {
        window
            .iter()
            .map(|(_, s)| f(s))
            .fold(0.0_f64, |b, x| if x.abs() > b.abs() { x } else { b })
    };
    let lowest_nose = window
        .iter()
        .map(|(_, s)| s.elevation)
        .fold(f64::MAX, f64::min);
    let q_min = window
        .iter()
        .map(|(_, s)| s.state.angular_velocity_body.y)
        .fold(0.0, f64::min);
    let bank = extreme(&|s| s.bank);
    let roll_rate = extreme(&|s| s.state.angular_velocity_body.x);

    // The pilot recovers: the glide's stick, wings levelled, until the flight path is
    // back to the glide's.
    let mut rudder = Rudder::new(pilot, recovery.yaw);
    let after = fly(
        aircraft,
        env,
        pilot,
        held_end.state,
        held,
        RECOVERY_TIME,
        |s| {
            if s.time > 1.0 && s.flight_path >= gamma_glide {
                return None;
            }
            Some(PilotControls {
                pitch: recovery.pitch,
                roll: wings_level(s.bank, s),
                yaw: rudder.command(s),
                throttle: 0.0,
            })
        },
    );
    let end = after.last();
    let recovered = end.flight_path >= gamma_glide;
    let side = if bank > 0.0 { "droite" } else { "gauche" };
    let nose = format!(
        "assiette {:+.0}° → {:+.0}°, jusqu'à {:.0}°/s",
        deg(reference),
        deg(lowest_nose),
        deg(-q_min)
    );
    let series = |f: &dyn Fn(&Sample) -> f64| {
        instants
            .iter()
            .map(|s| format!("{:+.0}°", deg(f(s))))
            .collect::<Vec<_>>()
            .join(" / ")
    };
    let timeline = format!(
        "inclinaison {} et assiette {} à {} s de l'incidence de décrochage",
        series(&|s| s.bank),
        series(&|s| s.elevation),
        DEPARTURE_INSTANTS.map(|t| format!("{t:.0}")).join(", ")
    );
    let what = if bank.abs() > DEPARTURE_BANK {
        format!(
            "**départ en roulis à {side}** : inclinaison jusqu'à {:.0}° (roulis jusqu'à {:.0}°/s), le nez baissant de {:.0}° ({nose})",
            deg(bank.abs()),
            deg(roll_rate.abs()),
            deg(reference - lowest_nose)
        )
    } else {
        format!(
            "abattée de {:.0}° ({nose}), inclinaison jusqu'à {:.0}° à {side} (roulis jusqu'à {:.0}°/s)",
            deg(reference - lowest_nose),
            deg(bank.abs()),
            deg(roll_rate.abs())
        )
    };
    let lost = at_break.altitude - end.altitude;
    let recovery_text = if recovered {
        format!(
            "remis en plané normal ({:.1}°) {:.1} s après le décrochage, {lost:.0} m perdus",
            deg(gamma_glide),
            held_end.time + end.time
        )
    } else {
        format!(
            "**pas revenu en plané normal en {:.0} s** ({lost:.0} m perdus)",
            held_end.time + end.time
        )
    };
    let text = format!(
        "{what} ; {timeline} ; cap {:+.0}° ; mesurés sur les {:.0} s qui suivent l'incidence de décrochage, avion livré à lui-même (manche tenu, ailerons et palonnier au trim de plané) ; incidence de décrochage atteinte à {:.0} km/h ({:.0} km/h en vitesse équivalente), dérapage {:+.1}°, après {:.0} s de décélération depuis {:.0} km/h (mesurée {:.2} m/s²), départ {:.1} s plus tard à {:.1}° d'incidence ; {stall_text} ; {recovery_text}",
        deg(instants[instants.len() - 1].heading - stalling.heading),
        DEPARTURE_WINDOW,
        kmh(stalling.speed),
        eas_kmh(stalling.speed, stalling.altitude),
        deg(stalling.beta),
        stalling.time,
        kmh(entry.speed),
        deceleration.unwrap_or(f64::NAN),
        at_break.time - stalling.time,
        deg(at_break.alpha)
    );
    Ok(Observed {
        text,
        finite: approach.finite && hold.finite && after.finite,
        completed: recovered,
        alpha: Some((max_alpha, stall.alpha)),
        stall_drop: Some((reference - lowest_nose, bank)),
        deceleration,
        trace: [
            approach.trace(0.0, false),
            hold.trace(at_break.time, false),
            after.trace(at_break.time + held_end.time, false),
        ]
        .concat(),
        ..Observed::default()
    })
}

/// Best glide at idle: golden-section search on the speed for the shallowest path.
fn best_glide(
    aircraft: &Aircraft,
    env: &dyn Environment,
    altitude: f64,
) -> Result<Trim, PerformanceError> {
    let stall = stall_speed(aircraft, env, altitude)?;
    let vmax = max_level_speed(aircraft, env, altitude)?.flight.speed;
    let path = |speed: f64| glide(aircraft, env, altitude, speed);
    let ratio = 0.5 * (5.0_f64.sqrt() - 1.0);
    let (mut lo, mut hi) = (1.05 * stall.speed, vmax);
    let mut x1 = hi - ratio * (hi - lo);
    let mut x2 = lo + ratio * (hi - lo);
    let (mut f1, mut f2) = (path(x1)?, path(x2)?);
    while hi - lo > GLIDE_TOLERANCE {
        if f1.flight.flight_path > f2.flight.flight_path {
            hi = x2;
            (x2, f2) = (x1, f1);
            x1 = hi - ratio * (hi - lo);
            f1 = path(x1)?;
        } else {
            lo = x1;
            (x1, f1) = (x2, f2);
            x2 = lo + ratio * (hi - lo);
            f2 = path(x2)?;
        }
    }
    Ok(if f1.flight.flight_path > f2.flight.flight_path {
        f1
    } else {
        f2
    })
}

fn flown_glide(
    aircraft: &Aircraft,
    env: &dyn Environment,
    altitude: f64,
) -> Result<Observed, String> {
    let best = best_glide(aircraft, env, altitude).map_err(error)?;
    let f = best.flight;
    let ratio = 1.0 / (-f.flight_path).tan();
    let text = format!(
        "finesse maximale {ratio:.1} à {:.0} km/h (pente {:.1}°, descente {:.1} m/s), moteur au ralenti à {:.0} tr/min",
        kmh(f.speed),
        deg(f.flight_path),
        -f.climb_rate(),
        rpm(f.rotor_speed)
    );
    Ok(Observed {
        text,
        finite: true,
        completed: true,
        glide_ratio: Some(ratio),
        ..Observed::default()
    })
}

/// Dive held at `angle` at full throttle, recording the pitching moment the aircraft
/// would have with the elevator neutral (nose-up: the dive is stable, the aircraft wants
/// to pull out); then, from the end of the held dive, two branches: the elevator
/// released to neutral, and a pull-out to level flight.
fn flown_dive(
    aircraft: &Aircraft,
    env: &dyn Environment,
    pilot: &Pilot,
    altitude: f64,
    angle: f64,
    seconds: f64,
    load_factor: f64,
) -> Result<Observed, String> {
    let stall = stall_speed_at(aircraft, env, altitude, 1.0).map_err(error)?;
    let trim = max_level_speed(aircraft, env, altitude).map_err(error)?;
    let c0 = trim.commands(aircraft);
    let mut entry = trim.flight;
    entry.flight_path = angle;
    let mut pull = Pull::new(
        pilot,
        stall.alpha - pilot.margin,
        c0.pitch,
        trim.flight.speed,
    );
    let mut rudder = Rudder::new(pilot, c0.yaw);
    // (time, stick, pitching moment with the elevator neutral)
    let mut held = Vec::new();
    let dive = fly(aircraft, env, pilot, entry.state(), c0, seconds, |s| {
        let target = path_load_factor(s, s.plane_path, angle);
        let c = PilotControls {
            pitch: pull.command(s, target),
            roll: wings_level(s.roll, s),
            yaw: rudder.command(s),
            throttle: 1.0,
        };
        let neutral = PilotControls { pitch: 0.0, ..c };
        let moment = aircraft.loads(&s.state, &neutral, env).wrench.moment.y;
        held.push((s.time, c.pitch, moment));
        Some(c)
    });
    let (d0, d1) = (dive.first(), dive.last());
    let established = Flown {
        samples: dive
            .samples
            .iter()
            .copied()
            .filter(|s| s.time >= DIVE_ENTRY)
            .collect(),
        finite: dive.finite,
    };
    let held_path = (
        established.min(|s| s.plane_path),
        established.max(|s| s.plane_path),
    );
    let settled: Vec<_> = held.iter().filter(|&&(t, ..)| t >= DIVE_ENTRY).collect();
    let nose_down =
        settled.iter().filter(|&&&(_, _, m)| m < 0.0).count() as f64 / settled.len().max(1) as f64;
    let late: Vec<f64> = held
        .iter()
        .filter(|&&(t, ..)| t >= seconds - 5.0)
        .map(|&(_, stick, _)| stick)
        .collect();
    let stick = late.iter().sum::<f64>() / late.len().max(1) as f64;
    let moment_text = if nose_down == 0.0 {
        "cabreur tout au long".to_owned()
    } else if nose_down == 1.0 {
        "**piqueur tout au long**".to_owned()
    } else {
        format!(
            "cabreur sur {:.0} % du piqué établi, **piqueur sur {:.0} %**",
            100.0 * (1.0 - nose_down),
            100.0 * nose_down
        )
    };
    let mut text = format!(
        "piqué tenu à {:.0}° pleins gaz pendant {seconds:.0} s (pente entre {:.0}° et {:.0}° une fois établi) : vitesse maximale {:.0} km/h, régime maximal {:.0} tr/min, facteur de charge établi {:.2} à {:.2}, {:.0} m perdus ; manche à {stick:+.2} pour tenir la pente ; moment de tangage profondeur au neutre {moment_text}",
        deg(angle),
        deg(held_path.0),
        deg(held_path.1),
        kmh(dive.max(|s| s.speed)),
        rpm(dive.max(|s| s.state.rotor_speed)),
        established.min(|s| s.load_factor),
        established.max(|s| s.load_factor),
        d0.altitude - d1.altitude
    );
    if !dive.finite {
        return Ok(Observed {
            text,
            held_path: Some(held_path),
            trace: dive.trace(0.0, true),
            ..Observed::default()
        });
    }

    // Branch 1: elevator released to neutral.
    let mut released_rudder = rudder.clone();
    let release = fly(
        aircraft,
        env,
        pilot,
        d1.state,
        released_controls(&d1),
        RELEASE_TIME,
        |s| {
            Some(PilotControls {
                pitch: 0.0,
                roll: wings_level(s.roll, s),
                yaw: released_rudder.command(s),
                throttle: 1.0,
            })
        },
    );
    let path_offset = d1.plane_path - release.first().plane_path;
    let change = release.last().plane_path + path_offset - d1.plane_path;
    let verdict = if change > RELEASE_THRESHOLD {
        "le piqué se redresse seul (rappelant)"
    } else if change < -RELEASE_THRESHOLD {
        "**le piqué s'accentue (engagement)**"
    } else {
        "pente à peu près tenue (neutre)"
    };
    text += &format!(
        " ; relâché au neutre {RELEASE_TIME:.0} s : pente {:.0}° → {:.0}°, {verdict}, facteur de charge jusqu'à {:.1}",
        deg(d1.plane_path),
        deg(d1.plane_path + change),
        release.max(|s| s.load_factor)
    );

    // Branch 2: pull-out to a level flight path.
    let pull_start = PilotControls {
        pitch: pull.command(&d1, load_factor),
        roll: wings_level(d1.roll, &d1),
        yaw: rudder.command(&d1),
        throttle: 1.0,
    };
    let out = fly(
        aircraft,
        env,
        pilot,
        d1.state,
        pull_start,
        RECOVERY_TIME,
        |s| {
            (s.flight_path < 0.0).then(|| PilotControls {
                pitch: pull.command(s, load_factor),
                roll: wings_level(s.roll, s),
                yaw: rudder.command(s),
                throttle: 1.0,
            })
        },
    );
    let end = out.last();
    let completed = end.flight_path >= 0.0;
    let max_alpha = dive.max(|s| s.alpha).max(out.max(|s| s.alpha));
    text += &if completed {
        format!(
            " ; ressource à {load_factor:.1} de consigne : trajectoire horizontale en {:.1} s, {:.0} m perdus, facteur de charge maximal {:.1}, incidence maximale {:.1}°, à {:.0} km/h",
            end.time,
            d1.altitude - end.altitude,
            out.max(|s| s.load_factor),
            deg(out.max(|s| s.alpha)),
            kmh(end.speed)
        )
    } else {
        format!(
            " ; **ressource non terminée en {RECOVERY_TIME:.0} s** (pente {:+.0}°, {:.0} m perdus)",
            deg(end.flight_path),
            d1.altitude - end.altitude
        )
    };
    Ok(Observed {
        text,
        finite: release.finite && out.finite,
        completed,
        load_factor: Some((out.max(|s| s.load_factor), load_factor)),
        alpha: Some((max_alpha, stall.alpha)),
        held_path: Some(held_path),
        release: Some(change),
        trace: [dive.trace(0.0, true), out.trace(d1.time, true)].concat(),
        ..Observed::default()
    })
}

/// Controls of the first sample of the release branch (they only enter its figures).
fn released_controls(s: &Sample) -> PilotControls {
    PilotControls {
        pitch: 0.0,
        roll: wings_level(s.roll, s),
        yaw: 0.0,
        throttle: 1.0,
    }
}

fn flown(
    aircraft: &Aircraft,
    env: &dyn Environment,
    pilot: &Pilot,
    manoeuvre: &Manoeuvre,
) -> Result<Observed, String> {
    match *manoeuvre {
        Manoeuvre::Loop {
            altitude,
            speed,
            load_factor,
        } => flown_loop(
            aircraft,
            env,
            pilot,
            altitude.si(),
            speed.map(|v| v.si()),
            load_factor,
        ),
        Manoeuvre::PowerOffStall { altitude } => flown_stall(aircraft, env, pilot, altitude.si()),
        Manoeuvre::Glide { altitude } => flown_glide(aircraft, env, altitude.si()),
        Manoeuvre::Dive {
            altitude,
            angle,
            seconds,
            load_factor,
        } => flown_dive(
            aircraft,
            env,
            pilot,
            altitude.si(),
            angle.si(),
            seconds,
            load_factor,
        ),
    }
}

/// Flies one qualitative scenario.
pub fn run(aircraft: &Aircraft, env: &dyn Environment, scenario: &Scenario) -> QualitativeOutcome {
    QualitativeOutcome {
        name: scenario.name.clone(),
        description: scenario.description.clone(),
        source: scenario.source.clone(),
        flown: scenario
            .manoeuvre
            .as_ref()
            .map(|m| flown(aircraft, env, &PILOT, m)),
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use escadrille_data::aircraft::{AircraftData, Loading, Rotation, load_aircraft};
    use escadrille_data::units::{Angle, Length};
    use glam::DQuat;

    use super::*;
    use crate::trim::tests::{STILL_AIR, biplane};

    /// The synthetic biplane, edited before it is built.
    fn biplane_with(edit: impl FnOnce(&mut AircraftData)) -> Aircraft {
        let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../crates/data/tests/fixtures/assets");
        let mut data = load_aircraft(&assets, "test_biplane").unwrap();
        edit(&mut data);
        data.build(&Loading { fuel_fraction: 1.0 })
            .unwrap()
            .aircraft
    }

    fn fly_on(aircraft: &Aircraft, pilot: &Pilot, manoeuvre: Manoeuvre) -> Observed {
        let o = flown(aircraft, &STILL_AIR, pilot, &manoeuvre).unwrap();
        assert!(o.finite, "{o:?}");
        o
    }

    fn fly_one(manoeuvre: Manoeuvre) -> Observed {
        fly_on(&biplane(), &PILOT, manoeuvre)
    }

    const LOOP: Manoeuvre = Manoeuvre::Loop {
        altitude: Length::M(2_000.0),
        speed: None,
        load_factor: 3.5,
    };

    fn dive(angle: f64) -> Manoeuvre {
        Manoeuvre::Dive {
            altitude: Length::M(3_000.0),
            angle: Angle::Deg(angle),
            seconds: 20.0,
            load_factor: 4.0,
        }
    }

    #[test]
    fn bank_is_positive_right_wing_down() {
        let a = biplane();
        let state = RigidBodyState {
            position_ned: DVec3::new(0.0, 0.0, -1_000.0),
            attitude: DQuat::from_euler(EulerRot::ZYX, 0.0, 0.0, 30.0 * DEG),
            velocity_body: DVec3::new(40.0, 0.0, 0.0),
            rotor_speed: 150.0,
            ..RigidBodyState::default()
        };
        assert!((state.attitude * DVec3::Y).z > 0.0, "right wing down");
        let s = Sample::new(&a, &STILL_AIR, 0.0, state, &PilotControls::default(), None);
        assert!((s.roll - 30.0 * DEG).abs() < 1e-12, "{s:?}");
        assert!((s.bank - 30.0 * DEG).abs() < 1e-12, "{s:?}");
    }

    #[test]
    fn load_factor_matches_the_curvature_of_the_path() {
        // A steady pull, wings level: n = cos γ + V γ̇ / g, measured independently of
        // the pilot from the trajectory.
        let a = biplane();
        let trim = max_level_speed(&a, &STILL_AIR, 2_000.0).unwrap();
        let c0 = trim.commands(&a);
        let pull = PilotControls {
            pitch: c0.pitch + 0.15,
            ..c0
        };
        let flown = fly(
            &a,
            &STILL_AIR,
            &PILOT,
            trim.flight.state(),
            pull,
            2.0,
            |_| Some(pull),
        );
        let s = &flown.samples;
        let mut checked = 0;
        for i in 1..s.len() - 1 {
            if s[i].time < 0.5 {
                continue;
            }
            let rate = (s[i + 1].plane_path - s[i - 1].plane_path) / (2.0 * PILOT.dt);
            let kinematic = s[i].plane_path.cos() + s[i].speed * rate / G0;
            assert!(
                (s[i].load_factor - kinematic).abs() < 0.05 + 0.03 * kinematic.abs(),
                "{} vs {kinematic} at {} s",
                s[i].load_factor,
                s[i].time
            );
            checked += 1;
        }
        assert!(checked > 100 && flown.max(|s| s.load_factor) > 2.0);
    }

    #[test]
    fn the_pilot_pulls_the_loop_at_its_load_factor_without_stalling() {
        let o = fly_one(LOOP);
        assert!(o.completed, "{o:?}");
        let (n, target) = o.load_factor.unwrap();
        assert!((n - target).abs() < 0.1 * target, "{o:?}");
        // The pilot's limit is 2° below the stall; its overshoot stays bounded.
        let (alpha, stall) = o.alpha.unwrap();
        assert!(alpha < stall - PILOT.margin + 2.5 * DEG, "{o:?}");
    }

    #[test]
    fn without_anticipation_the_loop_stalls() {
        let careless = Pilot { lead: 0.0, ..PILOT };
        let o = fly_on(&biplane(), &careless, LOOP);
        let (alpha, stall) = o.alpha.unwrap();
        assert!(alpha > stall, "{o:?}");
        assert!(o.text.contains("**décroché**"), "{o:?}");
    }

    #[test]
    fn a_loop_pulled_too_gently_is_not_closed() {
        // Pulled at only 1.2 g the aircraft climbs away and runs out of speed long
        // before the top: the outcome must say so rather than claim a loop.
        let o = fly_one(Manoeuvre::Loop {
            altitude: Length::M(2_000.0),
            speed: None,
            load_factor: 1.2,
        });
        assert!(!o.completed, "{o:?}");
        assert!(o.text.contains("non bouclée"), "{o:?}");
    }

    #[test]
    fn halving_the_time_step_does_not_change_the_loop() {
        let a = biplane();
        let fine = Pilot {
            dt: 0.5 * PILOT.dt,
            ..PILOT
        };
        let figures = |pilot: &Pilot| {
            let o = fly_on(&a, pilot, LOOP);
            (o.load_factor.unwrap().0, o.alpha.unwrap().0)
        };
        let (coarse, fine) = (figures(&PILOT), figures(&fine));
        assert!(
            (coarse.0 - fine.0).abs() < 0.01 * fine.0,
            "{coarse:?} {fine:?}"
        );
        assert!(
            (coarse.1 - fine.1).abs() < 0.01 * fine.1,
            "{coarse:?} {fine:?}"
        );
    }

    #[test]
    fn the_power_off_stall_breaks_at_a_knot_per_second_and_recovers() {
        let o = fly_one(Manoeuvre::PowerOffStall {
            altitude: Length::M(1_000.0),
        });
        let (alpha, stall) = o.alpha.unwrap();
        assert!(alpha >= stall, "{o:?}");
        let deceleration = o.deceleration.unwrap();
        assert!(
            (deceleration - DECELERATION).abs() < 0.2 * DECELERATION,
            "{o:?}"
        );
        // A departure: the nose drops, or a wing drops past DEPARTURE_BANK (with the
        // swirl of the idling propeller, the synthetic biplane rolls off).
        let (drop, bank) = o.stall_drop.unwrap();
        assert!(drop > 0.0 || bank.abs() > DEPARTURE_BANK, "{o:?}");
        assert!(o.completed, "back to a normal glide: {o:?}");
    }

    #[test]
    fn halving_the_time_step_does_not_change_the_stall() {
        // The departure grows exponentially: measured over a fixed window, it must still
        // converge with the time step.
        let a = biplane();
        let fine = Pilot {
            dt: 0.5 * PILOT.dt,
            ..PILOT
        };
        let stall = Manoeuvre::PowerOffStall {
            altitude: Length::M(1_000.0),
        };
        let coarse = fly_on(&a, &PILOT, stall.clone()).stall_drop.unwrap();
        let fine = fly_on(&a, &fine, stall).stall_drop.unwrap();
        assert!(
            (coarse.0 - fine.0).abs() < 0.05 * fine.0.abs() + 0.2 * DEG,
            "{coarse:?} {fine:?}"
        );
        assert!(
            (coarse.1 - fine.1).abs() < 0.05 * fine.1.abs() + 0.2 * DEG,
            "{coarse:?} {fine:?}"
        );
    }

    #[test]
    fn the_wing_drop_comes_from_the_propeller() {
        // The airframe is symmetric: turning the propeller the other way mirrors the
        // departure.
        let stall = Manoeuvre::PowerOffStall {
            altitude: Length::M(1_000.0),
        };
        let right = fly_one(stall.clone()).stall_drop.unwrap();
        let mirrored = biplane_with(|d| d.file.propeller.rotation = Rotation::CounterClockwise);
        let left = fly_on(&mirrored, &PILOT, stall).stall_drop.unwrap();
        assert!(right.1.abs() > 1.0 * DEG, "{right:?}");
        assert!(
            (right.1 + left.1).abs() < 0.01 * right.1.abs(),
            "{right:?} {left:?}"
        );
        assert!(
            (right.0 - left.0).abs() < 0.01 * right.0.abs() + 1e-9,
            "{right:?} {left:?}"
        );
    }

    #[test]
    fn best_glide_is_the_shallowest_and_flies_its_ratio() {
        let a = biplane();
        let best = best_glide(&a, &STILL_AIR, 1_000.0).unwrap();
        assert!(best.flight.flight_path < 0.0, "{best:?}");
        for dv in [-3.0, 3.0] {
            let other = glide(&a, &STILL_AIR, 1_000.0, best.flight.speed + dv).unwrap();
            assert!(other.flight.flight_path < best.flight.flight_path, "{dv}");
        }
        // Distance over height lost in a free glide from the trim.
        let ratio = fly_one(Manoeuvre::Glide {
            altitude: Length::M(1_000.0),
        })
        .glide_ratio
        .unwrap();
        let c = best.commands(&a);
        let flown = fly(&a, &STILL_AIR, &PILOT, best.flight.state(), c, 10.0, |_| {
            Some(c)
        });
        let (start, end) = (
            flown.first().state.position_ned,
            flown.last().state.position_ned,
        );
        let free = (end.x - start.x) / (end.z - start.z);
        assert!((free - ratio).abs() < 0.01 * ratio, "{free} vs {ratio}");
    }

    #[test]
    fn the_dive_is_held_then_pulled_out_at_the_load_factor() {
        for angle in [-80.0, -90.0] {
            let o = fly_one(dive(angle));
            let (low, high) = o.held_path.unwrap();
            let wanted = angle * DEG;
            assert!(
                (low - wanted).abs() < 2.0 * DEG && (high - wanted).abs() < 2.0 * DEG,
                "{angle}: {o:?}"
            );
            assert!(o.completed, "back to level flight: {o:?}");
            let (n, target) = o.load_factor.unwrap();
            assert!((n - target).abs() < 0.1 * target, "{o:?}");
        }
    }

    #[test]
    fn released_in_the_dive_the_stable_biplane_pulls_out_and_a_nose_heavy_one_tucks() {
        // Elevator neutral: the synthetic biplane's tail (−1° incidence) pitches it up;
        // set at +3°, the tail gives a nose-down moment at zero lift and the dive steepens.
        let stable = fly_one(dive(-60.0));
        assert!(stable.release.unwrap() > RELEASE_THRESHOLD, "{stable:?}");
        assert!(stable.text.contains("cabreur tout au long"), "{stable:?}");
        let nose_heavy = biplane_with(|d| {
            let tail = d.file.surfaces.iter_mut().find(|s| s.name == "tailplane");
            tail.unwrap().panels[0].incidence = Angle::Deg(3.0);
        });
        let tucking = fly_on(&nose_heavy, &PILOT, dive(-60.0));
        assert!(tucking.release.unwrap() < -RELEASE_THRESHOLD, "{tucking:?}");
        assert!(tucking.text.contains("**piqueur"), "{tucking:?}");
    }

    #[test]
    fn behaviours_without_a_manoeuvre_are_listed_not_flown() {
        let scenario = Scenario {
            name: "ground".to_owned(),
            description: "swings left on take-off".to_owned(),
            source: "T8".to_owned(),
            manoeuvre: None,
        };
        let outcome = run(&biplane(), &STILL_AIR, &scenario);
        assert!(outcome.flown.is_none() && outcome.finite());
        assert!(outcome.observed().contains("non simulé"));
    }
}
