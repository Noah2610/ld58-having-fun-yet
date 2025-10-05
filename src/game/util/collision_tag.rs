use avian2d::prelude::PhysicsLayer;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {}

#[derive(PhysicsLayer, Default)]
pub enum CollisionTag {
    #[default]
    Default,
    Solid,
    Entity,
    Player,
    Enemy,
    Bullet,
    Collectable,
}
