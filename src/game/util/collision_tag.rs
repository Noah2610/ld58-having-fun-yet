use avian2d::prelude::PhysicsLayer;

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
