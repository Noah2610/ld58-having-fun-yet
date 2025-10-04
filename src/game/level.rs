//! Spawn the main level.

use crate::{
    asset_tracking::LoadResource,
    game::{
        player::{Player, PlayerAssets},
        solid::Solid,
    },
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
pub fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands.spawn((
        Name::new("Level"),
        TiledMap(level_assets.map.clone()),
        TiledPhysicsSettings::<TiledPhysicsAvianBackend> {
            // objects_filter: TiledFilter::All,
            // objects_layer_filter: TiledFilter::Names(vec!["solid".into()]),
            ..default()
        },
        TilemapAnchor::Center,
        DespawnOnExit(Screen::Gameplay),
        // children![
        //     player(&player_assets),
        //     // (
        //     //     Name::new("Gameplay Music"),
        //     //     music(level_assets.music.clone())
        //     // )
        // ],
    ));
    // .observe(populate_objects);
}

// fn populate_objects(
//     _ev: On<TiledEvent<MapCreated>>,
//     mut commands: Commands,
//     assets: Res<PlayerAssets>,
//     players: Query<Entity, With<Player>>,
//     solids: Query<Entity, (With<Solid>, Without<Player>)>,
// ) {
//     for entity in players {
//         commands.entity(entity).insert(player(&assets));
//     }

//     for entity in solids {
//         commands.entity(entity).insert(solid());
//     }
// }
