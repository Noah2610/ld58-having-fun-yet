use crate::game::visuals::{
    AnimationDirection, HueAnimation, RotationAnimation, ScaleXAnimation, ScaleYAnimation,
    SetSpriteColor, VisualAnimation,
};
use bevy::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

pub fn plugin(_app: &mut App) {}

#[derive(
    Component, Reflect, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Default, Debug,
)]
#[reflect(Component)]
#[require(
    Name::new("Decoration"),
    SetSpriteColor(Color::hsl(0.0, 0.1, 0.25)),
    HueAnimation(VisualAnimation {
        period: 10.0,
        direction: AnimationDirection::Boomerang,
        time_offset: rand::rng().random_range(0.0 .. 10.0),
        ..default()
    }),

    // Transform,
    // RotationAnimation(VisualAnimation {
    //     period: 10.0,
    //     direction: AnimationDirection::Linear,
    //     time_offset: rand::rng().random_range(0.0 .. 10.0),
    //     ..default()
    // }),
    // ScaleXAnimation(VisualAnimation {
    //     period: 4.0,
    //     range: Some((0.7, 1.3)),
    //     direction: AnimationDirection::Boomerang,
    //     time_offset: rand::rng().random_range(0.0 .. 4.0),
    //     ..default()
    // }),
    // ScaleYAnimation(VisualAnimation {
    //     period: 4.0,
    //     range: Some((0.7, 1.3)),
    //     direction: AnimationDirection::Boomerang,
    //     time_offset: rand::rng().random_range(0.0 .. 4.0),
    //     ..default()
    // }),
)]
pub struct Decoration;
