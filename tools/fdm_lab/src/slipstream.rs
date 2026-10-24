//! Rolling moment of the propeller slipstream on a wing, against the powered model of
//! NACA TN 1146 (Purser and Spear, 1946; plan `docs/plans/m1-souffle-ailes.md`).
//!
//! Tail off, at α = 0 and T_c' = T / (q S) = 1.25, the model's rolling moment was 50 to
//! 60 % of the propeller torque (figure 13, three blade angles): the reaction of the engine,
//! less what the wing and fuselage took back from the swirl. The comparison model
//! (`tools/fdm_lab/comparison/aircraft/naca_tn1146`) is flown at the measured operating
//! point of each blade angle, its propeller table reproducing the measured thrust and
//! torque there (figure 7).

use std::f64::consts::TAU;
use std::path::Path;

use escadrille_data::aircraft::{Loading, load_aircraft};
use escadrille_data::load_checked;
use escadrille_fdm::aero::surface::ControlDeflections;
use escadrille_fdm::aircraft::Aircraft;
use escadrille_fdm::constants::RHO0;
use escadrille_fdm::environment::Environment;
use escadrille_fdm::rigid_body::RigidBodyState;
use glam::DVec3;

/// One tail-off test of TN 1146.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TunnelPoint {
    /// Blade angle at 0.75 R, degrees.
    pub blade_angle: f64,
    /// Propeller table of this blade angle, from the comparison assets root.
    pub table: &'static str,
    /// Advance ratio where T_c' = 1.25 (figure 7).
    pub advance_ratio: f64,
    /// Rolling moment over the torque, C_l / Q_c', lowest and highest of the tail-off points
    /// read on figure 13 (negative: rolling against the propeller rotation).
    pub measured: (f64, f64),
}

/// The three blade angles of TN 1146: J and the torque coefficient read on figure 7,
/// C_l on figure 13 (±0.0005), Q_c' = 0.3314 Q_c with Q_c = Q / (ρ V² D³).
pub const TN1146: [TunnelPoint; 3] = [
    TunnelPoint {
        blade_angle: 15.0,
        table: "aircraft/naca_tn1146/propeller_b15.ron",
        advance_ratio: 0.324,
        // C_l −0.019 and −0.020 for Q_c' 0.0381.
        measured: (-0.525, -0.499),
    },
    TunnelPoint {
        blade_angle: 25.0,
        table: "aircraft/naca_tn1146/propeller_b25.ron",
        advance_ratio: 0.406,
        // C_l −0.0345, −0.0315 and −0.0308 for Q_c' 0.0563.
        measured: (-0.612, -0.547),
    },
    TunnelPoint {
        blade_angle: 35.0,
        table: "aircraft/naca_tn1146/propeller_b35.ron",
        advance_ratio: 0.443,
        // C_l −0.0513 and −0.0488 for Q_c' 0.0908.
        measured: (-0.565, -0.537),
    },
];

/// Tunnel speed of TN 1146, m/s: q = 4.09 lb/sq ft at sea-level density (about 40 mph).
pub fn tn1146_speed() -> f64 {
    let q = 4.09 * 47.880_26;
    (2.0 * q / RHO0).sqrt()
}

/// The TN 1146 model with the propeller of one blade angle, from the comparison assets
/// root (`tools/fdm_lab/comparison`).
pub fn tn1146_model(assets: &Path, point: &TunnelPoint) -> Result<Aircraft, String> {
    let mut data = load_aircraft(assets, "naca_tn1146").map_err(|e| e.to_string())?;
    data.propeller = load_checked(&assets.join(point.table)).map_err(|e| e.to_string())?;
    data.build(&Loading { fuel_fraction: 1.0 })
        .map(|built| built.aircraft)
        .map_err(|e| e.to_string())
}

/// Loads of the airframe at α = 0, wings level, with the propeller at a given advance
/// ratio.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TailOffRoll {
    /// Propeller thrust, N.
    pub thrust: f64,
    /// Propeller torque, N·m, positive when absorbing.
    pub torque: f64,
    /// Aerodynamic rolling moment of the airframe about the centre of gravity, along the
    /// propeller's rotation axis, N·m: what the surfaces take back from the swirl.
    pub recovered: f64,
}

impl TailOffRoll {
    /// Fraction of the torque taken back by the airframe.
    pub fn recovered_fraction(&self) -> f64 {
        self.recovered / self.torque
    }

    /// Rolling moment of the whole model over the torque, as the balance measured it:
    /// the engine's reaction (−Q along the rotation axis) plus the airframe's moment.
    pub fn net_fraction(&self) -> f64 {
        self.recovered_fraction() - 1.0
    }
}

/// Airframe loads at sea level, α = 0, `speed` m/s, propeller at `advance_ratio`.
pub fn tail_off_roll(
    aircraft: &Aircraft,
    env: &dyn Environment,
    speed: f64,
    advance_ratio: f64,
) -> Option<TailOffRoll> {
    let propeller = &aircraft.propulsion()?.propeller;
    let diameter = propeller.geometry().diameter;
    let state = RigidBodyState {
        velocity_body: DVec3::X * speed,
        rotor_speed: TAU * speed / (advance_ratio * diameter),
        ..RigidBodyState::default()
    };
    let d = aircraft.evaluate_deflected(&state, &ControlDeflections::default(), 1.0, env);
    let loads = d.propeller?;
    Some(TailOffRoll {
        thrust: loads.thrust,
        torque: loads.torque,
        recovered: d.loads.wrench.moment.dot(propeller.rotation_axis()),
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use escadrille_fdm::environment::StandardEnvironment;

    use super::*;

    const STILL_AIR: StandardEnvironment = StandardEnvironment {
        wind_ned: DVec3::ZERO,
        ground_altitude: 0.0,
    };

    fn model(point: &TunnelPoint) -> Aircraft {
        let assets = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("comparison");
        tn1146_model(&assets, point).unwrap()
    }

    #[test]
    fn the_model_runs_at_the_measured_thrust_coefficient() {
        // T_c' = T / (q S), S = 9.40 sq ft.
        let (speed, area) = (tn1146_speed(), 9.40 * 0.3048 * 0.3048);
        let q = 0.5 * RHO0 * speed * speed;
        for point in &TN1146 {
            let roll =
                tail_off_roll(&model(point), &STILL_AIR, speed, point.advance_ratio).unwrap();
            let tc = roll.thrust / (q * area);
            assert!((tc - 1.25).abs() < 0.005, "{point:?}: T_c' {tc}");
            assert!(roll.torque > 0.0);
        }
    }

    #[test]
    fn the_measured_points_are_those_of_figure_13() {
        for point in &TN1146 {
            let (lo, hi) = point.measured;
            assert!(lo < hi && hi < 0.0 && lo > -1.0, "{point:?}");
        }
        assert!((tn1146_speed() - 17.88).abs() < 0.01, "{}", tn1146_speed());
    }
}
