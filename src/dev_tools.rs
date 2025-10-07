//! Development tools for the game. This plugin is only enabled in dev builds.

use crate::{Paused, screens::Screen};
use avian2d::prelude::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::{
    dev_tools::{
        fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
        states::log_transitions,
    },
    input::common_conditions::input_just_pressed,
    prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::{StateInspectorPlugin, WorldInspectorPlugin};

const TOGGLE_FPS_KEY: KeyCode = KeyCode::F1;
const TOGGLE_INSPECTOR_KEY: KeyCode = KeyCode::F2;
const TOGGLE_GIZMOS_KEY: KeyCode = KeyCode::F3;
const TOGGLE_UI_KEY: KeyCode = KeyCode::F4;

#[derive(States, Reflect, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[reflect(State)]
struct InspectorEnabled(bool);
impl Default for InspectorEnabled {
    fn default() -> Self {
        Self(true)
    }
}

pub(super) fn plugin(app: &mut App) {
    if !app.is_plugin_added::<EguiPlugin>() {
        app.add_plugins(EguiPlugin::default());
    }

    app.init_state::<InspectorEnabled>();

    app.add_plugins((
        FpsOverlayPlugin {
            config: FpsOverlayConfig {
                enabled: true,
                frame_time_graph_config: FrameTimeGraphConfig {
                    enabled: false,
                    ..default()
                },
                ..default()
            },
        },
        WorldInspectorPlugin::default().run_if(in_state(InspectorEnabled(true))),
        StateInspectorPlugin::<Screen>::default().run_if(in_state(InspectorEnabled(true))),
        StateInspectorPlugin::<Paused>::default().run_if(in_state(InspectorEnabled(true))),
        PhysicsDebugPlugin,
    ))
    .insert_gizmo_config(PhysicsGizmos::default(), GizmoConfig {
        enabled: false,
        ..default()
    });

    app.add_systems(Update, log_transitions::<Screen>)
        .add_systems(
            Update,
            (toggle_fps_overlay.run_if(input_just_pressed(TOGGLE_FPS_KEY)),),
        )
        .add_systems(
            Update,
            toggle_inspector.run_if(input_just_pressed(TOGGLE_INSPECTOR_KEY)),
        )
        .add_systems(
            Update,
            toggle_debug_ui.run_if(input_just_pressed(TOGGLE_UI_KEY)),
        )
        .add_systems(
            Update,
            toggle_gizmos.run_if(input_just_pressed(TOGGLE_GIZMOS_KEY)),
        );
}

fn toggle_fps_overlay(mut config: ResMut<FpsOverlayConfig>) {
    config.enabled ^= true;
}

fn toggle_inspector(
    mut next_state: ResMut<NextState<InspectorEnabled>>,
    state: Res<State<InspectorEnabled>>,
) {
    next_state.set(InspectorEnabled(!state.0));
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

fn toggle_gizmos(mut gizmo_configs: ResMut<GizmoConfigStore>) {
    gizmo_configs.config_mut::<PhysicsGizmos>().0.enabled ^= true;
}
