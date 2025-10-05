use crate::{
    AppSystems, GameplaySet,
    asset_tracking::LoadResource,
    game::aim::AimDirection,
    input::{PlayerAction, action_just_pressed},
};
use avian2d::{collision::collider::Sensor, math::Scalar, prelude::*};
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
    Sprite::default(),
    RigidBody::Dynamic,
    Mass(0.0),
    Collider::circle(6.5),
    CollisionEventsEnabled
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
}

impl FromWorld for BulletAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            spritesheet:      world
                .resource::<AssetServer>()
                .load("spritesheets/bullet.ase"),
            sfx_shoot:        world.resource::<AssetServer>().load("audio/sfx/shoot.ogg"),
            spawn_offset:     16.0,
            speed:            200.0,
            duration:         Duration::from_secs(1),
            velocity_damping: 2.0,
        }
    }
}

fn handle_spawn_bullet(
    mut commands: Commands,
    assets: Res<BulletAssets>,
    spawners: Query<
        (Entity, &Transform, &AimDirection),
        (With<BulletSpawner>, With<BulletAvailable>),
    >,
) {
    for (entity, transform, aim) in spawners {
        if let Some(dir) = &aim.0 {
            let dir_vec = dir.vec();
            let offset = (dir_vec * assets.spawn_offset).extend(0.0);

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
                .observe(handle_bullet_collision);

            commands.entity(entity).remove::<BulletAvailable>();
        }
    }
}

fn handle_bullet_timers(
    mut commands: Commands,
    time: Res<Time>,
    bullets: Query<
        (Entity, &mut BulletTimer, &mut AseAnimation),
        (With<Bullet>, Without<Collectable>),
    >,
    assets: Res<BulletAssets>,
) {
    let delta = time.delta();
    for (entity, mut timer, mut ase) in bullets {
        timer.0.tick(delta);
        if timer.0.is_finished() {
            ase.animation.play_loop("idle");
            commands
                .entity(entity)
                .remove::<BulletTimer>()
                .insert((Collectable, LinearDamping(assets.velocity_damping)));
        }
    }
}

fn handle_bullet_collision(
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
