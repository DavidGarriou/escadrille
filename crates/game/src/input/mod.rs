//! Action layer and gamepad (skill `manette`; plan `m2-vol-essai.md`, step 3).
//!
//! The game reads actions, never buttons. The « Vol d'essai » profile maps them by
//! position on the 8BitDo SN30 Pro (labels of its Nintendo layout in brackets):
//! - left stick: stick, roll (X) and pitch (Y);
//! - L2 / R2: rudder left / right, all or nothing on this gamepad, hence a ramp;
//! - d-pad ↑ / ↓: throttle, 5 % a press, 30 % a second held;
//! - L1 / R1: elevator neutral, nose down / up, 1 % a press;
//! - right stick: free look; R3: look ahead again;
//! - west (Y): change view; north (X): telemetry; Select: start again; Start: pause;
//! - d-pad ← / →: next start: on the field, on final, in flight (`crate::flight`);
//! - Home: next realism preset (`crate::realism`).
//!
//! Keyboard, for development (physical keys, labels of an AZERTY keyboard): arrows for
//! the stick, Q / D for the rudder, Page up / down for the throttle, Home / End for the
//! neutral, V view, T telemetry, R start again, N next start, P pause, I preset; G draws
//! the forces (`crate::dev`).
//!
//! The profile is written here for now; RON profiles and remapping come later.

pub mod processing;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use escadrille_fdm::aircraft::PilotControls;

use crate::dev::capture::CaptureRequest;
use crate::flight::{FlightSet, PilotInput, RestartFlight, StartCommands};
use processing::{Lever, around_neutral, expo, radial_dead_zone, ramp};

/// Dead zone of the sticks (skill `manette`: starting values, to be tuned in flight).
pub const DEAD_ZONE: f32 = 0.08;
/// Response curve of the stick: 0.4 at first (skill `manette`), « un peu trop sensible »
/// for the player in the test flight (2026-10-29), hence 0.6: half the stick gives 27 % of
/// the travel instead of 35 %, the ends still the whole of it.
pub const STICK_EXPO: f32 = 0.6;
/// Rudder by buttons: full travel in 0.4 s, back to the centre in 0.2 s.
pub const RUDDER_RISE_S: f32 = 0.4;
pub const RUDDER_RETURN_S: f32 = 0.2;
/// Stick by keyboard keys: as the rudder.
pub const KEY_RISE_S: f32 = 0.4;
/// Free look at full deflection, rad/s.
pub const LOOK_RATE: f32 = 2.6;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputSet;

/// The discrete actions of the profile.
#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    ChangeView,
    LookAhead,
    Telemetry,
    Pause,
    NextPreset,
    NextStart,
}

/// The continuous state of the pilot's controls, between frames.
#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub struct Controls {
    pub rudder: f32,
    pub throttle: Lever,
    pub neutral: Lever,
    /// Stick by keyboard keys, after their ramps.
    keys: Vec2,
    /// Free look rate, rad/s (yaw right, pitch up).
    pub look: Vec2,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            rudder: 0.0,
            throttle: Lever::new(0.0, 0.0, 1.0, 0.05, 0.30, 0.25),
            neutral: Lever::new(0.0, -0.5, 0.5, 0.01, 0.10, 0.40),
            keys: Vec2::ZERO,
            look: Vec2::ZERO,
        }
    }
}

impl Controls {
    /// The levers where a flight's start puts them: its throttle and its elevator.
    pub fn starting_from(&mut self, start: &PilotControls) {
        self.throttle.value = start.throttle as f32;
        self.neutral.value = start.pitch as f32;
        self.rudder = 0.0;
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Controls>()
            .add_message::<Action>()
            .add_systems(
                Update,
                read_controls.in_set(InputSet).before(FlightSet::Place),
            );
    }
}

/// What the action layer writes: the pilot's commands and the discrete actions.
#[derive(SystemParam)]
pub struct Outputs<'w> {
    input: ResMut<'w, PilotInput>,
    actions: MessageWriter<'w, Action>,
    restart: MessageWriter<'w, RestartFlight>,
}

