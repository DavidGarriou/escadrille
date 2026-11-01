//! The damage nodes of a model (skill `pipeline-assets`, plan `m4-degats-modele-3d.md`):
//! the hit volumes `hit_*` are never drawn, and the damaged variants `dmg_*` are drawn
//! only once their damage happens (M4). Both are hidden as soon as the model is spawned,
//! before its first frame is drawn.

use bevy::prelude::*;
use bevy::world_serialization::WorldInstanceReady;

/// A node hidden when its model is spawned.
pub fn hidden_at_spawn(name: &str) -> bool {
    name.starts_with("hit_") || name.starts_with("dmg_")
}

/// The glTF nodes, not the meshes Bevy spawns under them, which take their names from
/// the mesh (`dmg_prop_blade_1_stub.bois`): a node shown shows its meshes.
type Nodes<'w, 's> = Query<'w, 's, (&'static Name, &'static mut Visibility), Without<Mesh3d>>;

/// Hides the damage nodes of each model spawned. A global observer: it runs before the
/// observers of the model's own entity, which may show some of them (the gallery).
pub fn hide_damage_nodes(
    ready: On<WorldInstanceReady>,
    children: Query<&Children>,
    mut nodes: Nodes,
) {
    hide_under(ready.entity, &children, &mut nodes);
}

fn hide_under(root: Entity, children: &Query<&Children>, nodes: &mut Nodes) {
    for entity in children.iter_descendants(root) {
        if let Ok((name, mut visibility)) = nodes.get_mut(entity)
            && hidden_at_spawn(name)
        {
            *visibility = Visibility::Hidden;
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::ecs::system::RunSystemOnce;

    use super::*;

    #[test]
    fn hit_volumes_and_damaged_variants_are_hidden() {
        let mut world = World::new();
        let root = world.spawn(Name::new("Model")).id();
        let named = |world: &mut World, name: &str, parent: Entity| {
            world
                .spawn((
                    Name::new(name.to_owned()),
                    Visibility::default(),
                    ChildOf(parent),
                ))
                .id()
        };
        let body = named(&mut world, "body", root);
        let wing = named(&mut world, "wing_upper_r_mid", body);
        let hit = named(&mut world, "hit_engine", body);
        let prop = named(&mut world, "prop", body);
        let stub = named(&mut world, "dmg_prop_blade_1_stub", prop);
        // Its mesh, named after it: hidden with it, shown with it.
        let mesh = world
            .spawn((
                Name::new("dmg_prop_blade_1_stub.bois"),
                Mesh3d::default(),
                Visibility::default(),
                ChildOf(stub),
            ))
            .id();
        let wire = named(&mut world, "wire_flying_r_1", body);
        world
            .run_system_once(move |children: Query<&Children>, mut nodes: Nodes| {
                hide_under(root, &children, &mut nodes);
            })
            .unwrap();
        let visibility = |e: Entity| *world.get::<Visibility>(e).unwrap();
        for e in [hit, stub] {
            assert_eq!(visibility(e), Visibility::Hidden);
        }
        for e in [body, wing, prop, wire, mesh] {
            assert_eq!(visibility(e), Visibility::Inherited);
        }
    }
}
