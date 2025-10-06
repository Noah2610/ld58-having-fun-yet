use crate::{
    AppSystems, GameplaySet,
    asset_tracking::LoadResource,
    game::{enemy::Enemy, player::Player, survival_timer::SurvivalTimer},
};
use bevy::{ecs::relationship::RelatedSpawner, prelude::*};
use rand::Rng;

pub fn plugin(app: &mut App) {
    app.load_resource::<WavesManagerAssets>();
    app.add_systems(
        Update,
        handle_waves_manager
            .in_set(AppSystems::Update)
            .in_set(GameplaySet),
    );
}

/// Spawns waves of enemies at specific times based on survival time.
#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
#[require(Name::new("WavesManager"), WaveCounter, Transform, Visibility)]
pub struct WavesManager;

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
#[require(Name::new("Wave"), Transform, Visibility)]
pub struct Wave;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct WaveCounter(u32);

#[derive(Resource, Asset, Reflect, Clone)]
#[reflect(Resource)]
struct WavesManagerAssets {
    /// Spawn a new wave every N seconds of the survival timer
    spawn_every_n_secs:       u32,
    /// Base amount of enemies to spawn each wave
    initial_enemies:          u32,
    /// Spawns additional (wave_index * N) enemies each wave
    enemies_incr_per_wave:    u32,
    /// Distance to player to spawn enemies at, randomized in this range
    enemy_spawn_radius_range: (f32, f32),
}

impl Default for WavesManagerAssets {
    fn default() -> Self {
        Self {
            spawn_every_n_secs:       5,
            initial_enemies:          3,
            enemies_incr_per_wave:    1,
            enemy_spawn_radius_range: (100.0, 200.0),
        }
    }
}

fn handle_waves_manager(
    mut commands: Commands,
    survival_time: Res<SurvivalTimer>,
    assets: Res<WavesManagerAssets>,
    mut wave_managers: Query<(Entity, &mut WaveCounter), With<WavesManager>>,
    players: Query<(Entity, &Transform), With<Player>>,
) {
    let time_s = survival_time.0.elapsed().as_secs() as u32;
    let expected_waves = time_s / assets.spawn_every_n_secs;

    for (player_entity, player_transform) in players {
        let player_pos = player_transform.translation.truncate();

        for (manager_entity, mut wave_counter) in &mut wave_managers {
            let waves_to_spawn = expected_waves.checked_sub(wave_counter.0).unwrap_or(0);
            if waves_to_spawn <= 0 {
                continue;
            }

            for _ in 0 .. waves_to_spawn {
                commands.entity(manager_entity).with_child(wave(
                    &assets,
                    wave_counter.0,
                    player_pos,
                ));
                wave_counter.0 += 1;
            }
        }
    }
}

fn wave(assets: &WavesManagerAssets, wave_index: u32, player_pos: Vec2) -> impl Bundle {
    let assets = assets.clone();

    // let enemies_to_spawn = wave_index * assets.enemies_incr_per_wave
    //     + if wave_index == 0 { assets.initial_enemies
    //     } else {
    //         0
    //     };

    let enemies_to_spawn = assets.initial_enemies + (wave_index * assets.enemies_incr_per_wave);

    (
        Wave,
        Name::new(format!("Wave {}", wave_index)),
        Transform::from_translation(player_pos.extend(0.0)),
        Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
            for enemy_index in 0 .. enemies_to_spawn {
                let distance = rand::rng().random_range(
                    assets.enemy_spawn_radius_range.0 ..= assets.enemy_spawn_radius_range.1,
                );

                let angle = (enemy_index as f32 / enemies_to_spawn as f32) * std::f32::consts::TAU;
                let offset = Vec2::new(angle.cos(), angle.sin()) * distance;
                let transform = Transform::from_translation(offset.extend(0.0));

                parent.spawn((
                    Enemy,
                    Name::new(format!("Enemy W{}-I{}", wave_index, enemy_index)),
                    transform,
                ));
            }
        })),
    )
}
