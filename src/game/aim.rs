use crate::game::movement::Direction;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AimDirection(pub Option<Direction>);
