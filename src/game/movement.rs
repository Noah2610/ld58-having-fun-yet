use crate::{
    AppSystems,
    direction::Direction,
    game::health::Dead,
    game_state::ActiveGameplaySet,
    input::{ActionState, PlayerAction},
};
use avian2d::{math::Scalar, prelude::*};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (apply_movement, sync_walk_direction)
            .in_set(AppSystems::Update)
            .in_set(ActiveGameplaySet),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(Acceleration(1000.0), WalkDirection)]
pub struct MovementController;

#[derive(Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Acceleration(pub Scalar);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct WalkDirection(pub Option<Direction>);

fn apply_movement(
    time: Res<Time>,
    action_state: Res<ActionState<PlayerAction>>,
    mut controllers: Query<
        (&Acceleration, &mut LinearVelocity),
        (With<MovementController>, Without<Dead>),
    >,
) {
    const DEADZONE: Scalar = 0.1;

    let delta_time = time.delta_secs();

    let direction = action_state
        .clamped_axis_pair(&PlayerAction::Move)
        .normalize_or_zero();

    if direction.abs().max_element() < DEADZONE {
        return;
    }

    for (movement_acceleration, mut linear_velocity) in &mut controllers {
        **linear_velocity += direction * movement_acceleration.0 * delta_time;
    }
}

fn sync_walk_direction(
    action_state: Res<ActionState<PlayerAction>>,
    mut controllers: Query<&mut WalkDirection, With<MovementController>>,
) {
    const MIN_POS: Scalar = 0.1;
    const MAX_POS: Scalar = 1.0;
    const MIN_NEG: Scalar = -MIN_POS;
    const MAX_NEG: Scalar = -MAX_POS;

    let action = action_state
        .clamped_axis_pair(&PlayerAction::Move)
        .normalize_or_zero();

    for mut walk_direction in &mut controllers {
        walk_direction.0 = match (action.x, action.y) {
            (MAX_NEG ..= MIN_NEG, MIN_POS ..= MAX_POS) => Some(Direction::TopLeft),
            (MIN_POS ..= MAX_POS, MIN_POS ..= MAX_POS) => Some(Direction::TopRight),
            (MAX_NEG ..= MIN_NEG, MAX_NEG ..= MIN_NEG) => Some(Direction::BottomLeft),
            (MIN_POS ..= MAX_POS, MAX_NEG ..= MIN_NEG) => Some(Direction::BottomRight),
            (MIN_NEG .. MIN_POS, MIN_POS ..= MAX_POS) => Some(Direction::Top),
            (MIN_NEG .. MIN_POS, MAX_NEG ..= MIN_NEG) => Some(Direction::Bottom),
            (MAX_NEG ..= MIN_NEG, MIN_NEG .. MIN_POS) => Some(Direction::Left),
            (MIN_POS ..= MAX_POS, MIN_NEG .. MIN_POS) => Some(Direction::Right),
            (_, _) => None,
        }
    }
}
