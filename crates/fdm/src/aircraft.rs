//! An aircraft: mass, lifting system, parasitic drag, propeller and engine assembled into
//! one load model, integrated by [`step_rk4`] (skill `simulation-vol`).
//!
//! Order of evaluation for a state: air data (wind, density), propeller (thrust, torque,
//! slipstream), engine, lifting surfaces (slipstream everywhere, reduced by the jet-width
//! factor, lagged wing downwash on the tail), parasitic drag (whole slipstream), then the
//! sum.
//!
//! Wing downwash at the tail: ε = `factor` × C_L of the wings (Prandtl's lifting-line
//! result ε ∝ C_L, so that it collapses with the lift at the stall), reaching the tail
//! after l_t / V. The delay is modelled as a first-order lag, the auxiliary state
//! [`AUX_DOWNWASH`]: dε/dt = (factor × C_L − ε) V / l_t. It gives the classical
//! M_α̇ damping. The downwash velocity ε V is applied to the tail strips, opposite to the
//! lift direction of the plane of symmetry, and fades out when flying backwards.
//!
//! The fuselage is a slender body ([`SlenderBody`]): side force and destabilising Munk
//! moments.

use glam::DVec3;

use crate::aero::body::SlenderBody;
use crate::aero::drag::DragItem;
use crate::aero::surface::ControlDeflections;
use crate::aero::system::{LiftingSystem, SystemLoads};
use crate::environment::Environment;
use crate::propulsion::engine::{EngineTorques, InlineEngine};
use crate::propulsion::propeller::{
    Propeller, PropellerGeometry, PropellerLoads, Slipstream, WingWake,
};
use crate::rigid_body::{
    AUX_STATES, Loads, MassProperties, RigidBodyState, RotorTorques, step_rk4,
};

/// Index of the downwash lag (rad) in [`RigidBodyState::aux`].
pub const AUX_DOWNWASH: usize = 0;

/// Below this airspeed (m/s), the downwash lag uses this speed (keeps l_t / V finite).
const MIN_LAG_SPEED: f64 = 1.0;

/// Below this airspeed in the plane of symmetry (m/s), there is no downwash direction.
const MIN_DOWNWASH_SPEED: f64 = 1e-3;

/// Pilot commands: stick and rudder in [−1, 1] (positive = roll right, nose up, nose
/// right), throttle lever in [0, 1]. Out-of-range values are clamped.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct PilotControls {
    pub roll: f64,
    pub pitch: f64,
    pub yaw: f64,
    pub throttle: f64,
}

/// Control surface deflection for a full command, rad (same both ways for now).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ControlTravel {
    pub aileron: f64,
    pub elevator: f64,
    pub rudder: f64,
}

/// Wing downwash on the tail.
#[derive(Debug, Clone, PartialEq)]
pub struct Downwash {
    /// Surfaces whose lift makes the downwash (indices in the lifting system).
    pub wings: Vec<usize>,
    /// Surfaces that receive it.
    pub tail: Vec<usize>,
    /// dε / dC_L of the wings, rad (C: from the geometry).
    pub factor: f64,
    /// Distance travelled by the downwash from the wings to the tail, m.
    pub tail_arm: f64,
    /// Fraction of the slipstream's swirl left behind the wings, which straighten the rest
    /// (C: about one half, NACA Report 747 and TN 1146), 0 to 1.
    pub swirl_left: f64,
}

/// Propeller and the engine that drives it.
#[derive(Debug, Clone, PartialEq)]
pub struct Propulsion {
    pub propeller: Propeller,
    pub engine: InlineEngine,
}

/// Why an aircraft was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AircraftError {
    /// The propeller needs a rotor in the mass properties, turning about its rotation axis.
    RotorMismatch,
    BadDownwash,
    BadTravel,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Aircraft {
    mass: MassProperties,
    lifting: LiftingSystem,
    drag: Vec<DragItem>,
    bodies: Vec<SlenderBody>,
    propulsion: Option<Propulsion>,
    downwash: Option<Downwash>,
    /// Where the wings straighten the slipstream's swirl (with a propeller and wings).
    wing_wake: Option<WingWake>,
    travel: ControlTravel,
}

