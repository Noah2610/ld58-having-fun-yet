//! The main menu (seen on the title screen).

use crate::{asset_tracking::ResourceHandles, menus::Menu, screens::Screen, theme::widget};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Main), spawn_main_menu);
}

fn spawn_main_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Main Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Main),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent.spawn(widget::h1("Having Fun Yet?"));
            parent.spawn(widget::button("Play", enter_loading_or_gameplay_screen));
            parent.spawn(widget::button("Settings", open_settings_menu));
            #[cfg(not(target_family = "wasm"))]
            parent.spawn(widget::button("Exit", exit_app));
        })),
    ));
}

fn enter_loading_or_gameplay_screen(
    _: On<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(Screen::Gameplay);
    } else {
        next_screen.set(Screen::Loading);
    }
}

fn open_settings_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: On<Pointer<Click>>, mut app_exit: MessageWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
