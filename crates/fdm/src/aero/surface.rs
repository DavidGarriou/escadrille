//! Lifting surfaces split into spanwise strips (skill `simulation-vol`, `modele-aero.md`).
//!
//! For each strip: local air velocity v + ω × r, angle of attack in the strip's
//! chord–normal plane (the spanwise component is ignored), section coefficients from
//! the surface's 360° polar, lift perpendicular to the local in-plane velocity.
//!
//! Finite span is handled once, as an induced angle α_i = k C_L(α − α_i), k = 1 / (π e AR),
//! solved exactly with a bracketed root finder (Illinois method), and the force is tilted
//! back by α_i. In the linear range this gives the classical C_L = a₀ α / (1 + a₀ k) and
//! C_Di = k C_L². A fixed-point iteration would diverge when a₀ k > 1, i.e. e·AR < 2: every
//! fin and rudder of 1914-18 (aerodynamicist review, 2026-10-20).
//!
//! The exact solve is done between the two stall points of the table (maximum and
//! minimum C_L), where x + k C_L(x) is monotone, so that the solution is unique: checked
//! by [`LiftingSurface::new`], which requires every table slope there to exceed −1/k
//! (a dip before the stall, as in noisy data or a double stall, can break it at low
//! aspect ratio). Past the stall,
//! the equation may have several solutions (a lift-curve slope steeper than −1/k, common
//! on low aspect ratio fins), which would make the loads jump; and lifting-line induction
//! makes no sense in separated or reversed flow anyway. So beyond the stall the induced
//! angle keeps its stall value and fades linearly to zero at [`INDUCTION_FADE_END`] of
//! geometric incidence (or [`INDUCTION_FADE_MARGIN`] past the stall, whichever is larger).
//! Continuous, monotone, with no hysteresis; an estimate (E:) for the post-stall region,
//! accepted for M1 by the aerodynamicist (review of 2026-10-20). Reserve: at low aspect
//! ratio this makes the stall steeper than the section's (by a factor of about 2 at
//! AR 1.5), whereas a real low aspect ratio surface stalls gently; to revisit when fins
//! are made to stall (spins), with a wider fade or measured 3D polars for the fins.
//!
//! The planes of a biplane also induce on each other: an extra induced angle `mutual`,
//! computed by [`super::system::LiftingSystem`], shifts the target of the solve and
//! fades out past the stall with the surface's own induction.
//!
//! Known limits (E:, confidence C): one k per surface is exact only for a symmetric
//! loading; for antisymmetric loading (roll damping, ailerons) lifting-line theory gives a
//! lower slope, so the roll damping is overestimated (about 27 % for AR 5.5) and all strips
//! stall together. The target is a nonlinear lifting line with an influence matrix
//! (Phillips & Snyder, J. Aircraft 37(4), 2000), which will also give the biplane
//! interference and the downwash on the tail. Below AR ≈ 4, lifting-line theory
//! overestimates the lift slope (Helmbold or DATCOM corrections belong in the data, C:).

use std::f64::consts::PI;

use glam::DVec3;

use super::polar::{Coefficients, Polar, wrap_angle};
use crate::rigid_body::Wrench;

/// Pilot control channels, normalised commands in [−1, 1] are converted to surface
/// deflections (radians) by the aircraft model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlChannel {
    Aileron,
    Elevator,
    Rudder,
}

/// Deflections of the control surfaces, rad, positive in the direction of the pilot's
/// positive command (`modele-aero.md`): ailerons = roll right, elevator = nose up,
/// rudder = nose right.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct ControlDeflections {
    pub aileron: f64,
    pub elevator: f64,
    pub rudder: f64,
}

impl ControlDeflections {
    fn get(&self, channel: ControlChannel) -> f64 {
        match channel {
            ControlChannel::Aileron => self.aileron,
            ControlChannel::Elevator => self.elevator,
            ControlChannel::Rudder => self.rudder,
        }
    }
}

/// Effect of a control surface on a strip: the deflection shifts the strip's effective
/// angle of attack by `alpha_per_rad` × deflection (τ × sign, the sign saying which way
/// a positive command moves this strip's lift).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StripControl {
    pub channel: ControlChannel,
    pub alpha_per_rad: f64,
}

