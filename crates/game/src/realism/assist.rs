//! The flight assists (skill `realisme`; plan `m2-aides-pilotage.md`, step 3). They move
//! the commands as a pilot would, from what a pilot reads and feels (incidence,
//! sideslip, attitude, rates), never by a force: the aeroplane is the same at every
//! setting. Their gains come from the flight model (`assist_gains.ron`, computed by
//! `fdm_lab assist`), interpolated in equivalent airspeed.
//!
//! - **Coordination**: the rudder meets the adverse yaw of the roll rate and the
//!   aileron, and cancels the remaining sideslip (proportional and integral on β,
//!   damping on the yaw rate beyond that of the turn); the pilot's rudder adds to it.
//! - **Stall guard**: when the pilot's elevator would take the incidence past the stall's
//!   less [`STALL_MARGIN`], the elevator holding that limit replaces it. It does not give
//!   back the speed a zoom has spent: over the top of a climb too steep, the aeroplane
//!   falls through, as it would.
//! - **Virtual instructor**: the stick asks for a bank (x, up to [`INSTRUCTOR_BANK`]) and
//!   a load factor (y, from 0 to [`INSTRUCTOR_LOAD`] around level flight), the aileron
//!   and the elevator give them, within the guard's limit; coordinated. It also keeps
//!   the climb within what the speed allows ([`CLIMB_LIMIT`]).
//! - **On the ground** (plan `m2-train-sol.md`, step 6), none of that holds: the aeroplane
//!   stands past the guard's incidence, and sideslip means nothing at rest. They give
//!   way to the **heading hold**, on the rudder alone; the pilot's rudder adds to it and
//!   moves the heading held. With the throttle open, the instructor raises the tail and
//!   flies the aeroplane off.

use std::f64::consts::{PI, TAU};

use bevy::math::{DVec3, EulerRot};
use escadrille_data::assist::AssistFile;
pub use escadrille_data::assist::STALL_MARGIN;
use escadrille_fdm::aircraft::{Aircraft, PilotControls};
use escadrille_fdm::constants::{G0, RHO0};
use escadrille_fdm::environment::Environment;
use escadrille_fdm::rigid_body::RigidBodyState;

use super::{FlightAssist, RealismSettings};
use crate::flight::PilotInput;

/// Largest bank the instructor gives, rad (plan `m2-aides-pilotage.md`).
pub const INSTRUCTOR_BANK: f64 = 60.0 * PI / 180.0;
/// Fastest change of the bank the instructor asks for, rad/s: a smooth roll-in, the
/// aileron short of its stop, and less adverse yaw for the rudder to meet (full aileron
/// rolled the SPAD at 70°/s into 8° of sideslip).
pub const INSTRUCTOR_ROLL_RATE: f64 = 45.0 * PI / 180.0;
/// Largest load factor the instructor asks for (plan).
pub const INSTRUCTOR_LOAD: f64 = 3.5;
/// Time the instructor takes to bring the flight path back to level, stick released, or
/// within its climb limit, s: a gentle levelling, slower than the incidence and the bank.
pub const LEVELLING_S: f64 = 4.0;
/// Steepest climb the instructor allows, rad, while the guard's limit still gives
/// [`CLIMB_LOADS`].1 g or more; none below [`CLIMB_LOADS`].0 g, the speed of a turn
/// running out: the beginner is not left hanging at the top of a zoom.
pub const CLIMB_LIMIT: f64 = 30.0 * PI / 180.0;
pub const CLIMB_LOADS: (f64, f64) = (1.2, 2.0);
/// The incidence's integral runs only within this distance of its target, rad: the
/// steady elevator does the rest, and a large step does not wind it up.
const INTEGRAL_ZONE: f64 = 2.0 * PI / 180.0;
/// The same for the bank: the integral holds the aeroplane's overbanking in a steep
/// turn, not the roll into it.
const BANK_INTEGRAL_ZONE: f64 = 10.0 * PI / 180.0;
/// Pilot's rudder beyond which the coordination lets his feet have the sideslip they
/// ask for: its loop pauses, its integral held, the adverse yaw still met.
const PEDAL_PRESSED: f64 = 0.05;
/// Below this share of the lowest row's dynamic pressure, the gains stop growing: in a
/// tail slide they would ask for full travel at the slightest error anyway.
const LOWEST_PRESSURE_SHARE: f64 = 0.5;
/// How long after the last contact the aeroplane still counts as on the ground, s: a
/// bounce does not hand it back to the flight assists.
pub const BOUNCE_S: f64 = 0.5;
/// Lowest dynamic pressure at the rudder the heading hold divides by, Pa: at rest, engine
/// at idle, it would ask for full travel at the slightest error.
const LOWEST_RUDDER_PRESSURE: f64 = 100.0;
/// Throttle from which the instructor takes the aeroplane off.
pub const TAKE_OFF_THROTTLE: f64 = 0.5;
/// Attitude on the wheels, tail up, and of the first climb, rad (as `fdm_lab sol`).
const TAIL_UP_PITCH: f64 = 2.0 * PI / 180.0;
const CLIMB_PITCH: f64 = 7.0 * PI / 180.0;
/// The tail counts as up this close to its attitude, rad.
const TAIL_UP_MARGIN: f64 = 2.0 * PI / 180.0;
/// Load factor at the stall's incidence at which the instructor pulls: 1.2 times the
/// stall speed.
const ROTATION_LOAD: f64 = 1.44;

