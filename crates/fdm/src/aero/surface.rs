//! Lifting surfaces split into spanwise strips (skill `simulation-vol`, `modele-aero.md`).
//!
//! For each strip: local air velocity v + ω × r, angle of attack in the strip's
//! chord–normal plane (the spanwise component is ignored), section coefficients from
//! the surface's 360° polar, lift perpendicular to the local in-plane velocity.
//!
//! Finite span is handled once, as an induced angle: α_i = C_L / (π e AR), solved by
//! fixed-point iteration (C_L depends on α − α_i), and the force is tilted back by α_i.
//! In the linear range this gives the classical C_L = a₀ α / (1 + a₀ / (π e AR)) and
//! C_Di = C_L² / (π e AR); beyond the stall, the induced angle collapses with the lift.

use glam::DVec3;

use super::polar::{Coefficients, Polar};
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
    1.0 - (theta - theta.sin()) / std::f64::consts::PI
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
    pub aspect_ratio: f64,
    /// Span efficiency e.
    pub efficiency: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LiftingSurface {
    pub name: String,
    pub strips: Vec<Strip>,
    pub polar: Polar,
    pub induced: Option<InducedModel>,
}

/// Number of fixed-point iterations for the induced angle (converges geometrically
/// when a₀ / (π e AR) < 1, which holds for any real wing; fixed for determinism).
const INDUCED_ITERATIONS: usize = 8;

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
    /// Aerodynamic force and moment about the centre of gravity, body axes.
    ///
    /// `air_velocity` is the velocity of the centre of gravity relative to the air,
    /// body axes; `downwash` (rad) lowers the angle of attack of every strip (tail
    /// behind a wing), `density` in kg/m³.
    pub fn loads(
        &self,
        air_velocity: DVec3,
        angular_velocity: DVec3,
        density: f64,
        controls: &ControlDeflections,
        downwash: f64,
    ) -> Wrench {
        self.strips
            .iter()
            .map(|strip| {
                self.strip_loads(
                    strip,
                    air_velocity,
                    angular_velocity,
                    density,
                    controls,
                    downwash,
                )
                .0
            })
            .fold(Wrench::default(), |a, b| a + b)
    }

    /// Loads and diagnostics of one strip.
    pub fn strip_loads(
        &self,
        strip: &Strip,
        air_velocity: DVec3,
        angular_velocity: DVec3,
        density: f64,
        controls: &ControlDeflections,
        downwash: f64,
    ) -> (Wrench, StripState) {
        let v = air_velocity + angular_velocity.cross(strip.position);
        let (u_c, u_n) = (v.dot(strip.chord_axis), v.dot(strip.normal));
        let speed_sq = u_c * u_c + u_n * u_n;
        if speed_sq < MIN_STRIP_SPEED * MIN_STRIP_SPEED {
            return (Wrench::default(), StripState::default());
        }
        let speed = speed_sq.sqrt();
        let control_shift = strip
            .control
            .map_or(0.0, |c| c.alpha_per_rad * controls.get(c.channel));
        let alpha = (-u_n).atan2(u_c) + control_shift - downwash;

        let mut induced = 0.0;
        let mut coefficients = self.polar.coefficients(alpha);
        if let Some(model) = self.induced {
            let k = 1.0 / (std::f64::consts::PI * model.efficiency * model.aspect_ratio);
            for _ in 0..INDUCED_ITERATIONS {
                induced = coefficients.cl * k;
                coefficients = self.polar.coefficients(alpha - induced);
            }
            induced = coefficients.cl * k;
        }

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
