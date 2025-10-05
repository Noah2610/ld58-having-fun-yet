use crate::{
    AppSystems, GameplaySet,
    input::{ActionState, PlayerAction},
};
use avian2d::{math::Scalar, prelude::*};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        apply_movement
            .in_set(AppSystems::Update)
            .in_set(GameplaySet),
    );
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementController;

#[derive(Component, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Acceleration(pub Scalar);

fn apply_movement(
    time: Res<Time>,
    action_state: Res<ActionState<PlayerAction>>,
    mut controllers: Query<(&Acceleration, &mut LinearVelocity), With<MovementController>>,
) {
    const DEADZONE: f32 = 0.1;

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
