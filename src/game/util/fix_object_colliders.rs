use crate::{AppSystems, game_state::GameplaySet};
use avian2d::{
    parry::{math::Isometry, shape::SharedShape},
    prelude::*,
};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        PreUpdate,
        fix_object_colliders
            .in_set(GameplaySet)
            .in_set(AppSystems::Update),
    );
}

#[derive(Component, Default)]
#[require(Collider::default())] // Entity needs a collider in order for its children colliders to work
pub struct FixObjectColliders;

#[derive(Component)]
struct FixObjectCollidersInitialized;

/// Take child collider and assign to entity with offset for objects loaded from tiled.
/// Workaround for incorrect colliders (I'm probably doing something wrong)
fn fix_object_colliders(
    mut commands: Commands,
    objects: Query<
        (Entity, &Children, &mut Collider),
        (
            Added<FixObjectColliders>,
            Without<FixObjectCollidersInitialized>,
        ),
    >,
    mut colliders: Query<
        (&Collider, &ColliderAabb, &mut Transform),
        (With<Collider>, With<ChildOf>, Without<FixObjectColliders>),
    >,
) {
    for (entity, children, mut collider) in objects {
        // let mut shapes = Vec::<(Position, Rotation, Collider)>::new();

        for &child in children {
            if let Ok((child_collider, aabb, mut transform)) = colliders.get_mut(child) {
                *collider = child_collider.clone();

                // shapes.push((
                //     transform.translation.truncate().into(),
                //     (*transform).into(),
                //     child_collider.clone(),
                // ));

                // let half = (aabb.size() * 0.5).extend(0.0);
                // transform.translation.x -= half.x;
                // transform.translation.y += half.y;
            }
        }

        // if !shapes.is_empty() {
        //     *collider = Collider::compound(shapes);
        // }

        commands
            .entity(entity)
            .insert(FixObjectCollidersInitialized)
            .despawn_children();
    }
}
