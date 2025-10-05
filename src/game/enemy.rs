use crate::{AppSystems, GameplaySet, asset_tracking::LoadResource, game::util::CollisionTag};
use avian2d::{math::Scalar, prelude::*};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation, Aseprite};
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub fn plugin(app: &mut App) {
    app.load_resource::<EnemyAssets>();
    app.add_systems(
        Update,
        (post_add_enemy, run_enemy_behavior, handle_enemy_stun)
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

/// Marks an entity (player) which becomes the goal for enemies to move towards.
/// Each enemy picks the closest `EnemyGoal` entity as its goal.
#[derive(Component, Reflect, Clone, Copy, Debug, Default)]
#[reflect(Component)]
#[require(Transform)]
pub struct EnemyGoal;

#[derive(Component, Reflect, Clone, Copy, Debug, Default)]
#[reflect(Component)]
pub struct EnemyStunned;

#[derive(Component)]
struct EnemyStunnedTimer(Timer);

#[derive(Component)]
struct EnemyInitialized;

#[derive(Resource, Asset, Reflect, Clone)]
#[reflect(Resource)]
struct EnemyAssets {
    #[dependency]
    spritesheet:   Handle<Aseprite>,
    speed:         Scalar,
    stun_duration: Duration,
}

impl FromWorld for EnemyAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            spritesheet:   world
                .resource::<AssetServer>()
                .load("spritesheets/enemy.ase"),
            speed:         400.0,
            stun_duration: Duration::from_secs(2),
        }
    }
}

fn run_enemy_behavior(
    time: Res<Time>,
    assets: Res<EnemyAssets>,
    enemies: Query<
        (&Transform, &mut LinearVelocity),
        (With<Enemy>, Without<EnemyStunned>, Without<EnemyGoal>),
    >,
    goals: Query<&Transform, (With<EnemyGoal>, Without<Enemy>)>,
) {
    let delta = time.delta_secs();
    for (transform, mut velocity) in enemies {
        if let Some(target) = goals
            .into_iter()
            .fold(None, |acc: Option<(f32, Vec2)>, goal| {
                let distance = transform.translation.distance_squared(goal.translation);
                Some(match acc {
                    Some(nearest) => {
                        if distance < nearest.0 {
                            (distance, goal.translation.truncate())
                        } else {
                            nearest
                        }
                    },
                    None => (distance, goal.translation.truncate()),
                })
            })
            .map(|(_, goal)| goal)
        {
            let direction = (target - transform.translation.truncate()).normalize();
            velocity.0 += direction * assets.speed * delta;
        }
    }
}

fn handle_enemy_stun(
    time: Res<Time>,
    mut commands: Commands,
    assets: Res<EnemyAssets>,
    newly_stunned_enemies: Query<
        (Entity, &mut AseAnimation),
        (With<Enemy>, Added<EnemyStunned>, Without<EnemyStunnedTimer>),
    >,
    stunned_enemies: Query<
        (Entity, &mut EnemyStunnedTimer, &mut AseAnimation),
        (With<Enemy>, With<EnemyStunned>),
    >,
) {
    for (enemy, mut ase) in newly_stunned_enemies {
        commands.entity(enemy).insert(EnemyStunnedTimer(Timer::new(
            assets.stun_duration,
            TimerMode::Once,
        )));
        ase.animation.play_loop("stun");
    }

    let delta = time.delta();

    for (enemy, mut timer, mut ase) in stunned_enemies {
        timer.0.tick(delta);
        if timer.0.is_finished() {
            commands
                .entity(enemy)
                .remove::<(EnemyStunned, EnemyStunnedTimer)>();
            ase.animation.play_loop("idle");
        }
    }
}
