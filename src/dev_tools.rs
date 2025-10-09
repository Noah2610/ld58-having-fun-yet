use crate::{
    camera::MainCamera,
    game::visuals::{
        BackgroundHueAnimation, GlobalAnimationsEnabled, GlobalCameraAnimationsEnabled,
        GlobalColorAnimationsEnabled, GlobalTransformAnimationsEnabled,
    },
    input::*,
    quality::Quality,
    screens::Screen,
};
use avian2d::prelude::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::{
    dev_tools::{
        fps_overlay::{FpsOverlayConfig, FpsOverlayPlugin, FrameTimeGraphConfig},
        states::log_transitions,
    },
    prelude::*,
};
use bevy_egui::EguiPlugin;
use bevy_inspector_egui::quick::{
    FilterQueryInspectorPlugin, ResourceInspectorPlugin, StateInspectorPlugin, WorldInspectorPlugin,
};

#[derive(States, Reflect, Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct InspectorEnabled(bool);
impl Default for InspectorEnabled {
    fn default() -> Self {
        Self(true)
    }
}
#[derive(States, Reflect, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
struct VisualInspectorsEnabled(bool);
#[derive(States, Reflect, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
struct WorldInspectorEnabled(bool);

pub(super) fn plugin(app: &mut App) {
    if !app.is_plugin_added::<EguiPlugin>() {
        app.add_plugins(EguiPlugin::default());
    }

    app.init_state::<InspectorEnabled>()
        .init_state::<WorldInspectorEnabled>()
        .init_state::<VisualInspectorsEnabled>();

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
        WorldInspectorPlugin::default()
            .run_if(in_state(InspectorEnabled(true)).and(in_state(WorldInspectorEnabled(true)))),
        (
            ResourceInspectorPlugin::<BackgroundHueAnimation>::default().run_if(
                in_state(InspectorEnabled(true)).and(in_state(VisualInspectorsEnabled(true))),
            ),
            ResourceInspectorPlugin::<GlobalAnimationsEnabled>::default().run_if(
                in_state(InspectorEnabled(true)).and(in_state(VisualInspectorsEnabled(true))),
            ),
            ResourceInspectorPlugin::<GlobalCameraAnimationsEnabled>::default().run_if(
                in_state(InspectorEnabled(true)).and(in_state(VisualInspectorsEnabled(true))),
            ),
            ResourceInspectorPlugin::<GlobalColorAnimationsEnabled>::default().run_if(
                in_state(InspectorEnabled(true)).and(in_state(VisualInspectorsEnabled(true))),
            ),
            ResourceInspectorPlugin::<GlobalTransformAnimationsEnabled>::default().run_if(
                in_state(InspectorEnabled(true)).and(in_state(VisualInspectorsEnabled(true))),
            ),
        ),
        StateInspectorPlugin::<Quality>::default().run_if(in_state(InspectorEnabled(true))),
        FilterQueryInspectorPlugin::<With<MainCamera>>::default()
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
            (toggle_fps_overlay.run_if(action_just_pressed(DebugAction::ToggleFps)),),
        )
        .add_systems(
            Update,
            toggle_inspector.run_if(action_just_pressed(DebugAction::ToggleInspector)),
        )
        .add_systems(
            Update,
            (toggle_visual_inspectors, toggle_world_inspector)
                .run_if(action_just_pressed(DebugAction::ToggleVisualInspectors)),
        )
        .add_systems(
            Update,
            toggle_debug_ui.run_if(action_just_pressed(DebugAction::ToggleUiDebug)),
        )
        .add_systems(
            Update,
            toggle_gizmos.run_if(action_just_pressed(DebugAction::ToggleGizmos)),
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

fn toggle_world_inspector(
    mut next_state: ResMut<NextState<WorldInspectorEnabled>>,
    state: Res<State<WorldInspectorEnabled>>,
) {
    next_state.set(WorldInspectorEnabled(!state.0));
}

fn toggle_visual_inspectors(
    mut next_state: ResMut<NextState<VisualInspectorsEnabled>>,
    state: Res<State<VisualInspectorsEnabled>>,
) {
    next_state.set(VisualInspectorsEnabled(!state.0));
}

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

fn toggle_gizmos(mut gizmo_configs: ResMut<GizmoConfigStore>) {
    gizmo_configs.config_mut::<PhysicsGizmos>().0.enabled ^= true;
}