/// What the assists read of the flight.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Measured {
    /// True airspeed, m/s.
    pub airspeed: f64,
    /// Pa.
    pub dynamic_pressure: f64,
    pub alpha: f64,
    pub beta: f64,
    /// Bank φ and pitch θ, rad.
    pub bank: f64,
    pub pitch: f64,
    /// Flight path γ, positive climbing, rad.
    pub flight_path: f64,
    /// Roll, pitch and yaw rates (p, q, r), rad/s.
    pub rates: DVec3,
    pub heading: f64,
}

impl Measured {
    pub fn of(state: &RigidBodyState, env: &dyn Environment) -> Self {
        let wind = state.attitude.inverse() * env.wind_ned(state.position_ned);
        let air = state.velocity_body - wind;
        let airspeed = air.length().max(1e-3);
        let density = env.atmosphere(state.position_ned).density;
        let (heading, pitch, bank) = state.attitude.to_euler(EulerRot::ZYX);
        let ground = state.attitude * state.velocity_body;
        Self {
            airspeed,
            dynamic_pressure: 0.5 * density * airspeed * airspeed,
            alpha: air.z.atan2(air.x),
            beta: (air.y / airspeed).clamp(-1.0, 1.0).asin(),
            bank,
            pitch,
            flight_path: (-ground.z / ground.length().max(1e-3))
                .clamp(-1.0, 1.0)
                .asin(),
            rates: state.angular_velocity_body,
            heading,
        }
    }

    /// The body's pitch rate of a load factor in a coordinated turn at this attitude.
    fn pitch_rate_of(&self, load: f64) -> f64 {
        G0 / self.airspeed * (load - self.pitch.cos() * self.bank.cos())
    }
}

/// The assists' figures at one dynamic pressure (`escadrille_data::assist::AssistRow`).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Gains {
    /// Load factor per radian of incidence.
    pub load_per_alpha: f64,
    pub zero_load_alpha: f64,
    pub elevator_at_zero_load: f64,
    pub elevator_per_alpha: f64,
    pub elevator_per_pitch_rate: f64,
    pub rudder_per_roll_rate: f64,
    pub rudder_per_aileron: f64,
    pub aileron_per_roll_rate: f64,
    /// Aileron per radian of bank error, per rad/s of roll rate, per rad·s.
    pub roll: [f64; 3],
    /// Elevator per radian of incidence error, per rad/s of pitch rate, per rad·s.
    pub pitch: [f64; 3],
    /// Rudder per radian of sideslip, per rad/s of yaw rate, per rad·s.
    pub yaw: [f64; 3],
}

impl Gains {
    fn lerp(&self, other: &Self, t: f64) -> Self {
        let mix = |a: f64, b: f64| a + (b - a) * t;
        let mix_all = |a: &[f64], b: &[f64], out: &mut [f64]| {
            for ((o, a), b) in out.iter_mut().zip(a).zip(b) {
                *o = mix(*a, *b);
            }
        };
        let mut g = Self {
            load_per_alpha: mix(self.load_per_alpha, other.load_per_alpha),
            zero_load_alpha: mix(self.zero_load_alpha, other.zero_load_alpha),
            elevator_at_zero_load: mix(self.elevator_at_zero_load, other.elevator_at_zero_load),
            elevator_per_alpha: mix(self.elevator_per_alpha, other.elevator_per_alpha),
            elevator_per_pitch_rate: mix(
                self.elevator_per_pitch_rate,
                other.elevator_per_pitch_rate,
            ),
            rudder_per_roll_rate: mix(self.rudder_per_roll_rate, other.rudder_per_roll_rate),
            rudder_per_aileron: mix(self.rudder_per_aileron, other.rudder_per_aileron),
            aileron_per_roll_rate: mix(self.aileron_per_roll_rate, other.aileron_per_roll_rate),
            ..Self::default()
        };
        mix_all(&self.roll, &other.roll, &mut g.roll);
        mix_all(&self.pitch, &other.pitch, &mut g.pitch);
        mix_all(&self.yaw, &other.yaw, &mut g.yaw);
        g
    }

    /// The incidence of a load factor at constant speed.
    pub fn alpha_for(&self, load: f64) -> f64 {
        self.zero_load_alpha + load / self.load_per_alpha
    }

    /// The load factor of an incidence at constant speed.
    pub fn load_at(&self, alpha: f64) -> f64 {
        (alpha - self.zero_load_alpha) * self.load_per_alpha
    }

    /// The elevator command holding an incidence and a pitch rate in steady flight.
    pub fn elevator_for(&self, alpha: f64, pitch_rate: f64) -> f64 {
        self.elevator_at_zero_load
            + (alpha - self.zero_load_alpha) * self.elevator_per_alpha
            + pitch_rate * self.elevator_per_pitch_rate
    }
}

/// An aircraft's gains (`assist_gains.ron`): rows in equivalent airspeed, their gains
/// and load per incidence still normalised by the dynamic pressure.
#[derive(Debug, Clone, PartialEq)]
pub struct AssistTable {
    pub stall_alpha: f64,
    /// Rudder per radian of heading, per rad/s of yaw rate and per rad·s of the heading's
    /// integral on the ground, × Pa.
    pub ground_yaw: [f64; 3],
    rows: Vec<(f64, Gains)>,
}

impl From<&AssistFile> for AssistTable {
    fn from(file: &AssistFile) -> Self {
        let rows = file
            .rows
            .iter()
            .map(|r| {
                (
                    r.speed.si(),
                    Gains {
                        load_per_alpha: r.load_per_alpha,
                        zero_load_alpha: r.zero_load_alpha.si(),
                        elevator_at_zero_load: r.elevator_at_zero_load,
                        elevator_per_alpha: r.elevator_per_alpha,
                        elevator_per_pitch_rate: r.elevator_per_pitch_rate,
                        rudder_per_roll_rate: r.rudder_per_roll_rate,
                        rudder_per_aileron: r.rudder_per_aileron,
                        aileron_per_roll_rate: r.aileron_per_roll_rate,
                        roll: r.roll,
                        pitch: r.pitch,
                        yaw: r.yaw,
                    },
                )
            })
            .collect();
        Self {
            stall_alpha: file.stall_alpha.si(),
            ground_yaw: file.ground_yaw,
            rows,
        }
    }
}