/// Everything computed for one state, for telemetry, trim and tests.
#[derive(Debug, Clone, PartialEq)]
pub struct Diagnostics {
    pub loads: Loads,
    /// Velocity of the centre of gravity relative to the air, body axes, m/s.
    pub air_velocity: DVec3,
    pub density: f64,
    /// Lift coefficient of each lifting surface.
    pub lift_coefficients: Vec<f64>,
    pub propeller: Option<PropellerLoads>,
    pub engine: EngineTorques,
    /// Shaft speed relative to the airframe, rad/s.
    pub shaft_speed: f64,
    /// Downwash angle at the tail (lagged state), rad.
    pub downwash: f64,
}

impl Aircraft {
    pub fn new(
        mass: MassProperties,
        lifting: LiftingSystem,
        drag: Vec<DragItem>,
        bodies: Vec<SlenderBody>,
        propulsion: Option<Propulsion>,
        downwash: Option<Downwash>,
        travel: ControlTravel,
    ) -> Result<Self, AircraftError> {
        if let Some(p) = &propulsion {
            let axis = p.propeller.rotation_axis();
            let matches = mass
                .rotor()
                .is_some_and(|r| (r.axis - axis).length() < 1e-9);
            if !matches {
                return Err(AircraftError::RotorMismatch);
            }
        }
        if let Some(d) = &downwash {
            let n = lifting.surfaces().len();
            let valid = !d.wings.is_empty()
                && !d.tail.is_empty()
                && d.wings.iter().chain(&d.tail).all(|&i| i < n)
                && d.tail.iter().all(|i| !d.wings.contains(i))
                && d.factor.is_finite()
                && d.factor >= 0.0
                && d.tail_arm.is_finite()
                && d.tail_arm > 0.0
                && (0.0..=1.0).contains(&d.swirl_left);
            if !valid {
                return Err(AircraftError::BadDownwash);
            }
        }
        let t = travel;
        if ![t.aileron, t.elevator, t.rudder]
            .iter()
            .all(|v| v.is_finite() && *v >= 0.0)
        {
            return Err(AircraftError::BadTravel);
        }
        let wing_wake = match (&propulsion, &downwash) {
            (Some(p), Some(d)) => Some(wing_wake(&lifting, d, p.propeller.geometry())),
            _ => None,
        };
        Ok(Self {
            mass,
            lifting,
            drag,
            bodies,
            propulsion,
            downwash,
            wing_wake,
            travel,
        })
    }

    pub fn mass(&self) -> &MassProperties {
        &self.mass
    }

    pub fn lifting(&self) -> &LiftingSystem {
        &self.lifting
    }

    pub fn propulsion(&self) -> Option<&Propulsion> {
        self.propulsion.as_ref()
    }

    /// Control-surface deflections for full commands.
    pub fn travel(&self) -> ControlTravel {
        self.travel
    }

    /// Wing downwash on the tail, and which surfaces are the wings.
    pub fn downwash(&self) -> Option<&Downwash> {
        self.downwash.as_ref()
    }

    /// Where the wings straighten the slipstream's swirl.
    pub fn wing_wake(&self) -> Option<WingWake> {
        self.wing_wake
    }

    /// The same airframe without propeller and engine: no thrust and no slipstream, as in
    /// the « true drag » of full-scale glide tests flown at zero thrust (NACA Report 304).
    pub fn without_propulsion(&self) -> Self {
        Self {
            propulsion: None,
            wing_wake: None,
            ..self.clone()
        }
    }

    /// Control surface deflections for the pilot's commands.
    pub fn deflections(&self, controls: &PilotControls) -> ControlDeflections {
        let c = |v: f64| v.clamp(-1.0, 1.0);
        ControlDeflections {
            aileron: c(controls.roll) * self.travel.aileron,
            elevator: c(controls.pitch) * self.travel.elevator,
            rudder: c(controls.yaw) * self.travel.rudder,
        }
    }

