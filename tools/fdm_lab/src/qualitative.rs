//! Qualitative scenarios of `essais.ron` (M1 plan, step 6): behaviours from the pilots'
//! testimonies (fiche, section 6), flown by a simple scripted pilot with the player's
//! commands. The outcome is what the aircraft did, set beside the expected behaviour of
//! the scenario: testimonies are not measurements, so nothing here blocks the validation
//! (the shape of a behaviour is for the player and the aerodynamicist to judge).
//!
//! Every manoeuvre starts heading north, wings level, in still air.

use std::f64::consts::TAU;

use escadrille_data::essais::{Manoeuvre, Scenario};
use escadrille_fdm::aero::polar::wrap_angle;
use escadrille_fdm::aircraft::{Aircraft, PilotControls};
use escadrille_fdm::constants::G0;
use escadrille_fdm::environment::Environment;
use escadrille_fdm::rigid_body::RigidBodyState;
use glam::DVec3;

use crate::performance::{PerformanceError, Stall, max_level_speed, stall_speed};
use crate::trim::{Flight, LEVEL_AT_SPEED, PATH_AT_SPEED, Trim, solve_robust};

/// Integration step, s (ADR 0005: 120 Hz with two substeps).
const DT: f64 = 1.0 / 240.0;

/// Pilot's elevator: proportional and integral gains on the normalised margin (E:, a
/// brisk but smooth pull, reaching the load factor in about a second), load-factor error
/// that counts as a full margin, and how far ahead the pilot anticipates the incidence
/// (E:, feel of the controls and of the buffet).
const PULL_GAIN: f64 = 0.4;
const PULL_INTEGRAL: f64 = 1.5;
const LOAD_SCALE: f64 = 2.0;
const ALPHA_LEAD: f64 = 0.3;

/// Angle-of-attack margin that counts as much as [`LOAD_SCALE`], rad (E:).
const ALPHA_SCALE: f64 = 5.0 * std::f64::consts::PI / 180.0;

/// Margin the pilot keeps below the stall incidence, rad (E:, the buffet warns him).
const PILOT_MARGIN: f64 = 2.0 * std::f64::consts::PI / 180.0;

/// Time in which the pilot brings the flight path back to the dive angle, s (E:).
const PATH_TIME: f64 = 1.0;

/// Pilot's ailerons and rudder (E:): full aileron for half a radian of bank, damped by
/// the roll rate; full rudder for about 15° of sideslip.
const ROLL_GAIN: f64 = 2.0;
const ROLL_DAMPING: f64 = 0.5;
const YAW_GAIN: f64 = 4.0;

/// Power-off stall: entry speed over the idle stall speed (the usual 1.3, E:), rate at
/// which the stick is eased back, per second of full travel (E:, about a knot per
/// second of deceleration), nose-down pitch rate that marks the break (E:), pilot's
/// reaction time before recovering (E:) and window in which the nose and wing drops are
/// measured (E:).
const STALL_ENTRY: f64 = 1.3;
const STALL_STICK_RATE: f64 = 0.05;
const BREAK_PITCH_RATE: f64 = 3.0 * std::f64::consts::PI / 180.0;
const REACTION: f64 = 1.0;
const DROP_WINDOW: f64 = 3.0;

/// Longest time given to a manoeuvre, s.
const LOOP_TIME: f64 = 60.0;
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
}

/// One instant of a manoeuvre.
#[derive(Debug, Clone, Copy)]
struct Sample {
    time: f64,
    state: RigidBodyState,
    speed: f64,
    alpha: f64,
    beta: f64,
    altitude: f64,
    /// Flight-path angle, rad.
    flight_path: f64,
    /// Aerodynamic and propeller force along −z over the weight.
    load_factor: f64,
    /// Nose angle above the horizon in the initial vertical plane, unwrapped, rad (a
    /// loop adds 2π).
    pitch: f64,
    /// Bank out of that plane, rad (positive right wing down).
    roll: f64,
}

