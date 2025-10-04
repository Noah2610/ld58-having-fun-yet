// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

mod asset_tracking;
mod audio;
mod character_controller;
#[cfg(feature = "dev")]
mod dev_tools;
mod editor;
mod game;
mod game_state;
mod input;
mod menus;
mod screens;
mod theme;

use avian2d::prelude::PhysicsPlugins;
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_ecs_tiled::prelude::*;
use bevy_yoleck::{vpeol::VpeolCameraState, vpeol_2d::Vpeol2dCameraControl};
use game_state::{GameplaySet, Paused};
use std::path::PathBuf;

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        // Add Bevy plugins.
        app.add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    // Wasm builds will check for meta files (that don't exist) if this isn't set.
                    // This causes errors and even panics on web build on itch.
                    // See https://github.com/bevyengine/bevy_github_ci_template/issues/48.
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Window {
                        title: "LD58".to_string(),
                        resizable: false,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            PhysicsPlugins::default().with_length_unit(16.0),
            TiledPlugin(TiledPluginConfig {
                tiled_types_export_file: Some(PathBuf::from(
                    "./tiled/tiled_types_export.json",
                )),
                tiled_types_filter:      TiledFilter::from(
                    RegexSet::new([r"^ld58::.*$", r"^bevy_ecs::name::Name$"])
                        .expect(
                            "[TiledPluginConfig.tiled_types_filter] Expected \
                             regexes to be valid",
                        ),
                ),
            }),
            TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default(),
        ));

        #[cfg(feature = "dev")]
        app.add_plugins(dev_tools::plugin);

        // Add other plugins.
        app.add_plugins((
            game_state::plugin,
            editor::plugin,
            input::plugin,
            asset_tracking::plugin,
            character_controller::plugin,
            audio::plugin,
            game::plugin,
            menus::plugin,
            theme::plugin,
            screens::plugin,
        ));

        // Order new `AppSystems` variants by adding them here:
        app.configure_sets(
            Update,
            (
                AppSystems::TickTimers,
                AppSystems::RecordInput,
                AppSystems::Update,
            )
                .chain(),
        );

        // Spawn the main camera.
        app.add_systems(Startup, spawn_camera);
    }
}

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(
    SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord,
)]
enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        VpeolCameraState::default(),
        Vpeol2dCameraControl::default(),
    ));
}
