//! Flight: the flight model (`escadrille_fdm`) in the ECS (plan `m2-vol-essai.md`, step 2).
//!
//! - The simulation steps in `FixedUpdate` at [`STEP_HZ`] (RK4, ADR 0005), keeping the
//!   previous and the current state ([`simulation`]).
//! - In `Update`, the aircraft's transform is interpolated between them, relative to a
//!   floating render origin, through the one frame conversion of ADR 0006 ([`bridge`]).
//! - The 3D model's control surfaces and propeller follow the flight model
//!   ([`animation`]).

pub mod animation;
pub mod bridge;
pub mod simulation;

use std::path::Path;
use std::sync::Arc;

use bevy::math::DVec3;
use bevy::prelude::*;
use escadrille_data::aircraft::{Loading, load_aircraft};
use escadrille_data::model::contract;
use escadrille_data::scenario::load_scenario;
use escadrille_fdm::aircraft::PilotControls;

use crate::ASSETS_DIR;
use crate::dev::capture::CaptureHold;
use bridge::model_transform;
use simulation::{Player, Simulated};

/// Rate of the simulation, Hz: the step of ADR 0005.
pub const STEP_HZ: f64 = 120.0;

/// The scenario `just run` flies (plan `m2-vol-essai.md`).
pub const TEST_FLIGHT: &str = "vol-essai";

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum FlightSet {
    /// `FixedUpdate`: the flight model's step.
    Step,
    /// `Update`: restart, interpolated placement, the model's moving parts.
    Place,
}

/// What the pilot commands, out of the action layer (`crate::input`), for the next steps.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Default)]
pub struct PilotInput(pub PilotControls);

/// Back to the start of the scenario.
#[derive(Message, Debug, Clone, Copy, PartialEq, Eq)]
pub struct RestartFlight;

/// The commands holding the scenario's start: the pilot's levers go back to them at
/// each start (the resource is marked changed then).
#[derive(Resource, Debug, Clone, Copy, PartialEq, Default)]
pub struct StartCommands(pub PilotControls);

pub struct FlightPlugin;

impl Plugin for FlightPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Time::<Fixed>::from_hz(STEP_HZ))
            .init_resource::<PilotInput>()
            .init_resource::<StartCommands>()
            .init_resource::<simulation::RenderOrigin>()
            .add_message::<RestartFlight>()
            .add_systems(FixedUpdate, simulation::step.in_set(FlightSet::Step))
            .add_systems(
                Update,
                (
                    simulation::restart,
                    simulation::place,
                    animation::move_controls,
                    animation::turn_propeller,
                )
                    .chain()
                    .in_set(FlightSet::Place),
            );
    }
}

/// Spawns the player's aircraft of a scenario, its model under it, and sets the pilot's
/// commands to those holding its start. Returns the aircraft's entity and its centre of
/// gravity from the datum (body axes, m).
pub fn spawn_scenario(
    name: &str,
    commands: &mut Commands,
    asset_server: &AssetServer,
    hold: &mut CaptureHold,
) -> Result<(Entity, DVec3)> {
    let assets = Path::new(ASSETS_DIR);
    let scenario = load_scenario(assets, name)?;
    let data = load_aircraft(assets, &scenario.aircraft)?;
    let built = data.build(&Loading {
        fuel_fraction: scenario.fuel_fraction,
    })?;
    let cg = built.summary.cg_from_datum;
    let contract = contract(&data)?;
    let extremities = simulation::extremities(&data, &contract, cg)?;
    let start = scenario.initial_state();
    commands.insert_resource(PilotInput(scenario.commands()));
    commands.insert_resource(StartCommands(scenario.commands()));

    hold.0 += 1;
    let model = asset_server.load(
        GltfAssetLabel::Scene(0).from_asset(format!("aircraft/{}/model.glb", scenario.aircraft)),
    );
    let controls = contract
        .hinges
        .iter()
        .map(|h| (h.node.clone(), h.control))
        .collect();
    let entity = commands
        .spawn((
            Name::new(data.file.name.clone()),
            Player,
            Simulated {
                aircraft: Arc::new(built.aircraft),
                start,
                previous: start,
                state: start,
                controls: scenario.commands(),
                extremities,
                crashed: None,
            },
            Transform::default(),
            Visibility::default(),
        ))
        .with_children(|aircraft| {
            aircraft
                .spawn((
                    Name::new("Model"),
                    WorldAssetRoot(model),
                    model_transform(cg),
                    animation::ModelNodes { controls },
                ))
                .observe(animation::prepare_model);
        })
        .id();
    Ok((entity, cg))
}
