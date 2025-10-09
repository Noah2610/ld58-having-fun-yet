use bevy::prelude::*;

mod audio_settings;
mod video_settings;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        settings::plugin,
        audio_settings::plugin,
        video_settings::plugin,
    ));
}

mod settings {
    use crate::{
        menus::{Menu, pop_menu_on_click, to_menu_on_click},
        theme::{widget, widget::settings_list},
    };
    use bevy::prelude::*;

    pub(super) fn plugin(app: &mut App) {
        app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);
    }

    fn spawn_settings_menu(mut commands: Commands) {
        commands.spawn((
            widget::ui_root("Settings Menu"),
            GlobalZIndex(3),
            DespawnOnExit(Menu::Settings),
            children![
                widget::header("Settings"),
                grid(),
                widget::button("Back", pop_menu_on_click),
            ],
        ));
    }

    fn grid() -> impl Bundle {
        (settings_list(), children![
            widget::button("Audio Settings", to_menu_on_click(Menu::AudioSettings)),
            widget::button("Video Settings", to_menu_on_click(Menu::VideoSettings)),
        ])
    }
}
