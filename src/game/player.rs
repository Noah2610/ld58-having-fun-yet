use crate::{
    AppSystems,
    asset_tracking::LoadResource,
    game::{
        aim::AimController,
        bullet::BulletSpawner,
        enemy::{Enemy, EnemyGoal, EnemySettings, EnemyStunned},
        health::{Dead, Health},
        movement::{Acceleration, MovementController},
        util::CollisionTag,
        visuals::{AnimationDirection, HueAnimation, SetSpriteColor, VisualAnimation},
    },
    game_state::GameplaySet,
};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation, Aseprite};
use serde::{Deserialize, Serialize};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();

    app.add_systems(
        PreUpdate,
        post_add_player
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
            }))
            .observe(handle_enemy_collision);
    }
}

#[derive(
    Component, Reflect, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Default,
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
    Health::new(3),

    SetSpriteColor(Color::hsl(0.0, 0.8, 0.75)),
    HueAnimation(VisualAnimation {
        period: 8.0,
        direction: AnimationDirection::Linear,
        ..default()
    }),
)]
pub struct Player;

#[derive(Component)]
struct PlayerInitialized;

#[derive(Resource, Asset, Reflect, Clone)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    spritesheet: Handle<Aseprite>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            spritesheet: world
                .resource::<AssetServer>()
                .load("spritesheets/player.ase"),
        }
    }
}

fn handle_enemy_collision(
    trigger: On<CollisionStart>,
    mut players: Query<
        (&Transform, &mut LinearVelocity, Option<&mut Health>),
        (With<Player>, Without<Enemy>),
    >,
    enemies: Query<
        (&Transform, &EnemySettings),
        (
            With<Enemy>,
            Without<EnemyStunned>,
            Without<Dead>,
            Without<Player>,
        ),
    >,
) {
    let player = trigger.collider1;
    let enemy = trigger.collider2;

    if let (Ok((player_transform, mut velocity, health)), Ok((enemy_transform, enemy_settings))) =
        (players.get_mut(player), enemies.get(enemy))
    {
        let direction = (player_transform.translation.truncate()
            - enemy_transform.translation.truncate())
        .normalize_or_zero();
        velocity.0 += direction * enemy_settings.knockback_strength;
        if let Some(mut health) = health {
            health.damage(1);
        }
    }
}
