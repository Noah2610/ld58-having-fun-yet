use crate::{AppSystems, game_state::GameplaySet};
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        fix_object_colliders
            .in_set(GameplaySet)
            .in_set(AppSystems::Update),
    );
}

#[derive(Component, Default)]
pub struct FixObjectColliders;

#[derive(Component)]
#[require(Collider::default())] // Entity needs a collider in order for its children colliders to work
struct FixObjectCollidersInitialized;

/// Offset colliders of children for entities loaded through tiled.
/// Workaround for incorrect colliders (I'm probably doing something wrong)
fn fix_object_colliders(
    mut commands: Commands,
    objects: Query<
        (Entity, &Children),
        (
            Added<FixObjectColliders>,
            Without<FixObjectCollidersInitialized>,
        ),
    >,
    mut colliders: Query<(&ColliderAabb, &mut Transform), (With<Collider>, With<ChildOf>)>,
) {
    for (entity, children) in objects {
        commands
            .entity(entity)
            .insert(FixObjectCollidersInitialized);
        // Offset all children colliders to be centered on the entity
        for &child in children {
            if let Ok((aabb, mut transform)) = colliders.get_mut(child) {
                let half = (aabb.size() * 0.5).extend(0.0);
                transform.translation.x -= half.x;
                transform.translation.y += half.y;
            }
        }
    }
}
