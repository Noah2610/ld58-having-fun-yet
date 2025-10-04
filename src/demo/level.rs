//! Spawn the main level.

use crate::{
    asset_tracking::LoadResource,
    demo::player::{Player, PlayerAssets, player},
    screens::Screen,
};
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    map: Handle<TiledMapAsset>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        Self {
            map: world.resource::<AssetServer>().load("maps/dev.tmx"),
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    player_assets: Res<PlayerAssets>,
) {
    commands
        .spawn((
            Name::new("Level"),
            TiledMap(level_assets.map.clone()),
            TiledPhysicsSettings::<TiledPhysicsAvianBackend> {
                objects_filter: TiledFilter::All,
                objects_layer_filter: TiledFilter::Names(vec!["solid".into()]),
                ..default()
            },
            DespawnOnExit(Screen::Gameplay),
            // children![
            //     player(&player_assets),
            //     // (
            //     //     Name::new("Gameplay Music"),
            //     //     music(level_assets.music.clone())
            //     // )
            // ],
        ))
        .observe(
            |ev: On<TiledEvent<MapCreated>>,
             mut commands: Commands,
             assets: Res<PlayerAssets>,
             players: Query<Entity, With<Player>>| {
                for entity in players {
                    commands.entity(entity).insert(player(&assets));
                }
            },
        );
}
