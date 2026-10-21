//! Propulsion: fixed-pitch propeller and inline piston engine.
//!
//! Both act on the rotor of [`crate::rigid_body::MassProperties`]: the engine through
//! [`crate::rigid_body::RotorTorques::engine`] and `friction`, which depend on the shaft
//! speed **relative to the airframe** ([`crate::rigid_body::RigidBodyState::rotor_relative_speed`]),
//! the propeller through its thrust and its aerodynamic torque, which depend on the
//! **absolute** rotor speed (the blades see the airframe's own rotation about the shaft).

pub mod engine;
pub mod propeller;

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use glam::DVec3;

    use super::engine::InlineEngine;
    use super::engine::tests::engine;
    use super::propeller::Propeller;
    use super::propeller::tests::propeller;

    const RHO: f64 = 1.225;

    /// Net torque on the shaft at speed `omega` (airframe not rotating).
    fn net_torque(omega: f64, airspeed: f64, throttle: f64) -> f64 {
        thread_local! {
            static PARTS: (InlineEngine, Propeller) = (engine(), propeller());
        }
        PARTS.with(|(engine, propeller)| {
            let e = engine.torques(omega, throttle, 1.0);
            let p = propeller.loads(DVec3::new(airspeed, 0.0, 0.0), DVec3::ZERO, omega, RHO);
            e.combustion - e.friction - p.torque
        })
    }

    #[test]
    fn rotor_settles_where_engine_and_propeller_torques_balance() {
        let (airspeed, inertia) = (50.0, 1.2);
        // Equilibrium by bisection: the net torque falls as the speed rises.
        let (mut lo, mut hi) = (70.0, 400.0);
        assert!(net_torque(lo, airspeed, 1.0) > 0.0 && net_torque(hi, airspeed, 1.0) < 0.0);
        for _ in 0..200 {
            let mid = 0.5 * (lo + hi);
            if net_torque(mid, airspeed, 1.0) > 0.0 {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let equilibrium = 0.5 * (lo + hi);
        // Brake power equals propeller power there.
        let power = engine().brake_power(equilibrium, 1.0, 1.0);
        let p = propeller().loads(
            DVec3::new(airspeed, 0.0, 0.0),
            DVec3::ZERO,
            equilibrium,
            RHO,
        );
        assert!((power - p.torque * equilibrium).abs() < 1e-6 * power);
        assert!(p.thrust > 0.0);

        // The rotor reaches it from below and from above (RK4, 240 Hz, 10 s).
        for start in [80.0, 350.0] {
            let mut omega: f64 = start;
            let dt = 1.0 / 240.0;
            let f = |w: f64| net_torque(w, airspeed, 1.0) / inertia;
            for _ in 0..2_400 {
                let k1 = f(omega);
                let k2 = f(omega + 0.5 * dt * k1);
                let k3 = f(omega + 0.5 * dt * k2);
                let k4 = f(omega + dt * k3);
                omega += dt / 6.0 * (k1 + 2.0 * (k2 + k3) + k4);
            }
            assert!(
                (omega - equilibrium).abs() < 1e-6 * equilibrium,
                "{start}: {omega}"
            );
        }
        // Plausible advance ratio for a propeller matched to its engine.
        let j = airspeed / (equilibrium / (2.0 * PI) * 2.5);
        assert!((0.5..1.1).contains(&j), "J = {j}");
    }

    #[test]
    fn engine_idles_steadily_on_the_ground() {
        // Throttle closed, aircraft at rest: the rotor settles at a stable idle above the
        // firing ramp, whether it starts fast or slow (aerodynamicist review).
        let (inertia, dt) = (1.2, 1.0 / 240.0);
        let f = |w: f64| net_torque(w, 0.0, 0.0) / inertia;
        let mut ends = Vec::new();
        for start in [45.0, 200.0] {
            let mut omega: f64 = start;
            for _ in 0..(240 * 60) {
                let k1 = f(omega);
                let k2 = f(omega + 0.5 * dt * k1);
                let k3 = f(omega + 0.5 * dt * k2);
                let k4 = f(omega + dt * k3);
                omega += dt / 6.0 * (k1 + 2.0 * (k2 + k3) + k4);
            }
            ends.push(omega);
        }
        assert!((ends[0] - ends[1]).abs() < 1e-6, "{ends:?}");
        assert!(
            ends[0] > 40.0 && ends[0] < 100.0,
            "idle at {} rad/s",
            ends[0]
        );
    }

    #[test]
    fn closing_the_throttle_lowers_the_equilibrium_speed() {
        let speed_at = |throttle: f64| {
            let (mut lo, mut hi) = (61.0, 400.0);
            for _ in 0..200 {
                let mid = 0.5 * (lo + hi);
                if net_torque(mid, 40.0, throttle) > 0.0 {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            0.5 * (lo + hi)
        };
        let speeds = [1.0, 0.7, 0.4].map(speed_at);
        assert!(speeds[0] > speeds[1] && speeds[1] > speeds[2], "{speeds:?}");
    }
}
