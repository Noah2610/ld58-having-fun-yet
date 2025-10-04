//! Development tools for the game. This plugin is only enabled in dev builds.

use crate::screens::Screen;
use avian2d::prelude::{PhysicsDebugPlugin, PhysicsGizmos};
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

const TOGGLE_UI_KEY: KeyCode = KeyCode::F4;
const TOGGLE_GIZMOS_KEY: KeyCode = KeyCode::F3;

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
        PhysicsDebugPlugin,
    ))
    .insert_gizmo_config(PhysicsGizmos::default(), GizmoConfig {
        enabled: false,
        ..default()
    });

    app.add_systems(Update, log_transitions::<Screen>)
        .add_systems(
            Update,
            toggle_debug_ui.run_if(input_just_pressed(TOGGLE_UI_KEY)),
        )
        .add_systems(
            Update,
            toggle_gizmos.run_if(input_just_pressed(TOGGLE_GIZMOS_KEY)),
        );
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

fn toggle_gizmos(mut gizmo_configs: ResMut<GizmoConfigStore>) {
    gizmo_configs.config_mut::<PhysicsGizmos>().0.enabled ^= true;
}
