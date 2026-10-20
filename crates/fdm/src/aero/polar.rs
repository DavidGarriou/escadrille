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
//! reversed section). This is a modelling assumption (E:), to be reviewed by the
//! aerodynamicist; the flight envelope of the essais stays inside the table.

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
    BadMaxDrag,
}

/// Polar of a section, defined on the whole circle.
#[derive(Debug, Clone, PartialEq)]
pub struct Polar {
    points: Vec<PolarPoint>,
    /// Drag coefficient of the section broadside to the flow (α = ±90°).
    cd_max: f64,
    /// Drag at zero lift, used for the reversed-flow flat plate.
    cd0: f64,
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
        if !(cd_max.is_finite() && cd_max > 0.0) {
            return Err(PolarError::BadMaxDrag);
        }
        let mut polar = Self {
            points,
            cd_max,
            cd0: 0.0,
        };
        polar.cd0 = polar.interpolate(0.0).cd;
        Ok(polar)
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

    /// Thin-aerofoil textbook section (not historical data): C_L = 2πα, constant drag,
    /// table up to ±15°.
    pub(crate) fn thin_aerofoil() -> Polar {
        let points = (-15..=15)
            .map(|deg| {
                let alpha = f64::from(deg).to_radians();
                PolarPoint {
                    alpha,
                    cl: 2.0 * PI * alpha,
                    cd: 0.01 + 0.02 * alpha * alpha,
                    cm: -0.02,
                }
            })
            .collect();
        Polar::new(points, viterna_cd_max(6.0)).unwrap()
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
        assert_eq!(
            Polar::new(vec![p(-0.1), p(0.0), p(0.1)], 0.0),
            Err(PolarError::BadMaxDrag)
        );
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
