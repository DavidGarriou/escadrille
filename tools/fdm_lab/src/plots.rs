//! PNG plots of the flight-test report, read by the player and by Claude (player's
//! decision of 2026-10-22: `plotters` without system fonts, text drawn with the Go font
//! embedded here, `fonts/LICENSE-Go-font`).
//!
//! Chart rules (skill `dataviz`): one y-axis per panel (several units mean several
//! stacked panels sharing the x-axis), categorical colours in a fixed order, thin lines,
//! large dots for measured targets, a legend from two series on, recessive axes.

use std::path::Path;
use std::sync::Once;

use plotters::prelude::*;
use plotters::style::{FontStyle, register_font};

use crate::qualitative::TracePoint;

static FONT: &[u8] = include_bytes!("../fonts/Go-Regular.ttf");
static REGISTER: Once = Once::new();

const FAMILY: &str = "sans-serif";

/// Light chart surface, ink and chrome (skill `dataviz`, reference palette).
const SURFACE: RGBColor = RGBColor(0xfc, 0xfc, 0xfb);
const INK: RGBColor = RGBColor(0x0b, 0x0b, 0x0b);
const SECONDARY: RGBColor = RGBColor(0x52, 0x51, 0x4e);
const MUTED: RGBColor = RGBColor(0x89, 0x87, 0x81);
const GRID: RGBColor = RGBColor(0xe1, 0xe0, 0xd9);

/// Categorical slots in their fixed order (blue, orange, aqua): the first three pass
/// every colour-vision check together.
const SERIES: [RGBColor; 3] = [
    RGBColor(0x2a, 0x78, 0xd6),
    RGBColor(0xeb, 0x68, 0x34),
    RGBColor(0x1b, 0xaf, 0x7a),
];

const WIDTH: u32 = 1200;
const PANEL_HEIGHT: u32 = 300;

fn font() {
    REGISTER.call_once(|| {
        assert!(
            register_font(FAMILY, FontStyle::Normal, FONT).is_ok(),
            "embedded Go font"
        );
    });
}

/// A named series of (x, y) points.
#[derive(Debug, Clone, PartialEq)]
pub struct Series {
    pub label: String,
    pub points: Vec<(f64, f64)>,
    /// Drawn as large dots (measured values) instead of a line (computed curve).
    pub dots: bool,
}

impl Series {
    pub fn line(label: &str, points: Vec<(f64, f64)>) -> Self {
        Self {
            label: label.to_owned(),
            points,
            dots: false,
        }
    }

    pub fn dots(label: &str, points: Vec<(f64, f64)>) -> Self {
        Self {
            label: label.to_owned(),
            points,
            dots: true,
        }
    }
}

/// One panel: its y-axis and up to three series.
#[derive(Debug, Clone, PartialEq)]
pub struct Panel {
    pub y_label: String,
    pub series: Vec<Series>,
}

fn range(values: impl Iterator<Item = f64>) -> (f64, f64) {
    let (lo, hi) = values
        .filter(|v| v.is_finite())
        .fold((f64::MAX, f64::MIN), |(lo, hi), v| (lo.min(v), hi.max(v)));
    if lo > hi {
        return (0.0, 1.0);
    }
    let pad = if hi > lo { 0.05 * (hi - lo) } else { 1.0 };
    (lo - pad, hi + pad)
}

/// Tick label with as many decimals as the span of the axis calls for.
fn tick(span: f64) -> impl Fn(&f64) -> String {
    let decimals = if span >= 20.0 {
        0
    } else if span >= 2.0 {
        1
    } else {
        2
    };
    move |v: &f64| format!("{v:.decimals$}")
}

