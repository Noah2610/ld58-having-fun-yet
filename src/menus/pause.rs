//! The pause menu.

use crate::menus::Menu;
#[cfg(not(feature = "no_pause_ui"))]
use crate::{screens::Screen, theme::widget};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub(super) fn plugin(app: &mut App) {
    #[cfg(not(feature = "no_pause_ui"))]
    app.add_systems(OnEnter(Menu::Pause), spawn_pause_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Pause).and(input_just_pressed(KeyCode::Escape))),
    );
}

#[cfg(not(feature = "no_pause_ui"))]
fn spawn_pause_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Pause Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Pause),
        children![
            widget::header("Game paused"),
            widget::button("Continue", close_menu),
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
fn close_menu(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

#[cfg(not(feature = "no_pause_ui"))]
fn quit_to_title(_: On<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
