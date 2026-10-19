//! Gamepad lab (`just labo-manette`): raw and processed axes, buttons, trigger
//! histograms (analog or digital?), rumble test, and a text report for Claude.
//!
//! Dev-tool labels are in English, like identifiers; player-facing text goes
//! through Fluent once the UI exists.

pub mod stats;

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::Path;
use std::time::Duration;

use bevy::input::gamepad::{
    GamepadInput, GamepadRumbleIntensity, GamepadRumbleRequest, RawGamepadAxisChangedEvent,
    RawGamepadButtonChangedEvent,
};
use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use crate::states::AppState;
use stats::{BINS, TriggerHistogram, TriggerVerdict};

/// Where the lab writes its report.
pub const REPORT_PATH: &str = "target/labo-manette/rapport.txt";

/// Inputs that may carry the analog value of L2/R2, depending on the driver.
const TRIGGER_INPUTS: [GamepadInput; 4] = [
    GamepadInput::Button(GamepadButton::LeftTrigger2),
    GamepadInput::Button(GamepadButton::RightTrigger2),
    GamepadInput::Axis(GamepadAxis::LeftZ),
    GamepadInput::Axis(GamepadAxis::RightZ),
];

/// Raw values (before Bevy's dead zones and thresholds) and trigger histograms.
#[derive(Resource, Default)]
struct LabData {
    raw: HashMap<(Entity, GamepadInput), f32>,
    histograms: HashMap<(Entity, GamepadInput), TriggerHistogram>,
    status: String,
}

pub struct LaboManettePlugin;

impl Plugin for LaboManettePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LabData>()
            .add_systems(
                Update,
                (record_raw_inputs, quit_on_escape).run_if(in_state(AppState::LaboManette)),
            )
            .add_systems(
                EguiPrimaryContextPass,
                lab_ui.run_if(in_state(AppState::LaboManette)),
            );
    }
}

fn record_raw_inputs(
    mut data: ResMut<LabData>,
    mut buttons: MessageReader<RawGamepadButtonChangedEvent>,
    mut axes: MessageReader<RawGamepadAxisChangedEvent>,
) {
    let changes = buttons
        .read()
        .map(|e| (e.gamepad, GamepadInput::Button(e.button), e.value))
        .chain(
            axes.read()
                .map(|e| (e.gamepad, GamepadInput::Axis(e.axis), e.value)),
        );
    for (gamepad, input, value) in changes {
        data.raw.insert((gamepad, input), value);
        if TRIGGER_INPUTS.contains(&input) {
            data.histograms
                .entry((gamepad, input))
                .or_default()
                .record(value);
        }
    }
}

fn quit_on_escape(keys: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}

fn lab_ui(
    mut contexts: EguiContexts,
    mut data: ResMut<LabData>,
    gamepads: Query<(Entity, &Gamepad, Option<&Name>)>,
    mut rumble: MessageWriter<GamepadRumbleRequest>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    egui::Window::new("Gamepad lab")
        .default_pos([16.0, 16.0])
        .default_size([900.0, 820.0])
        .show(ctx, |ui| {
            ui.label(
                "Esc: quit. Pull L2 and R2 slowly, several times, to test whether they are analog.",
            );
            if ui.button(format!("Save report to {REPORT_PATH}")).clicked() {
                let report = report_text(&data, &gamepads);
                data.status = match write_report(Path::new(REPORT_PATH), &report) {
                    Ok(()) => format!("Report saved to {REPORT_PATH}"),
                    Err(err) => format!("Cannot save report: {err}"),
                };
            }
            if !data.status.is_empty() {
                ui.label(&data.status);
            }
            ui.separator();

            if gamepads.is_empty() {
                ui.heading("No gamepad detected: connect the SN30 Pro (USB cable, or Bluetooth if supported).");
                return;
            }

            egui::ScrollArea::vertical().show(ui, |ui| {
                for (entity, gamepad, name) in &gamepads {
                    gamepad_section(ui, entity, gamepad, name, &data, &mut rumble);
                    ui.separator();
                }
            });
        });
    Ok(())
}

fn gamepad_section(
    ui: &mut egui::Ui,
    entity: Entity,
    gamepad: &Gamepad,
    name: Option<&Name>,
    data: &LabData,
    rumble: &mut MessageWriter<GamepadRumbleRequest>,
) {
    ui.heading(name.map_or("(unnamed gamepad)", |n| n.as_str()));
    ui.label(format!(
        "vendor {} · product {}",
        hex_id(gamepad.vendor_id()),
        hex_id(gamepad.product_id())
    ));

    ui.horizontal(|ui| {
        ui.label("Rumble:");
        let mut add = |label: &str, intensity: GamepadRumbleIntensity| {
            if ui.button(label).clicked() {
                rumble.write(GamepadRumbleRequest::Add {
                    gamepad: entity,
                    intensity,
                    duration: Duration::from_millis(600),
                });
            }
        };
        add("weak motor", GamepadRumbleIntensity::WEAK_MAX);
        add("strong motor", GamepadRumbleIntensity::STRONG_MAX);
        add(
            "both, 25 %",
            GamepadRumbleIntensity {
                strong_motor: 0.25,
                weak_motor: 0.25,
            },
        );
        add("both, max", GamepadRumbleIntensity::MAX);
        if ui.button("stop").clicked() {
            rumble.write(GamepadRumbleRequest::Stop { gamepad: entity });
        }
    });

    ui.columns(2, |cols| {
        cols[0].strong("Axes (raw → processed by Bevy)");
        egui::Grid::new(("axes", entity))
            .striped(true)
            .show(&mut cols[0], |ui| {
                for axis in GamepadAxis::all() {
                    ui.label(format!("{axis:?}"));
                    ui.label(fmt_value(data.raw.get(&(entity, axis.into())).copied()));
                    ui.label(fmt_value(gamepad.get(axis)));
                    ui.end_row();
                }
            });

        cols[1].strong("Buttons (pressed · raw → processed)");
        egui::Grid::new(("buttons", entity))
            .striped(true)
            .show(&mut cols[1], |ui| {
                for button in GamepadButton::all() {
                    ui.label(format!("{button:?}"));
                    ui.label(if gamepad.pressed(button) {
                        "■"
                    } else {
                        "□"
                    });
                    ui.label(fmt_value(data.raw.get(&(entity, button.into())).copied()));
                    ui.label(fmt_value(gamepad.get(button)));
                    ui.end_row();
                }
            });
    });

    for input in TRIGGER_INPUTS {
        if let Some(histogram) = data.histograms.get(&(entity, input)) {
            ui.add_space(6.0);
            ui.strong(format!(
                "{} — {} samples, {} intermediate levels → {}",
                input_name(input),
                histogram.total(),
                histogram.intermediate_levels(),
                verdict_text(histogram.verdict())
            ));
            draw_histogram(ui, histogram);
        }
    }
}