impl AssistTable {
    /// The figures at a dynamic pressure: interpolated in equivalent airspeed, held at
    /// the table's ends, the gains divided by the dynamic pressure.
    pub fn at(&self, dynamic_pressure: f64) -> Gains {
        let lowest = 0.5 * RHO0 * self.rows[0].0.powi(2) * LOWEST_PRESSURE_SHARE;
        let pressure = dynamic_pressure.max(lowest);
        let speed = (2.0 * pressure / RHO0).sqrt();
        let i = self
            .rows
            .iter()
            .position(|(v, _)| *v > speed)
            .unwrap_or(self.rows.len())
            .clamp(1, self.rows.len() - 1);
        let ((v0, g0), (v1, g1)) = (&self.rows[i - 1], &self.rows[i]);
        let t = ((speed - v0) / (v1 - v0)).clamp(0.0, 1.0);
        let mut g = g0.lerp(g1, t);
        g.load_per_alpha *= pressure;
        for k in g.roll.iter_mut().chain(&mut g.pitch).chain(&mut g.yaw) {
            *k /= pressure;
        }
        g
    }
}

/// An angle brought within ±π.
fn wrap(angle: f64) -> f64 {
    (angle + PI).rem_euclid(TAU) - PI
}

/// What the assists read near the ground.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ground {
    /// A wheel or the skid carries something.
    pub touching: bool,
    /// Dynamic pressure at the rudder, Pa: free stream plus the slipstream's T / A.
    pub rudder_pressure: f64,
}

impl Ground {
    /// None away from the ground, where nothing here is worth a look at the propeller.
    pub fn of(aircraft: &Aircraft, state: &RigidBodyState, env: &dyn Environment) -> Option<Self> {
        let gear = aircraft.undercarriage()?;
        let touching = gear.loads(state, env).on_undercarriage();
        if !touching {
            return None;
        }
        let d = aircraft.evaluate(state, &PilotControls::default(), env);
        let diameter = aircraft
            .propulsion()
            .map_or(1.0, |p| p.propeller.geometry().diameter);
        let disc = PI * 0.25 * diameter * diameter;
        let thrust = d.propeller.map_or(0.0, |p| p.thrust).max(0.0);
        let speed = d.air_velocity.length();
        Some(Self {
            touching,
            rudder_pressure: 0.5 * d.density * speed * speed + thrust / disc,
        })
    }
}

/// The assists' memory between steps: the integrals of their errors.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct AssistState {
    /// Integral of the sideslip, rad·s.
    sideslip: f64,
    /// Integral of the instructor's bank error, rad·s.
    bank: f64,
    /// The bank the instructor is rolling to, rad: towards the stick's at
    /// [`INSTRUCTOR_ROLL_RATE`], from the aeroplane's own when it takes over.
    bank_target: Option<f64>,
    /// Integral of the incidence's error, rad·s: the instructor's, or the stall guard's
    /// while it holds.
    incidence: f64,
    /// Seconds since a wheel or the skid last touched; none in flight.
    since_contact: Option<f64>,
    /// The last dynamic pressure at the rudder on the ground, Pa.
    rudder_pressure: f64,
    /// The heading held on the ground.
    heading: Option<f64>,
    /// Integral of its error, rad·s: the foot held down against the propeller's torque.
    heading_integral: f64,
    /// The instructor's take-off has raised the tail.
    tail_up: bool,
}

impl AssistState {
    /// The commands of the next step, in the air or on the ground.
    pub fn next_commands(
        &mut self,
        settings: &RealismSettings,
        input: &PilotInput,
        m: &Measured,
        ground: Option<Ground>,
        table: &AssistTable,
        dt: f64,
    ) -> PilotControls {
        self.since_contact = match ground {
            Some(g) => {
                self.rudder_pressure = g.rudder_pressure;
                Some(0.0)
            }
            None => self.since_contact.map(|t| t + dt),
        };
        if self.since_contact.is_some_and(|t| t < BOUNCE_S) {
            return self.on_the_ground(settings, input, m, table, dt);
        }
        (self.heading, self.heading_integral) = (None, 0.0);
        self.tail_up = false;
        self.commands(settings.flight_assist, input, m, table, dt)
    }

    /// On the wheels: the flight assists give way to the heading hold and, for the
    /// instructor, to the take-off. Their integrals start again from nothing.
    fn on_the_ground(
        &mut self,
        settings: &RealismSettings,
        input: &PilotInput,
        m: &Measured,
        table: &AssistTable,
        dt: f64,
    ) -> PilotControls {
        (self.sideslip, self.bank, self.incidence) = (0.0, 0.0, 0.0);
        self.bank_target = None;
        let pilot = input.controls;
        let mut out = pilot;
        if settings.ground_steering {
            out.yaw = self.hold_heading(pilot.yaw, m, table, dt);
        }
        if settings.flight_assist == FlightAssist::Instructor && pilot.throttle >= TAKE_OFF_THROTTLE
        {
            let g = table.at(m.dynamic_pressure);
            out.roll = self.bank(0.0, 0.0, m, &g, dt);
            out.pitch = self.take_off(m, &g, table.stall_alpha);
        } else {
            self.tail_up = false;
        }
        out
    }

