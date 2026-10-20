//! Aerofoil polars over the full ±180° range.
//!
//! Inside the measured range, coefficients are linearly interpolated from the table
//! (sourced wind-tunnel data, see the research sheets). Beyond it:
//! - up to ±90°, the Viterna-Corrigan extension (Viterna & Corrigan, *Fixed pitch rotor
//!   performance of large horizontal axis wind turbines*, NASA CP-2230, 1982), matched
//!   to the last measured point, so lift and drag stay continuous;
//! - beyond ±90° (flow from the trailing edge), a flat plate: C_L = (C_Dmax / 2) sin 2α,
//!   C_D = C_Dmax sin²α + C_D0 cos²α, continuous with the above at ±90°.
//!
//! The pitching moment (about the quarter chord) beyond the table comes from the normal
//! force and a centre of pressure moving linearly from its last measured position to
//! mid-chord at ±90°, then to three-quarter chord at ±180° (the quarter chord of the
//! reversed section). Modelling assumption (E:), accepted for M1 by the aerodynamicist
//! (review of 2026-10-20): the linear law stays within 0.05 c of Rayleigh's free-streamline
//! flat plate, x_cp / c = 1/2 − (3/4) cos α / (4 + π sin α).
//!
//! Known limits, to revisit (aerodynamicist review): near ±180° a real section behaves
//! like a sharp-edged aerofoil (lift slope close to the forward one, stall about ±10°
//! from 180°; NACA TN 3361, Sandia SAND80-2114), and its drag there is 1.5 to 2 times
//! the forward drag. This matters for tail slides and flat spins, not for the M1 tests.
//!
//! Each table must go past the stall on both sides (checked by [`Polar::new`]): a table
//! that stops in the linear range would make the extension invent a gentle, stall-free
//! curve. Missing negative-stall data must be added explicitly as an estimate (E:).

use std::f64::consts::{FRAC_PI_2, PI};

/// One measured point of a polar.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PolarPoint {
    /// Angle of attack, rad.
    pub alpha: f64,
    pub cl: f64,
    pub cd: f64,
    /// Pitching moment about the quarter chord, positive nose up.
    pub cm: f64,
}

/// Lift, drag and pitching-moment coefficients.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Coefficients {
    pub cl: f64,
    pub cd: f64,
    pub cm: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolarError {
    TooFewPoints,
    NotStrictlyIncreasing,
    NonFinite,
    /// The table must contain α = 0 and stay strictly between −90° and +90°.
    BadRange,
    /// The table must end past the stall on both sides (lift falling towards each end).
    NoStallInTable,
    BadMaxDrag,
}

/// Polar of a section, defined on the whole circle.
#[derive(Debug, Clone, PartialEq)]
pub struct Polar {
    points: Vec<PolarPoint>,
    /// Drag coefficient of the section broadside to the flow (α = ±90°).
    cd_max: f64,
    /// Drag at α = 0, used for the reversed-flow flat plate.
    cd0: f64,
    /// Upper bound of |C_L| over the whole circle.
    max_abs_cl: f64,
    /// Points of minimum and maximum C_L of the table (negative and positive stall).
    stall: (PolarPoint, PolarPoint),
}

/// Maximum drag coefficient of a flat plate of aspect ratio `aspect_ratio`
/// (Viterna & Corrigan 1982: C_Dmax = 1.11 + 0.018 AR, for AR ≤ 50).
pub fn viterna_cd_max(aspect_ratio: f64) -> f64 {
    1.11 + 0.018 * aspect_ratio.min(50.0)
}

/// Wraps an angle into (−π, π].
pub fn wrap_angle(alpha: f64) -> f64 {
    let a = alpha.rem_euclid(2.0 * PI);
    if a > PI { a - 2.0 * PI } else { a }
}

