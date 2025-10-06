use bevy::prelude::*;

pub mod waves;
pub mod aim;
pub mod animation;
pub mod bullet;
pub mod enemy;
pub mod level;
pub mod movement;
pub mod player;
pub mod solid;
pub mod survival_timer;
pub mod visuals;

mod util;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        animation::plugin,
        level::plugin,
        movement::plugin,
        util::plugin,
        player::plugin,
        solid::plugin,
        aim::plugin,
        bullet::plugin,
        enemy::plugin,
        visuals::plugin,
        survival_timer::plugin,
        waves::plugin,
    ));
}
