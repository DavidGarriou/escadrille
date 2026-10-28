//! The test flight's overlays (plan `m2-vol-essai.md`, step 6), in the debug UI
//! (`bevy_egui`): telemetry, the controls at start and in pause, the crash notice. The
//! game's own texts will move to Fluent files with its first real screens.

use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};
use escadrille_fdm::constants::{G0, RHO0};

use crate::flight::simulation::{ENVIRONMENT, Player, RESTART_AFTER_S, Simulated};
use crate::input::{Action, Controls, InputSet};
use crate::states::AppState;

/// How long the controls stay on screen after the start, s.
pub const HELP_S: f32 = 20.0;

#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub struct Overlays {
    pub telemetry: bool,
    /// Real time since the flight started, s.
    pub since_start: f32,
}

impl Default for Overlays {
    fn default() -> Self {
        Self {
            telemetry: true,
            since_start: 0.0,
        }
    }
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Overlays>()
            .add_systems(
                Update,
                toggles.after(InputSet).run_if(in_state(AppState::Vol)),
            )
            .add_systems(
                EguiPrimaryContextPass,
                overlays.run_if(in_state(AppState::Vol)),
            );
    }
}

/// Telemetry on and off, and the pause: the virtual clock stops, the simulation with it.
fn toggles(
    real: Res<Time<Real>>,
    mut actions: MessageReader<Action>,
    mut overlays: ResMut<Overlays>,
    mut time: ResMut<Time<Virtual>>,
) {
    overlays.since_start += real.delta_secs();
    for action in actions.read() {
        match action {
            Action::Telemetry => overlays.telemetry = !overlays.telemetry,
            Action::Pause if time.is_paused() => time.unpause(),
            Action::Pause => time.pause(),
            _ => {}
        }
    }
}

/// The figures of the telemetry panel, from the flight model.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Telemetry {
    pub true_kmh: f64,
    pub indicated_kmh: f64,
    pub altitude_m: f64,
    pub climb_ms: f64,
    pub rpm: f64,
    pub alpha_deg: f64,
    pub beta_deg: f64,
    pub load_factor: f64,
}

impl Telemetry {
    pub fn of(sim: &Simulated) -> Self {
        let s = &sim.state;
        let d = sim.aircraft.evaluate(s, &sim.controls, &ENVIRONMENT);
        let air = d.air_velocity;
        let speed = air.length();
        let mass = sim.aircraft.mass().mass();
        Self {
            true_kmh: speed * 3.6,
            indicated_kmh: speed * (d.density / RHO0).sqrt() * 3.6,
            altitude_m: -s.position_ned.z,
            climb_ms: -(s.attitude * s.velocity_body).z,
            rpm: s.rotor_speed * 60.0 / std::f64::consts::TAU,
            alpha_deg: air.z.atan2(air.x).to_degrees(),
            beta_deg: (air.y / speed.max(1e-6)).asin().to_degrees(),
            load_factor: -d.loads.wrench.force.z / (mass * G0),
        }
    }
}

fn overlays(
    mut contexts: EguiContexts,
    overlays: Res<Overlays>,
    controls: Res<Controls>,
    time: Res<Time<Virtual>>,
    player: Query<&Simulated, With<Player>>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let Ok(sim) = player.single() else {
        return Ok(());
    };
    if overlays.telemetry {
        let t = Telemetry::of(sim);
        let c = sim.controls;
        egui::Window::new("Télémétrie")
            .anchor(egui::Align2::RIGHT_TOP, [-12.0, 12.0])
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                egui::Grid::new("telemetrie").num_columns(2).show(ui, |ui| {
                    let mut row = |label: &str, value: String| {
                        ui.label(label);
                        ui.monospace(value);
                        ui.end_row();
                    };
                    row("Vitesse indiquée", format!("{:5.0} km/h", t.indicated_kmh));
                    row("Vitesse vraie", format!("{:5.0} km/h", t.true_kmh));
                    row("Altitude", format!("{:5.0} m", t.altitude_m));
                    row("Variomètre", format!("{:+5.1} m/s", t.climb_ms));
                    row("Régime", format!("{:5.0} tr/min", t.rpm));
                    row("Gaz", format!("{:5.0} %", 100.0 * c.throttle));
                    row("Incidence", format!("{:+5.1}°", t.alpha_deg));
                    row("Dérapage", format!("{:+5.1}°", t.beta_deg));
                    row("Facteur de charge", format!("{:+5.2}", t.load_factor));
                    row(
                        "Manche",
                        format!("roulis {:+.2}, tangage {:+.2}", c.roll, c.pitch),
                    );
                    row("Palonnier", format!("{:+.2}", c.yaw));
                    row(
                        "Neutre de profondeur",
                        format!("{:+.3}", controls.neutral.value),
                    );
                });
            });
    }
    if let Some(t) = sim.crashed {
        centred(
            ctx,
            "ecrase",
            &format!(
                "Écrasé. Nouveau départ dans {:.0} s.",
                (RESTART_AFTER_S - t).max(0.0).ceil()
            ),
        );
    } else if time.is_paused() {
        centred(ctx, "pause", "Pause");
    }
    if time.is_paused() || overlays.since_start < HELP_S {
        egui::Window::new("Commandes")
            .anchor(egui::Align2::LEFT_TOP, [12.0, 12.0])
            .resizable(false)
            .collapsible(false)
            .show(ctx, |ui| {
                egui::Grid::new("commandes").num_columns(2).show(ui, |ui| {
                    for (input, action) in HELP {
                        ui.monospace(*input);
                        ui.label(*action);
                        ui.end_row();
                    }
                });
            });
    }
    Ok(())
}

fn centred(ctx: &egui::Context, id: &str, text: &str) {
    egui::Area::new(egui::Id::new(id))
        .anchor(egui::Align2::CENTER_CENTER, [0.0, -80.0])
        .show(ctx, |ui| {
            ui.label(egui::RichText::new(text).size(28.0).strong());
        });
}

/// The « Vol d'essai » profile (`crate::input`), as the player reads it on the SN30 Pro.
const HELP: &[(&str, &str)] = &[
    ("Stick gauche", "manche : roulis et tangage"),
    ("L2 / R2", "palonnier à gauche / à droite"),
    ("Croix ↑ / ↓", "gaz (appui : 5 %, maintien : 30 %/s)"),
    ("L1 / R1", "neutre de profondeur : piqué / cabré"),
    ("Stick droit", "regarder autour"),
    ("R3", "regarder devant"),
    ("Y", "vue : poursuite ou pilote"),
    ("X", "télémétrie"),
    ("Select", "nouveau départ"),
    ("Start", "pause (affiche ces commandes)"),
];