    /// The pilot's rudder, plus the one holding the heading he last left it at.
    fn hold_heading(&mut self, pilot: f64, m: &Measured, table: &AssistTable, dt: f64) -> f64 {
        let pressed = pilot.abs() > PEDAL_PRESSED;
        if pressed {
            self.heading = Some(m.heading);
        }
        let held = *self.heading.get_or_insert(m.heading);
        let pressure = self.rudder_pressure.max(LOWEST_RUDDER_PRESSURE);
        let [k_heading, k_rate, k_integral] = table.ground_yaw.map(|k| k / pressure);
        let error = wrap(m.heading - held);
        let free = pilot - k_heading * error - k_rate * m.rates.z;
        if !pressed {
            self.heading_integral = integrate(self.heading_integral, error, free, k_integral, dt);
        }
        (free - k_integral * self.heading_integral).clamp(-1.0, 1.0)
    }

    /// Stick forward until the tail is up, then the attitude that rolls on the wheels and,
    /// from 1.2 times the stall speed, the one that climbs away.
    fn take_off(&mut self, m: &Measured, g: &Gains, stall_alpha: f64) -> f64 {
        self.tail_up |= m.pitch < TAIL_UP_PITCH + TAIL_UP_MARGIN;
        if !self.tail_up {
            return -1.0;
        }
        let pitch = if g.load_at(stall_alpha) >= ROTATION_LOAD {
            CLIMB_PITCH
        } else {
            TAIL_UP_PITCH
        };
        let [k_pitch, k_rate, _] = g.pitch;
        (-k_pitch * (m.pitch - pitch) - k_rate * m.rates.y).clamp(-1.0, 1.0)
    }

    /// The commands of the next step: the pilot's, through the assist.
    pub fn commands(
        &mut self,
        assist: FlightAssist,
        input: &PilotInput,
        m: &Measured,
        table: &AssistTable,
        dt: f64,
    ) -> PilotControls {
        let pilot = input.controls;
        let g = table.at(m.dynamic_pressure);
        let limit = table.stall_alpha - STALL_MARGIN;
        match assist {
            FlightAssist::Off => {
                *self = Self::default();
                pilot
            }
            FlightAssist::Coordinated { stall_guard } => {
                self.bank = 0.0;
                self.bank_target = None;
                let pitch = if stall_guard {
                    self.guard(pilot.pitch, limit, m, &g, dt)
                } else {
                    self.incidence = 0.0;
                    pilot.pitch
                };
                PilotControls {
                    pitch,
                    yaw: self.coordinate(pilot.yaw, pilot.roll, m, &g, dt),
                    ..pilot
                }
            }
            FlightAssist::Instructor => {
                let wanted = INSTRUCTOR_BANK * input.stick.x;
                let previous = self.bank_target.unwrap_or(m.bank);
                let step = INSTRUCTOR_ROLL_RATE * dt;
                let target = previous + wrap(wanted - previous).clamp(-step, step);
                self.bank_target = Some(target);
                let roll = self.bank(target, (target - previous) / dt, m, &g, dt);
                let most = g.load_at(limit);
                let load = wanted_load(input.stick.y, most, m).min(most);
                let pitch = self.hold(g.alpha_for(load), m.pitch_rate_of(load), m, &g, dt);
                PilotControls {
                    roll,
                    pitch,
                    yaw: self.coordinate(pilot.yaw, roll, m, &g, dt),
                    throttle: pilot.throttle,
                }
            }
        }
    }

    /// Aileron giving a bank that moves at a roll rate: the aileron of that rate,
    /// corrected by the errors and, near a target at rest, their integral.
    fn bank(&mut self, bank: f64, rate: f64, m: &Measured, g: &Gains, dt: f64) -> f64 {
        let [k_bank, k_rate, k_integral] = g.roll;
        let error = wrap(m.bank - bank);
        let free = g.aileron_per_roll_rate * rate - k_bank * error - k_rate * (m.rates.x - rate);
        if error.abs() < BANK_INTEGRAL_ZONE && rate == 0.0 {
            self.bank = integrate(self.bank, error, free, k_integral, dt);
        }
        (free - k_integral * self.bank).clamp(-1.0, 1.0)
    }

    /// The pilot's rudder, plus the one meeting the adverse yaw of the roll and the
    /// aileron and cancelling the sideslip left; the yaw rate of the turn, and of a roll
    /// about the flight path, is left alone. With the pilot's foot on the rudder bar, the
    /// sideslip is his.
    fn coordinate(&mut self, pilot: f64, aileron: f64, m: &Measured, g: &Gains, dt: f64) -> f64 {
        let [k_beta, k_rate, k_integral] = g.yaw;
        let p = m.rates.x;
        let turn = G0 / m.airspeed * m.bank.sin() * m.pitch.cos() + p * m.alpha.tan();
        let adverse = g.rudder_per_roll_rate * p + g.rudder_per_aileron * aileron;
        if pilot.abs() > PEDAL_PRESSED {
            return (pilot + adverse).clamp(-1.0, 1.0);
        }
        // The pilot's foot counts in the stop the integral must not wind against.
        let free = pilot + adverse - k_beta * m.beta - k_rate * (m.rates.z - turn);
        self.sideslip = integrate(self.sideslip, m.beta, free, k_integral, dt);
        (free - k_integral * self.sideslip).clamp(-1.0, 1.0)
    }

    /// Elevator holding an incidence, at a pitch rate: the steady elevator of both,
    /// corrected by the errors and, near the target, their integral.
    fn hold(&mut self, alpha: f64, rate: f64, m: &Measured, g: &Gains, dt: f64) -> f64 {
        let [k_alpha, k_rate, k_integral] = g.pitch;
        let error = m.alpha - alpha;
        let free = g.elevator_for(alpha, rate) - k_alpha * error - k_rate * (m.rates.y - rate);
        if error.abs() < INTEGRAL_ZONE {
            self.incidence = integrate(self.incidence, error, free, k_integral, dt);
        }
        (free - k_integral * self.incidence).clamp(-1.0, 1.0)
    }

