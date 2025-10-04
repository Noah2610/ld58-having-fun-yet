//! Player-specific behavior.

use crate::{
    actions::Action,
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
use leafwing_input_manager::prelude::{
    GamepadControlAxis,
    InputMap,
    VirtualAxis,
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();
}

/// The player character.
pub fn player(max_speed: f32, player_assets: &PlayerAssets) -> impl Bundle {
    let player_animation = PlayerAnimation::new();

    (
        Name::new("Player"),
        Player,
        Sprite::from_atlas_image(player_assets.ducky.clone(), TextureAtlas {
            layout: player_assets.texture_atlas_layout.clone(),
            index:  player_animation.get_atlas_index(),
        }),
        Transform::from_scale(Vec2::splat(8.0).extend(1.0)),
        MovementController {
            max_speed,
            ..default()
        },
        ScreenWrap,
        player_animation,
        CharacterControllerBundle::new(Collider::circle(16.0)),
        InputMap::default()
            .with_axis(Action::Move, VirtualAxis::ad())
            .with_axis(Action::Move, VirtualAxis::horizontal_arrow_keys())
            .with_axis(Action::Move, GamepadControlAxis::LEFT_X)
            .with_axis(Action::Move, VirtualAxis::dpad_x())
            .with(Action::Jump, KeyCode::Space)
            .with(Action::Jump, KeyCode::KeyK)
            .with(Action::Jump, GamepadButton::South),
    )
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
struct Player;

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    ducky:                Handle<Image>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            ducky: world.resource::<AssetServer>().load_with_settings(
                "images/ducky.png",
                |settings: &mut ImageLoaderSettings| {
                    // Use `nearest` image sampling to preserve pixel art style.
                    settings.sampler = ImageSampler::nearest();
                },
            ),

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
