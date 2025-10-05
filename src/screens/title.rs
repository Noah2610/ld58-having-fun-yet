//! The title screen that appears after the splash screen.

use crate::{asset_tracking::LoadResource, menus::Menu, screens::Screen};
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), open_main_menu);
    app.add_systems(OnExit(Screen::Title), close_menu);
}

fn open_main_menu(
    mut next_menu: ResMut<NextState<Menu>>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    next_menu.set(Menu::Main);

    commands.spawn((
        Name::new("Title Level"),
        TiledMap(assets.load("maps/menu.tmx")),
        TiledPhysicsSettings::<TiledPhysicsAvianBackend> {
            objects_filter: TiledFilter::None,
            ..default()
        },
        TilemapAnchor::Center,
        DespawnOnExit(Screen::Title),
    ));
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