/// Flap effectiveness τ of a plain flap of chord ratio `chord_ratio` (thin-aerofoil
/// theory, Glauert): τ = 1 − (θ_f − sin θ_f) / π, with cos θ_f = 2 E − 1.
pub fn flap_effectiveness(chord_ratio: f64) -> f64 {
    let e = chord_ratio.clamp(0.0, 1.0);
    let theta = (2.0 * e - 1.0).acos();
    1.0 - (theta - theta.sin()) / PI
}

/// One spanwise strip.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Strip {
    /// Quarter-chord point, body axes, relative to the centre of gravity, m.
    pub position: DVec3,
    /// Unit vector along the chord, from trailing to leading edge (includes incidence).
    pub chord_axis: DVec3,
    /// Unit vector normal to the strip, on the side where positive lift acts.
    pub normal: DVec3,
    /// Planform area, m².
    pub area: f64,
    /// Chord, m.
    pub chord: f64,
    pub control: Option<StripControl>,
}

/// Finite-span correction of a surface.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InducedModel {
    /// Effective aspect ratio (biplane interference included by the aircraft data).
    aspect_ratio: f64,
    /// Span efficiency e.
    efficiency: f64,
}

/// Smallest e·AR accepted: below it, lifting-line induction is meaningless and the
/// post-stall fade could run past ±180°.
pub const MIN_EFFECTIVE_ASPECT_RATIO: f64 = 0.5;

impl InducedModel {
    /// `None` unless 0 < e ≤ 1 and e·AR ≥ [`MIN_EFFECTIVE_ASPECT_RATIO`].
    pub fn new(aspect_ratio: f64, efficiency: f64) -> Option<Self> {
        let valid = aspect_ratio.is_finite()
            && efficiency.is_finite()
            && efficiency > 0.0
            && efficiency <= 1.0
            && efficiency * aspect_ratio >= MIN_EFFECTIVE_ASPECT_RATIO;
        valid.then_some(Self {
            aspect_ratio,
            efficiency,
        })
    }

    /// Induced-angle factor k = 1 / (π e AR).
    pub fn factor(&self) -> f64 {
        1.0 / (PI * self.efficiency * self.aspect_ratio)
    }
}

/// Why a lifting surface was rejected.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceError {
    NoStrips,
    /// Non-finite position, axes not unit or not orthogonal, area or chord not positive.
    BadStrip,
    /// A table slope between the stall points is below −1/k: the induced angle would
    /// have several solutions (see the module documentation).
    InductionNotMonotone,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiftingSurface {
    name: String,
    strips: Vec<Strip>,
    polar: Polar,
    induced: Option<InducedModel>,
    section_slope: f64,
}

/// Induction is faded out completely beyond this geometric angle of attack (rad), unless
/// the stall is close to it (then [`INDUCTION_FADE_MARGIN`] past the stall).
pub const INDUCTION_FADE_END: f64 = 40.0 * PI / 180.0;

/// Smallest fade of the induction past the stall (rad).
pub const INDUCTION_FADE_MARGIN: f64 = 15.0 * PI / 180.0;

/// Lift slope of thin-aerofoil theory, per radian.
const THIN_AEROFOIL_SLOPE: f64 = 2.0 * PI;

/// Maximum iterations of the induced-angle root finder (converges in a few; the cap
/// keeps the cost bounded and the result deterministic).
const SOLVER_MAX_ITERATIONS: usize = 60;

/// Residual (rad) below which the induced-angle root finder stops.
const SOLVER_TOLERANCE: f64 = 1e-13;

/// Below this local airspeed, a strip produces no force (avoids 0/0 at rest).
const MIN_STRIP_SPEED: f64 = 1e-3;

/// Diagnostic output of one strip, for telemetry, gizmos and tests.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct StripState {
    /// Geometric angle of attack (control shift included), rad.
    pub alpha: f64,
    /// Induced angle, rad.
    pub induced_alpha: f64,
    pub coefficients: Coefficients,
    pub dynamic_pressure: f64,
}

