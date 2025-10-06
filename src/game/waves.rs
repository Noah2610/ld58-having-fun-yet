use crate::{
    AppSystems, GameplaySet,
    game::{
        enemy::{Enemy, EnemyVariant},
        player::Player,
        score::Score,
        survival_timer::SurvivalTimer,
    },
};
use bevy::{ecs::relationship::RelatedSpawner, prelude::*};
use rand::Rng;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        handle_waves_manager
            .in_set(AppSystems::Update)
            .in_set(GameplaySet),
    );
}

#[derive(Bundle)]
pub struct WavesManagerBundle {
    manager:  WavesManager,
    settings: WavesManagerSettings,
}

impl WavesManagerBundle {
    fn from_settings(settings: WavesManagerSettings) -> Self {
        Self {
            manager: WavesManager,
            settings,
        }
    }
}

pub fn waves_managers() -> Vec<WavesManagerBundle> {
    vec![
        WavesManagerBundle::from_settings(WavesManagerSettings::default()),
        WavesManagerBundle::from_settings(WavesManagerSettings {
            enemy_variant:            EnemyVariant::Bigger,
            spawn_every_n_secs:       60,
            initial_enemies:          1,
            enemies_incr_per_wave:    1,
            enemy_spawn_radius_range: (250.0, 300.0),
            score_mult:               500.0,
        }),
    ]
}

#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
struct WavesManagerSettings {
    /// Enemy variant this waves manager spawns
    enemy_variant:            EnemyVariant,
    /// Spawn a new wave every N seconds of the survival timer
    spawn_every_n_secs:       u32,
    /// Base amount of enemies to spawn each wave
    initial_enemies:          u32,
    /// Spawns additional (wave_index * N) enemies each wave
    enemies_incr_per_wave:    u32,
    /// Distance to player to spawn enemies at, randomized in this range
    enemy_spawn_radius_range: (f32, f32),
    /// Add (wave_index * score_mult) on new wave
    score_mult:               f32,
}

impl Default for WavesManagerSettings {
    fn default() -> Self {
        Self {
            enemy_variant:            EnemyVariant::Basic,
            spawn_every_n_secs:       5,
            initial_enemies:          1,
            enemies_incr_per_wave:    1,
            enemy_spawn_radius_range: (100.0, 200.0),
            score_mult:               1.0,
        }
    }
}

/// Spawns waves of enemies at specific times based on survival time.
#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
#[require(
    Name::new("WavesManager"),
    WaveCounter,
    WavesManagerSettings,
    Transform,
    Visibility
)]
pub struct WavesManager;

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
#[require(Name::new("Wave"), Transform, Visibility)]
pub struct Wave;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct WaveCounter(u32);

fn handle_waves_manager(
    mut commands: Commands,
    survival_time: Res<SurvivalTimer>,
    mut score: ResMut<Score>,
    mut wave_managers: Query<(Entity, &WavesManagerSettings, &mut WaveCounter), With<WavesManager>>,
    players: Query<&Transform, With<Player>>,
) {
    for player_transform in players {
        let player_pos = player_transform.translation.truncate();

        for (manager_entity, settings, mut wave_counter) in &mut wave_managers {
            let time_s = survival_time.0.elapsed().as_secs() as u32;
            let expected_waves = time_s / settings.spawn_every_n_secs;

            let waves_to_spawn = expected_waves.checked_sub(wave_counter.0).unwrap_or(0);
            if waves_to_spawn <= 0 {
                continue;
            }

            for _ in 0 .. waves_to_spawn {
                commands.entity(manager_entity).with_child(wave(
                    settings.clone(),
                    wave_counter.0,
                    player_pos,
                ));
                wave_counter.0 += 1;
                score.0 += (wave_counter.0 as f32 * settings.score_mult) as u32;
            }
        }
    }
}

fn wave(settings: WavesManagerSettings, wave_index: u32, player_pos: Vec2) -> impl Bundle {
    // let enemies_to_spawn = wave_index * assets.enemies_incr_per_wave
    //     + if wave_index == 0 { assets.initial_enemies
    //     } else {
    //         0
    //     };

    let enemies_to_spawn = settings.initial_enemies + (wave_index * settings.enemies_incr_per_wave);

    (
        Wave,
        Name::new(format!("Wave {}", wave_index)),
        Transform::from_translation(player_pos.extend(0.0)),
        Children::spawn(SpawnWith(move |parent: &mut RelatedSpawner<ChildOf>| {
            for enemy_index in 0 .. enemies_to_spawn {
                let distance = rand::rng().random_range(
                    settings.enemy_spawn_radius_range.0 ..= settings.enemy_spawn_radius_range.1,
                );

                let angle = (enemy_index as f32 / enemies_to_spawn as f32) * std::f32::consts::TAU;
                let offset = Vec2::new(angle.cos(), angle.sin()) * distance;
                let transform = Transform::from_translation(offset.extend(0.0));

                parent.spawn((
                    Enemy,
                    settings.enemy_variant,
                    Name::new(format!("Enemy W{}-I{}", wave_index, enemy_index)),
                    transform,
                ));
            }
        })),
    )
}