/// Reads the gamepad (the first one) and the keyboard, and writes the pilot's commands
/// and the discrete actions. A capture reads neither: the scene holds its commands, and a
/// gamepad left plugged in turned the camera by its drift, a few degrees from one capture
/// to the next.
fn read_controls(
    time: Res<Time<Real>>,
    capture: Option<Res<CaptureRequest>>,
    gamepads: Query<&Gamepad>,
    keys: Res<ButtonInput<KeyCode>>,
    mut controls: ResMut<Controls>,
    start: Res<StartCommands>,
    mut out: Outputs,
) {
    // Real time: the levers still move while the game is paused.
    let dt = time.delta_secs();
    let live = capture.is_none();
    let pad = gamepads.iter().next().filter(|_| live);
    let pressed = |b: GamepadButton| pad.is_some_and(|g| g.pressed(b));
    let just = |b: GamepadButton| pad.is_some_and(|g| g.just_pressed(b));
    let key = |k: KeyCode| live && keys.pressed(k);
    let key_just = |k: KeyCode| live && keys.just_pressed(k);

    // Stick: gamepad, or keyboard keys through a ramp.
    let stick = pad.map_or(Vec2::ZERO, |g| radial_dead_zone(g.left_stick(), DEAD_ZONE));
    let axis = |minus: KeyCode, plus: KeyCode| f32::from(key(plus)) - f32::from(key(minus));
    let c = &mut *controls;
    // At each start, the levers where the start holds the aircraft.
    if start.is_changed() {
        c.starting_from(&start.0);
    }
    c.keys.x = ramp(
        c.keys.x,
        axis(KeyCode::ArrowLeft, KeyCode::ArrowRight),
        dt,
        KEY_RISE_S,
        RUDDER_RETURN_S,
    );
    c.keys.y = ramp(
        c.keys.y,
        axis(KeyCode::ArrowDown, KeyCode::ArrowUp),
        dt,
        KEY_RISE_S,
        RUDDER_RETURN_S,
    );
    let roll = expo(stick.x, STICK_EXPO) + c.keys.x;
    // Stick pushed forward (+Y) is nose down.
    let pitch = -(expo(stick.y, STICK_EXPO) + c.keys.y);

    // Rudder from the triggers (or Q / D, physical A / D).
    let target = (f32::from(pressed(GamepadButton::RightTrigger2) || key(KeyCode::KeyD))
        - f32::from(pressed(GamepadButton::LeftTrigger2) || key(KeyCode::KeyA)))
    .clamp(-1.0, 1.0);
    c.rudder = ramp(c.rudder, target, dt, RUDDER_RISE_S, RUDDER_RETURN_S);

    // Throttle and neutral levers.
    let lever = |b: GamepadButton, k: KeyCode| (just(b) || key_just(k), pressed(b) || key(k));
    c.throttle.update(
        lever(GamepadButton::DPadDown, KeyCode::PageDown),
        lever(GamepadButton::DPadUp, KeyCode::PageUp),
        dt,
    );
    c.neutral.update(
        lever(GamepadButton::LeftTrigger, KeyCode::End),
        lever(GamepadButton::RightTrigger, KeyCode::Home),
        dt,
    );

    // Free look.
    let look = pad.map_or(Vec2::ZERO, |g| radial_dead_zone(g.right_stick(), DEAD_ZONE));
    c.look = Vec2::new(expo(look.x, STICK_EXPO), expo(look.y, STICK_EXPO)) * LOOK_RATE;

    let stick = Vec2::new(roll, pitch).clamp(Vec2::NEG_ONE, Vec2::ONE);
    *out.input = PilotInput {
        controls: PilotControls {
            roll: f64::from(stick.x),
            pitch: f64::from(around_neutral(stick.y, c.neutral.value)),
            yaw: f64::from(c.rudder),
            throttle: f64::from(c.throttle.value),
        },
        stick: stick.as_dvec2(),
    };

    for (button, k, action) in [
        (GamepadButton::West, KeyCode::KeyV, Action::ChangeView),
        (GamepadButton::RightThumb, KeyCode::KeyC, Action::LookAhead),
        (GamepadButton::North, KeyCode::KeyT, Action::Telemetry),
        (GamepadButton::Start, KeyCode::KeyP, Action::Pause),
        (GamepadButton::Mode, KeyCode::KeyI, Action::NextPreset),
        (GamepadButton::DPadRight, KeyCode::KeyN, Action::NextStart),
        (GamepadButton::DPadLeft, KeyCode::KeyN, Action::NextStart),
    ] {
        if just(button) || key_just(k) {
            out.actions.write(action);
        }
    }
    if just(GamepadButton::Select) || key_just(KeyCode::KeyR) {
        out.restart.write(RestartFlight);
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    /// The pilot's commands after one frame with the up arrow held, with or without a
    /// capture under way.
    fn commands_with_up_arrow(capturing: bool) -> PilotControls {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .init_resource::<Controls>()
            .init_resource::<PilotInput>()
            .insert_resource(StartCommands(PilotControls {
                roll: 0.0,
                pitch: -0.09,
                yaw: 0.0,
                throttle: 0.6,
            }))
            .add_message::<Action>()
            .add_message::<RestartFlight>()
            .add_systems(Update, read_controls);
        let mut keys = ButtonInput::<KeyCode>::default();
        keys.press(KeyCode::ArrowUp);
        app.insert_resource(keys);
        if capturing {
            app.insert_resource(CaptureRequest {
                scene: "test".to_owned(),
                frames: 1,
                size: (64, 64),
                png: PathBuf::from("test.png"),
                json: PathBuf::from("test.json"),
            });
        }
        // The clock's first update has no duration: the ramps move from the second.
        app.update();
        app.update();
        app.world().resource::<PilotInput>().controls
    }

    #[test]
    fn a_capture_holds_the_start_whatever_the_keyboard_or_gamepad() {
        // The levers are f32: the start's commands come back within their rounding.
        let close = |a: f64, b: f64| (a - b).abs() < 1e-6;
        let captured = commands_with_up_arrow(true);
        assert!(close(captured.pitch, -0.09), "{captured:?}");
        assert!(close(captured.throttle, 0.6), "{captured:?}");
        assert_eq!(captured.roll, 0.0);
        // Out of a capture, the same key pulls the stick.
        assert!(!close(commands_with_up_arrow(false).pitch, -0.09));
    }
}
