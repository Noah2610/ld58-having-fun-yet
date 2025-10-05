//! Player sprite animation.
//! This is based on multiple examples and may be very different for your game.
//! - [Sprite flipping](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_flipping.rs)
//! - [Sprite animation](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_animation.rs)
//! - [Timers](https://github.com/bevyengine/bevy/blob/latest/examples/time/timers.rs)

use crate::{
    AppSystems, GameplaySet,
    game::{
        movement::{Direction, WalkDirection},
        player::{Player, PlayerAssets},
    },
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{AnimationState, AseAnimation, Aseprite, NextFrameEvent};
use rand::prelude::IndexedRandom;

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
    mut query: Query<
        (
            &WalkDirection,
            &mut AseAnimation,
            &mut Sprite,
            &mut Transform,
        ),
        With<Player>,
    >,
) {
    use Direction::*;

    const TILT_DEG: f32 = 0.2;

    for (direction, mut ase, mut sprite, mut transform) in &mut query {
        transform.rotation = Quat::IDENTITY;

        match &direction.0 {
            Some(dir) => {
                ase.animation.play_loop("walk");
                match dir {
                    Top | Bottom => { /* Retain current flip */ },
                    Left => sprite.flip_x = true,
                    Right => sprite.flip_x = false,
                    TopLeft => {
                        sprite.flip_x = true;
                        transform.rotation = Quat::from_rotation_z(-TILT_DEG);
                    },
                    TopRight => {
                        sprite.flip_x = false;
                        transform.rotation = Quat::from_rotation_z(TILT_DEG);
                    },
                    BottomLeft => {
                        sprite.flip_x = true;
                        transform.rotation = Quat::from_rotation_z(TILT_DEG);
                    },
                    BottomRight => {
                        sprite.flip_x = false;
                        transform.rotation = Quat::from_rotation_z(-TILT_DEG);
                    },
                }
            },
            None => ase.animation.play_loop("idle"),
        }
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
