use crate::{
    AppSystems, GameplaySet,
    direction::Direction,
    game::{
        aim::AimDirection,
        movement::WalkDirection,
        player::{Player, PlayerAssets},
    },
};
use bevy::prelude::*;
use bevy_aseprite_ultra::prelude::{AnimationState, AseAnimation, NextFrameEvent};
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
            &AimDirection,
            &mut AseAnimation,
            &mut Sprite,
            &mut Transform,
        ),
        With<Player>,
    >,
) {
    use Direction::*;

    const TILT_DEG: f32 = 0.2;

    for (walk_direction, aim_direction, mut ase, mut sprite, mut transform) in &mut query {
        transform.rotation = Quat::IDENTITY;

        let (anim, anim_dir) = match (&walk_direction.0, &aim_direction.0) {
            (Some(_), Some(aim)) => (format!("walk-aim-{}", aim.abs_x()), Some(*aim)),
            (Some(walk), None) => ("walk".into(), Some(*walk)),
            (None, Some(aim)) => (format!("aim-{}", aim.abs_x()), Some(*aim)),
            (None, None) => ("idle".into(), None),
        };

        ase.animation.play_loop(anim);

        if let Some(dir) = anim_dir { match dir {
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
        } }
    }
}

fn play_step_sounds(
    trigger: On<NextFrameEvent>,
    mut commands: Commands,
    assets: Res<PlayerAssets>,
    players: Query<(&AseAnimation, &AnimationState), With<Player>>,
) {
    if let Ok(animation) = players.get(trigger.0)
        && animation.1.current_frame % 2 == 0
            && animation
                .0
                .animation
                .tag
                .as_ref()
                .map(|t| t.starts_with("walk"))
                .unwrap_or_default()
        {
            let rng = &mut rand::rng();
            let sfx = assets.steps.choose(rng).unwrap().clone();
            commands.spawn((AudioPlayer(sfx), PlaybackSettings::DESPAWN));
        }
}
