use crate::{
    menus::{Menu, MenuAction, action_just_pressed, pop_menu_on_click},
    theme::widget::{self, FullscreenToggleCheckbox, ValueChange, self_start, settings_list},
};
use bevy::{
    prelude::*,
    ui::Checked,
    window::{PrimaryWindow, Window, WindowMode},
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<IsFullscreen>();
    app.add_systems(OnEnter(Menu::VideoSettings), spawn_video_settings_menu);
    app.add_systems(
        Update,
        (
            toggle_fullscreen.run_if(action_just_pressed(MenuAction::ToggleFullscreen)),
            apply_fullscreen.run_if(resource_changed::<IsFullscreen>),
        )
            .chain(),
    );
}

fn toggle_fullscreen(mut fullscreen: ResMut<IsFullscreen>) {
    fullscreen.0 ^= true;
}

fn apply_fullscreen(
    fullscreen: Res<IsFullscreen>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut commands: Commands,
    checkboxes: Query<(Entity, Has<Checked>), With<FullscreenToggleCheckbox>>,
) {
    for (checkbox, checked) in checkboxes {
        match (fullscreen.0, checked) {
            (true, true) | (false, false) => {},
            (true, false) => {
                commands.entity(checkbox).insert(Checked);
            },
            (false, true) => {
                commands.entity(checkbox).remove::<Checked>();
            },
        }
    }

    window.mode = if fullscreen.0 {
        WindowMode::Fullscreen(MonitorSelection::Current, VideoModeSelection::Current)
    } else {
        WindowMode::default()
    };
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct IsFullscreen(bool);

fn spawn_video_settings_menu(mut commands: Commands, fullscreen: Res<IsFullscreen>) {
    commands.spawn((
        widget::ui_root("Video Settings Menu"),
        GlobalZIndex(4),
        DespawnOnExit(Menu::VideoSettings),
        children![
            widget::header("Video Settings"),
            video_settings_list(fullscreen.0),
            widget::button("Back", pop_menu_on_click),
        ],
    ));
}

fn video_settings_list(is_fullscreen: bool) -> impl Bundle {
    (settings_list(), children![fullscreen_toggle_widget(
        is_fullscreen
    )])
}

fn fullscreen_toggle_widget(is_fullscreen: bool) -> impl Bundle {
    (Name::new("Fullscreen Toggle"), self_start(), children![
        (
            // widget::label("Fullscreen?"),
            widget::checkbox(
                "Fullscreen?",
                is_fullscreen,
                |trigger: On<ValueChange<bool>>, mut fullscreen: ResMut<IsFullscreen>| {
                    fullscreen.0 = trigger.value;
                }
            ),
            // observe(checkbox_self_update),
            // observe(
            //     |trigger: On<ValueChange<bool>>,
            //      mut window: Single<&mut Window, With<PrimaryWindow>>| {
            //         window.mode = if dbg!(trigger.value) {
            //             WindowMode::Fullscreen(
            //                 MonitorSelection::Current,
            //                 VideoModeSelection::Current,
            //             )
            //         } else {
            //             WindowMode::default()
            //         };
            //     }
            // ),
            // Observer::new(checkbox_self_update),
            // widget::button_small(
            //     "Toggle Fullscreen",
            //     |_: On<Pointer<Click>>, mut window: Single<&mut Window, With<PrimaryWindow>>| {
            //         window.mode = if matches!(window.mode, WindowMode::Fullscreen(..)) {
            //             WindowMode::default()
            //         } else {
            //             WindowMode::Fullscreen(MonitorSelection::Current,
            // VideoModeSelection::Current)         };
            //     }
            // ),
            // Observer::new(
            //     |trigger: On<ValueChange<bool>>,
            //      mut window: Single<&mut Window, With<PrimaryWindow>>| {
            //         window.mode = if dbg!(trigger.value) {
            //             WindowMode::Fullscreen(MonitorSelection::Current,
            // VideoModeSelection::Current)         } else {
            //             WindowMode::default()
            //         };
            //     }
            // ),
        ),
        // .observe(
        //     |trigger: On<ValueChange<bool>>,
        //      mut window: Single<&mut Window, With<PrimaryWindow>>| {
        //         window.mode = if dbg!(trigger.value) {
        //             WindowMode::Fullscreen(
        //                 MonitorSelection::Current,
        //                 VideoModeSelection::Current,
        //             )
        //         } else {
        //             WindowMode::default()
        //         };
        //     }
        // )
    ])
}
