use crate::{
    game_state::{AppSystems, GameplaySet},
    input::{ActionState, MouseAction},
};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<MouseAimEnabled>();

    app.add_systems(OnEnter(MouseAimEnabled(true)), enable_mouse_aim);
    app.add_systems(OnExit(MouseAimEnabled(true)), disable_mouse_aim);

    app.add_systems(
        Update,
        handle_mouse_aim
            .run_if(mouse_aim_enabled)
            .in_set(GameplaySet)
            .in_set(AppSystems::Update),
    );
}

fn mouse_aim_enabled(mouse_enabled: Res<State<MouseAimEnabled>>) -> bool {
    mouse_enabled.0
}

#[derive(States, Reflect, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[reflect(State)]
pub struct MouseAimEnabled(pub bool);

impl FromWorld for MouseAimEnabled {
    fn from_world(_world: &mut World) -> Self {
        Self(false)
    }
}

fn enable_mouse_aim() {}

fn disable_mouse_aim() {}

fn handle_mouse_aim(action_state: Res<ActionState<MouseAction>>) {
    let pan_vec = action_state.axis_pair(&MouseAction::Pan);
    dbg!(pan_vec);
}
