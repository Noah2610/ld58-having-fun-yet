use crate::{
    AppSystems,
    asset_tracking::LoadResource,
    audio::sound_effect,
    game::{
        aim::AimController,
        bullet::BulletSpawner,
        enemy::{Enemy, EnemyGoal, EnemySettings, EnemyStunned},
        health::{Dead, Health},
        movement::{Acceleration, MovementController},
        util::CollisionTag,
        visuals::{
            AnimationDirection, HueAnimation, ScaleXAnimation, ScaleYAnimation, SetSpriteColor,
            VisualAnimation,
        },
    },
    game_state::{GameOver, GameplaySet},
};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation, Aseprite};
use serde::{Deserialize, Serialize};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();

    app.add_systems(Update, handle_player_death);
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
    // Collider::rectangle(8.0, 16.0),
    Collider::circle(8.0),
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
    Health::new(100),

    SetSpriteColor(Color::hsl(0.0, 0.8, 0.75)),
    HueAnimation(VisualAnimation {
        period: 8.0,
        direction: AnimationDirection::Linear,
        ..default()
    }),
    ScaleXAnimation(VisualAnimation{
        period: 4.0,
        direction: AnimationDirection::Boomerang,
        range: Some((0.5, 1.5)),
        time_offset: 0.0,
    }),
    ScaleYAnimation(VisualAnimation{
        period: 4.0,
        direction: AnimationDirection::Boomerang,
        range: Some((0.5, 1.5)),
        time_offset: 2.0,
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
    #[dependency]
    sfx_death:   Handle<AudioSource>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            spritesheet: world
                .resource::<AssetServer>()
                .load("spritesheets/player.ase"),
            sfx_death:   world.resource::<AssetServer>().load("audio/sfx/death.ogg"),
        }
    }
}

fn handle_enemy_collision(
    trigger: On<CollisionStart>,
    mut players: Query<
        (&GlobalTransform, &mut LinearVelocity, Option<&mut Health>),
        (With<Player>, Without<Enemy>),
    >,
    enemies: Query<
        (&GlobalTransform, &EnemySettings),
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
        let direction = (player_transform.translation().truncate()
            - enemy_transform.translation().truncate())
        .normalize_or_zero();
        velocity.0 += direction * enemy_settings.knockback_strength;
        if let Some(mut health) = health {
            health.damage(1);
        }
    }
}

fn handle_player_death(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameOver>>,
    assets: Res<PlayerAssets>,
    dead_players: Query<(), (With<Player>, Added<Dead>)>,
) {
    if !dead_players.is_empty() {
        commands.spawn(sound_effect(assets.sfx_death.clone()));
        next_state.set(GameOver(true));
    }
}
