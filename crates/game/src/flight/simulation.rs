//! The flight model's step and the aircraft's place in the rendered world.

use std::path::Path;
use std::sync::Arc;

use bevy::math::{DQuat, DVec3};
use bevy::prelude::*;
use escadrille_data::aircraft::AircraftData;
use escadrille_data::mockup::load_mockup;
use escadrille_data::model::Contract;
use escadrille_fdm::aircraft::{Aircraft, PilotControls};
use escadrille_fdm::environment::StandardEnvironment;
use escadrille_fdm::rigid_body::RigidBodyState;

use super::bridge::{render_transform, to_bevy};
use super::{PilotInput, RestartFlight, StartCommands};
use crate::ASSETS_DIR;

/// Still air over flat ground at sea level: the plain of the test flight.
pub const ENVIRONMENT: StandardEnvironment = StandardEnvironment {
    wind_ned: DVec3::ZERO,
    ground_altitude: 0.0,
};

/// Seconds between touching the ground and the new start.
pub const RESTART_AFTER_S: f32 = 3.0;

/// Step of the floating origin's grid, m: the render origin stays on the ground, on a
/// grid point under the player's aircraft, so that rendered positions stay small in f32.
/// The ground's texture repeats on the same grid (`crate::world`), so that a jump of the
/// origin does not show.
pub const ORIGIN_GRID_M: f64 = 2000.0;

/// The aircraft the camera follows and the render origin stays under.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Player;

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
    /// Body axes from the centre of gravity: the points that touch the ground first.
    pub extremities: Vec<DVec3>,
    /// Seconds since the aircraft touched the ground, while it waits for the restart.
    pub crashed: Option<f32>,
}

impl Simulated {
    /// Whether a point of the airframe is under the ground, or the state is broken.
    pub fn touches_ground(&self) -> bool {
        let s = &self.state;
        let finite = s.position_ned.is_finite()
            && s.attitude.is_finite()
            && s.velocity_body.is_finite()
            && s.angular_velocity_body.is_finite();
        !finite
            || self
                .extremities
                .iter()
                .any(|p| (s.position_ned + s.attitude * *p).z > -ENVIRONMENT.ground_altitude)
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

/// The points of an aircraft that touch the ground first, body axes from its centre of
/// gravity: wheels and tail skid (from its 3D model's file, until the undercarriage is
/// modelled), the tips of its lifting surfaces, the rim of its propeller's disc.
pub fn extremities(data: &AircraftData, contract: &Contract, cg: DVec3) -> Result<Vec<DVec3>> {
    let mut points = Vec::new();
    if let Some(mockup) = load_mockup(Path::new(ASSETS_DIR), &data.file.id)? {
        let w = mockup.wheels;
        let bottom = DVec3::from_array(w.centre_m) + DVec3::Z * (0.5 * w.diameter.si());
        points.extend([bottom, bottom * DVec3::new(1.0, -1.0, 1.0)]);
        points.push(DVec3::from_array(mockup.skid_m));
    }
    for surface in &data.file.surfaces {
        for panel in &surface.panels {
            let tip = DVec3::from_array(panel.tip_m);
            points.push(tip);
            if surface.mirrored {
                points.push(tip * DVec3::new(1.0, -1.0, 1.0));
            }
        }
    }
    let p = contract.propeller;
    let u = p.axis.any_orthonormal_vector();
    let v = p.axis.cross(u);
    points.extend((0..8).map(|k| {
        let a = std::f64::consts::TAU * f64::from(k) / 8.0;
        p.hub + (u * a.cos() + v * a.sin()) * p.radius
    }));
    Ok(points.into_iter().map(|p| p - cg).collect())
}

/// The player's aircraft takes the pilot's commands for the next step.
pub fn take_input(input: Res<PilotInput>, mut aircraft: Query<&mut Simulated, With<Player>>) {
    for mut sim in &mut aircraft {
        sim.controls = input.controls;
    }
}

/// One step of the flight model for each aircraft not on the ground, with its commands.
pub fn step(time: Res<Time<Fixed>>, mut aircraft: Query<&mut Simulated>) {
    let dt = time.delta_secs_f64();
    for mut sim in &mut aircraft {
        if sim.crashed.is_some() {
            continue;
        }
        let state = sim
            .aircraft
            .step(&sim.state, &sim.controls, &ENVIRONMENT, dt);
        sim.previous = sim.state;
        sim.state = state;
        if sim.touches_ground() {
            sim.state = sim.previous;
            sim.crashed = Some(0.0);
        }
    }
}

/// After [`RESTART_AFTER_S`] on the ground, or when asked, back to the start.
pub fn restart(
    time: Res<Time>,
    mut asked: MessageReader<RestartFlight>,
    mut start: ResMut<StartCommands>,
    mut aircraft: Query<&mut Simulated>,
) {
    let now = asked.read().count() > 0;
    for mut sim in &mut aircraft {
        if let Some(t) = sim.crashed.as_mut() {
            *t += time.delta_secs();
        }
        if now || sim.crashed.is_some_and(|t| t >= RESTART_AFTER_S) {
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
    use std::time::Duration;

    use bevy::time::TimeUpdateStrategy;
    use escadrille_data::aircraft::{Loading, load_aircraft};
    use escadrille_data::model::contract;
    use escadrille_data::scenario::load_scenario;

    use super::*;
    use crate::flight::{STEP_HZ, TEST_FLIGHT};

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
            .init_resource::<RenderOrigin>()
            .add_message::<RestartFlight>()
            .add_systems(FixedUpdate, (take_input, step).chain())
            .add_systems(Update, (restart, place).chain());
        app
    }

    fn spad(app: &mut App) -> Entity {
        let assets = Path::new(ASSETS_DIR);
        let scenario = load_scenario(assets, TEST_FLIGHT).unwrap();
        let data = load_aircraft(assets, &scenario.aircraft).unwrap();
        let built = data
            .build(&Loading {
                fuel_fraction: scenario.fuel_fraction,
            })
            .unwrap();
        let cg = built.summary.cg_from_datum;
        let extremities = extremities(&data, &contract(&data).unwrap(), cg).unwrap();
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
                    extremities,
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
