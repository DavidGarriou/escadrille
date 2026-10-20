//! Six-degree-of-freedom rigid body and its fixed-step RK4 integrator (ADR 0005).
//!
//! State: position in the NED frame, attitude quaternion (world ← body), linear and
//! angular velocities in body axes (x forward, y right, z down), about the centre of
//! gravity. Equations (skill `simulation-vol`, `modele-aero.md`):
//! - ṗ = q v
//! - q̇ = ½ q ⊗ (ω, 0)
//! - m (v̇ + ω × v) = F
//! - I ω̇ = M − ω × (I ω)
//!
//! The rotor angular momentum (engine and propeller) is added later.

use glam::{DMat3, DQuat, DVec3};

use crate::constants::G0;

/// Inertia about the centre of gravity, body axes, aeronautical convention:
/// products of inertia are positive integrals (`ixz` = ∫ x z dm), so the tensor holds −ixz.
/// Aircraft are symmetric about their x–z plane, hence no `ixy` or `iyz`.
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
}

/// Why mass properties were rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MassError {
    NonPositiveMass,
    InvalidInertia,
}

/// Mass and inertia, with the inverse tensor cached for the integrator.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MassProperties {
    mass: f64,
    inertia: DMat3,
    inertia_inv: DMat3,
}

impl MassProperties {
    pub fn new(mass: f64, inertia: Inertia) -> Result<Self, MassError> {
        if !(mass.is_finite() && mass > 0.0) {
            return Err(MassError::NonPositiveMass);
        }
        let i = inertia;
        // Positive definite, and the triangle inequalities of a physical body.
        let valid = [i.ixx, i.iyy, i.izz, i.ixz].iter().all(|v| v.is_finite())
            && i.ixx > 0.0
            && i.iyy > 0.0
            && i.izz > 0.0
            && i.ixx * i.izz > i.ixz * i.ixz
            && i.ixx + i.iyy >= i.izz
            && i.iyy + i.izz >= i.ixx
            && i.izz + i.ixx >= i.iyy;
        if !valid {
            return Err(MassError::InvalidInertia);
        }
        let tensor = inertia.tensor();
        Ok(Self {
            mass,
            inertia: tensor,
            inertia_inv: tensor.inverse(),
        })
    }

    pub fn mass(&self) -> f64 {
        self.mass
    }

    pub fn inertia(&self) -> DMat3 {
        self.inertia
    }
}

/// Force and moment about the centre of gravity, body axes. Includes gravity.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Wrench {
    pub force: DVec3,
    pub moment: DVec3,
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
}

