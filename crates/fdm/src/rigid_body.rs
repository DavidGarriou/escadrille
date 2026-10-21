//! Six-degree-of-freedom rigid body with an axisymmetric rotor (engine and propeller),
//! integrated with fixed-step RK4 (ADR 0005).
//!
//! State: centre of gravity in the NED frame, attitude quaternion (world ← body),
//! linear and angular velocities in body axes (x forward, y right, z down), and the
//! **absolute** axial angular speed Ω of the rotor about its body axis e.
//!
//! The airframe inertia I' excludes the rotor's axial inertia I_r (I' = I − I_r e eᵀ),
//! so that, with h = I_r Ω e (skill `simulation-vol`, `modele-aero.md`):
//! - ṗ = q v ; q̇ = ½ q ⊗ ω (ω as a pure quaternion)
//! - m (v̇ + ω × v) = F + m g
//! - I' ω̇ = M − ω × (I' ω + h) − Q_net e, where M includes the propeller's
//!   aerodynamic torque −Q_prop e
//! - I_r Ω̇ = Q_net = Q_engine − Q_prop − Q_friction
//!
//! The engine and friction torques are internal: the airframe feels their reaction
//! automatically; the gyroscopic coupling comes from ω × h. Nothing is added twice.
//! The relative speed of the rotor (engine rpm) is Ω − e·ω.

use glam::{DMat3, DQuat, DVec3};

use crate::constants::G0;

/// Inertia about the centre of gravity, body axes, aeronautical convention:
/// products of inertia are positive integrals (`ixz` = ∫ x z dm), so the tensor holds −ixz.
/// Aircraft are symmetric about their x–z plane, hence no `ixy` or `iyz`.
/// Data sheets give the whole aircraft with its rotor locked; [`MassProperties`]
/// removes the rotor's axial part itself.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Inertia {
    pub ixx: f64,
    pub iyy: f64,
    pub izz: f64,
    pub ixz: f64,
}

impl Inertia {
    pub fn tensor(&self) -> DMat3 {
        DMat3::from_cols(
            DVec3::new(self.ixx, 0.0, -self.ixz),
            DVec3::new(0.0, self.iyy, 0.0),
            DVec3::new(-self.ixz, 0.0, self.izz),
        )
    }

    /// Positive definite, and a physical body (triangle inequalities on the
    /// principal moments, including the x–z coupling by `ixz`).
    fn is_physical(&self) -> bool {
        let Inertia { ixx, iyy, izz, ixz } = *self;
        if ![ixx, iyy, izz, ixz].iter().all(|v| v.is_finite()) {
            return false;
        }
        let half_sum = 0.5 * (ixx + izz);
        let radius = (0.25 * (ixx - izz).powi(2) + ixz * ixz).sqrt();
        let (lambda_min, lambda_max) = (half_sum - radius, half_sum + radius);
        iyy > 0.0 && lambda_min > 0.0 && iyy <= ixx + izz && iyy >= lambda_max - lambda_min
    }
}

/// Engine crankshaft, propeller (and rotating cylinders of a rotary engine),
/// seen as one body symmetric about `axis`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rotor {
    /// Unit vector of the rotation axis, body axes; Ω > 0 is a right-handed rotation about it.
    pub axis: DVec3,
    /// Axial moment of inertia, kg·m².
    pub inertia: f64,
}

/// Why mass properties were rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MassError {
    NonPositiveMass,
    InvalidInertia,
    InvalidRotor,
}

/// Mass, airframe inertia I' (rotor axial inertia removed) and rotor, with the
/// inverse tensor cached for the integrator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MassProperties {
    mass: f64,
    inertia: DMat3,
    inertia_inv: DMat3,
    rotor: Option<Rotor>,
}

impl MassProperties {
    /// `inertia` is the whole aircraft about its centre of gravity, rotor included.
    pub fn new(mass: f64, inertia: Inertia, rotor: Option<Rotor>) -> Result<Self, MassError> {
        if !(mass.is_finite() && mass > 0.0) {
            return Err(MassError::NonPositiveMass);
        }
        if !inertia.is_physical() {
            return Err(MassError::InvalidInertia);
        }
        let mut tensor = inertia.tensor();
        if let Some(r) = rotor {
            let unit = (r.axis.length() - 1.0).abs() < 1e-9;
            if !(unit && r.inertia.is_finite() && r.inertia >= 0.0) {
                return Err(MassError::InvalidRotor);
            }
            let e = r.axis;
            tensor -= DMat3::from_cols(e * e.x, e * e.y, e * e.z) * r.inertia;
            if tensor.determinant() <= 0.0 {
                return Err(MassError::InvalidRotor);
            }
        }
        Ok(Self {
            mass,
            inertia: tensor,
            inertia_inv: tensor.inverse(),
            rotor,
        })
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    /// Airframe inertia I' (rotor axial inertia excluded), body axes.
    pub fn airframe_inertia(&self) -> DMat3 {
        self.inertia
    }

    pub fn rotor(&self) -> Option<Rotor> {
        self.rotor
    }
}

/// Force and moment about the centre of gravity, body axes.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Wrench {
    pub force: DVec3,
    pub moment: DVec3,
}

