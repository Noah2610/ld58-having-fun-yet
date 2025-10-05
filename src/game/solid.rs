use crate::game::util::{CollisionTag, FixObjectColliders};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_yoleck::prelude::YoleckComponent;
use serde::{Deserialize, Serialize};

pub fn plugin(_app: &mut App) {}

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
#[reflect(Component)]
#[require(
    Name::new("Solid"),
    FixObjectColliders,
    RigidBody::Static,
    CollisionLayers::new(CollisionTag::Solid, CollisionTag::Entity)
)]
pub struct Solid;

#[derive(Resource, Asset, Reflect, Clone)]
pub struct SolidAssets {}

impl FromWorld for SolidAssets {
    fn from_world(_world: &mut World) -> Self {
        SolidAssets {}
    }
}
