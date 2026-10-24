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
//! Slipstream (momentum theory): axial speed and swirl. Lifting strips feel less of it than
//! strip theory says, by the jet-width factor of R&M 2368 ([`Slipstream::lifting_flow`]).
//! The swirl comes from the torque:
//! r v_θ is conserved along a streamline of an inviscid axisymmetric flow, so it is zero
//! ahead of the disc and K = Q / ṁ just behind it (blended over ±5 % of the radius, C¹),
//! with a solid-body core near the hub (E:). Behind the wings only a fraction of it is
//! left ([`WingWake`]). The induced axial speed follows Froude on both sides of T = 0
//! (slowed air behind a braking propeller, held at −V/2), and the swirl does not depend on
//! the thrust's sign: a windmilling propeller turns the air against its rotation. The
//! slipstream runs straight back along the thrust axis (E:).

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

/// Speed through the disc below which the swirl fades out, m/s (E:: momentum theory has
/// no stream tube when the flow through the disc stops).
const SWIRL_FADE_SPEED: f64 = 2.0;

/// Half-width of the step of the swirl across the disc, fraction of the radius (C¹).
const SWIRL_STEP: f64 = 0.05;

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
        // Froude, T = 2 ρ A (V + v_i) v_i, on both sides of T = 0: a braking propeller
        // slows the air behind it (v_i < 0). At v_i = −V/2 the far wake would stop and
        // momentum theory fails (turbulent wake state): v_i is held there (E:).
        let induced = if density > 0.0 {
            let disc_loading = 2.0 * loads.thrust / (density * self.disc_area());
            0.5 * ((free_stream * free_stream + disc_loading).max(0.0).sqrt() - free_stream)
        } else {
            0.0
        };
        // Angular momentum flux: Q = ṁ r v_θ far downstream, whatever the sign of the
        // thrust (a windmilling propeller still turns the air, against its rotation):
        // the swirl must not jump when the thrust crosses zero (aerodynamicist review of
        // 2026-10-24: it did, and no idle glide could be trimmed there). Faded out as the
        // flow through the disc stops, where K = Q / ṁ would blow up.
        let through = free_stream + induced;
        let swirl = if through > 0.0 && density > 0.0 {
            loads.torque / (density * self.disc_area() * through)
                * smoothstep(through / SWIRL_FADE_SPEED)
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
            wing_wake: None,
        }
    }
}

/// Wake of the wings in the slipstream: behind them only a fraction of the swirl is left,
/// the wings having straightened the rest (NACA Report 747, NACA TN 1146).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WingWake {
    /// Distance of the wings' trailing edge behind the disc, m: the swirl falls from there.
    pub start: f64,
    /// Distance over which it falls to `swirl_left` (the wings' mean chord), m.
    pub length: f64,
    /// Fraction of the swirl left behind the wings, 0 to 1.
    pub swirl_left: f64,
}

/// Slipstream from Froude's momentum theory (`modele-aero.md`): induced speed v_i at the
/// disc, v(x) = v_i (1 + x / √(x² + R²)) at a distance x behind it (2 v_i far
/// downstream), in a stream tube that contracts so as to conserve the mass flow; swirl
/// r v_θ = K behind the disc and 0 ahead of it, with K = Q / ṁ, turning with the propeller.
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
    wing_wake: Option<WingWake>,
}

/// Largest expansion of the stream tube's area behind a braking propeller: momentum
/// theory's at v_i = −0.4 V, where the turbulent wake state begins (Glauert; E:).
const MAX_TUBE_EXPANSION: f64 = 3.0;

impl Slipstream {
    /// The same slipstream, its swirl partly straightened behind the wings.
    pub fn behind_wings(self, wake: WingWake) -> Self {
        Self {
            wing_wake: Some(wake),
            ..self
        }
    }

    /// The same slipstream without its swirl (diagnostics: what the swirl does).
    pub fn without_swirl(self) -> Self {
        Self { swirl: 0.0, ..self }
    }

