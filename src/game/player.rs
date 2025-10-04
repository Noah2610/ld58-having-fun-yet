//! Player-specific behavior.

use crate::{
    AppSystems,
    asset_tracking::LoadResource,
    game::{
        animation::PlayerAnimation,
        movement::{Acceleration, MovementController},
    },
    game_state::GameplaySet,
};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_yoleck::prelude::YoleckComponent;
use serde::{Deserialize, Serialize};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();

    app.add_systems(
        Update,
        insert_player_sprite_and_animation
            .in_set(GameplaySet)
            .in_set(AppSystems::Update),
    );
}

fn insert_player_sprite_and_animation(
    mut commands: Commands,
    assets: Res<PlayerAssets>,
    players: Query<Entity, (Added<Player>, Without<PlayerInitialized>)>,
) {
    for entity in players {
        let animation = PlayerAnimation::new();
        commands.entity(entity).insert((
            PlayerInitialized,
            Sprite::from_atlas_image(assets.ducky.clone(), TextureAtlas {
                layout: assets.texture_atlas_layout.clone(),
                index:  animation.get_atlas_index(),
            }),
            animation,
        ));
    }
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
#[require(
    Name::new("Player"),
    MovementController,
    Acceleration(1800.0),
    LinearDamping(15.0),
    RigidBody::Dynamic,
    // Collider::rectangle(32.0, 32.0),
    LockedAxes::ROTATION_LOCKED
)]
pub struct Player;

#[derive(Component)]
struct PlayerInitialized;

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
