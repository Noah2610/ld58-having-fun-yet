use bevy::prelude::*;

pub mod animation;
pub mod ground;
pub mod level;
pub mod movement;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        level::plugin,
        // movement::plugin,
        player::plugin,
        ground::plugin,
    ));
}
