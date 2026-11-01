//! Contact with the ground: wheels on rubber cord, tail skid and hard points of the airframe
//! (`modele-aero.md`, « Sol »; plan `docs/plans/m2-train-sol.md`, step 1).
//!
//! Each point of contact gives a normal force from how deep it is and how fast it sinks,
//! and a friction force in the plane of the ground. Nothing here steers the aircraft or
//! scripts a ground loop or a nose-over: they come from these forces and the rigid body.
//!
//! - **Wheel**: a sphere of the tyre's radius around the axle, so that the contact stays
//!   below it at any attitude. Tyre and rubber cord work in series ([`Spring`]). No brakes,
//!   and the wheels' own rotation is left out.
//! - **Skid**: a shoe on a [`Spring`]. It is the only brake, and it does not steer, but a
//!   narrow shoe cuts a groove in the turf and resists going sideways: behind the centre
//!   of gravity, that is what keeps the aircraft straight at low speed.
//! - Both rub the same way ([`Rubbing`]): a resistance along them; across them, a side
//!   force that grows with the slip angle up to a grip.
//! - **Hard point**: a part of the airframe that should not touch: a stiff spring and
//!   friction, to carry the aircraft through a ground loop or a nose-over.
//!
//! Friction is regularised below [`CREEP_SPEED`]: it never divides by a speed, and its
//! slope is bounded, which the explicit integrator needs. The price is a slow creep under
//! a steady force at rest. The ground is level: its height comes from
//! [`Environment::ground_altitude`], and its normal is the vertical.

use glam::DVec3;

use crate::environment::Environment;
use crate::rigid_body::{RigidBodyState, Wrench};

/// Speed of a contact point under which friction fades to zero, m/s (numerical, not
/// physical): small enough for the creep at rest to stay slow, large enough to keep the
/// slope of the skid's friction in yaw within what RK4 holds at the step of ADR 0005.
pub const CREEP_SPEED: f64 = 0.02;

/// Compression over which the damping comes in, m: a tyre gives no load in the instant
/// it touches, however fast it sinks (numerical; much less than the static compression).
const DAMPING_RAMP: f64 = 0.02;

/// Saturation of a ratio to ±1, linear near zero.
fn saturate(ratio: f64) -> f64 {
    ratio / (1.0 + ratio * ratio).sqrt()
}

/// What touches the ground (a tyre, a shoe) in series with rubber cord. The cord is wound
/// with a preload: it gives only beyond that load, up to the end of its travel, where a
/// stop takes over.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spring {
    /// Stiffness of what touches the ground, N/m.
    pub contact_stiffness: f64,
    /// Load at which the cord starts to give, N.
    pub preload: f64,
    /// Stiffness of the cord, N/m.
    pub stiffness: f64,
    /// Travel of the cord, m.
    pub travel: f64,
    /// Stiffness of the stop at the end of the travel, N/m.
    pub stop_stiffness: f64,
    /// Damping of the compression, N·s/m. Rubber gives most of its energy back.
    pub damping: f64,
}

impl Spring {
    /// Load and travel of the cord (N, m) at a total compression (m), damping excluded.
    pub fn load(&self, compression: f64) -> (f64, f64) {
        let contact = 1.0 / self.contact_stiffness;
        let cord = 1.0 / self.stiffness;
        if compression <= self.preload * contact {
            return (compression / contact, 0.0);
        }
        let stop_load = self.preload + self.stiffness * self.travel;
        let stop_compression = stop_load * contact + self.travel;
        if compression <= stop_compression {
            let load = (compression + self.preload * cord) / (contact + cord);
            return (load, (load - self.preload) * cord);
        }
        let stop = 1.0 / self.stop_stiffness;
        let load = stop_load + (compression - stop_compression) / (contact + stop);
        (load, self.travel + (load - stop_load) * stop)
    }

    /// Normal force on the ground from the spring's load at a compression and a sink rate
    /// (m/s, positive going down). The ground only pushes.
    fn normal(&self, load: f64, compression: f64, sink: f64) -> f64 {
        let ramp = (compression / DAMPING_RAMP).min(1.0);
        (load + self.damping * ramp * sink).max(0.0)
    }