fn draw_histogram(ui: &mut egui::Ui, histogram: &TriggerHistogram) {
    let bar_width = 18.0;
    let height = 80.0;
    let (rect, _) = ui.allocate_exact_size(
        egui::vec2(bar_width * (BINS + 1) as f32, height),
        egui::Sense::hover(),
    );
    let painter = ui.painter_at(rect);
    painter.rect_filled(rect, 2.0, egui::Color32::from_gray(30));
    let max = histogram.counts().iter().copied().max().unwrap_or(0).max(1) as f32;
    for (i, &count) in histogram.counts().iter().enumerate() {
        if count == 0 {
            continue;
        }
        // Square root scale, so that rare intermediate values stay visible next to 0 and 1.
        let h = height * (count as f32 / max).sqrt();
        let x = rect.left() + i as f32 * bar_width;
        let bar = egui::Rect::from_min_max(
            egui::pos2(x + 2.0, rect.bottom() - h),
            egui::pos2(x + bar_width - 2.0, rect.bottom()),
        );
        painter.rect_filled(bar, 1.0, egui::Color32::from_rgb(214, 196, 150));
    }
}

/// Plain-text report, read by Claude to settle open question 5 (triggers, rumble).
fn report_text(data: &LabData, gamepads: &Query<(Entity, &Gamepad, Option<&Name>)>) -> String {
    let mut out = String::from("Panache blanc — gamepad lab report\n\n");
    if gamepads.is_empty() {
        out.push_str("No gamepad detected.\n");
    }
    for (entity, gamepad, name) in gamepads {
        let _ = writeln!(
            out,
            "Gamepad: {} (vendor {}, product {})",
            name.map_or("(unnamed)", |n| n.as_str()),
            hex_id(gamepad.vendor_id()),
            hex_id(gamepad.product_id())
        );
        let raw: BTreeMap<String, f32> = data
            .raw
            .iter()
            .filter(|((e, _), _)| *e == entity)
            .map(|((_, input), v)| (input_name(*input), *v))
            .collect();
        let _ = writeln!(out, "Inputs seen (last raw value): {raw:?}");
        for input in TRIGGER_INPUTS {
            if let Some(h) = data.histograms.get(&(entity, input)) {
                let _ = writeln!(
                    out,
                    "{}: {} samples, {} intermediate levels, verdict {:?}, counts per 1/{BINS}: {:?}",
                    input_name(input),
                    h.total(),
                    h.intermediate_levels(),
                    h.verdict(),
                    h.counts()
                );
            }
        }
        out.push('\n');
    }
    out.push_str("Rumble: to be filled in by the player (felt / not felt, per button).\n");
    out
}

fn write_report(path: &Path, text: &str) -> std::io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(path, text)
}

fn input_name(input: GamepadInput) -> String {
    match input {
        GamepadInput::Axis(axis) => format!("axis {axis:?}"),
        GamepadInput::Button(button) => format!("button {button:?}"),
    }
}

fn verdict_text(verdict: TriggerVerdict) -> &'static str {
    match verdict {
        TriggerVerdict::NotEnoughData => "keep pressing",
        TriggerVerdict::Progressive => "PROGRESSIVE (analog)",
        TriggerVerdict::AllOrNothing => "ALL OR NOTHING (digital)",
        TriggerVerdict::Unclear => "unclear: pull slowly",
    }
}

fn fmt_value(value: Option<f32>) -> String {
    value.map_or_else(|| "—".to_owned(), |v| format!("{v:+.3}"))
}

fn hex_id(id: Option<u16>) -> String {
    id.map_or_else(|| "?".to_owned(), |v| format!("{v:#06x}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_values_and_ids() {
        assert_eq!(fmt_value(None), "—");
        assert_eq!(fmt_value(Some(0.5)), "+0.500");
        assert_eq!(hex_id(Some(0x2dc8)), "0x2dc8");
        assert_eq!(hex_id(None), "?");
    }

    #[test]
    fn writes_report_in_new_directory() {
        let dir = std::env::temp_dir().join(format!("escadrille-lab-{}", std::process::id()));
        let path = dir.join("sub/rapport.txt");
        write_report(&path, "ok").unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "ok");
        std::fs::remove_dir_all(dir).unwrap();
    }
}