impl Wrench {
    /// A force applied at `point` (body axes, relative to the centre of gravity).
    pub fn force_at(force: DVec3, point: DVec3) -> Self {
        Self {
            force,
            moment: point.cross(force),
        }
    }
}

impl std::ops::Add for Wrench {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self {
            force: self.force + rhs.force,
            moment: self.moment + rhs.moment,
        }
    }
}

impl std::ops::AddAssign for Wrench {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

/// Axial torques acting on the rotor, N·m, positive about the rotor axis.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct RotorTorques {
    /// Engine torque on the crankshaft (internal: reacts on the airframe).
    pub engine: f64,
    /// Aerodynamic torque of the propeller, opposing its rotation when it drives.
    pub propeller: f64,
    /// Bearing and pumping friction (internal).
    pub friction: f64,
}

/// Number of auxiliary states integrated with the body: first-order flow lags (wing
/// downwash reaching the tail, later dynamic stall). Their meaning belongs to the
/// aircraft model (`crate::aircraft`); unused ones stay at zero.
pub const AUX_STATES: usize = 2;

/// What the aircraft model computes for a given state, **gravity excluded**
/// (the integrator adds it, so that `wrench.force / m` is the specific force).
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Loads {
    /// Aerodynamic and contact forces and moments, without the propeller's axial torque.
    pub wrench: Wrench,
    pub rotor: RotorTorques,
    /// Time derivatives of the auxiliary states, per second.
    pub aux_rates: [f64; AUX_STATES],
}

