use bevy::prelude::*;

mod menu_action;
mod player_action;

pub use leafwing_input_manager::{common_conditions::*, prelude::*};
pub use menu_action::*;
pub use player_action::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((player_action::plugin, menu_action::plugin));

    #[cfg(not(target_family = "wasm"))]
    app.add_systems(
        Update,
        exit_app.run_if(action_just_pressed(MenuAction::QuitGame)),
    );
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
