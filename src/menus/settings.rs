use crate::{
    audio::{MusicVolume, SoundsVolume},
    input::{MenuAction, action_just_pressed},
    menus::Menu,
    theme::prelude::*,
};
use bevy::{audio::Volume, ecs::system::IntoObserverSystem, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Settings), spawn_settings_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Settings).and(action_just_pressed(MenuAction::Pause))),
    );

    app.add_systems(
        Update,
        (
            update_global_volume_label,
            update_music_volume_label,
            update_sounds_volume_label,
        )
            .run_if(in_state(Menu::Settings)),
    );
}

fn spawn_settings_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Settings Menu"),
        GlobalZIndex(2),
        DespawnOnExit(Menu::Settings),
        children![
            widget::header("Settings"),
            settings_grid(),
            widget::button("Back", go_back_on_click),
        ],
    ));
}

fn settings_grid() -> impl Bundle {
    (
        Name::new("Settings Grid"),
        Node {
            display: Display::Grid,
            row_gap: px(10),
            column_gap: px(30),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        children![
            (widget::label("Master Volume"), Node {
                justify_self: JustifySelf::End,
                ..default()
            }),
            global_volume_widget(),
            (widget::label("Music Volume"), Node {
                justify_self: JustifySelf::End,
                ..default()
            }),
            music_volume_widget(),
            (widget::label("Sound Effects Volume"), Node {
                justify_self: JustifySelf::End,
                ..default()
            }),
            sounds_volume_widget(),
        ],
    )
}

fn volume_widget<
    C: Component,
    LS: IntoObserverSystem<E, B, M>,
    RS: IntoObserverSystem<E, B, M>,
    E: EntityEvent,
    B: Bundle,
    M,
>(
    volume_label_marker: C,
    lower_volume: LS,
    raise_volume: RS,
) -> impl Bundle {
    (
        Node {
            justify_self: JustifySelf::Start,
            ..default()
        },
        children![
            widget::button_small("-", lower_volume),
            (
                Node {
                    padding: UiRect::horizontal(px(10)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(widget::label(""), volume_label_marker)],
            ),
            widget::button_small("+", raise_volume),
        ],
    )
}

fn global_volume_widget() -> impl Bundle {
    (
        Name::new("Global Volume Widget"),
        volume_widget(GlobalVolumeLabel, lower_global_volume, raise_global_volume),
    )
}

fn music_volume_widget() -> impl Bundle {
    (
        Name::new("Music Volume Widget"),
        volume_widget(MusicVolumeLabel, lower_music_volume, raise_music_volume),
    )
}

fn sounds_volume_widget() -> impl Bundle {
    (
        Name::new("Sounds Volume Widget"),
        volume_widget(SoundsVolumeLabel, lower_sounds_volume, raise_sounds_volume),
    )
}

const MIN_VOLUME: f32 = 0.0;
const MAX_VOLUME: f32 = 3.0;
const VOLUME_STEP: f32 = 0.1;

fn lower_global_volume(_: On<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let linear = (global_volume.volume.to_linear() - VOLUME_STEP).max(MIN_VOLUME);
    global_volume.volume = Volume::Linear(linear);
}

fn raise_global_volume(_: On<Pointer<Click>>, mut global_volume: ResMut<GlobalVolume>) {
    let linear = (global_volume.volume.to_linear() + VOLUME_STEP).min(MAX_VOLUME);
    global_volume.volume = Volume::Linear(linear);
}

fn lower_music_volume(_: On<Pointer<Click>>, mut volume: ResMut<MusicVolume>) {
    let linear = (volume.0.to_linear() - VOLUME_STEP).max(MIN_VOLUME);
    volume.0 = Volume::Linear(linear);
}

fn raise_music_volume(_: On<Pointer<Click>>, mut volume: ResMut<MusicVolume>) {
    let linear = (volume.0.to_linear() + VOLUME_STEP).min(MAX_VOLUME);
    volume.0 = Volume::Linear(linear);
}

fn lower_sounds_volume(_: On<Pointer<Click>>, mut volume: ResMut<SoundsVolume>) {
    let linear = (volume.0.to_linear() - VOLUME_STEP).max(MIN_VOLUME);
    volume.0 = Volume::Linear(linear);
}

fn raise_sounds_volume(_: On<Pointer<Click>>, mut volume: ResMut<SoundsVolume>) {
    let linear = (volume.0.to_linear() + VOLUME_STEP).min(MAX_VOLUME);
    volume.0 = Volume::Linear(linear);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct GlobalVolumeLabel;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct MusicVolumeLabel;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct SoundsVolumeLabel;

fn update_global_volume_label(
    global_volume: Res<GlobalVolume>,
    mut label: Single<&mut Text, With<GlobalVolumeLabel>>,
) {
    let percent = 100.0 * global_volume.volume.to_linear();
    label.0 = format!("{percent:3.0}%");
}

fn update_music_volume_label(
    volume: Res<MusicVolume>,
    mut label: Single<&mut Text, With<MusicVolumeLabel>>,
) {
    let percent = 100.0 * volume.0.to_linear();
    label.0 = format!("{percent:3.0}%");
}

fn update_sounds_volume_label(
    global_volume: Res<SoundsVolume>,
    mut label: Single<&mut Text, With<SoundsVolumeLabel>>,
) {
    let percent = 100.0 * global_volume.0.to_linear();
    label.0 = format!("{percent:3.0}%");
}

fn go_back_on_click(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pop);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pop);
}
