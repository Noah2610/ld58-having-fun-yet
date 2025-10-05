mod player_action;

use bevy::prelude::App;
pub use leafwing_input_manager::{common_conditions::*, prelude::*};
pub use player_action::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(player_action::plugin);
}
