use crate::{
    game::{
        bullet::{Bullet, RetrieveBall},
        enemy::EnemiesEnabled,
        player::{Invincible, Player},
        survival_timer::SurvivalTimer,
    },
    game_state::{ActiveGameplayForced, Paused},
    screens::Screen,
};
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use leafwing_input_manager::{common_conditions::action_just_pressed, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<DebugAction>::default())
        .init_resource::<ActionState<DebugAction>>()
        .init_resource::<RetrieveBallSpeed>()
        .insert_resource(DebugAction::default_input_map());

    app.init_state::<ActiveGameplayForced>();

    app.add_systems(
        Update,
        (
            (
                enable_pause.run_if(in_state(ActiveGameplayForced(false))),
                disable_pause.run_if(in_state(ActiveGameplayForced(true))),
            )
                .run_if(action_just_pressed(DebugAction::Pause)),
            (
                enable_enemy_behavior.run_if(in_state(EnemiesEnabled(false))),
                disable_enemy_behavior.run_if(in_state(EnemiesEnabled(true))),
            )
                .run_if(action_just_pressed(DebugAction::ToggleEnemyBehavior)),
            reset_survival_timer.run_if(action_just_pressed(DebugAction::ResetSurvivalTimer)),
            toggle_invincible.run_if(action_just_pressed(DebugAction::ToggleInvincible)),
            retrieve_ball.run_if(action_just_pressed(DebugAction::RetrieveBall)),
            handle_retrieve_ball,
            handle_survival_time_add,
        )
            .run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct RetrieveBallSpeed(f32);
impl Default for RetrieveBallSpeed {
    fn default() -> Self {
        RetrieveBallSpeed(200.0)
    }
}

fn retrieve_ball(
    mut commands: Commands,
    ball: Single<Entity, (With<Bullet>, Without<RetrieveBall>)>,
) {
    commands.entity(ball.entity()).insert(RetrieveBall);
}

fn handle_retrieve_ball(
    retrieve_ball_speed: Res<RetrieveBallSpeed>,
    balls: Query<(&GlobalTransform, &mut LinearVelocity), (With<Bullet>, With<RetrieveBall>)>,
    player: Single<&GlobalTransform, (With<Player>, Without<Bullet>, Without<RetrieveBall>)>,
) {
    let player_translation = player.translation().truncate();
    for (ball_transform, mut velocity) in balls {
        let direction =
            (player_translation - ball_transform.translation().truncate()).normalize_or_zero();
        velocity.0 = direction * retrieve_ball_speed.0;
    }
}

fn enable_pause(
    mut pause: ResMut<NextState<Paused>>,
    mut active: ResMut<NextState<ActiveGameplayForced>>,
) {
    pause.set(Paused(true));
    active.set(ActiveGameplayForced(true));
}

fn disable_pause(
    mut pause: ResMut<NextState<Paused>>,
    mut active: ResMut<NextState<ActiveGameplayForced>>,
) {
    pause.set(Paused(false));
    active.set(ActiveGameplayForced(false));
}

fn enable_enemy_behavior(mut state: ResMut<NextState<EnemiesEnabled>>) {
    state.set(EnemiesEnabled(true));
}

fn disable_enemy_behavior(mut state: ResMut<NextState<EnemiesEnabled>>) {
    state.set(EnemiesEnabled(false));
}

fn reset_survival_timer(mut timer: ResMut<SurvivalTimer>) {
    timer.0.reset();
}

fn toggle_invincible(
    mut commands: Commands,
    players: Query<(Entity, Has<Invincible>), With<Player>>,
) {
    for (entity, is_invincible) in players {
        if is_invincible {
            commands.entity(entity).remove::<Invincible>();
        } else {
            commands.entity(entity).insert(Invincible);
        }
    }
}

fn handle_survival_time_add(
    state: Res<ActionState<DebugAction>>,
    mut survival_timer: ResMut<SurvivalTimer>,
) {
    use std::time::Duration;

    for action in state.get_just_pressed() {
        if let DebugAction::SurvivalTimeAddSeconds(secs) = action {
            if secs.is_positive() {
                survival_timer.0.tick(Duration::from_secs(secs as u64));
            } else if secs.is_negative() {
                let new = Duration::from_secs_f32(
                    (survival_timer.0.elapsed_secs() - secs.abs() as f32).max(0.0),
                );
                survival_timer.0.set_elapsed(new);
            }
        }
    }
}

#[derive(Actionlike, Reflect, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DebugAction {
    ToggleFps,
    ToggleInspector,
    ToggleVisualInspectors,
    ToggleGizmos,
    ToggleUiDebug,
    /// Global pause toggle, but without pause menu UI
    Pause,
    /// Toggles enemy behavior, wave spawning, and survival timer
    ToggleEnemyBehavior,
    ResetSurvivalTimer,
    SurvivalTimeAddSeconds(i32),
    ToggleInvincible,
    RetrieveBall,
}

impl DebugAction {
    fn default_input_map() -> InputMap<Self> {
        use DebugAction::*;
        InputMap::default()
            .with(ToggleFps, KeyCode::F1)
            .with(ToggleInspector, KeyCode::F2)
            .with(ToggleVisualInspectors, KeyCode::F3)
            .with(ToggleGizmos, KeyCode::F3)
            .with(ToggleUiDebug, KeyCode::F4)
            .with(Pause, ModifierKey::Control.with(KeyCode::KeyO))
            .with(
                ToggleEnemyBehavior,
                ModifierKey::Control.with(KeyCode::KeyE),
            )
            .with(ResetSurvivalTimer, ModifierKey::Control.with(KeyCode::KeyR))
            .with(SurvivalTimeAddSeconds(10), KeyCode::Period)
            .with(SurvivalTimeAddSeconds(-10), KeyCode::Comma)
            .with(
                SurvivalTimeAddSeconds(60),
                ModifierKey::Control.with(KeyCode::Period),
            )
            .with(
                SurvivalTimeAddSeconds(-60),
                ModifierKey::Control.with(KeyCode::Comma),
            )
            .with(ToggleInvincible, ModifierKey::Control.with(KeyCode::KeyI))
            .with(RetrieveBall, KeyCode::KeyR)
    }
}
