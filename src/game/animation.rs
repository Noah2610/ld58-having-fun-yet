//! Player sprite animation.
//! This is based on multiple examples and may be very different for your game.
//! - [Sprite flipping](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_flipping.rs)
//! - [Sprite animation](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
//! - [Timers](https://github.com/bevyengine/bevy/blob/latest/examples/time/timers.rs)

use crate::{
    AppSystems,
    GameplaySet,
    game::player::{Player, PlayerAssets},
};
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation, Aseprite};
use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    // Animate based on controls.
    app.add_systems(
        Update,
        update_animation
            .in_set(AppSystems::Update)
            .in_set(GameplaySet),
    );
}

fn update_animation(
    mut query: Query<
        (&LinearVelocity, &mut AseAnimation, &mut Sprite),
        With<Player>,
    >,
) {
    for (velocity, mut ase, mut sprite) in &mut query {
        let dx = velocity.x;
        if dx.abs() > 0.1 {
            sprite.flip_x = dx < 0.0;
        }

        if velocity.0.abs().max_element() < 0.3 {
            ase.animation.play_loop("idle");
        } else {
            ase.animation.play_loop("walk");
        };
    }
}