    fn is_valid(&self) -> bool {
        [
            self.contact_stiffness,
            self.stiffness,
            self.travel,
            self.stop_stiffness,
        ]
        .iter()
        .all(|v| v.is_finite() && *v > 0.0)
            && self.preload.is_finite()
            && self.preload >= 0.0
            && self.damping.is_finite()
            && self.damping >= 0.0
    }
}

/// How a wheel or a skid's shoe rubs on the ground, relative to the normal load.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rubbing {
    /// Resistance along it.
    pub along: f64,
    /// Largest side force.
    pub grip: f64,
    /// Tangent of the slip angle at which the side force would reach the grip, were it
    /// to go on growing at its initial slope.
    pub cornering: f64,
}

impl Rubbing {
    fn is_valid(&self) -> bool {
        [self.along, self.grip]
            .iter()
            .all(|v| v.is_finite() && *v >= 0.0)
            && self.cornering.is_finite()
            && self.cornering > 0.0
    }

    /// Friction on the ground, NED frame, for a point moving at `velocity` under a normal
    /// force, with the axis of the wheel or shoe at `axis` (NED frame); and its speed
    /// along it.
    fn force(&self, velocity: DVec3, normal: f64, axis: DVec3) -> (DVec3, f64) {
        let side = DVec3::new(axis.x, axis.y, 0.0)
            .try_normalize()
            .unwrap_or(DVec3::Y);
        let along = DVec3::new(side.y, -side.x, 0.0);
        let rolling = velocity.dot(along);
        let slip = velocity.dot(side);
        let resistance = self.along * normal * saturate(rolling / CREEP_SPEED);
        let scale = CREEP_SPEED.hypot(self.cornering * rolling);
        let side_force = self.grip * normal * saturate(slip / scale);
        (-along * resistance - side * side_force, rolling)
    }
}

/// A wheel. Positions are in body axes from the centre of gravity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Wheel {
    /// Axle with the cord at rest, m.
    pub axle: DVec3,
    pub radius: f64,
    /// Unit vector along which the axle rises under load; it has to rise.
    pub guide: DVec3,
    /// As the ground feels it: compression and travel along the vertical.
    pub spring: Spring,
    pub rubbing: Rubbing,
}

/// A tail skid.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Skid {
    /// Point of the shoe with the cord at rest, m.
    pub shoe: DVec3,
    pub spring: Spring,
    pub rubbing: Rubbing,
}

/// A point of the airframe that is not meant to touch the ground.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HardPoint {
    pub position: DVec3,
    /// N/m.
    pub stiffness: f64,
    /// N·s/m.
    pub damping: f64,
    /// Friction force over the normal load.
    pub friction: f64,
}

/// Loads beyond which a part gives way, N.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContactLimits {
    pub wheel: f64,
    pub skid: f64,
    pub hard_point: f64,
}

/// Why an undercarriage was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroundError {
    /// A wheel, skid or hard point has a non-finite or non-positive value.
    BadContact,
    /// The guide of an axle is not a unit vector, or does not rise.
    BadGuide,
}

/// Which point touches, as an index in its list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContactPoint {
    Wheel(usize),
    Skid(usize),
    HardPoint(usize),
}

/// A point touching the ground, for telemetry, the assists and what breaks.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Contact {
    pub point: ContactPoint,
    /// Normal force, N.
    pub load: f64,
    /// Total compression, m.
    pub compression: f64,
    /// Travel of the axle along its guide, or of the skid's cord, m (0 for a hard point).
    pub cord_travel: f64,
    /// Speed along the wheel, m/s (0 for a skid or a hard point).
    pub rolling_speed: f64,
}

/// Forces of the ground on the aircraft in one state.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct GroundLoads {
    /// Force and moment about the centre of gravity, body axes.
    pub wrench: Wrench,
    /// The points touching the ground; none in the air, without any allocation.
    pub contacts: Vec<Contact>,
}

