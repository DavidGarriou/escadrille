//! The flight model's step and the aircraft's place in the rendered world.

use std::sync::Arc;

use bevy::math::{DQuat, DVec3};
use bevy::prelude::*;
use escadrille_fdm::aircraft::{Aircraft, PilotControls};
use escadrille_fdm::environment::StandardEnvironment;
use escadrille_fdm::ground::ContactPoint;
use escadrille_fdm::rigid_body::RigidBodyState;

use super::bridge::{render_transform, to_bevy};
use super::{PilotInput, RestartFlight, StartCommands, Starts};
use crate::input::Action;

/// Still air over flat ground at sea level: the plain of the test flight.
pub const ENVIRONMENT: StandardEnvironment = StandardEnvironment {
    wind_ned: DVec3::ZERO,
    ground_altitude: 0.0,
};

/// Seconds between an accident and the new start.
pub const RESTART_AFTER_S: f32 = 3.0;

/// Step of the floating origin's grid, m: the render origin stays on the ground, on a
/// grid point under the player's aircraft, so that rendered positions stay small in f32.
/// The ground's texture repeats on the same grid (`crate::world`), so that a jump of the
/// origin does not show.
pub const ORIGIN_GRID_M: f64 = 2000.0;

/// The aircraft the camera follows and the render origin stays under.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Player;

/// What ended the flight (plan `m2-train-sol.md`, decision 4). The real damage comes with
/// the M4: for now, the flight starts again.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Accident {
    /// A blade struck the ground.
    Propeller,
    /// A wing tip, the nose or the tail hit it too hard.
    Airframe,
    /// A leg or the skid gave way.
    Undercarriage,
    /// The simulation left the finite numbers.
    Broken,
}

impl Accident {
    /// What the player reads.
    pub fn text(self) -> &'static str {
        match self {
            Self::Propeller => "Hélice brisée au sol.",
            Self::Airframe => "Cellule brisée au sol.",
            Self::Undercarriage => "Train brisé.",
            Self::Broken => "Vol interrompu.",
        }
    }
}

/// An accident, and the seconds since.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Crash {
    pub cause: Accident,
    pub seconds: f32,
}

/// How much more than its limits a part takes before it gives way (a game rule, set by
/// the realism preset: `crate::realism`).
#[derive(Resource, Debug, Clone, Copy, PartialEq)]
pub struct LandingTolerance(pub f64);

impl Default for LandingTolerance {
    fn default() -> Self {
        Self(1.0)
    }
}

/// An aircraft flown by the flight model.
#[derive(Component, Debug, Clone)]
pub struct Simulated {
    pub aircraft: Arc<Aircraft>,
    /// State of the scenario's start.
    pub start: RigidBodyState,
    pub previous: RigidBodyState,
    pub state: RigidBodyState,
    /// The commands of the next step, then of the last one.
    pub controls: PilotControls,
    /// The accident that stopped the flight, while it waits for the restart.
    pub crashed: Option<Crash>,
}

impl Simulated {
    /// What this state breaks, if anything.
    pub fn accident(&self, tolerance: f64) -> Option<Accident> {
        let s = &self.state;
        let finite = s.position_ned.is_finite()
            && s.attitude.is_finite()
            && s.velocity_body.is_finite()
            && s.angular_velocity_body.is_finite();
        if !finite {
            return Some(Accident::Broken);
        }
        let clearance = self.aircraft.propeller_clearance(s, &ENVIRONMENT);
        if clearance.is_some_and(|c| c < 0.0) {
            return Some(Accident::Propeller);
        }
        let gear = self.aircraft.undercarriage()?;
        let point = gear.overloaded(&gear.loads(s, &ENVIRONMENT), tolerance)?;
        Some(match point {
            ContactPoint::HardPoint(_) => Accident::Airframe,
            ContactPoint::Wheel(_) | ContactPoint::Skid(_) => Accident::Undercarriage,
        })
    }

