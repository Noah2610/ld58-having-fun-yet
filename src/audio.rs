use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<MusicVolume>();
    app.init_resource::<SoundsVolume>();
    app.add_systems(
        PreUpdate,
        (
            // apply_global_volume.run_if(resource_changed::<GlobalVolume>),
            // apply_music_volume.run_if(resource_changed::<MusicVolume>),
            // apply_sounds_volume.run_if(resource_changed::<SoundsVolume>),
            apply_music_volume,
            apply_sounds_volume,
        ),
    );
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct MusicVolume(pub Volume);

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct SoundsVolume(pub Volume);

/// An organizational marker component that should be added to a spawned
/// [`AudioPlayer`] if it's in the general "music" category (e.g. global
/// background music, soundtrack).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Music;

/// A music audio instance.
pub fn music(handle: Handle<AudioSource>) -> impl Bundle {
    (
        AudioPlayer(handle),
        PlaybackSettings {
            mode: PlaybackMode::Loop,
            paused: true,
            ..default()
        },
        Music,
        StartedPaused,
    )
}

/// An organizational marker component that should be added to a spawned
/// [`AudioPlayer`] if it's in the general "sound effect" category (e.g.
/// footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Sound;

/// A sound effect audio instance.
pub fn sound_effect(handle: Handle<AudioSource>) -> impl Bundle {
    (
        AudioPlayer(handle),
        PlaybackSettings {
            mode: PlaybackMode::Despawn,
            paused: true,
            ..default()
        },
        Sound,
        StartedPaused,
    )
}

#[derive(Component)]
struct StartedPaused;

fn apply_music_volume(
    mut commands: Commands,
    global_volume: Res<GlobalVolume>,
    music_volume: Res<MusicVolume>,
    query: Query<
        (
            Entity,
            &PlaybackSettings,
            &mut AudioSink,
            Ref<Music>,
            Has<StartedPaused>,
        ),
        With<Music>,
    >,
) {
    let settings_changed = global_volume.is_changed() || music_volume.is_changed();
    for (entity, playback, mut sink, comp, unmute) in query {
        if unmute {
            sink.play();
            commands.entity(entity).remove::<StartedPaused>();
        }
        if comp.is_added() || settings_changed {
            let expected_volume = global_volume.volume * music_volume.0 * playback.volume;
            if (expected_volume.to_linear() - sink.volume().to_linear()).abs() > f32::EPSILON {
                sink.set_volume(expected_volume);
            }
        }
    }
}

fn apply_sounds_volume(
    mut commands: Commands,
    global_volume: Res<GlobalVolume>,
    sounds_volume: Res<SoundsVolume>,
    query: Query<
        (
            Entity,
            &PlaybackSettings,
            &mut AudioSink,
            Ref<Sound>,
            Has<StartedPaused>,
        ),
        With<Sound>,
    >,
) {
    let settings_changed = global_volume.is_changed() || sounds_volume.is_changed();
    for (entity, playback, mut sink, comp, unmute) in query {
        if unmute {
            sink.play();
            commands.entity(entity).remove::<StartedPaused>();
        }
        if comp.is_added() || settings_changed {
            let expected_volume = global_volume.volume * sounds_volume.0 * playback.volume;
            if (expected_volume.to_linear() - sink.volume().to_linear()).abs() > f32::EPSILON {
                sink.set_volume(expected_volume);
            }
        }
    }
}
