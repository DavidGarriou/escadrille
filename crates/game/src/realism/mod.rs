//! Realism (ADR 0003, skill `realisme`; plan `m2-aides-pilotage.md`): the settings, the
//! presets of `docs/vision.md`, and the flight assists ([`assist`]).
//!
//! No `if arcade`: each setting is a field of [`RealismSettings`], read by one system;
//! the presets only fill the fields. The Home button cycles them in flight.

pub mod assist;

use std::path::Path;
use std::sync::Arc;

use bevy::prelude::*;
use escadrille_data::assist::load_assist_gains;

use crate::ASSETS_DIR;
use crate::flight::simulation::{ENVIRONMENT, Player, Simulated};
use crate::flight::{AircraftId, FlightSet, PilotInput, StartCommands};
use crate::input::{Action, InputSet};
use assist::{AssistState, AssistTable, Measured};

/// The flight assist: it moves the commands as a pilot would, from what a pilot reads
/// and feels. Physical at every setting: the aeroplane stays the same.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlightAssist {
    /// Virtual instructor: the stick asks for a bank and a load factor; coordinated and
    /// guarded against the stall.
    Instructor,
    /// The rudder cancels the sideslip; with `stall_guard`, the elevator stops short of
    /// the stall.
    Coordinated {
        stall_guard: bool,
    },
    Off,
}

/// Realism settings. Every field is read by exactly one system. Fields marked
/// NON-PHYSICAL change the simulation away from reality (none yet).
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct RealismSettings {
    /// Read by the assists' system ([`assist_commands`]).
    pub flight_assist: FlightAssist,
}

/// The presets of `docs/vision.md`, « Assistance de pilotage ».
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Preset {
    /// « Wings »: the virtual instructor.
    Arcade,
    /// « Red Baron »: coordination and the stall guard. The default (player, 2026-10-29).
    #[default]
    Pilot,
    /// Coordination alone.
    Veteran,
    /// « IL-2 »: none.
    Ace,
}

impl Preset {
    pub fn settings(self) -> RealismSettings {
        let flight_assist = match self {
            Self::Arcade => FlightAssist::Instructor,
            Self::Pilot => FlightAssist::Coordinated { stall_guard: true },
            Self::Veteran => FlightAssist::Coordinated { stall_guard: false },
            Self::Ace => FlightAssist::Off,
        };
        RealismSettings { flight_assist }
    }

    pub fn next(self) -> Self {
        match self {
            Self::Arcade => Self::Pilot,
            Self::Pilot => Self::Veteran,
            Self::Veteran => Self::Ace,
            Self::Ace => Self::Arcade,
        }
    }

    /// Its name, as the player reads it.
    pub fn name(self) -> &'static str {
        match self {
            Self::Arcade => "Arcade",
            Self::Pilot => "Pilote",
            Self::Veteran => "Vétéran",
            Self::Ace => "As",
        }
    }

    /// What it does, in one line.
    pub fn summary(self) -> &'static str {
        match self {
            Self::Arcade => {
                "instructeur virtuel : le manche demande inclinaison et facteur de charge"
            }
            Self::Pilot => "palonnier automatique et garde anti-décrochage",
            Self::Veteran => "palonnier automatique",
            Self::Ace => "aucune aide",
        }
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RealismSet {
    /// `FixedUpdate`, between the pilot's commands and the step.
    Assist,
}

/// The assists of an aircraft: its gains, if it has some, and their memory.
#[derive(Component, Debug, Clone)]
pub struct Assist {
    table: Option<Arc<AssistTable>>,
    state: AssistState,
}

pub struct RealismPlugin;

impl Plugin for RealismPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Preset>()
            .insert_resource(Preset::default().settings())
            .configure_sets(
                FixedUpdate,
                RealismSet::Assist
                    .after(FlightSet::Command)
                    .before(FlightSet::Step),
            )
            .add_systems(
                FixedUpdate,
                (attach_assist, assist_commands)
                    .chain()
                    .in_set(RealismSet::Assist),
            )
            .add_systems(Update, change_preset.after(InputSet));
    }
}

/// The next preset, on the player's demand.
fn change_preset(
    mut actions: MessageReader<Action>,
    mut preset: ResMut<Preset>,
    mut settings: ResMut<RealismSettings>,
) {
    for action in actions.read() {
        if *action == Action::NextPreset {
            *preset = preset.next();
            *settings = preset.settings();
        }
    }
}

/// The player's aircraft not yet given its assists.
type WithoutAssist = (With<Player>, Without<Assist>);

/// Gives the player's aircraft its assists, from its gains file.
fn attach_assist(
    mut commands: Commands,
    aircraft: Query<(Entity, &AircraftId), WithoutAssist>,
) -> Result {
    for (entity, id) in &aircraft {
        let table = load_assist_gains(Path::new(ASSETS_DIR), &id.0)?
            .map(|file| Arc::new(AssistTable::from(&file)));
        if table.is_none() {
            warn!("{}: no assist_gains.ron, no flight assist", id.0);
        }
        commands.entity(entity).insert(Assist {
            table,
            state: AssistState::default(),
        });
    }
    Ok(())
}