/// Stacked panels sharing the x-axis, written to `path`.
pub fn draw(path: &Path, title: &str, x_label: &str, panels: &[Panel]) -> Result<(), String> {
    font();
    let error = |e: &dyn std::fmt::Display| format!("{}: {e}", path.display());
    let height = 60 + PANEL_HEIGHT * panels.len().max(1) as u32;
    let root = BitMapBackend::new(path, (WIDTH, height)).into_drawing_area();
    root.fill(&SURFACE).map_err(|e| error(&e))?;
    let root = root
        .titled(title, (FAMILY, 26).into_font().color(&INK))
        .map_err(|e| error(&e))?;
    let x_range = range(
        panels
            .iter()
            .flat_map(|p| p.series.iter().flat_map(|s| s.points.iter().map(|q| q.0))),
    );
    let areas = root.split_evenly((panels.len().max(1), 1));
    for (area, panel) in areas.iter().zip(panels) {
        let y_range = range(
            panel
                .series
                .iter()
                .flat_map(|s| s.points.iter().map(|q| q.1)),
        );
        let mut chart = ChartBuilder::on(area)
            .margin(12)
            .x_label_area_size(40)
            .y_label_area_size(80)
            .build_cartesian_2d(x_range.0..x_range.1, y_range.0..y_range.1)
            .map_err(|e| error(&e))?;
        chart
            .configure_mesh()
            .light_line_style(GRID.mix(0.0))
            .bold_line_style(GRID)
            .axis_style(MUTED)
            .label_style((FAMILY, 15).into_font().color(&SECONDARY))
            .axis_desc_style((FAMILY, 17).into_font().color(&INK))
            .x_label_formatter(&tick(x_range.1 - x_range.0))
            .y_label_formatter(&tick(y_range.1 - y_range.0))
            .x_desc(x_label)
            .y_desc(panel.y_label.as_str())
            .draw()
            .map_err(|e| error(&e))?;
        for (series, color) in panel.series.iter().zip(SERIES) {
            let style = color.stroke_width(2);
            if series.dots {
                chart
                    .draw_series(
                        series
                            .points
                            .iter()
                            .map(|&p| Circle::new(p, 6, color.filled())),
                    )
                    .map_err(|e| error(&e))?
                    .label(series.label.as_str())
                    .legend(move |(x, y)| Circle::new((x + 10, y), 5, color.filled()));
            } else {
                chart
                    .draw_series(LineSeries::new(series.points.iter().copied(), style))
                    .map_err(|e| error(&e))?
                    .label(series.label.as_str())
                    .legend(move |(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], style));
            }
        }
        if panel.series.len() > 1 {
            chart
                .configure_series_labels()
                .background_style(SURFACE.mix(0.9))
                .border_style(GRID)
                .label_font((FAMILY, 15).into_font().color(&INK))
                .position(SeriesLabelPosition::UpperRight)
                .draw()
                .map_err(|e| error(&e))?;
        }
    }
    root.present().map_err(|e| error(&e))
}

/// Time history of a qualitative manoeuvre: speed, angles, load factor, controls.
pub fn manoeuvre(path: &Path, title: &str, trace: &[TracePoint]) -> Result<(), String> {
    let series = |label: &str, f: fn(&TracePoint) -> f64| {
        Series::line(label, trace.iter().map(|p| (p.time, f(p))).collect())
    };
    let panels = [
        Panel {
            y_label: "vitesse (km/h)".into(),
            series: vec![series("vitesse", |p| p.speed * 3.6)],
        },
        Panel {
            y_label: "angles (°)".into(),
            series: vec![
                series("incidence", |p| p.alpha.to_degrees()),
                series("assiette", |p| p.pitch.to_degrees()),
                series("inclinaison", |p| p.bank.to_degrees()),
            ],
        },
        Panel {
            y_label: "facteur de charge".into(),
            series: vec![series("facteur de charge", |p| p.load_factor)],
        },
        Panel {
            y_label: "commandes (−1 à 1)".into(),
            series: vec![
                series("profondeur", |p| p.controls.pitch),
                series("ailerons", |p| p.controls.roll),
                series("direction", |p| p.controls.yaw),
            ],
        },
    ];
    draw(path, title, "temps (s)", &panels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draws_a_png_with_text() {
        let dir = std::env::temp_dir().join(format!("fdm_lab_plots_{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("essai.png");
        let panel = Panel {
            y_label: "altitude (m)".into(),
            series: vec![
                Series::line(
                    "modèle",
                    vec![(150.0, 0.0), (180.0, 2_000.0), (170.0, 4_000.0)],
                ),
                Series::dots("cible", vec![(192.0, 2_000.0)]),
            ],
        };
        draw(
            &path,
            "Vitesse maximale — essai",
            "vitesse (km/h)",
            &[panel],
        )
        .unwrap();
        let bytes = std::fs::read(&path).unwrap();
        assert_eq!(&bytes[1..4], b"PNG");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