    /// Loads (gravity excluded) and diagnostics for a state.
    pub fn evaluate(
        &self,
        state: &RigidBodyState,
        controls: &PilotControls,
        env: &dyn Environment,
    ) -> Diagnostics {
        self.evaluate_deflected(state, &self.deflections(controls), controls.throttle, env)
    }

    /// Same as [`Self::evaluate`] with the control-surface deflections given directly
    /// (rad, not limited to the travel): for the trim solver, which must not see the
    /// commands saturate.
    pub fn evaluate_deflected(
        &self,
        state: &RigidBodyState,
        deflections: &ControlDeflections,
        throttle: f64,
        env: &dyn Environment,
    ) -> Diagnostics {
        let position = state.position_ned;
        let atmosphere = env.atmosphere(position);
        let density = atmosphere.density;
        let wind_body = state.attitude.conjugate() * env.wind_ned(position);
        let air_velocity = state.velocity_body - wind_body;
        let omega = state.angular_velocity_body;
        let shaft_speed = state.rotor_relative_speed(&self.mass);

        let (propeller, slipstream, engine) = match &self.propulsion {
            Some(p) => {
                let loads = p
                    .propeller
                    .loads(air_velocity, omega, state.rotor_speed, density);
                let slipstream = self.slipstream(&loads, density);
                let engine =
                    p.engine
                        .torques(shaft_speed, throttle, p.engine.charge_ratio(&atmosphere));
                (Some(loads), slipstream, engine)
            }
            None => (None, None, EngineTorques::default()),
        };
        let blown = |r: DVec3| slipstream.map_or(DVec3::ZERO, |s| s.local_flow(r));
        let downwash = state.aux[AUX_DOWNWASH];
        let lifting = self.lifting_in(
            air_velocity,
            omega,
            density,
            downwash,
            deflections,
            slipstream.as_ref(),
        );

        let mut wrench = lifting.wrench;
        for item in &self.drag {
            wrench += item.loads(air_velocity, omega, density, blown);
        }
        for body in &self.bodies {
            wrench += body.loads(air_velocity, omega, density, blown);
        }
        if let Some(p) = &propeller {
            wrench += p.wrench;
        }

        let mut aux_rates = [0.0; AUX_STATES];
        if let Some(d) = &self.downwash {
            let surfaces = self.lifting.surfaces();
            let (mut lift, mut area) = (0.0, 0.0);
            for &i in &d.wings {
                let s: f64 = surfaces[i].strips().iter().map(|strip| strip.area).sum();
                lift += lifting.lift_coefficients[i] * s;
                area += s;
            }
            let target = d.factor * lift / area;
            let speed = air_velocity.length().max(MIN_LAG_SPEED);
            aux_rates[AUX_DOWNWASH] = (target - downwash) * speed / d.tail_arm;
        }

        let loads = Loads {
            wrench,
            rotor: RotorTorques {
                engine: engine.combustion,
                propeller: propeller.map_or(0.0, |p| p.torque),
                friction: engine.friction,
            },
            aux_rates,
        };
        Diagnostics {
            loads,
            air_velocity,
            density,
            lift_coefficients: lifting.lift_coefficients,
            propeller,
            engine,
            shaft_speed,
            downwash,
        }
    }

    /// The propeller's slipstream for these propeller loads, straightened behind the wings.
    pub fn slipstream(&self, propeller: &PropellerLoads, density: f64) -> Option<Slipstream> {
        let p = self.propulsion.as_ref()?;
        let free = p.propeller.slipstream(propeller, density);
        Some(self.wing_wake.map_or(free, |w| free.behind_wings(w)))
    }