/// The commands of the player's next step, through the assist of the settings.
fn assist_commands(
    time: Res<Time<Fixed>>,
    settings: Res<RealismSettings>,
    input: Res<PilotInput>,
    start: Res<StartCommands>,
    mut aircraft: Query<(&mut Simulated, &mut Assist), With<Player>>,
) {
    let dt = time.delta_secs_f64();
    for (mut sim, mut assist) in &mut aircraft {
        let Assist { table, state } = &mut *assist;
        if start.is_changed() || sim.crashed.is_some() {
            *state = AssistState::default();
        }
        let Some(table) = table else {
            continue;
        };
        if sim.crashed.is_none() {
            let measured = Measured::of(&sim.state, &ENVIRONMENT);
            sim.controls = state.commands(settings.flight_assist, &input, &measured, table, dt);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::math::DVec2;
    use bevy::time::TimeUpdateStrategy;
    use escadrille_data::aircraft::{Loading, load_aircraft};
    use escadrille_data::scenario::load_scenario;

    use super::*;
    use crate::flight::simulation::{step, take_input};
    use crate::flight::{STEP_HZ, TEST_FLIGHT};

    #[test]
    fn home_cycles_the_four_presets() {
        let mut preset = Preset::default();
        assert_eq!(preset, Preset::Pilot);
        let mut seen = Vec::new();
        for _ in 0..4 {
            seen.push(preset.settings().flight_assist);
            preset = preset.next();
        }
        assert_eq!(preset, Preset::Pilot);
        assert_eq!(
            seen,
            [
                FlightAssist::Coordinated { stall_guard: true },
                FlightAssist::Coordinated { stall_guard: false },
                FlightAssist::Off,
                FlightAssist::Instructor,
            ]
        );
    }

    /// The flight's fixed-step systems with the assists between them, the SPAD of the
    /// test flight, its stick to the right.
    fn flight(preset: Preset) -> (App, Entity) {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
            .insert_resource(Time::<Fixed>::from_hz(STEP_HZ))
            .insert_resource(TimeUpdateStrategy::ManualDuration(Duration::from_secs_f64(
                1.0 / STEP_HZ,
            )))
            .insert_resource(preset.settings())
            .init_resource::<StartCommands>()
            .configure_sets(
                FixedUpdate,
                (FlightSet::Command, RealismSet::Assist, FlightSet::Step).chain(),
            )
            .add_systems(
                FixedUpdate,
                (
                    take_input.in_set(FlightSet::Command),
                    (attach_assist, assist_commands)
                        .chain()
                        .in_set(RealismSet::Assist),
                    step.in_set(FlightSet::Step),
                ),
            );
        let assets = Path::new(ASSETS_DIR);
        let scenario = load_scenario(assets, TEST_FLIGHT).unwrap();
        let built = load_aircraft(assets, &scenario.aircraft)
            .unwrap()
            .build(&Loading {
                fuel_fraction: scenario.fuel_fraction,
            })
            .unwrap();
        let start = scenario.initial_state();
        let mut input = PilotInput::holding(scenario.commands());
        input.controls.roll = 1.0;
        input.stick = DVec2::new(1.0, 0.0);
        app.insert_resource(input);
        let entity = app
            .world_mut()
            .spawn((
                Player,
                AircraftId(scenario.aircraft.clone()),
                Simulated {
                    aircraft: Arc::new(built.aircraft),
                    start,
                    previous: start,
                    state: start,
                    controls: scenario.commands(),
                    extremities: Vec::new(),
                    crashed: None,
                },
            ))
            .id();
        (app, entity)
    }

    #[test]
    fn the_assist_sits_between_the_pilot_and_the_step() {
        // As: the flight model receives the pilot's full aileron, as it is.
        let (mut app, spad) = flight(Preset::Ace);
        for _ in 0..10 {
            app.update();
        }
        assert!(app.world().get::<Assist>(spad).is_some());
        let sim = app.world().get::<Simulated>(spad).unwrap();
        assert_eq!(sim.controls, app.world().resource::<PilotInput>().controls);
        // Arcade: the same stick asks for 60° of bank, the instructor eases the aileron
        // as it comes.
        let (mut app, spad) = flight(Preset::Arcade);
        for _ in 0..(3.0 * STEP_HZ) as usize {
            app.update();
        }
        let sim = app.world().get::<Simulated>(spad).unwrap();
        let bank = Measured::of(&sim.state, &ENVIRONMENT).bank;
        assert!(sim.controls.roll < 0.5, "{:?}", sim.controls);
        assert!((bank - assist::INSTRUCTOR_BANK).abs() < 0.3, "{bank}");
    }
}