    /// Kinetic energy carried by the swirl through the disc, W: ½ ρ ∫ v_θ² (V + v_i) dA
    /// over the free vortex and its solid-body core, π ρ (V + v_i) K² (ln(1/c) + ¼) with
    /// c = [`SWIRL_CORE`] (the blend at the tube's edge left out). What the airframe can
    /// take back from the swirl as thrust is bounded by it.
    pub fn rotational_power(&self, density: f64) -> f64 {
        let through = (self.free_stream + self.induced).max(0.0);
        PI * density * through * self.swirl * self.swirl * ((1.0 / SWIRL_CORE).ln() + 0.25)
    }

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

    /// Radius of the stream tube at `distance` behind the disc, m; its area grows at most
    /// [`MAX_TUBE_EXPANSION`] times behind a braking propeller.
    pub fn tube_radius(&self, distance: f64) -> f64 {
        let through = self.free_stream + self.induced;
        let ratio = through / (self.free_stream + self.axial_speed_at(distance));
        let ratio = if ratio.is_finite() && ratio >= 0.0 {
            ratio.min(MAX_TUBE_EXPANSION)
        } else {
            MAX_TUBE_EXPANSION
        };
        self.radius * ratio.sqrt()
    }

    /// Axial speed increment and swirl velocity of the slipstream at `point` (body axes,
    /// relative to the centre of gravity), both blended into the free stream at the
    /// tube's edge, with the distance behind the disc; `None` with no flow through the
    /// disc (no stream tube, and no swirl either).
    fn parts(&self, point: DVec3) -> Option<FlowParts> {
        if self.free_stream + self.induced <= 0.0 {
            return None;
        }
        let d = point - self.hub;
        let along = d.dot(self.axis);
        let distance = -along;
        let radial = (d - self.axis * along).length();
        let edge = (radial / self.tube_radius(distance) - (1.0 - SLIPSTREAM_EDGE))
            / (2.0 * SLIPSTREAM_EDGE);
        let weight = 1.0 - smoothstep(edge);
        let behind = smoothstep(distance / (2.0 * SWIRL_STEP * self.radius) + 0.5);
        // Free vortex outside the core, solid-body rotation inside.
        let core = SWIRL_CORE * self.tube_radius(distance);
        let straightened = self.wing_wake.map_or(1.0, |w| {
            1.0 - (1.0 - w.swirl_left) * smoothstep((distance - w.start) / w.length)
        });
        let circulation = self.swirl * behind * straightened;
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
        Some(FlowParts {
            axial: self.axial_speed_at(distance) * weight,
            tangential: tangential * weight,
            distance,
        })
    }

    /// Velocity of the air relative to the free stream at `point` (body axes, relative
    /// to the centre of gravity): the `local_flow` of drag items and bodies, which feel
    /// the whole slipstream.
    pub fn local_flow(&self, point: DVec3) -> DVec3 {
        self.parts(point)
            .map_or(DVec3::ZERO, |p| -self.axis * p.axial + p.tangential)
    }

    /// The slipstream as a lifting strip responds to it, at its quarter-chord `point`, for
    /// its `chord` (m) and section lift slope `section_slope` (per radian).
    ///
    /// A wing crossing a jet of finite width gains less lift than strip theory says: the
    /// jet's edge sheds vortices whose images in the jet reduce the extra lift. Squire and
    /// Chester (R&M 2368, 1945, part I, table 2) give the ratio η of the true increment
    /// to strip theory's for a wing of infinite span ([`jet_width_factor`]). It is applied
    /// to what the slipstream adds to the strip's lift:
    /// - the dynamic pressure: the strip sees the speed v_e with
    ///   v_e² − V² = η (v² − V²), V the free-stream axial speed and v the jet's;
    /// - the swirl incidence: the tangential velocity is scaled by η v / v_e, so that the
    ///   lift it adds (∝ v_e × tangential) is η times strip theory's.
    ///
    /// Applying the factor of a jet without rotation to the swirl is an assumption (E:),
    /// checked against the powered model of NACA TN 1146 (`fdm_lab souffle`).
    pub fn lifting_flow(&self, point: DVec3, chord: f64, section_slope: f64) -> DVec3 {
        let Some(p) = self.parts(point) else {
            return DVec3::ZERO;
        };
        let (free, jet) = (self.free_stream, self.free_stream + p.axial);
        let narrowness = section_slope * chord / (8.0 * PI * self.tube_radius(p.distance));
        let eta = jet_width_factor(free / jet, narrowness);
        let effective = (free * free + eta * (jet * jet - free * free))
            .max(0.0)
            .sqrt();
        let tangential = if effective > 0.0 {
            p.tangential * (eta * jet / effective)
        } else {
            DVec3::ZERO
        };
        -self.axis * (effective - free) + tangential
    }
}

