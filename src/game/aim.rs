use crate::{
    AppSystems, GameplaySet,
    direction::Direction,
    game::mouse_aim::MouseAimEnabled,
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
    mouse_aim_enabled: Res<State<MouseAimEnabled>>,
) {
    let action = action_state
        .clamped_axis_pair(&PlayerAction::Aim)
        .normalize_or_zero();

    for mut aim_direction in &mut controllers {
        if let Ok(dir) = Direction::try_from(action) {
            aim_direction.0 = Some(dir);
        } else if !mouse_aim_enabled.0 {
            aim_direction.0 = None;
        }
    }
}
