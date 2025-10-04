use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_yoleck::prelude::YoleckComponent;
use serde::{Deserialize, Serialize};

pub fn plugin(app: &mut App) {
}

#[derive(
    Component,
    Reflect,
    Serialize,
    Deserialize,
    YoleckComponent,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Default,
)]
pub struct Ground;

pub fn ground() -> impl Bundle {
    (
        Name::new("Ground"),
        Ground,
        Transform::default(),
        RigidBody::Static,
        Collider::rectangle(16.0, 16.0),
    )
}

#[derive(Resource, Asset, Reflect, Clone)]
pub struct GroundAssets {}

impl FromWorld for GroundAssets {
    fn from_world(_world: &mut World) -> Self {
        GroundAssets {}
    }
}
