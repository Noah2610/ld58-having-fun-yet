use crate::game::visuals::{
    AnimationDirection, HueAnimation, LightnessAnimation, PositionXAnimation, PositionYAnimation,
    RotationAnimation, SaturationAnimation, ScaleXAnimation, ScaleYAnimation, SetSpriteColor,
    VisualAnimation,
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

    Sprite::from_color(Color::WHITE, Vec2::new(16.0, 16.0)),

    SaturationAnimation(VisualAnimation {
        period: 10.0,
        direction: AnimationDirection::Boomerang,
        time_offset: rand::rng().random_range(0.0 .. 10.0),
        ..default()
    }),
    LightnessAnimation(VisualAnimation {
        period: 10.0,
        direction: AnimationDirection::Boomerang,
        time_offset: rand::rng().random_range(0.0 .. 10.0),
        ..default()
    }),

    PositionXAnimation(VisualAnimation {
        direction: AnimationDirection::Boomerang,
        range: Some((rand::rng().random_range(-128.0 .. -1.0), rand::rng().random_range(1.0 .. 128.0))),
        period: rand::rng().random_range(1.0 .. 8.0),
        // time_offset: rand::rng().random_range(0.0 .. 4.0),
        ..default()
    }),
    PositionYAnimation(VisualAnimation {
        direction: AnimationDirection::Boomerang,
        range: Some((rand::rng().random_range(-128.0 .. -1.0), rand::rng().random_range(1.0 .. 128.0))),
        period: rand::rng().random_range(1.0 .. 8.0),
        // time_offset: rand::rng().random_range(0.0 .. 4.0),
        ..default()
    }),
    ScaleXAnimation(VisualAnimation{
        direction: AnimationDirection::Boomerang,
        period: rand::rng().random_range(1.0 .. 8.0),
        range: Some((rand::rng().random_range(0.0 .. 0.5), rand::rng().random_range(0.5 .. 2.0))),
        time_offset: 0.0,
    }),
    ScaleYAnimation(VisualAnimation{
        direction: AnimationDirection::Boomerang,
        period: rand::rng().random_range(1.0 .. 8.0),
        range: Some((rand::rng().random_range(0.0 .. 0.5), rand::rng().random_range(0.5 .. 2.0))),
        time_offset: 2.0,
    }),
    RotationAnimation(VisualAnimation{
        direction: AnimationDirection::Boomerang,
        period: rand::rng().random_range(1.0 .. 8.0),
        range: Some((rand::rng().random_range(-1.0 .. 0.0), rand::rng().random_range(0.0 .. 1.0))),
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
