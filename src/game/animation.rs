//! Player sprite animation.
//! This is based on multiple examples and may be very different for your game.
//! - [Sprite flipping](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_flipping.rs)
//! - [Sprite animation](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
//! - [Timers](https://github.com/bevyengine/bevy/blob/latest/examples/time/timers.rs)

use crate::{
    AppSystems, GameplaySet,
    game::player::{Player, PlayerAssets},
};
use avian2d::prelude::LinearVelocity;
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{AnimationState, AseAnimation, Aseprite, NextFrameEvent};
use rand::{Rng, prelude::IndexedRandom};
use std::time::Duration;

pub(super) fn plugin(app: &mut App) {
    // Animate based on controls.
    app.add_systems(
        Update,
        update_animation
            .in_set(AppSystems::Update)
            .in_set(GameplaySet),
    )
    .add_observer(play_step_sounds);
}

fn update_animation(
    mut query: Query<(&LinearVelocity, &mut AseAnimation, &mut Sprite), With<Player>>,
) {
    for (velocity, mut ase, mut sprite) in &mut query {
        let dx = velocity.x;
        if dx.abs() > 0.1 {
            sprite.flip_x = dx < 0.0;
        }

        if velocity.0.abs().max_element() < 5.0 {
            ase.animation.play_loop("idle");
        } else {
            ase.animation.play_loop("walk");
        };
    }
}

fn play_step_sounds(
    _: On<NextFrameEvent>,
    mut commands: Commands,
    assets: Res<PlayerAssets>,
    animation: Single<(&AseAnimation, &AnimationState), With<Player>>,
) {
    if animation.1.current_frame % 2 == 0
        && animation
            .0
            .animation
            .tag
            .as_ref()
            .map(|t| t == "walk")
            .unwrap_or_default()
    {
        let rng = &mut rand::rng();
        let sfx = assets.steps.choose(rng).unwrap().clone();
        commands.spawn((AudioPlayer(sfx), PlaybackSettings::DESPAWN));
    }
}