    /// The pilot's elevator, or the one holding the limit if it is lower.
    fn guard(&mut self, pilot: f64, limit: f64, m: &Measured, g: &Gains, dt: f64) -> f64 {
        let rate = m.pitch_rate_of(g.load_at(limit));
        let held = self.hold(limit, rate, m, g, dt);
        if held < pilot {
            held
        } else {
            self.incidence = 0.0;
            pilot
        }
    }
}

/// One step of an integral `z` of `error` feeding back as −k z on top of `free`: it stops
/// while the command is at a stop and the error would push it further, and never asks
/// for more than the whole travel (anti-windup).
fn integrate(z: f64, error: f64, free: f64, k: f64, dt: f64) -> f64 {
    let output = free - k * z;
    let pushing = -k * error;
    let held = (output >= 1.0 && pushing > 0.0) || (output <= -1.0 && pushing < 0.0);
    let z = if held { z } else { z + error * dt };
    let bound = 2.0 / k.abs().max(1e-9);
    z.clamp(-bound, bound)
}

/// The instructor's load factor for the stick's y: that of a level turn at the present
/// bank, flight path brought back to level stick released, then the stick's share of the
/// way to [`INSTRUCTOR_LOAD`] (back) or to zero (forward); in any case no steeper climb
/// than the guard's largest load factor `most` allows ([`CLIMB_LIMIT`]).
fn wanted_load(y: f64, most: f64, m: &Measured) -> f64 {
    let level = if m.bank.cos() > 0.0 {
        (m.flight_path.cos() / m.bank.cos()).min(INSTRUCTOR_LOAD)
    } else {
        0.0
    };
    let per_rad = m.airspeed / (G0 * LEVELLING_S);
    let hold = (level - (1.0 - y.abs()) * per_rad * m.flight_path).clamp(0.0, INSTRUCTOR_LOAD);
    let wanted = if y >= 0.0 {
        hold + y * (INSTRUCTOR_LOAD - hold)
    } else {
        hold * (1.0 + y)
    };
    let (low, high) = CLIMB_LOADS;
    let climb = CLIMB_LIMIT * ((most - low) / (high - low)).clamp(0.0, 1.0);
    if m.flight_path > climb {
        wanted.min((level - per_rad * (m.flight_path - climb)).max(0.0))
    } else {
        wanted
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use bevy::math::DVec2;
    use escadrille_data::aircraft::{Loading, load_aircraft};
    use escadrille_data::assist::load_assist_gains;
    use escadrille_data::scenario::load_scenario;
    use escadrille_fdm::aircraft::Aircraft;

    use super::*;

    use crate::ASSETS_DIR;
    use crate::flight::simulation::ENVIRONMENT;
    use crate::flight::{ON_THE_FIELD, STEP_HZ};
    use crate::realism::Preset;

    struct Flight {
        aircraft: Aircraft,
        table: AssistTable,
        state: RigidBodyState,
        start: PilotControls,
    }

    fn spad() -> Flight {
        let assets = Path::new(ASSETS_DIR);
        let s = load_scenario(assets, "vol-essai").unwrap();
        let aircraft = load_aircraft(assets, &s.aircraft)
            .unwrap()
            .build(&Loading {
                fuel_fraction: s.fuel_fraction,
            })
            .unwrap()
            .aircraft;
        let table = AssistTable::from(&load_assist_gains(assets, &s.aircraft).unwrap().unwrap());
        Flight {
            aircraft,
            table,
            state: s.initial_state(),
            start: s.commands(),
        }
    }

    /// What a flight went through. Incidence and sideslip count while it had flying
    /// speed: while the guard's limit could still give 1 g (at the top of a zoom, no
    /// elevator holds anything).
    #[derive(Debug, Default)]
    struct Trace {
        max_alpha: f64,
        max_abs_beta: f64,
        min_speed: f64,
        lowest_altitude: f64,
        crashed: bool,
        last: Option<Measured>,
        altitude: f64,
    }

    /// Flies `seconds` from the test flight's start with the pilot's input given by `pilot`
    /// (time, start commands) and the assist; the trace from `settle` seconds.
    fn fly(
        assist: FlightAssist,
        seconds: f64,
        settle: f64,
        pilot: impl Fn(f64, &PilotControls) -> PilotInput,
    ) -> Trace {
        let mut f = spad();
        let mut memory = AssistState::default();
        let dt = 1.0 / STEP_HZ;
        let limit = f.table.stall_alpha - STALL_MARGIN;
        let mut trace = Trace {
            min_speed: f64::INFINITY,
            lowest_altitude: f64::INFINITY,
            ..Trace::default()
        };
        for k in 0..(seconds * STEP_HZ) as usize {
            let t = k as f64 * dt;
            let m = Measured::of(&f.state, &ENVIRONMENT);
            let input = pilot(t, &f.start);
            let controls = memory.commands(assist, &input, &m, &f.table, dt);
            f.state = f.aircraft.advance(&f.state, &controls, &ENVIRONMENT, dt);
            let altitude = -f.state.position_ned.z;
            if altitude <= 0.0 {
                trace.crashed = true;
                break;
            }
            if t >= settle {
                if f.table.at(m.dynamic_pressure).load_at(limit) >= 1.0 {
                    trace.max_alpha = trace.max_alpha.max(m.alpha);
                    trace.max_abs_beta = trace.max_abs_beta.max(m.beta.abs());
                }
                trace.min_speed = trace.min_speed.min(m.airspeed);
                trace.lowest_altitude = trace.lowest_altitude.min(altitude);
            }
            trace.last = Some(m);
            trace.altitude = altitude;
        }
        trace
    }

    fn stick(x: f64, y: f64, throttle: f64, start: &PilotControls) -> PilotInput {
        PilotInput {
            controls: PilotControls {
                roll: x,
                pitch: y,
                yaw: 0.0,
                throttle,
            },
            stick: DVec2::new(x, y),
        }
        .with_neutral(start.pitch)
    }

    impl PilotInput {
        /// The stick around the start's elevator, as the action layer gives it.
        fn with_neutral(mut self, neutral: f64) -> Self {
            let s = self.stick.y;
            self.controls.pitch = if s >= 0.0 {
                neutral + s * (1.0 - neutral)
            } else {
                neutral + s * (1.0 + neutral)
            };
            self
        }
    }

    const PILOT: FlightAssist = FlightAssist::Coordinated { stall_guard: true };

    /// Prints a flight, every half second, for diagnosis (`cargo test -p escadrille
    /// print_a_flight -- --ignored --nocapture`): `ASSIST=off|veteran|pilot|arcade`,
    /// `STICK=x,y`, `THROTTLE`, `SECONDS`, `ROLL_FOR` (seconds the stick's x is held),
    /// `CUT_AT` (second the throttle goes to idle), `EVERY` (steps between lines, 60).
    #[test]
    #[ignore]
    fn print_a_flight() {
        let var = |k: &str, d: &str| std::env::var(k).unwrap_or(d.to_owned());
        let assist = match var("ASSIST", "pilot").as_str() {
            "off" => FlightAssist::Off,
            "veteran" => FlightAssist::Coordinated { stall_guard: false },
            "arcade" => FlightAssist::Instructor,
            _ => PILOT,
        };
        let xy: Vec<f64> = var("STICK", "0,1")
            .split(',')
            .map(|v| v.parse().unwrap())
            .collect();
        let throttle: f64 = var("THROTTLE", "1").parse().unwrap();
        let seconds: f64 = var("SECONDS", "20").parse().unwrap();
        let roll_for: f64 = var("ROLL_FOR", "1000").parse().unwrap();
        let cut_at: f64 = var("CUT_AT", "1000").parse().unwrap();
        let every: usize = var("EVERY", "60").parse().unwrap();
        let mut f = spad();
        let mut memory = AssistState::default();
        let dt = 1.0 / STEP_HZ;
        for k in 0..(seconds * STEP_HZ) as usize {
            let t = k as f64 * dt;
            let m = Measured::of(&f.state, &ENVIRONMENT);
            let x = if t < roll_for { xy[0] } else { 0.0 };
            let input = stick(x, xy[1], if t < cut_at { throttle } else { 0.0 }, &f.start);
            let c = memory.commands(assist, &input, &m, &f.table, dt);
            if k % every == 0 {
                eprintln!(
                    "t {t:5.1} V {:5.1} h {:6.0} a {:+5.1} b {:+5.1} phi {:+6.1} th {:+5.1} gam {:+5.1} pqr {:+5.2} {:+5.2} {:+5.2} out {:+.2} {:+.2} {:+.2}",
                    m.airspeed * 3.6,
                    -f.state.position_ned.z,
                    m.alpha.to_degrees(),
                    m.beta.to_degrees(),
                    m.bank.to_degrees(),
                    m.pitch.to_degrees(),
                    m.flight_path.to_degrees(),
                    m.rates.x,
                    m.rates.y,
                    m.rates.z,
                    c.roll,
                    c.pitch,
                    c.yaw,
                );
            }
            f.state = f.aircraft.advance(&f.state, &c, &ENVIRONMENT, dt);
        }
    }

    #[test]
    fn the_ace_flies_without_any_assist() {
        // No cheat: in As, the commands are the pilot's, exactly, and nothing is kept.
        assert_eq!(Preset::Ace.settings().flight_assist, FlightAssist::Off);
        let f = spad();
        let m = Measured::of(&f.state, &ENVIRONMENT);
        let mut memory = AssistState::default();
        let input = stick(0.3, -0.7, 0.8, &f.start);
        let out = memory.commands(FlightAssist::Off, &input, &m, &f.table, 1.0 / STEP_HZ);
        assert_eq!(out, input.controls);
        assert_eq!(memory, AssistState::default());
    }

    /// Short of the stall, with room for the guard's overshoot of its limit in a zoom
    /// (1.3° on the SPAD).
    fn guarded(t: &Trace) -> bool {
        t.max_alpha < spad().table.stall_alpha - 0.25 * STALL_MARGIN
    }

    #[test]
    fn full_back_stick_stalls_the_ace_and_not_the_pilot_or_the_beginner() {
        let back = |_: f64, s: &PilotControls| stick(0.0, 1.0, 1.0, s);
        // The aeroplane as it is: full back stick goes far past the stall.
        let ace = fly(FlightAssist::Off, 8.0, 0.0, back);
        assert!(ace.max_alpha > spad().table.stall_alpha + 0.05, "{ace:?}");
        // Guarded: held at the limit while there is speed to hold it. The pilot zooms
        // and falls through over the top, again and again, without a spin; the
        // instructor keeps the climb within the speed.
        for assist in [PILOT, FlightAssist::Instructor] {
            let t = fly(assist, 30.0, 1.0, back);
            assert!(guarded(&t), "{assist:?} {t:?}");
            assert!(t.lowest_altitude > 900.0, "{assist:?} {t:?}");
        }
        let t = fly(FlightAssist::Instructor, 30.0, 1.0, back);
        assert!(t.min_speed > 70.0 / 3.6, "{t:?}");
    }

    #[test]
    fn a_steep_turn_loses_height_rather_than_stall() {
        // The beginner's tight turn that stalled the player (2026-10-29). The pilot rolls
        // to about 60° then pulls the stick right back.
        let pilot = fly(PILOT, 20.0, 1.0, |t, s| {
            let x = if t < 1.2 { 0.5 } else { 0.0 };
            stick(x, if t < 1.2 { 0.0 } else { 1.0 }, s.throttle, s)
        });
        assert!(guarded(&pilot) && !pilot.crashed, "{pilot:?}");
        // The beginner pushes the stick into the corner: 60° of bank, the load the
        // speed allows.
        let beginner = fly(FlightAssist::Instructor, 30.0, 1.0, |_, s| {
            stick(1.0, 1.0, s.throttle, s)
        });
        assert!(guarded(&beginner) && !beginner.crashed, "{beginner:?}");
        let bank = beginner.last.unwrap().bank;
        assert!((bank - INSTRUCTOR_BANK).abs() < 0.05, "{bank} {beginner:?}");
        // Coordinated, a few degrees of sideslip as it rolls in.
        assert!(beginner.max_abs_beta < 0.07, "{beginner:?}");
    }

    #[test]
    fn cutting_the_throttle_in_a_climb_keeps_the_beginner_flying() {
        // A climb at full throttle, then idle, the stick held where it was. Unassisted,
        // and with the stall guard alone, the SPAD zooms to 45° and stalls over the top
        // at 46 km/h, where no elevator holds anything (the guard does not give the
        // speed back). The instructor keeps the climb within the speed.
        let pilot =
            |t: f64, s: &PilotControls| stick(0.0, 0.05, if t < 6.0 { 1.0 } else { 0.0 }, s);
        let ace = fly(FlightAssist::Off, 30.0, 1.0, pilot);
        assert!(ace.min_speed < 50.0 / 3.6, "{ace:?}");
        let beginner = fly(FlightAssist::Instructor, 30.0, 1.0, pilot);
        assert!(guarded(&beginner) && !beginner.crashed, "{beginner:?}");
        assert!(beginner.min_speed > 70.0 / 3.6, "{beginner:?}");
    }

    #[test]
    fn the_instructor_holds_level_flight_hands_off() {
        // Stick released for a minute: wings level, height held.
        let t = fly(FlightAssist::Instructor, 60.0, 0.0, |_, s| {
            stick(0.0, 0.0, s.throttle, s)
        });
        let m = t.last.unwrap();
        assert!(m.bank.abs() < 0.02, "{m:?}");
        assert!((t.altitude - 1000.0).abs() < 15.0, "{t:?}");
    }

    #[test]
    fn the_instructor_banks_and_levels_within_seconds() {
        // Half the stick to the right: 30° of bank within 3 s; released at 10 s: wings
        // level within 4 s (the bank loop was four times too slow before the
        // aerodynamicist's review, 2026-10-29).
        let bank_at = |seconds: f64| {
            fly(FlightAssist::Instructor, seconds, 0.0, |t, s| {
                stick(if t < 10.0 { 0.5 } else { 0.0 }, 0.0, s.throttle, s)
            })
            .last
            .unwrap()
            .bank
        };
        let banked = bank_at(3.0);
        assert!(
            (banked - 0.5 * INSTRUCTOR_BANK).abs() < 3f64.to_radians(),
            "{banked}"
        );
        let level = bank_at(14.0);
        assert!(level.abs() < 3f64.to_radians(), "{level}");
    }

    #[test]
    fn coordination_cancels_the_sideslip_of_a_rolling_turn() {
        // Into a 30° turn by the aileron alone, then the stick centred: coordinated, the
        // sideslip stays a fraction of the uncoordinated one.
        let pilot = |t: f64, s: &PilotControls| {
            let x = if t < 1.5 { 0.5 } else { 0.0 };
            stick(x, 0.1, s.throttle, s)
        };
        let free = fly(FlightAssist::Off, 10.0, 0.0, pilot);
        let coordinated = fly(
            FlightAssist::Coordinated { stall_guard: false },
            10.0,
            0.0,
            pilot,
        );
        assert!(
            coordinated.max_abs_beta < 0.5 * free.max_abs_beta,
            "{} {}",
            coordinated.max_abs_beta,
            free.max_abs_beta
        );
    }

    #[test]
    fn the_pilot_s_rudder_still_yaws_the_aeroplane() {
        // Coordinated, full right rudder for three seconds: the sideslip is the pilot's.
        let pilot = |t: f64, s: &PilotControls| {
            let mut input = stick(0.0, 0.0, s.throttle, s);
            input.controls.yaw = if t < 3.0 { 1.0 } else { 0.0 };
            input
        };
        let veteran = fly(
            FlightAssist::Coordinated { stall_guard: false },
            3.0,
            1.0,
            pilot,
        );
        let ace = fly(FlightAssist::Off, 3.0, 1.0, pilot);
        assert!(
            veteran.max_abs_beta > 0.7 * ace.max_abs_beta && ace.max_abs_beta > 0.05,
            "{} {}",
            veteran.max_abs_beta,
            ace.max_abs_beta
        );
    }

    /// Largest error from the start's heading while a wheel or the skid touches, rad.
    type Swing = f64;

    /// Rolls from the field's start with the pilot's input given by `pilot` (time, state):
    /// what the pilot gave and the flight model received at each step, the last state and
    /// the swing on the ground.
    fn roll(
        preset: Preset,
        seconds: f64,
        pilot: impl Fn(f64, &Measured) -> PilotControls,
    ) -> (
        Vec<PilotControls>,
        Vec<PilotControls>,
        RigidBodyState,
        Swing,
    ) {
        let assets = Path::new(ASSETS_DIR);
        let s = load_scenario(assets, ON_THE_FIELD).unwrap();
        let aircraft = load_aircraft(assets, &s.aircraft)
            .unwrap()
            .build(&Loading {
                fuel_fraction: s.fuel_fraction,
            })
            .unwrap()
            .aircraft;
        let table = AssistTable::from(&load_assist_gains(assets, &s.aircraft).unwrap().unwrap());
        let settings = preset.settings();
        let mut memory = AssistState::default();
        let mut state = s.initial_state();
        let dt = 1.0 / STEP_HZ;
        let (mut given, mut received) = (Vec::new(), Vec::new());
        let start = Measured::of(&state, &ENVIRONMENT).heading;
        let mut swing: Swing = 0.0;
        for k in 0..(seconds * STEP_HZ) as usize {
            let m = Measured::of(&state, &ENVIRONMENT);
            let controls = pilot(k as f64 * dt, &m);
            let input = PilotInput::holding(controls);
            let ground = Ground::of(&aircraft, &state, &ENVIRONMENT);
            if ground.is_some() {
                swing = swing.max(wrap(m.heading - start).abs());
            }
            let c = memory.next_commands(&settings, &input, &m, ground, &table, dt);
            state = aircraft.advance(&state, &c, &ENVIRONMENT, dt);
            given.push(controls);
            received.push(c);
        }
        (given, received, state, swing)
    }

    fn full_throttle(_: f64, _: &Measured) -> PilotControls {
        PilotControls {
            throttle: 1.0,
            ..PilotControls::default()
        }
    }

    /// Stick pushed while the tail is down, held off the wheels at flying speed.
    fn pilot_take_off(_: f64, m: &Measured) -> PilotControls {
        PilotControls {
            throttle: 1.0,
            pitch: if m.airspeed > 26.0 {
                0.1
            } else if m.pitch > 0.05 {
                -1.0
            } else {
                -0.2
            },
            ..PilotControls::default()
        }
    }

    #[test]
    fn at_rest_the_stall_guard_leaves_the_stick_alone() {
        let stick_back = |_: f64, _: &Measured| PilotControls {
            pitch: 0.4,
            ..PilotControls::default()
        };
        let (given, received, ..) = roll(Preset::Pilot, 2.0, stick_back);
        let last = received.len() - 1;
        assert_eq!(received[last].pitch, given[last].pitch);
    }

    #[test]
    fn the_ace_gets_no_help_on_the_ground() {
        let (given, received, ..) = roll(Preset::Ace, 4.0, pilot_take_off);
        assert_eq!(given, received);
    }

    #[test]
    fn the_veteran_keeps_his_own_feet_on_the_ground() {
        let (given, received, ..) = roll(Preset::Veteran, 4.0, pilot_take_off);
        assert!(
            given.iter().zip(&received).all(|(g, r)| g.yaw == r.yaw),
            "the rudder is the pilot's"
        );
    }

    #[test]
    fn the_pilot_preset_holds_the_heading_on_the_take_off_roll() {
        let (.., state, swing) = roll(Preset::Pilot, 16.0, pilot_take_off);
        assert!(
            -state.position_ned.z > 5.0,
            "it flies: {}",
            state.position_ned
        );
        assert!(swing < 0.175, "{}°", swing.to_degrees());
    }

    #[test]
    fn without_it_the_same_take_off_goes_round() {
        let (.., swing) = roll(Preset::Ace, 16.0, pilot_take_off);
        assert!(swing > 0.5, "{}°", swing.to_degrees());
    }

    #[test]
    fn the_instructor_flies_off_on_the_throttle_alone() {
        let (.., state, swing) = roll(Preset::Arcade, 20.0, full_throttle);
        let m = Measured::of(&state, &ENVIRONMENT);
        assert!(
            -state.position_ned.z > 2.0,
            "it flies: {}",
            state.position_ned
        );
        assert!(swing < 0.175, "{}°", swing.to_degrees());
        assert!(m.bank.abs() < 0.2, "{}", m.bank);
    }

    #[test]
    fn the_pilot_s_rudder_moves_the_heading_held() {
        let turning = |t: f64, _: &Measured| PilotControls {
            throttle: 0.5,
            pitch: if t < 6.0 { -1.0 } else { 0.0 },
            yaw: if t < 6.0 { 1.0 } else { 0.0 },
            ..PilotControls::default()
        };
        let (.., state, _) = roll(Preset::Pilot, 13.0, turning);
        let m = Measured::of(&state, &ENVIRONMENT);
        assert!(
            wrap(m.heading - PI / 2.0) > 0.08,
            "it turned right, and stayed there: {}",
            m.heading
        );
    }

    #[test]
    fn the_gains_follow_the_dynamic_pressure() {
        let table = spad().table;
        let slow = table.at(0.5 * RHO0 * (100.0_f64 / 3.6).powi(2));
        let fast = table.at(0.5 * RHO0 * (200.0_f64 / 3.6).powi(2));
        // A faster flight needs less aileron for the same bank, and its load factor
        // grows faster with the incidence.
        assert!(fast.roll[0] < slow.roll[0]);
        assert!(fast.load_per_alpha > 3.0 * slow.load_per_alpha);
        // Past the ends, held, without blowing up.
        let crawl = table.at(1.0);
        assert!(crawl.roll[0].is_finite() && crawl.roll[0] > slow.roll[0]);
        assert!((wrap(TAU + 0.1) - 0.1).abs() < 1e-12);
        assert!((wrap(PI + 0.1) - (0.1 - PI)).abs() < 1e-12);
    }
}
