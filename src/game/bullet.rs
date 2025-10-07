use crate::{
    AppSystems, GameplaySet,
    asset_tracking::LoadResource,
    game::{
        aim::AimDirection,
        enemy::{Enemy, EnemySettings, EnemyStunned},
        health::{Dead, Health},
        score::Score,
        util::CollisionTag,
        visuals::{AnimationDirection, HueAnimation, SetSpriteColor, VisualAnimation},
    },
    input::{PlayerAction, action_just_pressed},
    screens::Screen,
};
use avian2d::{math::Scalar, prelude::*};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation, Aseprite};
use std::time::Duration;

pub fn plugin(app: &mut App) {
    app.load_resource::<BulletAssets>();

    app.add_systems(
        Update,
        handle_bullet_timers
            .in_set(AppSystems::TickTimers)
            .in_set(GameplaySet),
    );

    app.add_systems(
        Update,
        handle_spawn_bullet
            .run_if(action_just_pressed(PlayerAction::Shoot))
            .in_set(AppSystems::Update)
            .in_set(GameplaySet),
    );
}

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
#[require(
    Name::new("Bullet"),
    DespawnOnExit::<_>(Screen::Gameplay),
    Sprite::default(),
    RigidBody::Dynamic,
    Collider::circle(6.5),
    Mass(0.5),
    CollisionLayers::new(
        [CollisionTag::Bullet, CollisionTag::Entity],
        [CollisionTag::Solid, CollisionTag::Enemy],
    ),
    Restitution {
        coefficient: 0.8,
        combine_rule: CoefficientCombine::Max
    },
    MaxLinearSpeed(200.0),
    AngularDamping(1.0),
    CollisionEventsEnabled,

    SetSpriteColor(Color::hsl(0.0, 0.9, 0.4)),
    HueAnimation(VisualAnimation {
        range: Some((0.0, 70.0)),
        period: 1.0,
        direction: AnimationDirection::Boomerang,
        ..default()
    }),
)]
pub struct Bullet;

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
#[require(BulletAvailable)]
pub struct BulletSpawner;

#[derive(Component, Reflect, Clone, Copy, Default)]
#[component(storage = "SparseSet")]
#[reflect(Component)]
struct BulletAvailable;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct BulletTimer(Timer);

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Collectable;

#[derive(Resource, Asset, Reflect, Clone)]
#[reflect(Resource)]
struct BulletAssets {
    #[dependency]
    spritesheet:      Handle<Aseprite>,
    #[dependency]
    sfx_shoot:        Handle<AudioSource>,
    spawn_offset:     Scalar,
    speed:            Scalar,
    duration:         Duration,
    velocity_damping: Scalar,
    player_knockback: Scalar,
}

impl FromWorld for BulletAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            spritesheet:      world
                .resource::<AssetServer>()
                .load("spritesheets/bullet.ase"),
            sfx_shoot:        world.resource::<AssetServer>().load("audio/sfx/shoot.ogg"),
            spawn_offset:     8.0,
            speed:            200.0,
            duration:         Duration::from_millis(500),
            velocity_damping: 2.0,
            player_knockback: 400.0,
        }
    }
}

fn handle_spawn_bullet(
    mut commands: Commands,
    assets: Res<BulletAssets>,
    spawners: Query<
        (Entity, &Transform, &AimDirection, &mut LinearVelocity),
        (With<BulletSpawner>, With<BulletAvailable>),
    >,
) {
    for (entity, transform, aim, mut velocity) in spawners {
        if let Some(dir) = &aim.0 {
            let dir_vec = dir.vec();
            let offset = (dir_vec * assets.spawn_offset).extend(0.0);
            let knockback = dir.opposite().vec() * assets.player_knockback;

            velocity.0 += knockback;

            commands
                .spawn((
                    Bullet,
                    BulletTimer(Timer::new(assets.duration, TimerMode::Once)),
                    AudioPlayer(assets.sfx_shoot.clone()),
                    AseAnimation {
                        aseprite:  assets.spritesheet.clone(),
                        animation: Animation::tag("fly"),
                    },
                    Transform::from_translation(transform.translation + offset),
                    LinearVelocity(dir_vec * assets.speed),
                ))
                .observe(handle_collect_bullet)
                .observe(handle_bullet_enemy_collision);

            commands.entity(entity).remove::<BulletAvailable>();
        }
    }
}

fn handle_bullet_timers(
    mut commands: Commands,
    time: Res<Time>,
    bullets: Query<
        (
            Entity,
            &mut BulletTimer,
            &mut AseAnimation,
            &mut CollisionLayers,
        ),
        (With<Bullet>, Without<Collectable>),
    >,
    assets: Res<BulletAssets>,
) {
    let delta = time.delta();
    for (entity, mut timer, mut ase, mut collision_layers) in bullets {
        timer.0.tick(delta);
        if timer.0.is_finished() {
            ase.animation.play_loop("idle");
            collision_layers.memberships = CollisionTag::Collectable.into();
            collision_layers.filters |= CollisionTag::Player;
            commands
                .entity(entity)
                .remove::<BulletTimer>()
                .insert((Collectable, LinearDamping(assets.velocity_damping)));
        }
    }
}

fn handle_collect_bullet(
    trigger: On<CollisionStart>,
    mut commands: Commands,
    bullets: Query<(), (With<Bullet>, With<Collectable>)>,
    spawners: Query<(), (With<BulletSpawner>, Without<BulletAvailable>)>,
) {
    let bullet = trigger.collider1;
    let spawner = trigger.collider2;
    if bullets.contains(bullet) && spawners.contains(spawner) {
        commands.entity(bullet).despawn();
        commands.entity(spawner).insert(BulletAvailable);
    }
}

fn handle_bullet_enemy_collision(
    trigger: On<CollisionStart>,
    mut commands: Commands,
    mut score: ResMut<Score>,
    bullets: Query<(&GlobalTransform, &LinearVelocity), (With<Bullet>, Without<Enemy>)>,
    mut enemies: Query<
        (
            &GlobalTransform,
            &EnemySettings,
            &mut LinearVelocity,
            Option<&mut Health>,
            Has<EnemyStunned>,
        ),
        (With<Enemy>, Without<Dead>, Without<Bullet>),
    >,
) {
    let bullet = trigger.collider1;
    let enemy = trigger.collider2;

    if let (
        Ok((bullet_transform, bullet_velocity)),
        Ok((enemy_transform, settings, mut enemy_velocity, health, is_stunned)),
    ) = (bullets.get(bullet), enemies.get_mut(enemy))
    {
        const MAX_SPEED_FOR_DAMAGE: f32 = 30.0;

        if bullet_velocity.length() < MAX_SPEED_FOR_DAMAGE {
            return;
        }

        let bullet_translation = bullet_transform.translation();
        let enemy_translation = enemy_transform.translation();

        let direction =
            (enemy_translation.truncate() - bullet_translation.truncate()).normalize_or_zero();

        enemy_velocity.0 += direction * settings.knockback_strength_bullet;

        if !is_stunned {
            commands.entity(enemy).insert(EnemyStunned);

            if let Some(mut health) = health {
                health.damage(1);
                if !health.is_alive() {
                    score.0 += settings.score_worth;
                }
            }
        }
    }
}
