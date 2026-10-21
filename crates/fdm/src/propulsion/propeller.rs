//! Fixed-pitch propeller (skill `simulation-vol`, `modele-aero.md`, section « Hélice »).
//!
//! Inside the table: measured or computed coefficients C_T(J) and C_P(J), with
//! J = V / (n D), T = C_T ρ n² D⁴, P = C_P ρ n³ D⁵ and Q = P / (2π n).
//!
//! These coefficients blow up when n → 0 (stopped or windmilling propeller, tail slide).
//! The propeller is therefore described everywhere in the four-quadrant form used for
//! ship propellers (Wageningen B-series; van Lammeren, van Manen & Oosterveld, *The
//! Wageningen B-screw series*, Trans. SNAME 77, 1969): advance angle
//! β = atan2(V_a, 0.7 π n D), reference speed V_r² = V_a² + (0.7 π n D)²,
//! T = C_T* · ½ ρ V_r² · (π/4) D² and Q = C_Q* · ½ ρ V_r² · (π/4) D³. Everything stays
//! finite at n = 0 and V = 0. Inside the table, C_T* = 8 C_T / (π (J² + (0.7π)²)) and
//! C_Q* = 8 C_Q / (π (J² + (0.7π)²)), with C_Q = C_P / 2π.
//!
//! Beyond the table, C_T* and C_Q* are interpolated linearly in β between the table's
//! ends and three estimated anchors (E: in the data, until the blade-element tool of
//! `fdm_lab` computes them): stopped propeller with the flow from ahead (β = 90°) or
//! from behind (β = −90°), and propeller turning backwards in still air (β = ±180°).
//!
//! Energy: anchors and table must never let the propeller give more power than it
//! receives, Q Ω − T V_a ≥ 0, i.e. C_Q* cos β / 0.35 − C_T* sin β ≥ 0 over the whole
//! circle (checked by [`Propeller::new`]; inside the table it is η ≤ 1).
//!
//! Cross flow V⊥ (inflow angle, yaw rate at the hub):
//! - **P-factor**: the blade moving with the cross flow works harder and the thrust line
//!   shifts towards it (after JSBSim's `p_factor`). The asymmetry is proportional to
//!   V⊥ / (blade speed), so the shift is `p_factor` × V⊥ V_blade / V_r² (E:): it vanishes
//!   at rest and when the propeller stops, and reverses when it turns backwards
//!   (aerodynamicist review of 2026-10-21: a shift growing like the sine of the inflow
//!   angle saturated at low speed).
//! - **Normal force** (Ribner, *Propellers in yaw*, NACA Report 819, 1945): an in-plane
//!   force opposing the cross flow, ½ ρ V_r A c_n V⊥, with `normal_force` = c_n (C: from
//!   a blade-element calculation, or E:). Ahead of the centre of gravity it destabilises
//!   in pitch and yaw.
//!
//! Slipstream (momentum theory): axial speed and swirl, the swirl from the torque,
//! r v_θ = Q / ṁ far downstream and half that at the disc, with the same law along the
//! axis as the axial speed and a solid-body core near the hub (E:). It runs straight
//! back along the thrust axis (E:).

use std::f64::consts::{FRAC_PI_2, PI};

use glam::DVec3;

use crate::aero::polar::wrap_angle;
use crate::rigid_body::Wrench;

/// The advance angle and the reference speed use the blade speed at 0.7 R: 0.7 π n D.
const K07: f64 = 0.7 * PI;

/// Relative half-width of the slipstream's edge, over which it blends into the free
/// stream (E:, avoids load jumps when a strip sits on the edge).
const SLIPSTREAM_EDGE: f64 = 0.1;

/// Radius of the slipstream's swirl core, as a fraction of the tube radius (E:: the hub
/// and blade roots carry little load).
const SWIRL_CORE: f64 = 0.25;

/// Step of the energy check of the coefficients over the whole circle, rad.
const ENERGY_CHECK_STEP: f64 = 0.1 * PI / 180.0;

/// One point of the propeller table.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PropellerPoint {
    /// Advance ratio J = V / (n D).
    pub j: f64,
    pub ct: f64,
    pub cp: f64,
}

/// Four-quadrant coefficients C_T* and C_Q*.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuadrantPoint {
    pub ct: f64,
    pub cq: f64,
}