    /// Back to the scenario's start.
    pub fn restart(&mut self) {
        self.previous = self.start;
        self.state = self.start;
        self.crashed = None;
    }
}

/// Render origin, NED, m: on the ground, on the grid of [`ORIGIN_GRID_M`].
#[derive(Resource, Debug, Clone, Copy, PartialEq, Default)]
pub struct RenderOrigin(pub DVec3);

impl RenderOrigin {
    /// The grid point under a position.
    pub fn under(position_ned: DVec3) -> Self {
        let snap = |v: f64| (v / ORIGIN_GRID_M).round() * ORIGIN_GRID_M;
        Self(DVec3::new(snap(position_ned.x), snap(position_ned.y), 0.0))
    }
}

/// The player's aircraft takes the pilot's commands for the next step.
pub fn take_input(input: Res<PilotInput>, mut aircraft: Query<&mut Simulated, With<Player>>) {
    for mut sim in &mut aircraft {
        sim.controls = input.controls;
    }
}

/// One step of the flight model for each aircraft still in one piece, with its commands.
pub fn step(
    time: Res<Time<Fixed>>,
    tolerance: Res<LandingTolerance>,
    mut aircraft: Query<&mut Simulated>,
) {
    let dt = time.delta_secs_f64();
    for mut sim in &mut aircraft {
        if sim.crashed.is_some() {
            continue;
        }
        let state = sim
            .aircraft
            .advance(&sim.state, &sim.controls, &ENVIRONMENT, dt);
        sim.previous = sim.state;
        sim.state = state;
        if let Some(cause) = sim.accident(tolerance.0) {
            sim.state = sim.previous;
            sim.crashed = Some(Crash {
                cause,
                seconds: 0.0,
            });
        }
    }
}

/// The next of the starts, on the player's demand: Select then takes it.
pub fn choose_start(mut actions: MessageReader<Action>, mut starts: ResMut<Starts>) {
    for action in actions.read() {
        if *action == Action::NextStart && !starts.list.is_empty() {
            starts.chosen = (starts.chosen + 1) % starts.list.len();
        }
    }
}

/// After [`RESTART_AFTER_S`] since an accident, or when asked, back to the chosen start.
pub fn restart(
    time: Res<Time>,
    mut asked: MessageReader<RestartFlight>,
    starts: Res<Starts>,
    mut start: ResMut<StartCommands>,
    mut aircraft: Query<&mut Simulated>,
) {
    let now = asked.read().count() > 0;
    for mut sim in &mut aircraft {
        if let Some(crash) = sim.crashed.as_mut() {
            crash.seconds += time.delta_secs();
        }
        let due = sim.crashed.is_some_and(|c| c.seconds >= RESTART_AFTER_S);
        if now || due {
            if let Some(chosen) = starts.current() {
                sim.start = chosen.state;
                start.0 = chosen.controls;
            }
            sim.restart();
            start.set_changed();
        }
    }
}

