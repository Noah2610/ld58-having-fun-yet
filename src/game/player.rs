use crate::{
    AppSystems,
    asset_tracking::LoadResource,
    game::{
        aim::AimController,
        bullet::BulletSpawner,
        movement::{Acceleration, MovementController},
        util::FixObjectColliders,
    },
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
    players: Query<Entity, (Added<Player>, Without<PlayerInitialized>)>,
) {
    for entity in players {
        commands
            .entity(entity)
            .insert((PlayerInitialized, AseAnimation {
                aseprite:  assets.spritesheet.clone(),
                animation: Animation::tag("idle"),
            }));
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
    FixObjectColliders,
    Sprite::default(),
    Anchor::CENTER,
    MovementController,
    AimController,
    Acceleration(1800.0),
    LinearDamping(15.0),
    RigidBody::Dynamic,
    LockedAxes::ROTATION_LOCKED,
    BulletSpawner
)]
pub struct Player;

#[derive(Component)]
struct PlayerInitialized;

#[derive(Resource, Asset, Reflect, Clone)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    spritesheet: Handle<Aseprite>,
    #[dependency]
    pub steps:   Vec<Handle<AudioSource>>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            spritesheet: world
                .resource::<AssetServer>()
                .load("spritesheets/player.ase"),
            steps:       {
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
