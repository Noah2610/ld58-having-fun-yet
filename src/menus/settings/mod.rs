use bevy::prelude::*;

mod audio_settings;
mod video_settings;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        settings_menu::plugin,
        audio_settings::plugin,
        video_settings::plugin,
    ));
}

mod settings_menu {
    use crate::{
        camera::MainCamera,
        menus::{Menu, pop_menu_on_click, to_menu_on_click},
        screens::Screen,
        theme::{widget, widget::settings_list},
    };
    use bevy::{post_process::bloom::Bloom, prelude::*};

    pub(super) fn plugin(app: &mut App) {
        app.init_resource::<PrevBloom>()
            .init_resource::<IsBlurred>();
        app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);

        app.add_systems(
            OnTransition {
                exited:  Menu::None,
                entered: Menu::Pause,
            },
            add_blur,
        );
        app.add_systems(
            OnTransition {
                exited:  Menu::VideoSettings,
                entered: Menu::Settings,
            },
            add_blur,
        );
        app.add_systems(
            OnTransition {
                exited:  Menu::VideoSettings,
                entered: Menu::Pop,
            },
            add_blur,
        );

        app.add_systems(
            OnTransition {
                exited:  Menu::Pause,
                entered: Menu::None,
            },
            remove_blur,
        );
        app.add_systems(
            OnTransition {
                exited:  Menu::Pause,
                entered: Menu::Pop,
            },
            remove_blur,
        );
        app.add_systems(
            OnTransition {
                exited:  Menu::Settings,
                entered: Menu::VideoSettings,
            },
            remove_blur,
        );

        app.add_systems(OnEnter(Screen::Gameplay), remove_blur);
    }

    #[derive(Resource, Default)]
    struct IsBlurred(bool);
    #[derive(Resource, Default)]
    struct PrevBloom(Option<Bloom>);

    fn add_blur(
        mut commands: Commands,
        mut is_blurred: ResMut<IsBlurred>,
        mut prev_bloom: ResMut<PrevBloom>,
        camera: Single<(Entity, Option<&Bloom>), With<MainCamera>>,
    ) {
        if is_blurred.0 {
            return;
        }

        is_blurred.0 = true;
        prev_bloom.0 = camera.1.cloned();
        commands.entity(camera.0).insert(Bloom::SCREEN_BLUR);
    }

    fn remove_blur(
        mut commands: Commands,
        mut is_blurred: ResMut<IsBlurred>,
        prev_bloom: Res<PrevBloom>,
        camera: Single<(Entity, Has<Bloom>), With<MainCamera>>,
    ) {
        if !is_blurred.0 {
            return;
        }

        is_blurred.0 = false;
        if let Some(bloom) = &prev_bloom.0 {
            commands.entity(camera.0).insert(bloom.clone());
        } else if camera.1 {
            commands.entity(camera.0).remove::<Bloom>();
        }
    }

    fn spawn_settings_menu(mut commands: Commands) {
        commands.spawn((
            widget::ui_root("Settings Menu"),
            GlobalZIndex(3),
            DespawnOnExit(Menu::Settings),
            children![
                widget::h2("Settings"),
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
