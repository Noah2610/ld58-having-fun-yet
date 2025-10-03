//! Development tools for the game. This plugin is only enabled in dev builds.

use crate::screens::Screen;
use bevy::{
    dev_tools::{
        fps_overlay::{
            FpsOverlayConfig,
            FpsOverlayPlugin,
            FrameTimeGraphConfig,
        },
        states::log_transitions,
    },
    input::common_conditions::input_just_pressed,
    prelude::*,
};
use bevy_inspector_egui::{
    bevy_egui::EguiPlugin,
    quick::{StateInspectorPlugin, WorldInspectorPlugin},
};

#[derive(States, Reflect, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[reflect(State)]
struct InspectorEnabled(bool);

impl Default for InspectorEnabled {
    fn default() -> Self {
        InspectorEnabled(true)
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_state::<InspectorEnabled>();

    app.add_plugins((
        FpsOverlayPlugin {
            config: FpsOverlayConfig {
                frame_time_graph_config: FrameTimeGraphConfig {
                    enabled: false,
                    ..default()
                },
                ..default()
            },
            ..default()
        },
        EguiPlugin::default(),
        WorldInspectorPlugin::default()
            .run_if(in_state(InspectorEnabled(true))),
        StateInspectorPlugin::<Screen>::default()
            .run_if(in_state(InspectorEnabled(true))),
    ));

    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
