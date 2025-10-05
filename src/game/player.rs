use crate::{
    AppSystems,
    asset_tracking::LoadResource,
    game::{
        aim::AimController,
        bullet::BulletSpawner,
        enemy::{Enemy, EnemyGoal},
        movement::{Acceleration, MovementController},
        util::{CollisionTag, FixObjectColliders},
    },
    game_state::GameplaySet,
};
use avian2d::prelude::*;
use bevy::{prelude::*, sprite::Anchor};
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation, Aseprite};
use bevy_yoleck::prelude::YoleckComponent;
use serde::{Deserialize, Serialize};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();

    app.add_systems(
        PreUpdate,
        post_add_player
            .in_set(GameplaySet)
            .in_set(AppSystems::Update),
    );

    app.add_systems(
        Update,
        handle_enemy_collision
            .in_set(GameplaySet)
            .in_set(AppSystems::Update),
    );
}

fn post_add_player(
    mut commands: Commands,
    assets: Res<PlayerAssets>,
    players: Query<Entity, (Added<Player>, Without<PlayerInitialized>)>,
) {
    for entity in players {
        commands
            .entity(entity)
            .insert((PlayerInitialized, AseAnimation {
                aseprite:  assets.spritesheet.clone(),
                animation: Animation::tag("idle"),
            }));
    }
}

#[derive(
    Component,
    Reflect,
    Serialize,
    Deserialize,
    YoleckComponent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
)]
#[reflect(Component)]
#[require(
    Name::new("Player"),
    // FixObjectColliders,
    Sprite::default(),
    MovementController,
    AimController,
    Acceleration(1800.0),
    LinearDamping(15.0),
    RigidBody::Dynamic,
    Collider::rectangle(8.0, 16.0),
    Mass(1.0),
    CollisionLayers::new(
        [CollisionTag::Player, CollisionTag::Entity],
        [CollisionTag::Solid, CollisionTag::Enemy, CollisionTag::Collectable],
    ),
    CollisionEventsEnabled,
    Restitution {
        coefficient: 0.1,
        combine_rule: CoefficientCombine::Max
    },
    LockedAxes::ROTATION_LOCKED,
    BulletSpawner,
    EnemyGoal,
)]
pub struct Player;

#[derive(Component)]
struct PlayerInitialized;

#[derive(Resource, Asset, Reflect, Clone)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    spritesheet:        Handle<Aseprite>,
    #[dependency]
    pub steps:          Vec<Handle<AudioSource>>,
    knockback_strength: f32,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            spritesheet:        world
                .resource::<AssetServer>()
                .load("spritesheets/player.ase"),
            steps:              {
                let assets = world.resource::<AssetServer>();
                vec![
                    assets.load("audio/steps/step1.ogg"),
                    assets.load("audio/steps/step2.ogg"),
                    assets.load("audio/steps/step3.ogg"),
                    assets.load("audio/steps/step4.ogg"),
                ]
            },
            knockback_strength: 400.0,
        }
    }
}

fn handle_enemy_collision(
    contact_graph: Res<ContactGraph>,
    assets: Res<PlayerAssets>,
    players: Query<(Entity, &mut LinearVelocity), (With<Player>, Without<Enemy>)>,
    enemies: Query<Entity, (With<Enemy>, Without<Player>)>,
) {
    for (player, mut velocity) in players {
        let mut knockback = Vec2::ZERO;
        let mut has_collision = false;

        for enemy in enemies {
            if let Some((_, contact_pair)) = contact_graph.get(enemy, player) {
                if contact_pair.is_touching() {
                    has_collision = true;
                    for manifold in &contact_pair.manifolds {
                        knockback += manifold.normal.normalize();
                    }
                }
            }
        }

        if has_collision {
            velocity.0 += knockback * assets.knockback_strength;
        }
    }
}