impl GroundLoads {
    /// Whether the aircraft stands on its wheels or skid, in any part.
    pub fn on_undercarriage(&self) -> bool {
        self.contacts
            .iter()
            .any(|c| !matches!(c.point, ContactPoint::HardPoint(_)) && c.load > 0.0)
    }
}

/// The points of an aircraft that can touch the ground.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Undercarriage {
    wheels: Vec<Wheel>,
    skids: Vec<Skid>,
    hard_points: Vec<HardPoint>,
    /// Without limits, nothing gives way.
    limits: Option<ContactLimits>,
    /// Distance from the centre of gravity beyond which no point reaches, m.
    reach: f64,
}

/// A state, seen from the ground.
struct Seen<'a> {
    state: &'a RigidBodyState,
    env: &'a dyn Environment,
}

impl Seen<'_> {
    fn ned(&self, point: DVec3) -> DVec3 {
        self.state.position_ned + self.state.attitude * point
    }

    /// Depth of a NED position below the ground, m.
    fn depth(&self, ned: DVec3) -> f64 {
        ned.z + self.env.ground_altitude(ned)
    }

    /// Velocity relative to the ground of a point of the airframe, NED frame, m/s.
    fn velocity(&self, point: DVec3) -> DVec3 {
        let s = self.state;
        s.attitude * (s.velocity_body + s.angular_velocity_body.cross(point))
    }

    /// The normal force and friction at a point, as a wrench about the centre of gravity.
    fn wrench(&self, point: DVec3, normal: f64, friction: DVec3) -> Wrench {
        let force = self.state.attitude.conjugate() * (friction - DVec3::Z * normal);
        Wrench::force_at(force, point)
    }
}

/// Friction in every direction: a scraping wing tip.
fn sliding(velocity: DVec3, normal: f64, friction: f64) -> DVec3 {
    let slide = DVec3::new(velocity.x, velocity.y, 0.0);
    -slide * (friction * normal / slide.length().hypot(CREEP_SPEED))
}

impl Undercarriage {
    pub fn new(
        wheels: Vec<Wheel>,
        skids: Vec<Skid>,
        hard_points: Vec<HardPoint>,
    ) -> Result<Self, GroundError> {
        let positive = |v: f64| v.is_finite() && v > 0.0;
        let friction = |v: f64| v.is_finite() && v >= 0.0;
        let wheels_valid = wheels.iter().all(|w| {
            w.axle.is_finite() && positive(w.radius) && w.spring.is_valid() && w.rubbing.is_valid()
        });
        let skids_valid = skids
            .iter()
            .all(|s| s.shoe.is_finite() && s.spring.is_valid() && s.rubbing.is_valid());
        let points_valid = hard_points.iter().all(|h| {
            h.position.is_finite()
                && positive(h.stiffness)
                && h.damping.is_finite()
                && h.damping >= 0.0
                && friction(h.friction)
        });
        if !(wheels_valid && skids_valid && points_valid) {
            return Err(GroundError::BadContact);
        }
        let rises = |w: &Wheel| (w.guide.length() - 1.0).abs() < 1e-9 && w.guide.z < 0.0;
        if !wheels.iter().all(rises) {
            return Err(GroundError::BadGuide);
        }
        let reach = wheels
            .iter()
            .map(|w| w.axle.length() + w.radius)
            .chain(skids.iter().map(|s| s.shoe.length()))
            .chain(hard_points.iter().map(|h| h.position.length()))
            .fold(0.0, f64::max);
        Ok(Self {
            wheels,
            skids,
            hard_points,
            limits: None,
            reach,
        })
    }

    /// The same undercarriage, with the loads at which its parts give way.
    pub fn with_limits(self, limits: ContactLimits) -> Self {
        Self {
            limits: Some(limits),
            ..self
        }
    }

    /// The first point loaded beyond its limit times `tolerance`.
    pub fn overloaded(&self, loads: &GroundLoads, tolerance: f64) -> Option<ContactPoint> {
        let limits = self.limits?;
        loads
            .contacts
            .iter()
            .map(|c| (c.point, c.load))
            .find_map(|(point, load)| {
                let limit = match point {
                    ContactPoint::Wheel(_) => limits.wheel,
                    ContactPoint::Skid(_) => limits.skid,
                    ContactPoint::HardPoint(_) => limits.hard_point,
                };
                (load > limit * tolerance).then_some(point)
            })
    }

