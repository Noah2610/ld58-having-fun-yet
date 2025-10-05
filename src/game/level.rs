//! Spawn the main level.

use crate::{
    asset_tracking::LoadResource,
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
            // objects_filter: TiledFilter::None,
            objects_filter: TiledFilter::Names(vec!["solid".into()]),
            objects_layer_filter: TiledFilter::Names(vec!["solid".into()]),
            // backend: TiledPhysicsAvianBackend::Polyline,
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
//     trigger: On<TiledEvent<ObjectCreated>>,
//     mut commands: Commands,
//     objects: Query<&TiledObject, With<Solid>>,
// ) {
//     if let Ok(object) = objects.get(trigger.origin) {
//         match object {
//             TiledObject::Rectangle { width, height } => {
//                 commands
//                     .entity(trigger.entity)
//                     .insert(Collider::rectangle(*width, *height));
//             },
//             _ => (),
//         }
//     }
// }
