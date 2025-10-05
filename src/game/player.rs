use crate::{
    AppSystems,
    asset_tracking::LoadResource,
    game::movement::{Acceleration, MovementController},
    game_state::GameplaySet,
};
use avian2d::prelude::*;
use bevy::{prelude::*, sprite::Anchor};
use bevy_aseprite_ultra::prelude::{Animation, AseAnimation, Aseprite};
use bevy_yoleck::prelude::YoleckComponent;
use serde::{Deserialize, Serialize};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<PlayerAssets>();

    app.add_systems(
        Update,
        post_add_player
            .in_set(GameplaySet)
            .in_set(AppSystems::Update),
    );
}

fn post_add_player(
    mut commands: Commands,
    assets: Res<PlayerAssets>,
    players: Query<(Entity, &Children), (Added<Player>, Without<PlayerInitialized>)>,
    mut colliders: Query<(&ColliderAabb, &mut Transform), (With<Collider>, With<ChildOf>)>,
) {
    for (entity, children) in players {
        // Create animation and sprite from spritesheet for the player
        commands
            .entity(entity)
            .insert((PlayerInitialized, AseAnimation {
                aseprite:  assets.spritesheet.clone(),
                animation: Animation::tag("idle"),
            }));

        // Offset all children colliders to be centered on the player
        for &child in children {
            if let Ok((aabb, mut transform)) = colliders.get_mut(child) {
                let half = (aabb.size() * 0.5).extend(0.0);
                transform.translation.x -= half.x;
                transform.translation.y += half.y;
            }
        }
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
    Sprite::default(),
    Anchor::CENTER,
    MovementController,
    Acceleration(1800.0),
    LinearDamping(15.0),
    RigidBody::Dynamic,
    Collider::default(), // Player needs a collider in order for its children colliders to work
    LockedAxes::ROTATION_LOCKED
)]
pub struct Player;

#[derive(Component)]
struct PlayerInitialized;

#[derive(Resource, Asset, Reflect, Clone)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    spritesheet:          Handle<Aseprite>,
    texture_atlas_layout: Handle<TextureAtlasLayout>,
    #[dependency]
    pub steps:            Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            spritesheet:          world
                .resource::<AssetServer>()
                .load("spritesheets/player.ase"),
            texture_atlas_layout: world.resource_mut::<Assets<TextureAtlasLayout>>().add(
                TextureAtlasLayout::from_grid(UVec2::splat(32), 6, 2, Some(UVec2::splat(1)), None),
            ),
            steps:                {
                let assets = world.resource::<AssetServer>();
                vec![
                    assets.load("audio/steps/step1.ogg"),
                    assets.load("audio/steps/step2.ogg"),
                    assets.load("audio/steps/step3.ogg"),
                    assets.load("audio/steps/step4.ogg"),
                ]
            },
        }
    }
}
