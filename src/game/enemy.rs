use crate::{
    AppSystems, GameplaySet,
    asset_tracking::LoadResource,
    game::{
        health::{Dead, Health},
        util::{CollisionTag, SetScale},
        visuals::{AnimationDirection, HueAnimation, SetSpriteColor, VisualAnimation},
    },
};
use avian2d::{math::Scalar, prelude::*};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation, Aseprite};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub fn plugin(app: &mut App) {
    app.load_resource::<EnemyAssets>();
    app.add_systems(
        Update,
        (
            post_add_enemy,
            handle_variant_change,
            run_enemy_behavior,
            handle_enemy_stun,
        )
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

fn handle_variant_change(
    mut commands: Commands,
    enemies: Query<
        (Entity, &EnemyVariant, Option<&mut EnemyInitializedVariant>),
        (Changed<EnemyVariant>, With<Enemy>),
    >,
) {
    for (entity, variant, initialized_variant_opt) in enemies {
        let no_change = initialized_variant_opt
            .as_ref()
            .map(|v| &v.0 == variant)
            .unwrap_or(false);
        if no_change {
            continue;
        }
        commands.entity(entity).insert((
            EnemyInitializedVariant(*variant),
            EnemyVariantBundle::from(*variant),
        ));
    }
}

#[derive(Component, Reflect, Serialize, Deserialize, Clone, Copy, Debug, Default)]
#[reflect(Component)]
#[require(
    Name::new("Enemy"),
    EnemyVariant,
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

    Health::new(1),

    SetSpriteColor(Color::hsl(0.0, 0.6, 0.8)),
    HueAnimation(VisualAnimation {
        // hue_range: (40.0, 180.0),
        period: 6.0,
        direction: AnimationDirection::Boomerang,
        time_offset: rand::rng().random_range(0.0 .. 6.0),
        ..default()
    }),
)]
pub struct Enemy;

#[derive(
    Component, Reflect, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug, Default,
)]
pub enum EnemyVariant {
    #[default]
    Basic,
    Bigger,
}

#[derive(Bundle)]
struct EnemyVariantBundle {
    settings: EnemySettings,
    scale:    SetScale,
    health:   Health,
}

impl From<EnemyVariant> for EnemyVariantBundle {
    fn from(variant: EnemyVariant) -> Self {
        match variant {
            EnemyVariant::Basic => Self {
                settings: EnemySettings {
                    speed:                     400.0,
                    stun_duration:             Duration::from_secs(2),
                    knockback_strength:        300.0,
                    knockback_strength_bullet: 800.0,
                    score_worth:               10,
                },
                scale:    Vec2::splat(1.0).into(),
                health:   Health::new(1),
            },
            EnemyVariant::Bigger => Self {
                settings: EnemySettings {
                    speed:                     200.0,
                    stun_duration:             Duration::from_secs(4),
                    knockback_strength:        600.0,
                    knockback_strength_bullet: 1200.0,
                    score_worth:               100,
                },
                scale:    Vec2::splat(2.0).into(),
                health:   Health::new(3),
            },
        }
    }
}

#[derive(Component, Reflect, Clone, Debug)]
#[reflect(Component)]
pub struct EnemySettings {
    pub speed:                     Scalar,
    pub stun_duration:             Duration,
    /// For knocking back player
    pub knockback_strength:        Scalar,
    /// Own knockback when hit by bullet
    pub knockback_strength_bullet: Scalar,
    pub score_worth:               u32,
}

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

#[derive(Component)]
struct EnemyInitializedVariant(EnemyVariant);

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

fn run_enemy_behavior(
    time: Res<Time>,
    enemies: Query<
        (&GlobalTransform, &EnemySettings, &mut LinearVelocity),
        (
            With<Enemy>,
            Without<EnemyStunned>,
            Without<Dead>,
            Without<EnemyGoal>,
        ),
    >,
    goals: Query<&GlobalTransform, (With<EnemyGoal>, Without<Enemy>)>,
) {
    let delta = time.delta_secs();
    for (transform, settings, mut velocity) in enemies {
        let translation = transform.translation();

        if let Some(target) = goals
            .into_iter()
            .fold(None, |acc: Option<(f32, Vec2)>, goal| {
                let goal_translation = goal.translation();
                let distance = translation.distance_squared(goal_translation);
                Some(match acc {
                    Some(nearest) => {
                        if distance < nearest.0 {
                            (distance, goal_translation.truncate())
                        } else {
                            nearest
                        }
                    },
                    None => (distance, goal_translation.truncate()),
                })
            })
            .map(|(_, goal)| goal)
        {
            let direction = (target - translation.truncate()).normalize();
            velocity.0 += direction * settings.speed * delta;
        }
    }
}

fn handle_enemy_stun(
    time: Res<Time>,
    mut commands: Commands,
    newly_stunned_enemies: Query<
        (Entity, &EnemySettings, &mut AseAnimation),
        (With<Enemy>, Added<EnemyStunned>, Without<EnemyStunnedTimer>),
    >,
    stunned_enemies: Query<
        (Entity, &mut EnemyStunnedTimer, &mut AseAnimation),
        (With<Enemy>, With<EnemyStunned>),
    >,
) {
    for (enemy, settings, mut ase) in newly_stunned_enemies {
        commands.entity(enemy).insert(EnemyStunnedTimer(Timer::new(
            settings.stun_duration,
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