impl LiftingSurface {
    pub fn new(
        name: String,
        strips: Vec<Strip>,
        polar: Polar,
        induced: Option<InducedModel>,
    ) -> Result<Self, SurfaceError> {
        if strips.is_empty() {
            return Err(SurfaceError::NoStrips);
        }
        let unit = |v: DVec3| (v.length() - 1.0).abs() < 1e-9;
        if strips.iter().any(|s| {
            !(s.position.is_finite()
                && unit(s.chord_axis)
                && unit(s.normal)
                && s.chord_axis.dot(s.normal).abs() < 1e-9
                && s.area.is_finite()
                && s.area > 0.0
                && s.chord.is_finite()
                && s.chord > 0.0
                && s.control.is_none_or(|c| c.alpha_per_rad.is_finite()))
        }) {
            return Err(SurfaceError::BadStrip);
        }
        if let Some(model) = induced {
            let min_slope = -1.0 / model.factor();
            let (neg, pos) = polar.stall_points();
            let points = polar.table_points();
            let monotone = points.windows(2).all(|w| {
                w[1].alpha <= neg.alpha
                    || w[0].alpha >= pos.alpha
                    || (w[1].cl - w[0].cl) / (w[1].alpha - w[0].alpha) > min_slope
            });
            if !monotone {
                return Err(SurfaceError::InductionNotMonotone);
            }
        }
        let section_slope = polar.zero_lift_slope().unwrap_or(THIN_AEROFOIL_SLOPE);
        Ok(Self {
            name,
            strips,
            polar,
            induced,
            section_slope,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn strips(&self) -> &[Strip] {
        &self.strips
    }

    pub fn polar(&self) -> &Polar {
        &self.polar
    }

    pub fn induced(&self) -> Option<InducedModel> {
        self.induced
    }

    /// Section lift slope at zero lift, per radian (2π if the table's lift does not
    /// change sign between its stall points): sets how much of the slipstream the strips
    /// feel ([`crate::propulsion::propeller::Slipstream::lifting_flow`]).
    pub fn section_slope(&self) -> f64 {
        self.section_slope
    }

    /// Aerodynamic force and moment about the centre of gravity, body axes.
    ///
    /// `air_velocity` is the velocity of the centre of gravity relative to the free-stream
    /// air, body axes; `local_flow(strip)` is the extra air velocity at the strip's
    /// quarter-chord point (wing downwash on the tail, propeller slipstream, ground
    /// effect…), body axes, given the whole strip because the slipstream's effect depends
    /// on its chord; `density` in kg/m³.
    pub fn loads(
        &self,
        air_velocity: DVec3,
        angular_velocity: DVec3,
        density: f64,
        controls: &ControlDeflections,
        local_flow: impl Fn(&Strip) -> DVec3,
    ) -> Wrench {
        self.loads_with_mutual(
            air_velocity,
            angular_velocity,
            density,
            controls,
            local_flow,
            0.0,
        )
        .0
    }

    /// Same as [`Self::loads`], with an extra induced angle `mutual` (rad) coming from
    /// the other planes of a biplane ([`super::system::LiftingSystem`]); also returns
    /// the surface's lift coefficient, weighted by each strip's dynamic pressure and area.
    pub fn loads_with_mutual(
        &self,
        air_velocity: DVec3,
        angular_velocity: DVec3,
        density: f64,
        controls: &ControlDeflections,
        local_flow: impl Fn(&Strip) -> DVec3,
        mutual: f64,
    ) -> (Wrench, f64) {
        let (mut wrench, mut lift, mut weight) = (Wrench::default(), 0.0, 0.0);
        for strip in &self.strips {
            let v = air_velocity + angular_velocity.cross(strip.position) - local_flow(strip);
            let (w, state) = self.strip_loads(strip, v, density, controls, mutual);
            wrench += w;
            lift += state.coefficients.cl * state.dynamic_pressure * strip.area;
            weight += state.dynamic_pressure * strip.area;
        }
        let cl = if weight > 0.0 { lift / weight } else { 0.0 };
        (wrench, cl)
    }

    /// Effective angle of attack x = α − α_i for geometric angle of attack α (rad),
    /// own-induction factor k and induced angle `mutual` from the other planes.
    fn effective_alpha(&self, alpha: f64, k: f64, mutual: f64) -> f64 {
        let alpha = wrap_angle(alpha);
        let target = alpha - mutual;
        let (neg, pos) = self.polar.stall_points();
        // Values of α − mutual at which each stall is reached.
        let (alpha_neg, alpha_pos) = (neg.alpha + k * neg.cl, pos.alpha + k * pos.cl);
        if target > alpha_pos || target < alpha_neg {
            // Total induced angle at the stall, faded with the geometric incidence,
            // worked on the positive side (mirrored for the negative stall).
            let (side, stall, induced) = if target > alpha_pos {
                (1.0, alpha_pos + mutual, k * pos.cl + mutual)
            } else {
                (-1.0, alpha_neg + mutual, k * neg.cl + mutual)
            };
            let (a, stall) = (side * alpha, side * stall);
            // Bounded by 180°, so that the induction is zero there from both sides.
            let end = INDUCTION_FADE_END
                .max(stall + INDUCTION_FADE_MARGIN)
                .min(PI);
            let weight = if end > stall {
                ((end - a) / (end - stall)).clamp(0.0, 1.0)
            } else {
                0.0
            };
            return alpha - induced * weight;
        }
        // Attached flow: solve x + k C_L(x) = α − mutual on [x_neg, x_pos], where the
        // residual goes from ≤ 0 to ≥ 0 (Illinois variant of regula falsi).
        let residual = |x: f64| x + k * self.polar.coefficients(x).cl - target;
        let (mut lo, mut hi) = (neg.alpha, pos.alpha);
        let (mut f_lo, mut f_hi) = (residual(lo), residual(hi));
        let mut x = target;
        let mut last_side = 0;
        for _ in 0..SOLVER_MAX_ITERATIONS {
            if f_hi - f_lo <= 0.0 {
                break;
            }
            x = (lo * f_hi - hi * f_lo) / (f_hi - f_lo);
            let f = residual(x);
            if f.abs() < SOLVER_TOLERANCE || hi - lo < SOLVER_TOLERANCE {
                break;
            }
            if f < 0.0 {
                (lo, f_lo) = (x, f);
                if last_side == -1 {
                    f_hi *= 0.5; // Illinois step: avoid a stagnant end point.
                }
                last_side = -1;
            } else {
                (hi, f_hi) = (x, f);
                if last_side == 1 {
                    f_lo *= 0.5;
                }
                last_side = 1;
            }
        }
        x
    }

    /// Loads and diagnostics of one strip, for `v` the strip's velocity relative to the
    /// local air (body axes) and `mutual` the induced angle from the other planes (rad).
    pub fn strip_loads(
        &self,
        strip: &Strip,
        v: DVec3,
        density: f64,
        controls: &ControlDeflections,
        mutual: f64,
    ) -> (Wrench, StripState) {
        let (u_c, u_n) = (v.dot(strip.chord_axis), v.dot(strip.normal));
        let speed_sq = u_c * u_c + u_n * u_n;
        if speed_sq < MIN_STRIP_SPEED * MIN_STRIP_SPEED {
            return (Wrench::default(), StripState::default());
        }
        let speed = speed_sq.sqrt();
        let control_shift = strip
            .control
            .map_or(0.0, |c| c.alpha_per_rad * controls.get(c.channel));
        let alpha = wrap_angle((-u_n).atan2(u_c) + control_shift);

        let effective = self.induced.map_or(alpha, |model| {
            self.effective_alpha(alpha, model.factor(), mutual)
        });
        let induced = wrap_angle(alpha - effective);
        let coefficients = self.polar.coefficients(effective);

        // Directions in the chord–normal plane: drag along the local flow, lift
        // perpendicular to it (equal to `normal` when the flow is along the chord),
        // both tilted back by the induced angle.
        let drag_dir = -(strip.chord_axis * u_c + strip.normal * u_n) / speed;
        let lift_dir = (strip.normal * u_c - strip.chord_axis * u_n) / speed;
        let (s, c) = induced.sin_cos();
        let (lift_dir, drag_dir) = (lift_dir * c + drag_dir * s, drag_dir * c - lift_dir * s);

        let q = 0.5 * density * speed_sq;
        let force = (lift_dir * coefficients.cl + drag_dir * coefficients.cd) * (q * strip.area);
        let pitch_axis = strip.chord_axis.cross(strip.normal);
        let mut wrench = Wrench::force_at(force, strip.position);
        wrench.moment += pitch_axis * (q * strip.area * strip.chord * coefficients.cm);
        let state = StripState {
            alpha,
            induced_alpha: induced,
            coefficients,
            dynamic_pressure: q,
        };
        (wrench, state)
    }
}

#[cfg(test)]
mod tests;