/// Axial and swirl parts of the slipstream at a point.
#[derive(Debug, Clone, Copy, PartialEq)]
struct FlowParts {
    /// Axial speed increment over the free stream, m/s.
    axial: f64,
    tangential: DVec3,
    /// Distance behind the disc, m.
    distance: f64,
}

/// Speed ratios V/v of the rows of [`JET_FACTORS`]: R&M 2368, table 2 (V/v = 0, 0.5 and
/// 0.833, i.e. v/V = 1.2), and 1, where the increment is strip theory's (Koning, quoted by
/// R&M 2368, p. 7).
const JET_SPEED_RATIOS: [f64; 4] = [0.0, 0.5, 1.0 / 1.2, 1.0];

/// Narrowness 1/μ = a₀ c / (8π R) of the columns of [`JET_FACTORS`]: μ = ∞, 5, 2.5 and 1.
const JET_NARROWNESS: [f64; 4] = [0.0, 0.2, 0.4, 1.0];

/// ΔL / ΔL_strip, R&M 2368, table 2 (S:), with a first column of 1 for a jet infinitely
/// wider than the chord, where strip theory holds (E:: the table stops at μ = 5).
const JET_FACTORS: [[f64; 4]; 4] = [
    [1.0, 0.4081, 0.2628, 0.1257],
    [1.0, 0.6225, 0.4880, 0.3062],
    [1.0, 0.8863, 0.7861, 0.5685],
    [1.0, 1.0, 1.0, 1.0],
];

/// Jet-width factor η of R&M 2368 (Squire and Chester, 1945, part I, table 2): extra lift
/// of a wing of infinite span crossing a circular jet, over the extra lift of strip
/// theory, for the speed ratio `speed_ratio` = V/v of the stream to the jet and the
/// narrowness a₀ c / (8π R) = 1/μ of the jet (section lift slope a₀, chord c, jet radius
/// R).
///
/// Bilinear in V/v and 1/μ. Beyond the table: a jet narrower than μ = 1 keeps the factor of
/// μ = 1 (E:); a jet slower than the stream (V/v > 1, braking propeller) takes the factor
/// of the inverse ratio, its images having the same strength with the opposite sign (E:).
pub fn jet_width_factor(speed_ratio: f64, narrowness: f64) -> f64 {
    let ratio = if speed_ratio > 1.0 {
        1.0 / speed_ratio
    } else {
        speed_ratio
    };
    if !(ratio.is_finite() && narrowness.is_finite()) {
        return 1.0;
    }
    let locate = |grid: &[f64; 4], x: f64| {
        let x = x.clamp(grid[0], grid[3]);
        let i = grid.partition_point(|&g| g <= x).clamp(1, 3);
        (i, (x - grid[i - 1]) / (grid[i] - grid[i - 1]))
    };
    let (row, t) = locate(&JET_SPEED_RATIOS, ratio.max(0.0));
    let (column, u) = locate(&JET_NARROWNESS, narrowness.max(0.0));
    let at = |r: usize| {
        let line = &JET_FACTORS[r];
        line[column - 1] + u * (line[column] - line[column - 1])
    };
    at(row - 1) + t * (at(row) - at(row - 1))
}

/// 0 below 0, 1 above 1, C¹ in between.
fn smoothstep(x: f64) -> f64 {
    let s = x.clamp(0.0, 1.0);
    s * s * (3.0 - 2.0 * s)
}

#[cfg(test)]
pub(crate) mod tests;
