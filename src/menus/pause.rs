use crate::{
    input::{MenuAction, action_just_pressed},
    menus::Menu,
    screens::Screen,
    theme::widget,
};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    #[cfg(not(feature = "no_pause_ui"))]
    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);
    app.add_systems(
        Update,
        close_menu.run_if(in_state(Menu::Pause).and(
            action_just_pressed(MenuAction::Cancel).or(action_just_pressed(MenuAction::Pause)),
        )),
    );
}

#[cfg(not(feature = "no_pause_ui"))]
fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Pause Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Pause),
        children![
            widget::h2("Game paused"),
            widget::button("Continue", on_continue),
            widget::button("Settings", open_settings_menu),
            widget::button("Quit to title", quit_to_title),
        ],
    ));
}

#[cfg(not(feature = "no_pause_ui"))]
fn open_settings_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Settings);
}

#[cfg(not(feature = "no_pause_ui"))]
fn on_continue(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pop);
}

#[cfg(not(feature = "no_pause_ui"))]
fn quit_to_title(_: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pop);
}
