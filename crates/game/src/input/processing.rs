//! Treatment of the raw inputs (skill `manette`, « Architecture »), without hardware:
//! radial dead zone, response curve, ramps for commands given by buttons, incremental
//! throttle, and the elevator's neutral.

use bevy::math::Vec2;

/// Radial dead zone of a stick: nothing inside `d`, then the rest of the travel stretched
/// over the whole range, the direction kept: x' = (|x| − d) / (1 − d).
pub fn radial_dead_zone(v: Vec2, d: f32) -> Vec2 {
    let length = v.length();
    if length <= d {
        return Vec2::ZERO;
    }
    v * ((length.min(1.0) - d) / (1.0 - d) / length)
}

/// Response curve: y = sign(x) × ((1 − k) |x| + k |x|³), finer near the centre.
pub fn expo(x: f32, k: f32) -> f32 {
    let a = x.abs().min(1.0);
    x.signum() * ((1.0 - k) * a + k * a * a * a)
}

/// A command given by buttons (all or nothing) moved towards its target at a limited
/// rate: away from the centre in `rise_s` for the whole travel, back towards it in
/// `return_s`, as a pilot's foot on the rudder bar.
pub fn ramp(current: f32, target: f32, dt: f32, rise_s: f32, return_s: f32) -> f32 {
    let rising = target != 0.0 && (target - current) * current >= 0.0;
    let rate = if rising { 1.0 / rise_s } else { 1.0 / return_s };
    let step = rate * dt;
    let next = current + (target - current).clamp(-step, step);
    // Back through the centre: stop there, then rise on the other side.
    if current != 0.0 && next.signum() != current.signum() && !rising {
        0.0
    } else {
        next
    }
}

/// A lever moved by two buttons: a short press moves it by `step`, holding it longer
/// than `hold_s` moves it at `rate` per second.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Lever {
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub rate: f32,
    pub hold_s: f32,
    /// How long the button has been held, s.
    held: f32,
}

impl Lever {
    pub fn new(value: f32, min: f32, max: f32, step: f32, rate: f32, hold_s: f32) -> Self {
        Self {
            value,
            min,
            max,
            step,
            rate,
            hold_s,
            held: 0.0,
        }
    }

    /// One frame of the two buttons (`down`, `up`), `pressed` meaning just pressed.
    pub fn update(&mut self, down: (bool, bool), up: (bool, bool), dt: f32) {
        let direction = match ((down.0 || down.1), (up.0 || up.1)) {
            (true, false) => -1.0,
            (false, true) => 1.0,
            _ => {
                self.held = 0.0;
                return;
            }
        };
        if down.0 || up.0 {
            self.value += direction * self.step;
            self.held = 0.0;
        } else {
            self.held += dt;
            if self.held > self.hold_s {
                self.value += direction * self.rate * dt;
            }
        }
        self.value = self.value.clamp(self.min, self.max);
    }
}

/// The elevator command for a stick position `s` (−1 to 1, positive nose up) around the
/// neutral `n`: the centre gives `n`, the ends still give ±1.
pub fn around_neutral(s: f32, n: f32) -> f32 {
    if s >= 0.0 {
        n + s * (1.0 - n)
    } else {
        n + s * (1.0 + n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_dead_zone_cuts_the_centre_and_keeps_the_ends() {
        assert_eq!(radial_dead_zone(Vec2::new(0.05, 0.05), 0.08), Vec2::ZERO);
        let full = radial_dead_zone(Vec2::new(1.0, 0.0), 0.08);
        assert!((full.x - 1.0).abs() < 1e-6);
        let half = radial_dead_zone(Vec2::new(0.0, -0.54), 0.08);
        assert!((half.y + 0.5).abs() < 1e-6, "{half}");
        // Radial: a diagonal keeps its direction.
        let d = radial_dead_zone(Vec2::new(0.5, 0.5), 0.08);
        assert!((d.x - d.y).abs() < 1e-6);
    }

    #[test]
    fn the_curve_is_finer_at_the_centre() {
        assert_eq!(expo(1.0, 0.4), 1.0);
        assert_eq!(expo(-1.0, 0.4), -1.0);
        assert!((expo(0.5, 0.4) - 0.35).abs() < 1e-6);
        assert_eq!(expo(0.0, 0.4), 0.0);
    }

    #[test]
    fn the_rudder_rises_in_its_time_and_returns_faster() {
        let dt = 1.0 / 60.0;
        let mut r = 0.0;
        let mut frames = 0;
        while r < 1.0 {
            r = ramp(r, 1.0, dt, 0.4, 0.2);
            frames += 1;
        }
        // 0.4 s at 60 frames a second, within a frame of rounding.
        assert!((24..=25).contains(&frames), "{frames}");
        let mut frames = 0;
        while r > 0.0 {
            r = ramp(r, 0.0, dt, 0.4, 0.2);
            frames += 1;
        }
        assert!((12..=13).contains(&frames), "{frames}");
        // From full right to full left: back to the centre fast, then out slowly.
        let mut r = 1.0;
        for _ in 0..13 {
            r = ramp(r, -1.0, dt, 0.4, 0.2);
            if r == 0.0 {
                break;
            }
        }
        assert_eq!(r, 0.0);
        r = ramp(r, -1.0, dt, 0.4, 0.2);
        assert!((r + dt / 0.4).abs() < 1e-6, "{r}");
    }

    #[test]
    fn the_throttle_steps_on_a_press_and_runs_when_held() {
        let mut t = Lever::new(0.5, 0.0, 1.0, 0.05, 0.30, 0.25);
        t.update((false, false), (true, true), 0.01);
        assert!((t.value - 0.55).abs() < 1e-6);
        // Held for a second: nothing for a quarter, then 30 % a second.
        for _ in 0..100 {
            t.update((false, false), (false, true), 0.01);
        }
        assert!((t.value - (0.55 + 0.30 * 0.75)).abs() < 0.01, "{}", t.value);
        for _ in 0..1000 {
            t.update((false, false), (false, true), 0.01);
        }
        assert_eq!(t.value, 1.0);
        t.update((true, true), (false, false), 0.01);
        assert!((t.value - 0.95).abs() < 1e-6);
    }

    #[test]
    fn the_neutral_moves_the_centre_and_keeps_the_ends() {
        let n = -0.108;
        assert_eq!(around_neutral(0.0, n), n);
        assert_eq!(around_neutral(1.0, n), 1.0);
        assert_eq!(around_neutral(-1.0, n), -1.0);
        assert!(around_neutral(0.5, n) > n);
    }
}