impl Default for RigidBodyState {
    fn default() -> Self {
        Self {
            position_ned: DVec3::ZERO,
            attitude: DQuat::IDENTITY,
            velocity_body: DVec3::ZERO,
            angular_velocity_body: DVec3::ZERO,
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
}

impl RigidBodyState {
    /// Velocity relative to the ground, NED frame, m/s.
    pub fn velocity_ned(&self) -> DVec3 {
        self.attitude * self.velocity_body
    }

    /// Translational plus rotational kinetic energy, J.
    pub fn kinetic_energy(&self, mass: &MassProperties) -> f64 {
        let w = self.angular_velocity_body;
        0.5 * mass.mass * self.velocity_body.length_squared() + 0.5 * w.dot(mass.inertia * w)
    }

    /// Angular momentum about the centre of gravity, NED frame, kg·m²/s.
    pub fn angular_momentum_ned(&self, mass: &MassProperties) -> DVec3 {
        self.attitude * (mass.inertia * self.angular_velocity_body)
    }

    fn derivative(&self, mass: &MassProperties, wrench: Wrench) -> StateDerivative {
        let v = self.velocity_body;
        let w = self.angular_velocity_body;
        let omega = DQuat::from_xyzw(w.x, w.y, w.z, 0.0);
        StateDerivative {
            position: self.attitude * v,
            attitude: self.attitude.mul_quat(omega) * 0.5,
            velocity: wrench.force / mass.mass - w.cross(v),
            angular_velocity: mass.inertia_inv * (wrench.moment - w.cross(mass.inertia * w)),
        }
    }

    /// `self + h · d`, without renormalising the quaternion (RK4 intermediate stages).
    fn advanced(&self, d: &StateDerivative, h: f64) -> Self {
        Self {
            position_ned: self.position_ned + d.position * h,
            attitude: self.attitude + d.attitude * h,
            velocity_body: self.velocity_body + d.velocity * h,
            angular_velocity_body: self.angular_velocity_body + d.angular_velocity * h,
        }
    }
}

/// Advances `state` by `dt` seconds with the classical fourth-order Runge-Kutta method.
///
/// `wrench` returns the total force and moment (gravity included) for a given state;
/// it is called four times per step, always with the same commands and environment.
pub fn step_rk4(
    state: &RigidBodyState,
    mass: &MassProperties,
    dt: f64,
    mut wrench: impl FnMut(&RigidBodyState) -> Wrench,
) -> RigidBodyState {
    let k1 = state.derivative(mass, wrench(state));
    let s2 = state.advanced(&k1, 0.5 * dt);
    let k2 = s2.derivative(mass, wrench(&s2));
    let s3 = state.advanced(&k2, 0.5 * dt);
    let k3 = s3.derivative(mass, wrench(&s3));
    let s4 = state.advanced(&k3, dt);
    let k4 = s4.derivative(mass, wrench(&s4));

    let sixth = dt / 6.0;
    let mut next = RigidBodyState {
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
    };
    next.attitude = next.attitude.normalize();
    next
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::FRAC_PI_6;

    const DT: f64 = 1.0 / 240.0;

    fn body(ixx: f64, iyy: f64, izz: f64) -> MassProperties {
        MassProperties::new(
            700.0,
            Inertia {
                ixx,
                iyy,
                izz,
                ixz: 0.0,
            },
        )
        .unwrap()
    }

    fn gravity_only(mass: f64) -> impl FnMut(&RigidBodyState) -> Wrench {
        move |s| Wrench {
            force: gravity_body(s.attitude, mass),
            moment: DVec3::ZERO,
        }
    }

    fn simulate(
        mut state: RigidBodyState,
        mass: &MassProperties,
        dt: f64,
        duration: f64,
        mut wrench: impl FnMut(&RigidBodyState) -> Wrench,
    ) -> RigidBodyState {
        let steps = (duration / dt).round() as usize;
        for _ in 0..steps {
            state = step_rk4(&state, mass, dt, &mut wrench);
        }
        state
    }

    #[test]
    fn rejects_unphysical_mass_properties() {
        let ok = Inertia {
            ixx: 1.0,
            iyy: 2.0,
            izz: 2.5,
            ixz: 0.1,
        };
        assert_eq!(
            MassProperties::new(0.0, ok),
            Err(MassError::NonPositiveMass)
        );
        let flat = Inertia { izz: 4.0, ..ok }; // ixx + iyy < izz
        assert_eq!(
            MassProperties::new(1.0, flat),
            Err(MassError::InvalidInertia)
        );
        assert!(MassProperties::new(1.0, ok).is_ok());
    }

    #[test]
    fn free_fall_matches_exact_solution_in_any_attitude() {
        // RK4 integrates a constant acceleration exactly (up to rounding).
        let mass = body(1_000.0, 1_500.0, 2_200.0);
        let attitude = DQuat::from_euler(glam::EulerRot::ZYX, 0.7, FRAC_PI_6, -0.4);
        let start = RigidBodyState {
            attitude,
            ..Default::default()
        };
        let end = simulate(start, &mass, DT, 3.0, gravity_only(mass.mass()));
        let expected_z = 0.5 * G0 * 9.0;
        assert!((end.position_ned.z - expected_z).abs() < 1e-9, "{end:?}");
        assert!(end.position_ned.truncate().length() < 1e-9);
        assert!((end.velocity_ned() - DVec3::new(0.0, 0.0, G0 * 3.0)).length() < 1e-9);
        // Components, not `angle_between`: acos cannot resolve angles below ~1e-8.
        assert!((end.attitude - attitude).length() < 1e-12, "{end:?}");
    }

    #[test]
    fn steady_rotation_about_principal_axis() {
        let mass = body(1_000.0, 1_500.0, 2_200.0);
        let rate = 0.8; // rad/s, roll
        let start = RigidBodyState {
            angular_velocity_body: DVec3::new(rate, 0.0, 0.0),
            ..Default::default()
        };
        let end = simulate(start, &mass, DT, 2.0, |_| Wrench::default());
        let expected = DQuat::from_rotation_x(rate * 2.0);
        assert!(end.attitude.angle_between(expected) < 1e-9, "{end:?}");
        assert!((end.angular_velocity_body - start.angular_velocity_body).length() < 1e-12);
    }

    #[test]
    fn torque_free_motion_conserves_energy_and_angular_momentum() {
        // Asymmetric body spinning near its unstable intermediate axis: the motion is
        // violent (tumbling), but energy and world-frame angular momentum are conserved.
        let mass = body(1.0, 2.0, 3.0);
        let start = RigidBodyState {
            angular_velocity_body: DVec3::new(0.05, 2.0, 0.05),
            ..Default::default()
        };
        let e0 = start.kinetic_energy(&mass);
        let l0 = start.angular_momentum_ned(&mass);
        let end = simulate(start, &mass, DT, 20.0, |_| Wrench::default());
        let e1 = end.kinetic_energy(&mass);
        let l1 = end.angular_momentum_ned(&mass);
        assert!(((e1 - e0) / e0).abs() < 1e-7, "energy {e0} -> {e1}");
        assert!(
            (l1 - l0).length() / l0.length() < 1e-6,
            "momentum {l0} -> {l1}"
        );
        // It did tumble: the body-frame rates changed a lot.
        assert!((end.angular_velocity_body - start.angular_velocity_body).length() > 0.5);
    }

    #[test]
    fn symmetric_top_precesses_at_the_analytic_rate() {
        // Ixx = Iyy: ω⊥ turns about the body z axis at λ = (Izz − Ixx) / Ixx · ωz.
        let (ixx, izz, wz, a) = (2.0, 3.0, 4.0, 0.3);
        let mass = body(ixx, ixx, izz);
        let start = RigidBodyState {
            angular_velocity_body: DVec3::new(a, 0.0, wz),
            ..Default::default()
        };
        let t = 1.5;
        let end = simulate(start, &mass, DT, t, |_| Wrench::default());
        let lambda = (izz - ixx) / ixx * wz;
        let expected = DVec3::new(a * (lambda * t).cos(), a * (lambda * t).sin(), wz);
        assert!(
            (end.angular_velocity_body - expected).length() < 1e-8,
            "{} vs {expected}",
            end.angular_velocity_body
        );
    }

    #[test]
    fn product_of_inertia_couples_roll_and_yaw() {
        // A pure roll moment on a body with ixz ≠ 0 also starts a yaw rate.
        let mass = MassProperties::new(
            700.0,
            Inertia {
                ixx: 1_000.0,
                iyy: 1_500.0,
                izz: 2_200.0,
                ixz: 100.0,
            },
        )
        .unwrap();
        let end = simulate(RigidBodyState::default(), &mass, DT, 0.1, |_| Wrench {
            force: DVec3::ZERO,
            moment: DVec3::new(500.0, 0.0, 0.0),
        });
        assert!(end.angular_velocity_body.x > 0.0);
        assert!(end.angular_velocity_body.z > 0.0, "{end:?}");
    }

    #[test]
    fn error_shrinks_like_fourth_order() {
        // Halving the step divides the error by about 2⁴ = 16.
        let mass = body(1.0, 2.0, 3.0);
        let start = RigidBodyState {
            angular_velocity_body: DVec3::new(0.3, 1.0, 0.6),
            ..Default::default()
        };
        let reference = simulate(start, &mass, 1.0 / 3_840.0, 2.0, |_| Wrench::default());
        let error = |dt: f64| {
            let s = simulate(start, &mass, dt, 2.0, |_| Wrench::default());
            (s.angular_velocity_body - reference.angular_velocity_body).length()
        };
        let ratio = error(1.0 / 60.0) / error(1.0 / 120.0);
        assert!((10.0..24.0).contains(&ratio), "error ratio {ratio}");
    }

    #[test]
    fn integration_is_deterministic() {
        let mass = body(1.0, 2.0, 3.0);
        let start = RigidBodyState {
            velocity_body: DVec3::new(50.0, 1.0, -2.0),
            angular_velocity_body: DVec3::new(0.3, 1.0, 0.6),
            ..Default::default()
        };
        let run = || simulate(start, &mass, DT, 5.0, gravity_only(mass.mass()));
        let (a, b) = (run(), run());
        assert_eq!(a.position_ned.to_array(), b.position_ned.to_array());
        assert_eq!(a.attitude.to_array(), b.attitude.to_array());
    }

    #[test]
    fn gravity_points_down_in_body_axes_when_pitched_up() {
        // Nose 90° up: "down" is −x in body axes.
        let attitude = DQuat::from_rotation_y(std::f64::consts::FRAC_PI_2);
        let g = gravity_body(attitude, 1.0);
        assert!((g - DVec3::new(-G0, 0.0, 0.0)).length() < 1e-12, "{g}");
    }
}
