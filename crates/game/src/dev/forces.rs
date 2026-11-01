//! Force gizmos, the G key (skill `bevy`, section 8; plan `m2-train-sol.md`, step 7): what
//! acts on the player's aircraft, on top of the world: the sum of the air's forces from
//! the centre of gravity, the propeller's thrust, and the ground's at each contact.

use bevy::math::DVec3;
use bevy::prelude::*;
use escadrille_fdm::ground::ContactPoint;

use crate::flight::bridge::to_bevy;
use crate::flight::simulation::{ENVIRONMENT, Player, Simulated};

/// Length of an arrow per newton, m: the weight of the SPAD draws about 3.5 m.
const METRES_PER_NEWTON: f32 = 0.0005;

const AIR: Color = Color::srgb(0.3, 0.7, 1.0);
const THRUST: Color = Color::srgb(1.0, 0.6, 0.1);
const GROUND: Color = Color::srgb(0.4, 1.0, 0.3);
const AIRFRAME: Color = Color::srgb(1.0, 0.2, 0.2);

/// The gizmos are drawn.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ForceGizmos(pub bool);

pub fn toggle(keys: Res<ButtonInput<KeyCode>>, mut shown: ResMut<ForceGizmos>) {
    if keys.just_pressed(KeyCode::KeyG) {
        shown.0 = !shown.0;
    }
}

pub fn draw(mut gizmos: Gizmos, aircraft: Query<(&Simulated, &Transform), With<Player>>) {
    for (sim, transform) in &aircraft {
        let d = sim
            .aircraft
            .evaluate(&sim.state, &sim.controls, &ENVIRONMENT);
        let place = |body: DVec3| transform.transform_point(to_bevy(body).as_vec3());
        let mut arrow = |at: DVec3, force: DVec3, colour: Color| {
            let start = place(at);
            let vector = transform.rotation * to_bevy(force).as_vec3() * METRES_PER_NEWTON;
            gizmos.arrow(start, start + vector, colour);
        };
        let thrust = d.propeller.map_or(DVec3::ZERO, |p| p.wrench.force);
        let air = d.loads.wrench.force - thrust - d.ground.wrench.force;
        arrow(DVec3::ZERO, air, AIR);
        arrow(DVec3::ZERO, thrust, THRUST);
        for c in &d.ground.contacts {
            let colour = match c.point {
                ContactPoint::HardPoint(_) => AIRFRAME,
                ContactPoint::Wheel(_) | ContactPoint::Skid(_) => GROUND,
            };
            arrow(c.at, c.force, colour);
        }
    }
}