    /// Loads of the lifting surfaces in a state, in a given slipstream: the aircraft's own
    /// ([`Self::slipstream`]) in [`Self::evaluate`], or a variant of it for diagnostics.
    pub fn lifting_loads(
        &self,
        state: &RigidBodyState,
        deflections: &ControlDeflections,
        env: &dyn Environment,
        slipstream: Option<&Slipstream>,
    ) -> SystemLoads {
        let position = state.position_ned;
        let wind_body = state.attitude.conjugate() * env.wind_ned(position);
        self.lifting_in(
            state.velocity_body - wind_body,
            state.angular_velocity_body,
            env.atmosphere(position).density,
            state.aux[AUX_DOWNWASH],
            deflections,
            slipstream,
        )
    }

    /// Lifting surfaces in the slipstream (reduced by the jet-width factor), with the lagged
    /// wing downwash on the tail.
    fn lifting_in(
        &self,
        air_velocity: DVec3,
        omega: DVec3,
        density: f64,
        downwash: f64,
        deflections: &ControlDeflections,
        slipstream: Option<&Slipstream>,
    ) -> SystemLoads {
        let downwash_velocity = downwash_velocity(air_velocity, downwash);
        let is_tail = |i: usize| self.downwash.as_ref().is_some_and(|d| d.tail.contains(&i));
        let surfaces = self.lifting.surfaces();
        self.lifting
            .loads(air_velocity, omega, density, deflections, |i, strip| {
                let blown = slipstream.map_or(DVec3::ZERO, |s| {
                    s.lifting_flow(strip.position, strip.chord, surfaces[i].section_slope())
                });
                if is_tail(i) {
                    blown + downwash_velocity
                } else {
                    blown
                }
            })
    }

    /// Loads for a state (gravity excluded).
    pub fn loads(
        &self,
        state: &RigidBodyState,
        controls: &PilotControls,
        env: &dyn Environment,
    ) -> Loads {
        self.evaluate(state, controls, env).loads
    }

    /// Advances the aircraft by `dt` seconds (RK4, ADR 0005).
    pub fn step(
        &self,
        state: &RigidBodyState,
        controls: &PilotControls,
        env: &dyn Environment,
        dt: f64,
    ) -> RigidBodyState {
        step_rk4(state, &self.mass, dt, |s| self.loads(s, controls, env))
    }
}

/// The wings' wake in the slipstream: from their rearmost trailing edge, over their mean
/// chord, down to the swirl the wings leave.
fn wing_wake(
    lifting: &LiftingSystem,
    downwash: &Downwash,
    propeller: &PropellerGeometry,
) -> WingWake {
    let surfaces = lifting.surfaces();
    let (mut start, mut area, mut chord_area) = (f64::NEG_INFINITY, 0.0, 0.0);
    for strip in downwash.wings.iter().flat_map(|&i| surfaces[i].strips()) {
        let trailing_edge = strip.position - strip.chord_axis * (0.75 * strip.chord);
        start = start.max(-(trailing_edge - propeller.hub).dot(propeller.thrust_axis));
        area += strip.area;
        chord_area += strip.area * strip.chord;
    }
    WingWake {
        start,
        length: chord_area / area,
        swirl_left: downwash.swirl_left,
    }
}

/// Downwash velocity at the tail for a downwash angle ε: ε V opposite to the lift
/// direction of the plane of symmetry, sideslip excluded, and none when flying backwards
/// (the tail is then upstream of the wings): weighted by max(0, u / V).
fn downwash_velocity(air_velocity: DVec3, downwash: f64) -> DVec3 {
    let symmetric = DVec3::new(air_velocity.x, 0.0, air_velocity.z);
    let speed = symmetric.length();
    if speed <= MIN_DOWNWASH_SPEED {
        return DVec3::ZERO;
    }
    let lift_direction = DVec3::new(symmetric.z, 0.0, -symmetric.x) / speed;
    let forward = (air_velocity.x / air_velocity.length()).max(0.0);
    -lift_direction * (downwash * speed * forward)
}

#[cfg(test)]
mod tests;