/// Estimated coefficients outside the table (see the module documentation).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuadrantAnchors {
    /// Stopped, flow from ahead (β = 90°): drag (C_T* < 0); the flow turns the propeller
    /// in its normal direction (C_Q* < 0).
    pub stopped: QuadrantPoint,
    /// Stopped, flow from behind (β = −90°): pushed forwards (C_T* > 0) and turned
    /// backwards (C_Q* > 0).
    pub reversed_flow: QuadrantPoint,
    /// Turning backwards in still air (β = ±180°): thrust backwards (C_T* < 0) and a
    /// torque opposing the backward rotation (C_Q* < 0).
    pub reversed_rotation: QuadrantPoint,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PropellerGeometry {
    /// Diameter, m.
    pub diameter: f64,
    /// Hub centre, body axes, relative to the centre of gravity, m.
    pub hub: DVec3,
    /// Unit vector of positive thrust, body axes (+x for a tractor or a pusher).
    pub thrust_axis: DVec3,
    /// Normal rotation is right-handed about the thrust axis (clockwise seen from the
    /// cockpit of a tractor); otherwise left-handed.
    pub right_handed: bool,
    /// P-factor: shift of the thrust line per unit of V⊥ V_blade / V_r², m (E:).
    pub p_factor: f64,
    /// Normal-force coefficient c_n: in-plane force ½ ρ V_r A c_n V⊥ (C: or E:).
    pub normal_force: f64,
}

/// Why a propeller was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PropellerError {
    TooFewPoints,
    NonFinite,
    NotStrictlyIncreasing,
    /// The table must start at J = 0 (static point, measured or computed).
    BadRange,
    /// The static point must pull and absorb power.
    NoStaticThrust,
    /// An anchor has the wrong sign (see [`QuadrantAnchors`]).
    BadAnchors,
    /// Somewhere on the circle, the propeller would give more power than it receives.
    CreatesEnergy,
    BadGeometry,
}

/// Loads of the propeller for one state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PropellerLoads {
    /// Thrust at the (P-factor shifted) hub, without the axial torque.
    pub wrench: Wrench,
    /// Thrust along the thrust axis, N.
    pub thrust: f64,
    /// Aerodynamic torque, positive when it opposes the normal rotation, N·m: the
    /// `propeller` term of [`crate::rigid_body::RotorTorques`].
    pub torque: f64,
    /// Axial inflow speed at the hub, m/s.
    pub axial_speed: f64,
    /// Advance angle β, rad.
    pub advance_angle: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Propeller {
    geometry: PropellerGeometry,
    table: Vec<PropellerPoint>,
    anchors: QuadrantAnchors,
}

impl Propeller {
    pub fn new(
        table: Vec<PropellerPoint>,
        anchors: QuadrantAnchors,
        geometry: PropellerGeometry,
    ) -> Result<Self, PropellerError> {
        if table.len() < 2 {
            return Err(PropellerError::TooFewPoints);
        }
        let a = anchors;
        let anchor_values = [
            a.stopped.ct,
            a.stopped.cq,
            a.reversed_flow.ct,
            a.reversed_flow.cq,
            a.reversed_rotation.ct,
            a.reversed_rotation.cq,
        ];
        if table
            .iter()
            .any(|p| ![p.j, p.ct, p.cp].iter().all(|v| v.is_finite()))
            || !anchor_values.iter().all(|v| v.is_finite())
        {
            return Err(PropellerError::NonFinite);
        }
        if table.windows(2).any(|w| w[1].j <= w[0].j) {
            return Err(PropellerError::NotStrictlyIncreasing);
        }
        if table[0].j != 0.0 {
            return Err(PropellerError::BadRange);
        }
        if !(table[0].ct > 0.0 && table[0].cp > 0.0) {
            return Err(PropellerError::NoStaticThrust);
        }
        if !(a.stopped.ct < 0.0
            && a.stopped.cq < 0.0
            && a.reversed_flow.ct > 0.0
            && a.reversed_flow.cq > 0.0
            && a.reversed_rotation.ct < 0.0
            && a.reversed_rotation.cq < 0.0)
        {
            return Err(PropellerError::BadAnchors);
        }
        let g = geometry;
        if !(g.diameter.is_finite()
            && g.diameter > 0.0
            && g.hub.is_finite()
            && g.thrust_axis.is_finite()
            && (g.thrust_axis.length() - 1.0).abs() < 1e-9
            && g.p_factor.is_finite()
            && g.p_factor >= 0.0
            && g.normal_force.is_finite()
            && g.normal_force >= 0.0)
        {
            return Err(PropellerError::BadGeometry);
        }
        let propeller = Self {
            geometry,
            table,
            anchors,
        };
        let steps = (2.0 * PI / ENERGY_CHECK_STEP).ceil() as i32;
        let creates_energy = (0..=steps).any(|i| {
            let beta = -PI + f64::from(i) * ENERGY_CHECK_STEP;
            let c = propeller.coefficients(beta);
            // (Q Ω − T V_a) / (½ ρ V_r³ A), with Ω = 2 V_r cos β / (0.7 D).
            c.cq * beta.cos() / 0.35 - c.ct * beta.sin() < -1e-12
        });
        if creates_energy {
            return Err(PropellerError::CreatesEnergy);
        }
        Ok(propeller)
    }

