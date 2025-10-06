//! The screen state for the main gameplay.

use crate::{
    Paused,
    game::{level::spawn_level, survival_timer::TimeSurvivedValueUi},
    menus::Menu,
    screens::Screen,
    theme::widget,
};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), (spawn_level, spawn_ui));

    // Toggle pause on key press.
    app.add_systems(
        Update,
        (
            (pause, spawn_pause_overlay, open_pause_menu).run_if(
                in_state(Screen::Gameplay)
                    .and(in_state(Menu::None))
                    .and(input_just_pressed(KeyCode::KeyP).or(input_just_pressed(KeyCode::Escape))),
            ),
            close_menu.run_if(
                in_state(Screen::Gameplay)
                    .and(not(in_state(Menu::None)))
                    .and(input_just_pressed(KeyCode::KeyP)),
            ),
        ),
    );
    app.add_systems(OnExit(Screen::Gameplay), (close_menu, unpause));
    app.add_systems(
        OnEnter(Menu::None),
        unpause.run_if(in_state(Screen::Gameplay)),
    );
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn((
        Name::new("SurvivalTimer UI"),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            width: percent(100),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexStart,
            row_gap: Val::Px(4.0),
            ..default()
        },
        DespawnOnExit(Screen::Gameplay),
        Pickable::IGNORE,
        children![
            (
                Name::new("SurvivalTimer text"),
                Text::new("Time Survived:"),
                TextFont::from_font_size(16.0),
                TextColor(Color::WHITE)
            ),
            (
                Name::new("SurvivalTimer value"),
                Text::new("00:00"),
                TimeSurvivedValueUi,
                TextFont::from_font_size(24.0),
                TextColor(Color::WHITE)
            )
        ],
    ));
}

fn unpause(mut next_pause: ResMut<NextState<Paused>>) {
    next_pause.set(Paused(false));
}

fn pause(mut next_pause: ResMut<NextState<Paused>>) {
    next_pause.set(Paused(true));
}

fn spawn_pause_overlay(mut commands: Commands) {
    commands.spawn((
        Name::new("Pause Overlay"),
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        GlobalZIndex(1),
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)),
        DespawnOnExit(Paused(true)),
    ));
}

fn open_pause_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pause);
}

fn close_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