    pub fn wheels(&self) -> &[Wheel] {
        &self.wheels
    }

    pub fn skids(&self) -> &[Skid] {
        &self.skids
    }

    pub fn hard_points(&self) -> &[HardPoint] {
        &self.hard_points
    }

    /// Forces of the ground in a state. Far from the ground, nothing is computed.
    pub fn loads(&self, state: &RigidBodyState, env: &dyn Environment) -> GroundLoads {
        let seen = Seen { state, env };
        let mut out = GroundLoads::default();
        if seen.depth(state.position_ned) < -self.reach {
            return out;
        }
        for (i, wheel) in self.wheels.iter().enumerate() {
            self.wheel(&seen, i, wheel, &mut out);
        }
        for (i, skid) in self.skids.iter().enumerate() {
            let shoe = seen.ned(skid.shoe);
            let compression = seen.depth(shoe);
            if compression <= 0.0 {
                continue;
            }
            let velocity = seen.velocity(skid.shoe);
            let (load, cord_travel) = skid.spring.load(compression);
            let normal = skid.spring.normal(load, compression, velocity.z);
            let axis = state.attitude * DVec3::Y;
            let (friction, _) = skid.rubbing.force(velocity, normal, axis);
            out.wrench += seen.wrench(skid.shoe, normal, friction);
            out.contacts.push(Contact {
                point: ContactPoint::Skid(i),
                load: normal,
                compression,
                cord_travel,
                rolling_speed: 0.0,
            });
        }
        for (i, point) in self.hard_points.iter().enumerate() {
            let compression = seen.depth(seen.ned(point.position));
            if compression <= 0.0 {
                continue;
            }
            let velocity = seen.velocity(point.position);
            let ramp = (compression / DAMPING_RAMP).min(1.0);
            let normal =
                (point.stiffness * compression + point.damping * ramp * velocity.z).max(0.0);
            let friction = sliding(velocity, normal, point.friction);
            out.wrench += seen.wrench(point.position, normal, friction);
            out.contacts.push(Contact {
                point: ContactPoint::HardPoint(i),
                load: normal,
                compression,
                cord_travel: 0.0,
                rolling_speed: 0.0,
            });
        }
        out
    }

    fn wheel(&self, seen: &Seen, index: usize, wheel: &Wheel, out: &mut GroundLoads) {
        let axle = seen.ned(wheel.axle);
        let compression = wheel.radius + seen.depth(axle);
        if compression <= 0.0 {
            return;
        }
        let attitude = seen.state.attitude;
        let down = attitude.conjugate() * DVec3::Z;
        let (load, rise) = wheel.spring.load(compression);
        let cord_travel = rise / -wheel.guide.z;
        let tyre = load / wheel.spring.contact_stiffness;
        let point = wheel.axle + wheel.guide * cord_travel + down * (wheel.radius - tyre);
        let velocity = seen.velocity(point);
        let normal = wheel.spring.normal(load, compression, velocity.z);
        let (friction, rolling_speed) = wheel.rubbing.force(velocity, normal, attitude * DVec3::Y);

        out.wrench += seen.wrench(point, normal, friction);
        out.contacts.push(Contact {
            point: ContactPoint::Wheel(index),
            load: normal,
            compression,
            cord_travel,
            rolling_speed,
        });
    }
}

/// Height of the lowest point of a propeller's disc above the ground, m. `hub` and `axis`
/// are in body axes, from the centre of gravity; negative when a blade would strike.
pub fn disc_clearance(
    state: &RigidBodyState,
    hub: DVec3,
    axis: DVec3,
    radius: f64,
    env: &dyn Environment,
) -> f64 {
    let down = state.attitude.conjugate() * DVec3::Z;
    let in_disc = (down - axis * down.dot(axis)).normalize_or_zero();
    let lowest = hub + in_disc * radius;
    let seen = Seen { state, env };
    -seen.depth(seen.ned(lowest))
}

#[cfg(test)]
mod tests;
