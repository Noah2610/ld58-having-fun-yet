use crate::{
    game::visuals::VisualIntensity,
    menus::{Menu, MenuAction, action_just_pressed, pop_menu_on_click},
    theme::widget::{
        self, FullscreenToggleCheckbox, ValueChange, self_end, self_start, settings_grid_2x,
        settings_list,
    },
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
    app.add_systems(
        Update,
        update_intensity_ui_value.run_if(in_state(Menu::VideoSettings)),
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

fn spawn_video_settings_menu(
    mut commands: Commands,
    fullscreen: Res<IsFullscreen>,
    intensity: Res<VisualIntensity>,
) {
    commands.spawn((
        widget::ui_root("Video Settings Menu"),
        GlobalZIndex(4),
        DespawnOnExit(Menu::VideoSettings),
        children![
            widget::h2("Video Settings"),
            video_settings_grid(fullscreen.0, intensity.0),
            widget::button("Back", pop_menu_on_click),
        ],
    ));
}

fn video_settings_grid(is_fullscreen: bool, intensity: f32) -> impl Bundle {
    (settings_list(), children![
        fullscreen_toggle_widget(is_fullscreen),
        visual_intensity_widget(intensity),
    ])
}

fn visual_intensity_widget(intensity: f32) -> impl Bundle {
    (
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            column_gap: px(24),
            ..default()
        },
        children![
            (widget::label("Animations Intensity"), self_end(),),
            widget::analog_slider(
                VisualIntensityUiValue,
                decrease_intensity,
                increase_intensity
            ),
        ],
    )
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

#[derive(Component, Clone, Copy, Debug, Default)]
struct VisualIntensityUiValue;

const MIN_INTENSITY: f32 = 0.0;
const MAX_INTENSITY: f32 = 8.0;
const INTENSITY_STEP: f32 = 0.1;

fn increase_intensity(_: On<Pointer<Click>>, mut intensity: ResMut<VisualIntensity>) {
    intensity.0 = (intensity.0 + INTENSITY_STEP)
        .min(MAX_INTENSITY)
        .max(MIN_INTENSITY);
}

fn decrease_intensity(_: On<Pointer<Click>>, mut intensity: ResMut<VisualIntensity>) {
    intensity.0 = (intensity.0 - INTENSITY_STEP)
        .min(MAX_INTENSITY)
        .max(MIN_INTENSITY);
}

fn update_intensity_ui_value(
    intensity: Res<VisualIntensity>,
    mut ui_value: Single<&mut Text, With<VisualIntensityUiValue>>,
) {
    let percent = 100.0 * intensity.0;
    ui_value.0 = format!("{percent:3.0}%");
}