impl Sample {
    fn new(
        aircraft: &Aircraft,
        env: &dyn Environment,
        time: f64,
        state: RigidBodyState,
        controls: &PilotControls,
        previous_pitch: Option<f64>,
    ) -> Self {
        let v = state.velocity_body;
        let speed = v.length();
        let force = aircraft.loads(&state, controls, env).wrench.force;
        let nose = state.attitude * DVec3::X;
        let raw = (-nose.z).atan2(nose.x);
        let (wing, belly) = (state.attitude * DVec3::Y, state.attitude * DVec3::Z);
        let velocity_ned = state.attitude * v;
        let ratio = |a: f64| if speed > 0.0 { a / speed } else { 0.0 };
        Self {
            time,
            state,
            speed,
            alpha: v.z.atan2(v.x),
            beta: ratio(v.y).asin(),
            altitude: -state.position_ned.z,
            flight_path: ratio(-velocity_ned.z).asin(),
            load_factor: -force.z / (aircraft.mass().mass() * G0),
            pitch: previous_pitch.map_or(raw, |p| p + wrap_angle(raw - p)),
            roll: (-belly.y).atan2(wing.y),
        }
    }
}

struct Flown {
    samples: Vec<Sample>,
    finite: bool,
}

/// Flies from `start` until the pilot returns `None` or `seconds` have passed.
fn fly(
    aircraft: &Aircraft,
    env: &dyn Environment,
    start: RigidBodyState,
    controls: PilotControls,
    seconds: f64,
    mut pilot: impl FnMut(&Sample) -> Option<PilotControls>,
) -> Flown {
    let mut samples = vec![Sample::new(aircraft, env, 0.0, start, &controls, None)];
    let steps = (seconds / DT).round() as u32;
    for i in 1..=steps {
        let last = samples[samples.len() - 1];
        let Some(c) = pilot(&last) else { break };
        let state = aircraft.step(&last.state, &c, env, DT);
        let values = [
            state.velocity_body.to_array(),
            state.angular_velocity_body.to_array(),
            state.position_ned.to_array(),
        ];
        if !(values.iter().flatten().all(|v| v.is_finite()) && state.rotor_speed.is_finite()) {
            return Flown {
                samples,
                finite: false,
            };
        }
        let time = f64::from(i) * DT;
        samples.push(Sample::new(
            aircraft,
            env,
            time,
            state,
            &c,
            Some(last.pitch),
        ));
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
struct Pull {
    alpha_limit: f64,
    base: f64,
    reference_speed: f64,
    integral: f64,
    last_alpha: Option<f64>,
}

impl Pull {
    fn new(alpha_limit: f64, base: f64, reference_speed: f64) -> Self {
        Self {
            alpha_limit,
            base,
            reference_speed,
            integral: 0.0,
            last_alpha: None,
        }
    }

    fn command(&mut self, s: &Sample, target: f64) -> f64 {
        let rate = self.last_alpha.map_or(0.0, |a| (s.alpha - a) / DT);
        self.last_alpha = Some(s.alpha);
        let error = ((target - s.load_factor) / LOAD_SCALE)
            .min((self.alpha_limit - s.alpha - ALPHA_LEAD * rate) / ALPHA_SCALE);
        let scale = (self.reference_speed / s.speed.max(1.0)).powi(2).min(1.0);
        // The integral stays within what the stick can add (no wind-up at the stops).
        self.integral = (self.integral + scale * PULL_INTEGRAL * error * DT)
            .clamp(-1.0 - self.base, 1.0 - self.base);
        (self.base + scale * PULL_GAIN * error + self.integral).clamp(-1.0, 1.0)
    }
}

/// Ailerons holding the wings in the initial vertical plane.
fn wings_level(s: &Sample) -> f64 {
    (-ROLL_GAIN * s.roll - ROLL_DAMPING * s.state.angular_velocity_body.x).clamp(-1.0, 1.0)
}

/// Rudder keeping the sideslip small, around the trim's rudder.
fn coordinated(s: &Sample, base: f64) -> f64 {
    (base + YAW_GAIN * s.beta).clamp(-1.0, 1.0)
}

fn deg(angle: f64) -> f64 {
    angle.to_degrees()
}

fn kmh(speed: f64) -> f64 {
    speed * 3.6
}

fn error(e: PerformanceError) -> String {
    format!("{e:?}")
}

/// Incidence the pilot does not go past: a little below that of the idle stall (lowest
/// steady speed).
fn alpha_limit(stall: &Stall) -> f64 {
    stall.alpha - PILOT_MARGIN
}

/// Loop from the maximum level speed at full throttle.
fn flown_loop(
    aircraft: &Aircraft,
    env: &dyn Environment,
    altitude: f64,
    load_factor: f64,
) -> Result<Observed, String> {
    let stall = stall_speed(aircraft, env, altitude).map_err(error)?;
    let trim = max_level_speed(aircraft, env, altitude).map_err(error)?;
    let c0 = trim.commands(aircraft);
    let start = trim.flight.state();
    let mut pull = Pull::new(alpha_limit(&stall), c0.pitch, trim.flight.speed);
    let pitch0 = trim.flight.pitch();
    let flown = fly(aircraft, env, start, c0, LOOP_TIME, |s| {
        (s.pitch - pitch0 < TAU).then(|| PilotControls {
            pitch: pull.command(s, load_factor),
            roll: wings_level(s),
            yaw: coordinated(s, c0.yaw),
            throttle: 1.0,
        })
    });
    let samples = &flown.samples;
    let entry = samples[0];
    let last = samples[samples.len() - 1];
    let top = samples
        .iter()
        .copied()
        .fold(entry, |a, b| if b.altitude > a.altitude { b } else { a });
    let max = |f: fn(&Sample) -> f64| samples.iter().map(f).fold(f64::MIN, f64::max);
    let stalled = samples
        .iter()
        .find(|s| s.alpha > stall.alpha)
        .map(|s| s.pitch - pitch0);
    let completed = last.pitch - pitch0 >= TAU;
    let mut text = if completed {
        format!(
            "**bouclée** en {:.1} s ; sommet à +{:.0} m, à {:.0} km/h (décrochage au ralenti : {:.0} km/h) ; sortie à {:+.0} m de l'entrée, {:.0} km/h",
            last.time,
            top.altitude - entry.altitude,
            kmh(top.speed),
            kmh(stall.speed),
            last.altitude - entry.altitude,
            kmh(last.speed)
        )
    } else {
        format!(
            "**non bouclée** : rotation maximale {:.0}° en {:.0} s ; sommet à +{:.0} m, à {:.0} km/h (décrochage au ralenti : {:.0} km/h)",
            deg(max(|s| s.pitch) - pitch0),
            last.time,
            top.altitude - entry.altitude,
            kmh(top.speed),
            kmh(stall.speed)
        )
    };
    text += &format!(
        " ; entrée à {:.0} km/h, facteur de charge maximal {:.1} (consigne {load_factor:.1}), incidence maximale {:.1}° (décrochage {:.1}°), inclinaison hors du plan jusqu'à {:.0}°",
        kmh(entry.speed),
        max(|s| s.load_factor),
        deg(max(|s| s.alpha)),
        deg(stall.alpha),
        deg(max(|s| s.roll.abs()))
    );
    text += &match stalled {
        Some(at) => format!(" ; **décroché** à {:.0}° de rotation", deg(at)),
        None => " ; sans décrocher".to_owned(),
    };
    Ok(Observed {
        text,
        finite: flown.finite,
        completed,
        load_factor: Some((max(|s| s.load_factor), load_factor)),
        alpha: Some((max(|s| s.alpha), stall.alpha)),
        held_path: None,
    })
}

/// Power-off stall from level flight at 1.3 times the idle stall speed.
fn flown_stall(
    aircraft: &Aircraft,
    env: &dyn Environment,
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
    let glide = glide_at(aircraft, env, altitude, entry_speed).map_err(error)?;
    let c0 = level.commands(aircraft);
    let recovery = glide.commands(aircraft);
    let gamma_glide = glide.flight.flight_path;

    // Approach: idle, wings held level, stick eased back to the stop. The break is the
    // nose dropping past the stall incidence with the stick still back.
    let mut stick = c0.pitch;
    let mut beyond = false;
    let mut at_stop = None::<f64>;
    let approach = fly(aircraft, env, level.flight.state(), c0, 120.0, |s| {
        beyond |= s.alpha >= stall.alpha;
        if beyond && s.state.angular_velocity_body.y < -BREAK_PITCH_RATE {
            return None;
        }
        if at_stop.is_some_and(|t| s.time - t > 10.0) {
            return None;
        }
        stick = (stick + STALL_STICK_RATE * DT).min(1.0);
        if stick >= 1.0 && at_stop.is_none() {
            at_stop = Some(s.time);
        }
        Some(PilotControls {
            pitch: stick,
            roll: wings_level(s),
            yaw: coordinated(s, recovery.yaw),
            throttle: 0.0,
        })
    });
    let entry = approach.samples[0];
    let at_break = approach.samples[approach.samples.len() - 1];
    let max_alpha = approach
        .samples
        .iter()
        .map(|s| s.alpha)
        .fold(f64::MIN, f64::max);
    let broke = beyond && at_break.state.angular_velocity_body.y < -BREAK_PITCH_RATE;
    if !approach.finite || !broke {
        let text = format!(
            "**pas d'abattée** : manche en butée, l'avion s'enfonce à {:.0} km/h, incidence maximale {:.1}° (décrochage au ralenti {:.1}°, à {:.0} km/h){}",
            kmh(at_break.speed),
            deg(max_alpha),
            deg(stall.alpha),
            kmh(stall.speed),
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
            ..Observed::default()
        });
    }

    // At the break: controls held for the reaction time, then the glide's stick, wings
    // levelled, until the flight path is back to the glide's.
    let held = PilotControls {
        pitch: stick,
        roll: wings_level(&at_break),
        yaw: coordinated(&at_break, recovery.yaw),
        throttle: 0.0,
    };
    let after = fly(aircraft, env, at_break.state, held, RECOVERY_TIME, |s| {
        if s.time < REACTION {
            return Some(held);
        }
        if s.time > REACTION + 1.0 && s.flight_path >= gamma_glide {
            return None;
        }
        Some(PilotControls {
            pitch: recovery.pitch,
            roll: wings_level(s),
            yaw: coordinated(s, recovery.yaw),
            throttle: 0.0,
        })
    });
    // Pitch continues from the approach's unwrapped angle.
    let offset = at_break.pitch - after.samples[0].pitch;
    let window = after.samples.iter().filter(|s| s.time <= DROP_WINDOW);
    let (lowest_nose, widest_bank) = window.fold((f64::MAX, 0.0_f64), |(n, b), s| {
        (n.min(s.pitch + offset), b.max(s.roll.abs()))
    });
    let end = after.samples[after.samples.len() - 1];
    let recovered = end.flight_path >= gamma_glide;
    let text = format!(
        "abattée de {:.0}° en {DROP_WINDOW:.0} s (assiette {:+.0}° → {:+.0}°), inclinaison jusqu'à {:.0}° ; décroche à {:.0} km/h et {:.1}° d'incidence (en vol stabilisé : {:.0} km/h) après {:.0} s de manche tiré depuis {:.0} km/h ; {}",
        deg(at_break.pitch - lowest_nose),
        deg(at_break.pitch),
        deg(lowest_nose),
        deg(widest_bank),
        kmh(at_break.speed),
        deg(at_break.alpha),
        kmh(stall.speed),
        at_break.time,
        kmh(entry.speed),
        if recovered {
            format!(
                "remis en plané normal ({:.1}°) en {:.1} s, {:.0} m perdus",
                deg(gamma_glide),
                end.time,
                at_break.altitude - end.altitude
            )
        } else {
            format!(
                "**pas revenu en plané normal en {RECOVERY_TIME:.0} s** ({:.0} m perdus)",
                at_break.altitude - end.altitude
            )
        }
    );
    Ok(Observed {
        text,
        finite: approach.finite && after.finite,
        completed: recovered,
        alpha: Some((max_alpha, stall.alpha)),
        ..Observed::default()
    })
}

/// Steady glide at idle at a given speed.
fn glide_at(
    aircraft: &Aircraft,
    env: &dyn Environment,
    altitude: f64,
    speed: f64,
) -> Result<Trim, PerformanceError> {
    let mut guess = Flight::guess(altitude, speed, 0.0);
    guess.flight_path = -0.12;
    Ok(solve_robust(aircraft, env, guess, PATH_AT_SPEED)?)
}

/// Best glide at idle: golden-section search on the speed for the shallowest path.
fn best_glide(
    aircraft: &Aircraft,
    env: &dyn Environment,
    altitude: f64,
) -> Result<Trim, PerformanceError> {
    let stall = stall_speed(aircraft, env, altitude)?;
    let vmax = max_level_speed(aircraft, env, altitude)?.flight.speed;
    let path = |speed: f64| glide_at(aircraft, env, altitude, speed);
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
    let text = format!(
        "finesse maximale {:.1} à {:.0} km/h (pente {:.1}°, descente {:.1} m/s), moteur au ralenti, hélice entraînée à {:.0} tr/min",
        1.0 / (-f.flight_path).tan(),
        kmh(f.speed),
        deg(f.flight_path),
        -f.climb_rate(),
        f.rotor_speed * 30.0 / std::f64::consts::PI
    );
    Ok(Observed {
        text,
        finite: true,
        completed: true,
        ..Observed::default()
    })
}

/// Dive held at `angle` and full throttle, then a pull-out. While the pilot holds the
/// dive, the pitching moment the aircraft would have with the elevator neutral is
/// recorded: nose-up means the dive is stable (the aircraft wants to pull out).
fn flown_dive(
    aircraft: &Aircraft,
    env: &dyn Environment,
    altitude: f64,
    angle: f64,
    seconds: f64,
    load_factor: f64,
) -> Result<Observed, String> {
    let stall = stall_speed(aircraft, env, altitude).map_err(error)?;
    let trim = max_level_speed(aircraft, env, altitude).map_err(error)?;
    let c0 = trim.commands(aircraft);
    let mut entry = trim.flight;
    entry.flight_path = angle;
    let mut pull = Pull::new(alpha_limit(&stall), c0.pitch, trim.flight.speed);
    // (time, stick, pitching moment with the elevator neutral, speed)
    let mut held = Vec::new();
    let first = fly(aircraft, env, entry.state(), c0, seconds, |s| {
        // Load factor that brings the path back to the dive angle in PATH_TIME.
        let target = s.flight_path.cos() + s.speed * (angle - s.flight_path) / (PATH_TIME * G0);
        let c = PilotControls {
            pitch: pull.command(s, target),
            roll: wings_level(s),
            yaw: coordinated(s, c0.yaw),
            throttle: 1.0,
        };
        let neutral = PilotControls { pitch: 0.0, ..c };
        let moment = aircraft.loads(&s.state, &neutral, env).wrench.moment.y;
        held.push((s.time, c.pitch, moment, s.speed));
        Some(c)
    });
    let dive = &first.samples;
    let (d0, d1) = (dive[0], dive[dive.len() - 1]);
    let max =
        |samples: &[Sample], f: fn(&Sample) -> f64| samples.iter().map(f).fold(f64::MIN, f64::max);
    let min =
        |samples: &[Sample], f: fn(&Sample) -> f64| samples.iter().map(f).fold(f64::MAX, f64::min);
    // Settled part of the dive: after the first two seconds.
    let settled: Vec<Sample> = dive.iter().copied().filter(|s| s.time >= 2.0).collect();
    let tuck = held
        .iter()
        .find(|&&(t, _, moment, _)| t >= 2.0 && moment < 0.0)
        .map(|&(_, _, _, speed)| speed);
    let late: Vec<f64> = held
        .iter()
        .filter(|&&(t, ..)| t >= seconds - 5.0)
        .map(|&(_, stick, ..)| stick)
        .collect();
    let stick = late.iter().sum::<f64>() / late.len().max(1) as f64;
    let mut text = format!(
        "piqué tenu à {:.0}° pleins gaz pendant {seconds:.0} s (pente entre {:.0}° et {:.0}°) : vitesse maximale {:.0} km/h, régime maximal {:.0} tr/min, facteur de charge {:.1} à {:.1}, {:.0} m perdus ; manche à {stick:+.2} pour tenir la pente ; profondeur au neutre, {}",
        deg(angle),
        deg(min(&settled, |s| s.flight_path)),
        deg(max(&settled, |s| s.flight_path)),
        kmh(max(dive, |s| s.speed)),
        max(dive, |s| s.state.rotor_speed) * 30.0 / std::f64::consts::PI,
        min(dive, |s| s.load_factor),
        max(dive, |s| s.load_factor),
        d0.altitude - d1.altitude,
        match tuck {
            None => "moment cabreur tout au long (rappelant)".to_owned(),
            Some(speed) => format!(
                "**moment piqueur à partir de {:.0} km/h (engagement)**",
                kmh(speed)
            ),
        }
    );
    let held_path = Some((
        min(&settled, |s| s.flight_path),
        max(&settled, |s| s.flight_path),
    ));
    if !first.finite {
        return Ok(Observed {
            text,
            held_path,
            ..Observed::default()
        });
    }

    let start = PilotControls {
        pitch: pull.command(&d1, load_factor),
        roll: wings_level(&d1),
        yaw: coordinated(&d1, c0.yaw),
        throttle: 1.0,
    };
    let second = fly(aircraft, env, d1.state, start, RECOVERY_TIME, |s| {
        (s.pitch < 0.0).then(|| PilotControls {
            pitch: pull.command(s, load_factor),
            roll: wings_level(s),
            yaw: coordinated(s, c0.yaw),
            throttle: 1.0,
        })
    });
    let out = &second.samples;
    let end = out[out.len() - 1];
    text += &if end.pitch >= 0.0 {
        format!(
            " ; ressource à {load_factor:.1} de consigne : en palier en {:.1} s, {:.0} m perdus, facteur de charge maximal {:.1}, incidence maximale {:.1}°, à {:.0} km/h",
            end.time,
            d1.altitude - end.altitude,
            max(out, |s| s.load_factor),
            deg(max(out, |s| s.alpha)),
            kmh(end.speed)
        )
    } else {
        format!(
            " ; **ressource non terminée en {RECOVERY_TIME:.0} s** (assiette {:+.0}°, {:.0} m perdus)",
            deg(end.pitch),
            d1.altitude - end.altitude
        )
    };
    Ok(Observed {
        text,
        finite: second.finite,
        completed: end.pitch >= 0.0,
        load_factor: Some((max(out, |s| s.load_factor), load_factor)),
        alpha: Some((
            max(dive, |s| s.alpha).max(max(out, |s| s.alpha)),
            stall.alpha,
        )),
        held_path,
    })
}

fn flown(
    aircraft: &Aircraft,
    env: &dyn Environment,
    manoeuvre: &Manoeuvre,
) -> Result<Observed, String> {
    match *manoeuvre {
        Manoeuvre::Loop {
            altitude,
            load_factor,
        } => flown_loop(aircraft, env, altitude.si(), load_factor),
        Manoeuvre::PowerOffStall { altitude } => flown_stall(aircraft, env, altitude.si()),
        Manoeuvre::Glide { altitude } => flown_glide(aircraft, env, altitude.si()),
        Manoeuvre::Dive {
            altitude,
            angle,
            seconds,
            load_factor,
        } => flown_dive(
            aircraft,
            env,
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
        flown: scenario.manoeuvre.as_ref().map(|m| flown(aircraft, env, m)),
    }
}

#[cfg(test)]
mod tests {
    use escadrille_data::units::{Angle, Length};

    use super::*;
    use crate::trim::tests::{STILL_AIR, biplane};

    fn fly_one(manoeuvre: Manoeuvre) -> Observed {
        let scenario = Scenario {
            name: "test".to_owned(),
            description: "test".to_owned(),
            source: "test".to_owned(),
            manoeuvre: Some(manoeuvre),
        };
        let outcome = run(&biplane(), &STILL_AIR, &scenario);
        assert!(outcome.finite(), "{outcome:?}");
        outcome.flown.unwrap().unwrap()
    }

    #[test]
    fn the_pilot_pulls_the_loop_at_its_load_factor_without_stalling() {
        let o = fly_one(Manoeuvre::Loop {
            altitude: Length::M(2_000.0),
            load_factor: 3.5,
        });
        assert!(o.completed, "{o:?}");
        let (n, target) = o.load_factor.unwrap();
        assert!((n - target).abs() < 0.1 * target, "{o:?}");
        let (alpha, stall) = o.alpha.unwrap();
        assert!(alpha < stall, "the pilot keeps below the stall: {o:?}");
    }

    #[test]
    fn a_loop_pulled_too_gently_is_not_closed() {
        // Pulled at only 1.2 g the aircraft climbs away and runs out of speed long
        // before the top: the outcome must say so rather than claim a loop.
        let o = fly_one(Manoeuvre::Loop {
            altitude: Length::M(2_000.0),
            load_factor: 1.2,
        });
        assert!(!o.completed, "{o:?}");
        assert!(o.text.contains("non bouclée"), "{o:?}");
    }

    #[test]
    fn the_power_off_stall_breaks_past_the_stall_incidence_and_recovers() {
        let o = fly_one(Manoeuvre::PowerOffStall {
            altitude: Length::M(1_000.0),
        });
        let (alpha, stall) = o.alpha.unwrap();
        assert!(alpha >= stall, "{o:?}");
        assert!(o.completed, "back to a normal glide: {o:?}");
        assert!(o.text.starts_with("abattée"), "{o:?}");
    }

    #[test]
    fn best_glide_is_the_shallowest() {
        let a = biplane();
        let best = best_glide(&a, &STILL_AIR, 1_000.0).unwrap();
        assert!(best.flight.flight_path < 0.0, "{best:?}");
        for dv in [-3.0, 3.0] {
            let other = glide_at(&a, &STILL_AIR, 1_000.0, best.flight.speed + dv).unwrap();
            assert!(other.flight.flight_path < best.flight.flight_path, "{dv}");
        }
        assert!(
            fly_one(Manoeuvre::Glide {
                altitude: Length::M(1_000.0)
            })
            .text
            .contains("finesse maximale")
        );
    }

    #[test]
    fn the_dive_is_held_then_pulled_out_at_the_load_factor() {
        let angle = -80f64.to_radians();
        let o = fly_one(Manoeuvre::Dive {
            altitude: Length::M(3_000.0),
            angle: Angle::Deg(-80.0),
            seconds: 20.0,
            load_factor: 4.0,
        });
        let (low, high) = o.held_path.unwrap();
        assert!(
            (low - angle).abs() < 2f64.to_radians() && (high - angle).abs() < 2f64.to_radians(),
            "{o:?}"
        );
        assert!(o.completed, "back to level flight: {o:?}");
        let (n, target) = o.load_factor.unwrap();
        assert!((n - target).abs() < 0.1 * target, "{o:?}");
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