/// Weight of the body expressed in body axes, N.
pub fn gravity_body(attitude: DQuat, mass: f64) -> DVec3 {
    attitude.conjugate() * DVec3::new(0.0, 0.0, mass * G0)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct RigidBodyState {
    /// Centre of gravity in the NED frame, m.
    pub position_ned: DVec3,
    /// Rotation from body to world (NED) axes; unit quaternion.
    pub attitude: DQuat,
    /// Velocity of the centre of gravity relative to the ground, body axes, m/s.
    pub velocity_body: DVec3,
    /// Angular velocity (p, q, r), body axes, rad/s.
    pub angular_velocity_body: DVec3,
    /// Absolute axial angular speed of the rotor, rad/s (0 without rotor).
    pub rotor_speed: f64,
    /// Auxiliary states (see [`AUX_STATES`]).
    pub aux: [f64; AUX_STATES],
}

impl Default for RigidBodyState {
    fn default() -> Self {
        Self {
            position_ned: DVec3::ZERO,
            attitude: DQuat::IDENTITY,
            velocity_body: DVec3::ZERO,
            angular_velocity_body: DVec3::ZERO,
            rotor_speed: 0.0,
            aux: [0.0; AUX_STATES],
        }
    }
}

/// Time derivative of a [`RigidBodyState`].
#[derive(Debug, Clone, Copy)]
struct StateDerivative {
    position: DVec3,
    attitude: DQuat,
    velocity: DVec3,
    angular_velocity: DVec3,
    rotor_speed: f64,
    aux: [f64; AUX_STATES],
}

impl RigidBodyState {
    /// Velocity relative to the ground, NED frame, m/s.
    pub fn velocity_ned(&self) -> DVec3 {
        self.attitude * self.velocity_body
    }

    /// Rotor speed relative to the airframe (engine shaft speed), rad/s.
    pub fn rotor_relative_speed(&self, mass: &MassProperties) -> f64 {
        mass.rotor.map_or(0.0, |r| {
            self.rotor_speed - r.axis.dot(self.angular_velocity_body)
        })
    }

    fn rotor_momentum(&self, mass: &MassProperties) -> DVec3 {
        mass.rotor
            .map_or(DVec3::ZERO, |r| r.axis * (r.inertia * self.rotor_speed))
    }

    /// Kinetic energy of airframe and rotor, J.
    pub fn kinetic_energy(&self, mass: &MassProperties) -> f64 {
        let w = self.angular_velocity_body;
        let rotor = mass
            .rotor
            .map_or(0.0, |r| 0.5 * r.inertia * self.rotor_speed.powi(2));
        0.5 * mass.mass * self.velocity_body.length_squared()
            + 0.5 * w.dot(mass.inertia * w)
            + rotor
    }

    /// Total angular momentum about the centre of gravity, NED frame, kg·m²/s.
    pub fn angular_momentum_ned(&self, mass: &MassProperties) -> DVec3 {
        self.attitude * (mass.inertia * self.angular_velocity_body + self.rotor_momentum(mass))
    }

    /// Same state with a unit attitude quaternion (what the load model sees).
    fn normalized(&self) -> Self {
        Self {
            attitude: self.attitude.normalize(),
            ..*self
        }
    }

    fn derivative(&self, mass: &MassProperties, loads: &Loads) -> StateDerivative {
        let v = self.velocity_body;
        let w = self.angular_velocity_body;
        let unit_attitude = self.attitude.normalize();
        let force = loads.wrench.force + gravity_body(unit_attitude, mass.mass);

        let mut moment = loads.wrench.moment;
        let mut rotor_speed_dot = 0.0;
        if let Some(r) = mass.rotor {
            let t = loads.rotor;
            let net = t.engine - t.propeller - t.friction;
            // External propeller torque −Q_prop e, plus the −Q_net e term of I' ω̇:
            // the airframe ends up feeling −(Q_engine − Q_friction) e.
            moment -= r.axis * (t.propeller + net);
            if r.inertia > 0.0 {
                rotor_speed_dot = net / r.inertia;
            }
        }
        let momentum = mass.inertia * w + self.rotor_momentum(mass);

        StateDerivative {
            position: self.attitude * v,
            attitude: self.attitude.mul_quat(DQuat::from_xyzw(w.x, w.y, w.z, 0.0)) * 0.5,
            velocity: force / mass.mass - w.cross(v),
            angular_velocity: mass.inertia_inv * (moment - w.cross(momentum)),
            rotor_speed: rotor_speed_dot,
            aux: loads.aux_rates,
        }
    }

    /// `self + h · d`, without renormalising the quaternion (RK4 intermediate stages).
    fn advanced(&self, d: &StateDerivative, h: f64) -> Self {
        Self {
            position_ned: self.position_ned + d.position * h,
            attitude: self.attitude + d.attitude * h,
            velocity_body: self.velocity_body + d.velocity * h,
            angular_velocity_body: self.angular_velocity_body + d.angular_velocity * h,
            rotor_speed: self.rotor_speed + d.rotor_speed * h,
            aux: std::array::from_fn(|i| self.aux[i] + d.aux[i] * h),
        }
    }
}

/// Advances `state` by `dt` seconds with the classical fourth-order Runge-Kutta method.
///
/// `loads` returns the forces, moments and rotor torques (gravity excluded) for a
/// state; it is called four times per step, on trial states, so it must not have side
/// effects (damage, contact events, telemetry are handled once per accepted step).
/// It always receives a unit attitude quaternion.
pub fn step_rk4(
    state: &RigidBodyState,
    mass: &MassProperties,
    dt: f64,
    loads: impl Fn(&RigidBodyState) -> Loads,
) -> RigidBodyState {
    let eval = |s: &RigidBodyState| s.derivative(mass, &loads(&s.normalized()));
    let k1 = eval(state);
    let k2 = eval(&state.advanced(&k1, 0.5 * dt));
    let k3 = eval(&state.advanced(&k2, 0.5 * dt));
    let k4 = eval(&state.advanced(&k3, dt));

    let sixth = dt / 6.0;
    let combine = |a: f64, b: f64, c: f64, d: f64| (a + 2.0 * (b + c) + d) * sixth;
    let next = RigidBodyState {
        position_ned: state.position_ned
            + (k1.position + (k2.position + k3.position) * 2.0 + k4.position) * sixth,
        attitude: state.attitude
            + (k1.attitude + (k2.attitude + k3.attitude) * 2.0 + k4.attitude) * sixth,
        velocity_body: state.velocity_body
            + (k1.velocity + (k2.velocity + k3.velocity) * 2.0 + k4.velocity) * sixth,
        angular_velocity_body: state.angular_velocity_body
            + (k1.angular_velocity
                + (k2.angular_velocity + k3.angular_velocity) * 2.0
                + k4.angular_velocity)
                * sixth,
        rotor_speed: state.rotor_speed
            + combine(
                k1.rotor_speed,
                k2.rotor_speed,
                k3.rotor_speed,
                k4.rotor_speed,
            ),
        aux: std::array::from_fn(|i| {
            state.aux[i] + combine(k1.aux[i], k2.aux[i], k3.aux[i], k4.aux[i])
        }),
    };
    next.normalized()
}

#[cfg(test)]
mod tests;