    pub fn geometry(&self) -> &PropellerGeometry {
        &self.geometry
    }

    /// Axis about which the normal rotation is positive: the rotor axis of
    /// [`crate::rigid_body::Rotor`].
    pub fn rotation_axis(&self) -> DVec3 {
        if self.geometry.right_handed {
            self.geometry.thrust_axis
        } else {
            -self.geometry.thrust_axis
        }
    }

    /// Disc area, m².
    pub fn disc_area(&self) -> f64 {
        0.25 * PI * self.geometry.diameter.powi(2)
    }

    /// Four-quadrant coefficients from a table point.
    fn starred(j: f64, ct: f64, cp: f64) -> QuadrantPoint {
        let scale = 8.0 / (PI * (j * j + K07 * K07));
        QuadrantPoint {
            ct: scale * ct,
            cq: scale * cp / (2.0 * PI),
        }
    }

    fn table_point(&self, i: usize) -> QuadrantPoint {
        let p = self.table[i];
        Self::starred(p.j, p.ct, p.cp)
    }

    /// C_T* and C_Q* at any advance angle (radians, wrapped to ±180°).
    pub fn coefficients(&self, advance_angle: f64) -> QuadrantPoint {
        let beta = wrap_angle(advance_angle);
        let last = self.table.len() - 1;
        let beta_max = (self.table[last].j / K07).atan();
        let lerp = |a: QuadrantPoint, b: QuadrantPoint, t: f64| QuadrantPoint {
            ct: a.ct + t * (b.ct - a.ct),
            cq: a.cq + t * (b.cq - a.cq),
        };
        let anchors = &self.anchors;
        if (0.0..=beta_max).contains(&beta) {
            // Inside the table: linear in J, like the table itself.
            let j = K07 * beta.tan();
            let i = self.table.partition_point(|p| p.j <= j).clamp(1, last);
            let (a, b) = (self.table[i - 1], self.table[i]);
            let t = (j - a.j) / (b.j - a.j);
            return Self::starred(j, a.ct + t * (b.ct - a.ct), a.cp + t * (b.cp - a.cp));
        }
        if beta > beta_max && beta <= FRAC_PI_2 {
            let t = (beta - beta_max) / (FRAC_PI_2 - beta_max);
            lerp(self.table_point(last), anchors.stopped, t)
        } else if beta > FRAC_PI_2 {
            let t = (beta - FRAC_PI_2) / FRAC_PI_2;
            lerp(anchors.stopped, anchors.reversed_rotation, t)
        } else if beta >= -FRAC_PI_2 {
            lerp(
                self.table_point(0),
                anchors.reversed_flow,
                -beta / FRAC_PI_2,
            )
        } else {
            let t = (-FRAC_PI_2 - beta) / FRAC_PI_2;
            lerp(anchors.reversed_flow, anchors.reversed_rotation, t)
        }
    }

    /// Thrust, P-factor and aerodynamic torque.
    ///
    /// `air_velocity`: velocity of the centre of gravity relative to the air, body axes;
    /// `rotor_speed`: **absolute** axial speed of the rotor about [`Self::rotation_axis`],
    /// rad/s ([`crate::rigid_body::RigidBodyState::rotor_speed`]).
    pub fn loads(
        &self,
        air_velocity: DVec3,
        angular_velocity: DVec3,
        rotor_speed: f64,
        density: f64,
    ) -> PropellerLoads {
        let g = &self.geometry;
        let axis = g.thrust_axis;
        let v = air_velocity + angular_velocity.cross(g.hub);
        let axial = v.dot(axis);
        let cross_flow = v - axis * axial;
        let blade = K07 * rotor_speed / (2.0 * PI) * g.diameter;
        let beta = axial.atan2(blade);
        let c = self.coefficients(beta);
        let force_scale = 0.5 * density * (axial * axial + blade * blade) * self.disc_area();
        let thrust = c.ct * force_scale;
        let torque = c.cq * force_scale * g.diameter;

        let reference_sq = axial * axial + blade * blade;
        let (offset, normal) = if reference_sq > 0.0 {
            let sense = if g.right_handed { 1.0 } else { -1.0 };
            let offset = cross_flow.cross(axis) * (sense * g.p_factor * blade / reference_sq);
            let normal = -cross_flow
                * (0.5 * density * reference_sq.sqrt() * self.disc_area() * g.normal_force);
            (offset, normal)
        } else {
            (DVec3::ZERO, DVec3::ZERO)
        };
        PropellerLoads {
            wrench: Wrench::force_at(axis * thrust, g.hub + offset)
                + Wrench::force_at(normal, g.hub),
            thrust,
            torque,
            axial_speed: axial,
            advance_angle: beta,
        }
    }

