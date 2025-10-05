use crate::{
    AppSystems, GameplaySet,
    asset_tracking::LoadResource,
    game::util::{CollisionTag, FixObjectColliders},
};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation, Aseprite};
use serde::{Deserialize, Serialize};

pub fn plugin(app: &mut App) {
    app.load_resource::<EnemyAssets>();
    app.add_systems(
        Update,
        post_add_enemy
            .in_set(AppSystems::Update)
            .in_set(GameplaySet),
    );
}

fn post_add_enemy(
    mut commands: Commands,
    assets: Res<EnemyAssets>,
    enemies: Query<Entity, (Added<Enemy>, Without<EnemyInitialized>)>,
) {
    for entity in enemies {
        commands
            .entity(entity)
            .insert((EnemyInitialized, AseAnimation {
                aseprite:  assets.spritesheet.clone(),
                animation: Animation::tag("idle"),
            }));
    }
}

#[derive(Component, Reflect, Serialize, Deserialize, Clone, Copy, Debug, Default)]
#[reflect(Component)]
#[require(
    Name::new("Enemy"),
    // FixObjectColliders,
    Sprite::default(),
    RigidBody::Dynamic,
    Collider::rectangle(14.0, 16.0),
    Mass(1.0),
    CollisionLayers::new(
        [CollisionTag::Enemy, CollisionTag::Entity],
        [CollisionTag::Solid, CollisionTag::Player, CollisionTag::Bullet, CollisionTag::Enemy],
    ),
    LockedAxes::ROTATION_LOCKED,
    LinearDamping(10.0),
)]
pub struct Enemy;

#[derive(Component)]
struct EnemyInitialized;

#[derive(Resource, Asset, Reflect, Clone)]
#[reflect(Resource)]
struct EnemyAssets {
    #[dependency]
    spritesheet: Handle<Aseprite>,
}

impl FromWorld for EnemyAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            spritesheet: world
                .resource::<AssetServer>()
                .load("spritesheets/enemy.ase"),
        }
    }
}
