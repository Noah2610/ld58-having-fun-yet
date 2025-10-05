use crate::{AppSystems, game_state::GameplaySet};
use avian2d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        PreUpdate,
        fix_object_colliders
            .in_set(GameplaySet)
            .in_set(AppSystems::Update),
    );
}

/// Workaround for collider issues.
/// Take child collider and add to this entity, then despawn children.
#[derive(Component, Default)]
#[require(Collider::default())]
pub struct FixObjectColliders;

#[derive(Component)]
struct FixObjectCollidersInitialized;

/// Take child collider and assign to entity for FixObjectColliders objects loaded from tiled.
/// Workaround for collider issues (I'm probably doing something wrong)
fn fix_object_colliders(
    mut commands: Commands,
    objects: Query<
        (Entity, &Children, &mut Collider),
        (
            Added<FixObjectColliders>,
            Without<FixObjectCollidersInitialized>,
        ),
    >,
    colliders: Query<&Collider, (With<Collider>, With<ChildOf>, Without<FixObjectColliders>)>,
) {
    for (entity, children, mut collider) in objects {
        for &child in children {
            if let Ok(child_collider) = colliders.get(child) {
                *collider = child_collider.clone();
                break;
            }
        }

        commands
            .entity(entity)
            .insert(FixObjectCollidersInitialized)
            .despawn_children();
    }
}