    /// Slipstream of the propeller for these loads (momentum theory).
    pub fn slipstream(&self, loads: &PropellerLoads, density: f64) -> Slipstream {
        let free_stream = loads.axial_speed.max(0.0);
        let induced = if loads.thrust > 0.0 && density > 0.0 {
            let disc_loading = 2.0 * loads.thrust / (density * self.disc_area());
            0.5 * ((free_stream * free_stream + disc_loading).sqrt() - free_stream)
        } else {
            // Windmilling (braking) states are outside simple momentum theory.
            0.0
        };
        // Angular momentum flux: Q = ṁ r v_θ far downstream.
        let mass_flow = density * self.disc_area() * (free_stream + induced);
        let swirl = if induced > 0.0 {
            loads.torque / mass_flow
        } else {
            0.0
        };
        Slipstream {
            hub: self.geometry.hub,
            axis: self.geometry.thrust_axis,
            rotation_axis: self.rotation_axis(),
            radius: 0.5 * self.geometry.diameter,
            free_stream,
            induced,
            swirl,
        }
    }
}

/// Slipstream from Froude's momentum theory (`modele-aero.md`): induced speed v_i at the
/// disc, v(x) = v_i (1 + x / √(x² + R²)) at a distance x behind it (2 v_i far
/// downstream), in a stream tube that contracts so as to conserve the mass flow; swirl
/// r v_θ = K (1 + x / √(x² + R²)) / 2, with K = Q / ṁ, turning with the propeller.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Slipstream {
    hub: DVec3,
    axis: DVec3,
    rotation_axis: DVec3,
    radius: f64,
    free_stream: f64,
    induced: f64,
    /// K = r v_θ far downstream, m²/s.
    swirl: f64,
}

impl Slipstream {
    /// Induced axial speed at the disc, m/s.
    pub fn induced_speed(&self) -> f64 {
        self.induced
    }

    /// r v_θ of the swirl far downstream, m²/s (positive: turning with the propeller).
    pub fn swirl(&self) -> f64 {
        self.swirl
    }

    /// Induced axial speed at `distance` behind the disc (negative ahead of it), m/s.
    pub fn axial_speed_at(&self, distance: f64) -> f64 {
        let r = self.radius;
        self.induced * (1.0 + distance / (distance * distance + r * r).sqrt())
    }

    /// Radius of the stream tube at `distance` behind the disc, m.
    pub fn tube_radius(&self, distance: f64) -> f64 {
        let ratio =
            (self.free_stream + self.induced) / (self.free_stream + self.axial_speed_at(distance));
        self.radius * ratio.sqrt()
    }

    /// Velocity of the air relative to the free stream at `point` (body axes, relative
    /// to the centre of gravity): the `local_flow` of the surfaces and drag items.
    pub fn local_flow(&self, point: DVec3) -> DVec3 {
        if self.induced <= 0.0 {
            return DVec3::ZERO;
        }
        let d = point - self.hub;
        let along = d.dot(self.axis);
        let distance = -along;
        let radial = (d - self.axis * along).length();
        let edge = (radial / self.tube_radius(distance) - (1.0 - SLIPSTREAM_EDGE))
            / (2.0 * SLIPSTREAM_EDGE);
        let weight = 1.0 - smoothstep(edge);
        let along_axis = self.axial_speed_at(distance) / (2.0 * self.induced);
        // Free vortex outside the core, solid-body rotation inside.
        let core = SWIRL_CORE * self.tube_radius(distance);
        let circulation = self.swirl * along_axis;
        let tangential = if radial > 0.0 {
            let speed = if radial >= core {
                circulation / radial
            } else {
                circulation * radial / (core * core)
            };
            self.rotation_axis.cross(d - self.axis * along) * (speed / radial)
        } else {
            DVec3::ZERO
        };
        (-self.axis * self.axial_speed_at(distance) + tangential) * weight
    }
}

/// 0 below 0, 1 above 1, C¹ in between.
fn smoothstep(x: f64) -> f64 {
    let s = x.clamp(0.0, 1.0);
    s * s * (3.0 - 2.0 * s)
}

#[cfg(test)]
pub(crate) mod tests;
