use bevy::prelude::*;

pub mod aim;
pub mod animation;
pub mod level;
pub mod movement;
pub mod player;
pub mod solid;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        level::plugin,
        movement::plugin,
        player::plugin,
        solid::plugin,
        aim::plugin,
    ));
}
