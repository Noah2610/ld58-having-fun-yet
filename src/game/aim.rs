use crate::{
    AppSystems, GameplaySet,
    direction::Direction,
    input::{ActionState, PlayerAction},
};
use avian2d::math::Scalar;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        sync_aim_direction
            .in_set(AppSystems::Update)
            .in_set(GameplaySet),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
#[require(AimDirection)]
pub struct AimController;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AimDirection(pub Option<Direction>);

fn sync_aim_direction(
    action_state: Res<ActionState<PlayerAction>>,
    mut controllers: Query<&mut AimDirection, With<AimController>>,
) {
    const MIN_POS: Scalar = 0.1;
    const MAX_POS: Scalar = 1.0;
    const MIN_NEG: Scalar = -MIN_POS;
    const MAX_NEG: Scalar = -MAX_POS;

    let action = action_state
        .clamped_axis_pair(&PlayerAction::Aim)
        .normalize_or_zero();

    for mut aim_direction in &mut controllers {
        aim_direction.0 = match (action.x, action.y) {
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
