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
//! - Home: next realism preset (`crate::realism`).
//!
//! Keyboard, for development (physical keys, labels of an AZERTY keyboard): arrows for
//! the stick, Q / D for the rudder, Page up / down for the throttle, Home / End for the
//! neutral, V view, T telemetry, R start again, P pause, I preset.
//!
//! The profile is written here for now; RON profiles and remapping come later.

pub mod processing;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use escadrille_fdm::aircraft::PilotControls;

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
/// and the discrete actions.
fn read_controls(
    time: Res<Time<Real>>,
    gamepads: Query<&Gamepad>,
    keys: Res<ButtonInput<KeyCode>>,
    mut controls: ResMut<Controls>,
    start: Res<StartCommands>,
    mut out: Outputs,
) {
    // Real time: the levers still move while the game is paused.
    let dt = time.delta_secs();
    let pad = gamepads.iter().next();
    let pressed = |b: GamepadButton| pad.is_some_and(|g| g.pressed(b));
    let just = |b: GamepadButton| pad.is_some_and(|g| g.just_pressed(b));
    let key = |k: KeyCode| keys.pressed(k);
    let key_just = |k: KeyCode| keys.just_pressed(k);

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
    ] {
        if just(button) || key_just(k) {
            out.actions.write(action);
        }
    }
    if just(GamepadButton::Select) || key_just(KeyCode::KeyR) {
        out.restart.write(RestartFlight);
    }
}