impl Polar {
    pub fn new(points: Vec<PolarPoint>, cd_max: f64) -> Result<Self, PolarError> {
        if points.len() < 3 {
            return Err(PolarError::TooFewPoints);
        }
        if points
            .iter()
            .any(|p| ![p.alpha, p.cl, p.cd, p.cm].iter().all(|v| v.is_finite()))
        {
            return Err(PolarError::NonFinite);
        }
        if points.windows(2).any(|w| w[1].alpha <= w[0].alpha) {
            return Err(PolarError::NotStrictlyIncreasing);
        }
        let (first, last) = (points[0].alpha, points[points.len() - 1].alpha);
        if !(first < 0.0 && last > 0.0 && first > -FRAC_PI_2 && last < FRAC_PI_2) {
            return Err(PolarError::BadRange);
        }
        let n = points.len();
        if !(points[n - 1].cl < points[n - 2].cl && points[0].cl > points[1].cl) {
            return Err(PolarError::NoStallInTable);
        }
        if !(cd_max.is_finite() && cd_max > 0.0) {
            return Err(PolarError::BadMaxDrag);
        }
        let first = points[0];
        let mut polar = Self {
            points,
            cd_max,
            cd0: 0.0,
            max_abs_cl: 0.0,
            stall: (first, first),
        };
        let pick = |better: fn(f64, f64) -> bool| {
            polar
                .points
                .iter()
                .copied()
                .reduce(|a, b| if better(b.cl, a.cl) { b } else { a })
                .unwrap()
        };
        polar.stall = (pick(|b, a| b < a), pick(|b, a| b > a));
        polar.cd0 = polar.interpolate(0.0).cd;
        // Sampled every 0.5°, with a margin for what lies between samples.
        let max_sampled = (0..720)
            .map(|i| polar.coefficients(-PI + f64::from(i) * PI / 360.0).cl.abs())
            .fold(0.0, f64::max);
        let max_table = polar.points.iter().map(|p| p.cl.abs()).fold(0.0, f64::max);
        polar.max_abs_cl = 1.1 * max_sampled.max(max_table) + 0.1;
        Ok(polar)
    }

    /// Upper bound of |C_L| over the whole circle.
    pub fn max_abs_cl(&self) -> f64 {
        self.max_abs_cl
    }

    /// Table points of minimum and maximum lift (negative and positive stall).
    pub fn stall_points(&self) -> (PolarPoint, PolarPoint) {
        self.stall
    }

    /// Measured range of the table, rad.
    pub fn table_range(&self) -> (f64, f64) {
        (
            self.points[0].alpha,
            self.points[self.points.len() - 1].alpha,
        )
    }

    fn interpolate(&self, alpha: f64) -> Coefficients {
        let i = self
            .points
            .partition_point(|p| p.alpha <= alpha)
            .clamp(1, self.points.len() - 1);
        let (a, b) = (self.points[i - 1], self.points[i]);
        let t = (alpha - a.alpha) / (b.alpha - a.alpha);
        Coefficients {
            cl: a.cl + t * (b.cl - a.cl),
            cd: a.cd + t * (b.cd - a.cd),
            cm: a.cm + t * (b.cm - a.cm),
        }
    }

