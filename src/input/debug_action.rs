use crate::{
    game::{enemy::EnemiesEnabled, survival_timer::SurvivalTimer},
    game_state::{ActiveGameplayForced, Paused},
    screens::Screen,
};
use bevy::prelude::*;
use leafwing_input_manager::{common_conditions::action_just_pressed, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<DebugAction>::default())
        .init_resource::<ActionState<DebugAction>>()
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
            handle_survival_time_add,
        )
            .run_if(in_state(Screen::Gameplay)),
    );
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
    ToggleGizmos,
    ToggleUiDebug,
    /// Global pause toggle, but without pause menu UI
    Pause,
    /// Toggles enemy behavior, wave spawning, and survival timer
    ToggleEnemyBehavior,
    ResetSurvivalTimer,
    SurvivalTimeAddSeconds(i32),
}

impl DebugAction {
    fn default_input_map() -> InputMap<Self> {
        use DebugAction::*;
        InputMap::default()
            .with(ToggleFps, KeyCode::F1)
            .with(ToggleInspector, KeyCode::F2)
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
    }
}
