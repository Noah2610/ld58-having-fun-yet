use crate::{
    camera::MainCamera,
    game::visuals::VisualIntensity,
    menus::{Menu, MenuAction, action_just_pressed, pop_menu_on_click},
    theme::widget::{self, ValueChange, self_end, self_start, settings_list},
};
use bevy::{
    image::{ImageSampler, ImageSamplerDescriptor},
    post_process::bloom::Bloom,
    prelude::*,
    ui::Checked,
    window::{PrimaryWindow, Window, WindowMode},
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<IsFullscreen>()
        .init_resource::<BloomEnabled>()
        .init_resource::<PrevBloom>()
        .init_resource::<PixelPerfectEnabled>();
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
        (
            update_intensity_ui_value,
            apply_bloom_enabled.run_if(resource_changed::<BloomEnabled>),
            apply_pixel_perfect_enabled.run_if(resource_changed::<PixelPerfectEnabled>),
        )
            .run_if(in_state(Menu::VideoSettings)),
    );
}

fn toggle_fullscreen(mut fullscreen: ResMut<IsFullscreen>) {
    fullscreen.0 ^= true;
}

#[derive(Component, Default)]
struct FullscreenToggleCheckbox;

#[derive(Component, Default)]
struct BloomToggleCheckbox;

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

fn apply_bloom_enabled(
    mut commands: Commands,
    bloom_enabled: Res<BloomEnabled>,
    mut prev_bloom: ResMut<PrevBloom>,
    camera_bloom: Single<(Entity, Option<&Bloom>), With<MainCamera>>,
) {
    if bloom_enabled.0 {
        if camera_bloom.1.is_none() {
            commands
                .entity(camera_bloom.0)
                .insert(prev_bloom.0.as_ref().map(|b| b.clone()).unwrap_or_default());
        }
    } else {
        if let Some(bloom) = camera_bloom.1 {
            prev_bloom.0 = Some(bloom.clone());
            commands.entity(camera_bloom.0).remove::<Bloom>();
        }
    }
}

fn apply_pixel_perfect_enabled(
    pixel_perfect_enabled: Res<PixelPerfectEnabled>,
    mut images: ResMut<Assets<Image>>,
) {
    let descriptor = if pixel_perfect_enabled.0 {
        ImageSamplerDescriptor::nearest()
    } else {
        ImageSamplerDescriptor::linear()
    };
    for (_, image) in images.iter_mut() {
        image.sampler = ImageSampler::Descriptor(descriptor.clone());
    }
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct IsFullscreen(bool);

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct BloomEnabled(bool);
impl Default for BloomEnabled {
    fn default() -> Self {
        Self(true)
    }
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct PrevBloom(Option<Bloom>);

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct PixelPerfectEnabled(bool);
impl Default for PixelPerfectEnabled {
    fn default() -> Self {
        Self(true)
    }
}

fn spawn_video_settings_menu(
    mut commands: Commands,
    fullscreen: Res<IsFullscreen>,
    bloom: Res<BloomEnabled>,
    pixel_perfect: Res<PixelPerfectEnabled>,
) {
    commands.spawn((
        widget::ui_root("Video Settings Menu"),
        GlobalZIndex(4),
        DespawnOnExit(Menu::VideoSettings),
        children![
            widget::h2("Video Settings"),
            video_settings_grid(fullscreen.0, bloom.0, pixel_perfect.0),
            widget::button("Back", pop_menu_on_click),
        ],
    ));
}

fn video_settings_grid(
    is_fullscreen: bool,
    has_bloom: bool,
    is_pixel_perfect: bool,
) -> impl Bundle {
    (settings_list(), children![
        fullscreen_toggle_widget(is_fullscreen),
        image_sampler_widget(is_pixel_perfect),
        bloom_toggle_widget(has_bloom),
        visual_intensity_widget(),
    ])
}

fn visual_intensity_widget() -> impl Bundle {
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

fn image_sampler_widget(is_pixel_perfect: bool) -> impl Bundle {
    (Name::new("Image Sampler Toggle"), self_start(), children![
        (widget::checkbox(
            BloomToggleCheckbox,
            "Pixel-perfect? ",
            is_pixel_perfect,
            |trigger: On<ValueChange<bool>>, mut pixel_perfect: ResMut<PixelPerfectEnabled>| {
                pixel_perfect.0 = trigger.value;
            }
        ),),
    ])
}

fn bloom_toggle_widget(has_bloom: bool) -> impl Bundle {
    (Name::new("Bloom Toggle"), self_start(), children![(
        widget::checkbox(
            BloomToggleCheckbox,
            "Bloom? ",
            has_bloom,
            |trigger: On<ValueChange<bool>>, mut bloom: ResMut<BloomEnabled>| {
                bloom.0 = trigger.value;
            }
        ),
    ),])
}

fn fullscreen_toggle_widget(is_fullscreen: bool) -> impl Bundle {
    (Name::new("Fullscreen Toggle"), self_start(), children![
        (
            // widget::label("Fullscreen?"),
            widget::checkbox(
                FullscreenToggleCheckbox,
                "Fullscreen? ",
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
