//! Player-specific behavior.

use crate::{
    asset_tracking::LoadResource,
    character_controller::CharacterControllerBundle,
    demo::{
        animation::PlayerAnimation,
        movement::{MovementController, ScreenWrap},
    },
};
use avian2d::prelude::Collider;
use bevy::{
    image::{ImageLoaderSettings, ImageSampler},
    prelude::*,
};
use bevy_yoleck::prelude::YoleckComponent;
use serde::{Deserialize, Serialize};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();
}

/// The player character.
pub fn player(player_assets: &PlayerAssets) -> impl Bundle {
    let player_animation = PlayerAnimation::new();

    (
        Name::new("Player"),
        Player,
        Sprite::from_atlas_image(player_assets.ducky.clone(), TextureAtlas {
            layout: player_assets.texture_atlas_layout.clone(),
            index:  player_animation.get_atlas_index(),
        }),
        // Transform::from_scale(Vec2::splat(8.0).extend(1.0)),
        ScreenWrap,
        player_animation,
        CharacterControllerBundle::new(Collider::rectangle(16.0, 16.0)),
    )
}

#[derive(
    Component,
    Reflect,
    Serialize,
    Deserialize,
    YoleckComponent,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
)]
#[reflect(Component)]
pub struct Player;

#[derive(Resource, Asset, Reflect, Clone)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    ducky:                Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            ducky:                world
                .resource::<AssetServer>()
                .load("images/ducky.png"),
            texture_atlas_layout: world
                .resource_mut::<Assets<TextureAtlasLayout>>()
                .add(TextureAtlasLayout::from_grid(
                    UVec2::splat(32),
                    6,
                    2,
                    Some(UVec2::splat(1)),
                    None,
                )),
        }
    }
}