/// Each aircraft between its last two states, at the fraction of the fixed step already
/// run, relative to the render origin; the origin follows the player.
pub fn place(
    time: Res<Time<Fixed>>,
    mut origin: ResMut<RenderOrigin>,
    mut aircraft: Query<(&Simulated, &mut Transform, Has<Player>)>,
) {
    let a = time.overstep_fraction_f64().clamp(0.0, 1.0);
    let pose = |sim: &Simulated| -> (DVec3, DQuat) {
        (
            sim.previous.position_ned.lerp(sim.state.position_ned, a),
            sim.previous.attitude.slerp(sim.state.attitude, a),
        )
    };
    if let Some((sim, _, _)) = aircraft.iter().find(|(_, _, player)| *player) {
        let under = RenderOrigin::under(pose(sim).0);
        if under != *origin {
            *origin = under;
        }
    }
    for (sim, mut transform, _) in &mut aircraft {
        let (position, attitude) = pose(sim);
        *transform = render_transform(position, attitude, to_bevy(origin.0));
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::time::Duration;

    use bevy::time::TimeUpdateStrategy;
    use escadrille_data::aircraft::{Loading, load_aircraft};
    use escadrille_data::scenario::load_scenario;

    use super::*;
    use crate::ASSETS_DIR;
    use crate::flight::{FlightStart, ON_THE_FIELD, STEP_HZ, TEST_FLIGHT};

    /// An app with the flight's systems and none of the rendering, its time advanced by
    /// one fixed step per update.
    fn app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(Time::<Fixed>::from_hz(STEP_HZ))
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
                1.0 / STEP_HZ,
            )))
            .init_resource::<PilotInput>()
            .init_resource::<StartCommands>()
            .init_resource::<Starts>()
            .init_resource::<LandingTolerance>()
            .init_resource::<RenderOrigin>()
            .add_message::<RestartFlight>()
            .add_message::<Action>()
            .add_systems(FixedUpdate, (take_input, step).chain())
            .add_systems(Update, (choose_start, restart, place).chain());
        app
    }

    fn spad(app: &mut App) -> Entity {
        spad_at(app, TEST_FLIGHT)
    }

    fn spad_at(app: &mut App, name: &str) -> Entity {
        let assets = Path::new(ASSETS_DIR);
        let scenario = load_scenario(assets, name).unwrap();
        let data = load_aircraft(assets, &scenario.aircraft).unwrap();
        let built = data
            .build(&Loading {
                fuel_fraction: scenario.fuel_fraction,
            })
            .unwrap();
        let start = scenario.initial_state();
        app.insert_resource(PilotInput::holding(scenario.commands()));
        app.world_mut()
            .spawn((
                Player,
                Simulated {
                    aircraft: Arc::new(built.aircraft),
                    start,
                    previous: start,
                    state: start,
                    controls: scenario.commands(),
                    crashed: None,
                },
                Transform::default(),
            ))
            .id()
    }

    #[test]
    fn the_test_flight_holds_its_level_flight_for_a_minute() {
        let mut app = app();
        let spad = spad(&mut app);
        let started = std::time::Instant::now();
        for _ in 0..60 * 120 {
            app.update();
        }
        let elapsed = started.elapsed();
        let sim = app.world().get::<Simulated>(spad).unwrap();
        assert!(sim.crashed.is_none());
        let climbed = sim.start.position_ned.z - sim.state.position_ned.z;
        let flown = sim.state.position_ned - sim.start.position_ned;
        assert!(climbed.abs() < 2.0, "{climbed} m");
        // East at 160 km/h for a minute: about 2.67 km.
        assert!(
            (flown.y - 2666.7).abs() < 20.0 && flown.x.abs() < 50.0,
            "{flown}"
        );
        // The render origin followed, on its grid, and the aircraft is drawn near it.
        let origin = app.world().resource::<RenderOrigin>().0;
        assert_eq!(origin, DVec3::new(0.0, 2000.0, 0.0));
        let drawn = app.world().get::<Transform>(spad).unwrap().translation;
        // Within half a grid step of the origin across, at its altitude above it.
        assert!(
            drawn.x.hypot(drawn.z) < 0.5 * std::f32::consts::SQRT_2 * 2000.0,
            "{drawn}"
        );
        assert!((drawn.y - 1000.0).abs() < 2.0, "{drawn}");
        eprintln!("60 s of flight in {elapsed:?}");
    }

    #[test]
    fn touching_the_ground_waits_then_starts_again() {
        let mut app = app();
        let spad = spad(&mut app);
        // Full nose down, full throttle: into the ground from 1 000 m.
        app.insert_resource(PilotInput::holding(PilotControls {
            pitch: -1.0,
            throttle: 1.0,
            ..default()
        }));
        let mut steps = 0;
        while app
            .world()
            .get::<Simulated>(spad)
            .unwrap()
            .crashed
            .is_none()
        {
            app.update();
            steps += 1;
            assert!(steps < 120 * 120, "still flying after two minutes");
        }
        let sim = app.world().get::<Simulated>(spad).unwrap();
        assert!(
            sim.state.position_ned.z > -10.0,
            "{}",
            sim.state.position_ned
        );
        for _ in 0..(RESTART_AFTER_S * 120.0) as usize + 2 {
            app.update();
        }
        let sim = app.world().get::<Simulated>(spad).unwrap();
        assert!(sim.crashed.is_none());
        assert!(
            sim.state.position_ned.z < -900.0,
            "{}",
            sim.state.position_ned
        );
    }

    #[test]
    fn on_the_field_it_stands_still_and_rolls_when_the_throttle_opens() {
        let mut app = app();
        let spad = spad_at(&mut app, ON_THE_FIELD);
        for _ in 0..3 * 120 {
            app.update();
        }
        let sim = app.world().get::<Simulated>(spad).unwrap();
        assert!(sim.crashed.is_none(), "{:?}", sim.crashed);
        let moved = sim.state.position_ned - sim.start.position_ned;
        assert!(moved.length() < 0.05, "{moved}");
        app.insert_resource(PilotInput::holding(PilotControls {
            throttle: 1.0,
            pitch: -0.5,
            ..default()
        }));
        for _ in 0..5 * 120 {
            app.update();
        }
        let sim = app.world().get::<Simulated>(spad).unwrap();
        assert!(sim.crashed.is_none(), "{:?}", sim.crashed);
        let moved = sim.state.position_ned - sim.start.position_ned;
        assert!(moved.y > 5.0, "it rolled east: {moved}");
    }

    #[test]
    fn a_drop_onto_the_field_breaks_the_undercarriage_or_the_aeroplane() {
        let mut app = app();
        let spad = spad_at(&mut app, ON_THE_FIELD);
        {
            let mut sim = app.world_mut().get_mut::<Simulated>(spad).unwrap();
            sim.state.position_ned.z -= 3.0;
            sim.state.velocity_body = sim.state.attitude.inverse() * DVec3::Z * 8.0;
            sim.previous = sim.state;
        }
        for _ in 0..120 {
            app.update();
        }
        let sim = app.world().get::<Simulated>(spad).unwrap();
        let cause = sim.crashed.expect("8 m/s is more than it takes").cause;
        assert_ne!(cause, Accident::Broken);
    }

    #[test]
    fn the_player_chooses_the_next_start() {
        let mut app = app();
        let spad = spad(&mut app);
        let elsewhere = RigidBodyState {
            position_ned: DVec3::new(0.0, -700.0, -150.0),
            ..RigidBodyState::default()
        };
        app.insert_resource(Starts {
            list: vec![
                FlightStart {
                    label: "ici",
                    state: app.world().get::<Simulated>(spad).unwrap().start,
                    controls: PilotControls::default(),
                },
                FlightStart {
                    label: "ailleurs",
                    state: elsewhere,
                    controls: PilotControls {
                        throttle: 0.3,
                        ..default()
                    },
                },
            ],
            chosen: 0,
        });
        app.world_mut().write_message(Action::NextStart);
        app.world_mut().write_message(RestartFlight);
        app.update();
        let sim = app.world().get::<Simulated>(spad).unwrap();
        assert_eq!(sim.start, elsewhere);
        assert_eq!(app.world().resource::<StartCommands>().0.throttle, 0.3);
    }

    #[test]
    fn the_origin_snaps_to_its_grid() {
        assert_eq!(
            RenderOrigin::under(DVec3::new(2999.0, -1001.0, -1000.0)).0,
            DVec3::new(2000.0, -2000.0, 0.0)
        );
        assert_eq!(
            RenderOrigin::under(DVec3::new(900.0, 0.0, 0.0)).0,
            DVec3::ZERO
        );
    }
}
