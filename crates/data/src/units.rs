//! Quantities typed with the unit of their source, converted to SI on loading
//! (`format-avion.md`): a value is copied as the source writes it, with no conversion
//! by hand.

use serde::Deserialize;

/// Foot, m (international foot, exact).
pub const FOOT: f64 = 0.3048;
/// Inch, m (exact).
pub const INCH: f64 = 0.0254;
/// Pound (avoirdupois), kg (exact).
pub const POUND: f64 = 0.453_592_37;
/// Mile per hour, m/s (exact).
pub const MPH: f64 = 0.447_04;
/// Knot, m/s (exact: 1 852 m per hour).
pub const KNOT: f64 = 1_852.0 / 3_600.0;
/// Mechanical horsepower (550 ft·lbf/s), W.
pub const HORSEPOWER: f64 = 550.0 * FOOT * POUND * 9.806_65;
/// Metric horsepower (PS, cheval-vapeur: 75 kgf·m/s), W.
pub const METRIC_HORSEPOWER: f64 = 75.0 * 9.806_65;
/// Imperial gallon, L (exact).
pub const IMPERIAL_GALLON: f64 = 4.546_09;
/// US gallon, L (exact).
pub const US_GALLON: f64 = 3.785_411_784;

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum Length {
    M(f64),
    Cm(f64),
    Mm(f64),
    Ft(f64),
    In(f64),
    /// Feet and inches, as British and American sources write them.
    FtIn(f64, f64),
}

impl Length {
    /// Metres.
    pub fn si(self) -> f64 {
        match self {
            Self::M(v) => v,
            Self::Cm(v) => v * 0.01,
            Self::Mm(v) => v * 0.001,
            Self::Ft(v) => v * FOOT,
            Self::In(v) => v * INCH,
            Self::FtIn(ft, inches) => ft * FOOT + inches * INCH,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum Area {
    M2(f64),
    Ft2(f64),
}

impl Area {
    /// Square metres.
    pub fn si(self) -> f64 {
        match self {
            Self::M2(v) => v,
            Self::Ft2(v) => v * FOOT * FOOT,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum Mass {
    Kg(f64),
    Lb(f64),
}

impl Mass {
    /// Kilograms.
    pub fn si(self) -> f64 {
        match self {
            Self::Kg(v) => v,
            Self::Lb(v) => v * POUND,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum Speed {
    Ms(f64),
    Kmh(f64),
    Mph(f64),
    Kt(f64),
    /// Feet per minute (rates of climb).
    FtMin(f64),
}

impl Speed {
    /// Metres per second.
    pub fn si(self) -> f64 {
        match self {
            Self::Ms(v) => v,
            Self::Kmh(v) => v / 3.6,
            Self::Mph(v) => v * MPH,
            Self::Kt(v) => v * KNOT,
            Self::FtMin(v) => v * FOOT / 60.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum Power {
    W(f64),
    Kw(f64),
    /// Mechanical horsepower (British and American sources).
    Hp(f64),
    /// Metric horsepower (German sources).
    Ps(f64),
    /// Cheval-vapeur (French sources), the same unit as `Ps`.
    Ch(f64),
}

impl Power {
    /// Watts.
    pub fn si(self) -> f64 {
        match self {
            Self::W(v) => v,
            Self::Kw(v) => v * 1_000.0,
            Self::Hp(v) => v * HORSEPOWER,
            Self::Ps(v) | Self::Ch(v) => v * METRIC_HORSEPOWER,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum Volume {
    L(f64),
    ImpGal(f64),
    UsGal(f64),
}

impl Volume {
    /// Litres (not SI, but the natural unit with a density in kg/L).
    pub fn litres(self) -> f64 {
        match self {
            Self::L(v) => v,
            Self::ImpGal(v) => v * IMPERIAL_GALLON,
            Self::UsGal(v) => v * US_GALLON,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum Angle {
    Rad(f64),
    Deg(f64),
}

impl Angle {
    /// Radians.
    pub fn si(self) -> f64 {
        match self {
            Self::Rad(v) => v,
            Self::Deg(v) => v.to_radians(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum RotationSpeed {
    RadS(f64),
    Rpm(f64),
}

impl RotationSpeed {
    /// Radians per second.
    pub fn si(self) -> f64 {
        match self {
            Self::RadS(v) => v,
            Self::Rpm(v) => v * std::f64::consts::PI / 30.0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
pub enum Duration {
    S(f64),
    Min(f64),
    /// Minutes and seconds, as period climb tables write them.
    MinS(f64, f64),
}

impl Duration {
    /// Seconds.
    pub fn si(self) -> f64 {
        match self {
            Self::S(v) => v,
            Self::Min(v) => v * 60.0,
            Self::MinS(m, s) => m * 60.0 + s,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() <= 1e-12 * b.abs().max(1.0)
    }

    #[test]
    fn conversions_match_the_definitions() {
        assert!(close(Length::Ft(10_000.0).si(), 3_048.0));
        assert!(close(Length::FtIn(25.0, 8.0).si(), 7.8232));
        assert!(close(Length::Mm(120.0).si(), 0.12));
        assert!(close(Area::Ft2(100.0).si(), 9.290_304));
        assert!(close(Mass::Lb(1_000.0).si(), 453.592_37));
        assert!(close(Speed::Kmh(180.0).si(), 50.0));
        assert!(close(Speed::Mph(100.0).si(), 44.704));
        assert!(close(Speed::Kt(100.0).si(), 51.444_444_444_444_44));
        assert!(close(Speed::FtMin(100.0).si(), 0.508));
        // Well-known values: 1 hp = 745.70 W, 1 ch = 735.50 W.
        assert!((Power::Hp(1.0).si() - 745.699_872).abs() < 1e-6);
        assert!(close(Power::Ch(150.0).si(), 110_324.812_5));
        assert_eq!(Power::Ch(150.0).si(), Power::Ps(150.0).si());
        assert!(close(Volume::ImpGal(20.0).litres(), 90.921_8));
        assert!(close(Angle::Deg(180.0).si(), std::f64::consts::PI));
        assert!(close(
            RotationSpeed::Rpm(2_100.0).si(),
            70.0 * std::f64::consts::PI
        ));
        assert!(close(Duration::MinS(6.0, 40.0).si(), 400.0));
    }

    #[test]
    fn quantities_parse_from_ron() {
        let speed: Speed = ron::from_str("Kmh(192.0)").unwrap();
        assert_eq!(speed, Speed::Kmh(192.0));
        let length: Length = ron::from_str("FtIn(25, 8)").unwrap();
        assert_eq!(length, Length::FtIn(25.0, 8.0));
    }
}