    /// Coefficients at any angle of attack (radians, any value; wrapped to ±180°).
    pub fn coefficients(&self, alpha: f64) -> Coefficients {
        let alpha = wrap_angle(alpha);
        let (lo, hi) = self.table_range();
        if (lo..=hi).contains(&alpha) {
            return self.interpolate(alpha);
        }
        // Work on the positive side, mirroring the negative one (the edge point is the
        // table's lower or upper end, so asymmetric sections stay continuous).
        let (sign, edge) = if alpha > hi {
            (1.0, self.points[self.points.len() - 1])
        } else {
            (-1.0, self.points[0])
        };
        let a = alpha.abs();
        let a_s = edge.alpha.abs();
        let (cl_s, cd_s) = (sign * edge.cl, edge.cd);
        let (cl_pos, cd) = if a <= FRAC_PI_2 {
            // Viterna-Corrigan, matched at the edge of the table.
            let (s, c) = a_s.sin_cos();
            let a1 = 0.5 * self.cd_max;
            let b2 = (cd_s - self.cd_max * s * s) / c;
            let a2 = (cl_s - self.cd_max * s * c) * s / (c * c);
            let (sa, ca) = a.sin_cos();
            (
                a1 * (2.0 * a).sin() + a2 * ca * ca / sa,
                self.cd_max * sa * sa + b2 * ca,
            )
        } else {
            let (sa, ca) = a.sin_cos();
            (
                0.5 * self.cd_max * (2.0 * a).sin(),
                self.cd_max * sa * sa + self.cd0 * ca * ca,
            )
        };
        let cl = sign * cl_pos;

        // Pitching moment from the normal force and a moving centre of pressure.
        let normal = |cl: f64, cd: f64, a: f64| cl * a.cos() + cd * a.sin().abs();
        let edge_normal = normal(cl_s, cd_s, a_s);
        let x_edge = if edge_normal.abs() > 1e-6 {
            (0.25 - sign * edge.cm / edge_normal).clamp(0.0, 1.0)
        } else {
            0.25
        };
        let x_cp = if a <= FRAC_PI_2 {
            x_edge + (0.5 - x_edge) * (a - a_s) / (FRAC_PI_2 - a_s)
        } else {
            0.5 + 0.25 * (a - FRAC_PI_2) / FRAC_PI_2
        };
        let cm = -sign * normal(cl_pos, cd, a) * (x_cp - 0.25);
        Coefficients { cl, cd, cm }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Thin-aerofoil textbook section (not historical data): C_L = 2πα up to ±14°,
    /// then a stall down to 80 % of the peak at ±20°.
    pub(crate) fn thin_aerofoil() -> Polar {
        Polar::new(stalling_points(-0.02), viterna_cd_max(6.0)).unwrap()
    }

    pub(crate) fn stalling_points(cm: f64) -> Vec<PolarPoint> {
        let peak = 2.0 * PI * 14f64.to_radians();
        (-20..=20)
            .map(|deg: i32| {
                let alpha = f64::from(deg).to_radians();
                let a = deg.unsigned_abs();
                let cl = if a <= 14 {
                    2.0 * PI * alpha
                } else {
                    alpha.signum() * peak * (1.0 - 0.2 * f64::from(a - 14) / 6.0)
                };
                let cd = 0.01
                    + 0.02 * alpha * alpha
                    + if a > 14 {
                        0.02 * f64::from(a - 14)
                    } else {
                        0.0
                    };
                PolarPoint { alpha, cl, cd, cm }
            })
            .collect()
    }

    #[test]
    fn rejects_bad_tables() {
        let p = |alpha: f64| PolarPoint {
            alpha,
            cl: 0.0,
            cd: 0.01,
            cm: 0.0,
        };
        assert_eq!(
            Polar::new(vec![p(-0.1), p(0.1)], 1.2),
            Err(PolarError::TooFewPoints)
        );
        assert_eq!(
            Polar::new(vec![p(-0.1), p(0.2), p(0.1)], 1.2),
            Err(PolarError::NotStrictlyIncreasing)
        );
        assert_eq!(
            Polar::new(vec![p(0.1), p(0.2), p(0.3)], 1.2),
            Err(PolarError::BadRange)
        );
        let stall = |alpha: f64, cl: f64| PolarPoint {
            alpha,
            cl,
            cd: 0.01,
            cm: 0.0,
        };
        let stalled = vec![
            stall(-0.2, -0.5),
            stall(-0.1, -0.6),
            stall(0.1, 0.6),
            stall(0.2, 0.5),
        ];
        assert_eq!(
            Polar::new(stalled.clone(), 0.0),
            Err(PolarError::BadMaxDrag)
        );
        assert!(Polar::new(stalled, 1.2).is_ok());
        // Stops in the linear range on the negative side (like the RAF 15 table of
        // NACA Report 233, which ends at −1.5°).
        let linear_negative = vec![
            stall(-0.03, -0.1),
            stall(0.0, 0.1),
            stall(0.2, 1.2),
            stall(0.3, 1.0),
        ];
        assert_eq!(
            Polar::new(linear_negative, 1.2),
            Err(PolarError::NoStallInTable)
        );
    }

    #[test]
    fn flat_plate_values_beyond_ninety_degrees() {
        let polar = thin_aerofoil();
        let cd_max = viterna_cd_max(6.0);
        // Centre of pressure at mid-chord broadside: cm = ∓C_Dmax / 4.
        assert!((polar.coefficients(FRAC_PI_2).cm + cd_max / 4.0).abs() < 1e-9);
        assert!((polar.coefficients(-FRAC_PI_2).cm - cd_max / 4.0).abs() < 1e-9);
        assert!((polar.coefficients(135f64.to_radians()).cl + cd_max / 2.0).abs() < 1e-9);
        // Near 180°, centre of pressure at 3/4 chord: cm ≈ −C_N / 2.
        let a = 179f64.to_radians();
        let c = polar.coefficients(a);
        let normal = c.cl * a.cos() + c.cd * a.sin();
        // The centre of pressure is at 0.747 c at 179° (0.75 c at 180°).
        assert!((c.cm + 0.5 * normal).abs() < 1e-2 * normal.abs(), "{c:?}");
    }

    #[test]
    fn max_abs_cl_bounds_the_whole_circle() {
        let polar = thin_aerofoil();
        for i in 0..3_600 {
            let a = -PI + 2.0 * PI * f64::from(i) / 3_600.0;
            assert!(polar.coefficients(a).cl.abs() < polar.max_abs_cl());
        }
    }

    #[test]
    fn interpolates_inside_the_table() {
        let polar = thin_aerofoil();
        let c = polar.coefficients(5f64.to_radians());
        assert!((c.cl - 2.0 * PI * 5f64.to_radians()).abs() < 1e-12);
        let mid = polar.coefficients(5.5f64.to_radians());
        assert!(mid.cl > c.cl);
    }

    #[test]
    fn extension_is_continuous_at_both_table_edges() {
        let polar = thin_aerofoil();
        let (lo, hi) = polar.table_range();
        for edge in [lo, hi] {
            let eps = 1e-9 * edge.signum();
            let inside = polar.coefficients(edge - eps);
            let outside = polar.coefficients(edge + eps);
            assert!(
                (inside.cl - outside.cl).abs() < 1e-6,
                "{inside:?} {outside:?}"
            );
            assert!(
                (inside.cd - outside.cd).abs() < 1e-6,
                "{inside:?} {outside:?}"
            );
            assert!(
                (inside.cm - outside.cm).abs() < 1e-6,
                "{inside:?} {outside:?}"
            );
        }
    }

    #[test]
    fn broadside_and_reversed_flow_values() {
        let polar = thin_aerofoil();
        for a in [FRAC_PI_2, -FRAC_PI_2] {
            let c = polar.coefficients(a);
            assert!(c.cl.abs() < 1e-9, "{c:?}");
            assert!((c.cd - viterna_cd_max(6.0)).abs() < 1e-9, "{c:?}");
        }
        let c = polar.coefficients(PI);
        assert!(c.cl.abs() < 1e-9);
        assert!((c.cd - 0.01).abs() < 1e-9, "{c:?}");
    }

    #[test]
    fn continuous_and_finite_over_the_whole_circle() {
        let polar = thin_aerofoil();
        let mut previous = polar.coefficients(-PI);
        for i in 1..=36_000 {
            let a = -PI + 2.0 * PI * f64::from(i) / 36_000.0;
            let c = polar.coefficients(a);
            assert!(
                c.cl.is_finite() && c.cd.is_finite() && c.cm.is_finite(),
                "{a}"
            );
            assert!(c.cd > 0.0, "{a}: {c:?}");
            // No jump larger than what a 0.01° step can produce.
            assert!(
                (c.cl - previous.cl).abs() < 0.01,
                "{a}: {previous:?} -> {c:?}"
            );
            assert!(
                (c.cd - previous.cd).abs() < 0.01,
                "{a}: {previous:?} -> {c:?}"
            );
            assert!(
                (c.cm - previous.cm).abs() < 0.01,
                "{a}: {previous:?} -> {c:?}"
            );
            previous = c;
        }
    }

    #[test]
    fn angles_are_wrapped() {
        let polar = thin_aerofoil();
        let a = 0.3;
        let (c1, c2) = (polar.coefficients(a), polar.coefficients(a + 2.0 * PI));
        // a + 2π − 2π differs from a by rounding only.
        assert!((c1.cl - c2.cl).abs() < 1e-12 && (c1.cd - c2.cd).abs() < 1e-12);
        assert_eq!(wrap_angle(PI), PI);
        assert_eq!(wrap_angle(-PI), PI);
    }

    #[test]
    fn stall_drops_lift_beyond_the_table() {
        let polar = thin_aerofoil();
        let at_edge = polar.coefficients(15f64.to_radians()).cl;
        let beyond = polar.coefficients(30f64.to_radians()).cl;
        assert!(beyond < at_edge, "{at_edge} -> {beyond}");
    }
}
