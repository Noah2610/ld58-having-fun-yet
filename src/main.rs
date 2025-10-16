// Support configuring Bevy lints within code.
#![cfg_attr(bevy_lint, feature(register_tool), register_tool(bevy))]
// Disable console on Windows for non-dev builds.
#![cfg_attr(not(feature = "dev_tools"), windows_subsystem = "windows")]

mod asset_tracking;
mod audio;
mod camera;
#[cfg(feature = "dev_tools")]
mod dev_tools;
mod direction;
mod game;
mod game_state;
mod input;
mod menus;
mod quality;
mod screens;
mod state_history;
mod theme;

use crate::theme::widget::UiWidgetsPlugins;
use avian2d::prelude::{Gravity, PhysicsPlugins};
use bevy::{asset::AssetMetaCheck, prelude::*};
use bevy_aseprite_ultra::AsepriteUltraPlugin;
use bevy_ecs_tiled::prelude::*;
use game_state::{AppSystems, GameplaySet, Paused};

fn main() -> AppExit {
    App::new().add_plugins(AppPlugin).run()
}

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
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
                        title: "Having Fun Yet?".to_string(),
                        resizable: false,
                        #[cfg(feature = "no_window_decorations")]
                        decorations: false,
                        ..default()
                    }
                    .into(),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
            UiWidgetsPlugins,
            PhysicsPlugins::default().with_length_unit(16.0),
            TiledPlugin(TiledPluginConfig {
                tiled_types_export_file: get_tiled_types_export_path(),
                tiled_types_filter:      TiledFilter::from(
                    RegexSet::new([r"^ld58::.+$", r"^bevy_ecs.+$"]).expect(
                        "[TiledPluginConfig.tiled_types_filter] Expected regexes to be valid",
                    ),
                ),
            }),
            TiledPhysicsPlugin::<TiledPhysicsAvianBackend>::default(),
            AsepriteUltraPlugin,
        ));

        app.insert_resource(Gravity::ZERO)
            .insert_resource(ClearColor(Color::BLACK));

        #[cfg(feature = "dev_tools")]
        app.add_plugins(dev_tools::plugin);

        // Add other plugins.
        app.add_plugins((
            game_state::plugin,
            input::plugin,
            asset_tracking::plugin,
            audio::plugin,
            game::plugin,
            menus::plugin,
            theme::plugin,
            screens::plugin,
            camera::plugin,
            quality::plugin,
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

        #[cfg(feature = "dev")]
        app.set_error_handler(bevy::ecs::error::panic);

        #[cfg(not(feature = "dev"))]
        app.set_error_handler(bevy::ecs::error::error);
    }
}

#[cfg(not(feature = "dev_tools"))]
fn get_tiled_types_export_path() -> Option<PathBuf> {
    None
}

#[cfg(feature = "dev_tools")]
fn get_tiled_types_export_path() -> Option<std::path::PathBuf> {
    use std::{
        fs,
        path::{Path, PathBuf},
    };

    fn get_export_path() -> Result<PathBuf> {
        const PATH: &str = "./tiled/tiled_types_export.json";

        let path = Path::new(PATH);

        if let Some(parent) = path.parent()
            && !parent.is_dir()
        {
            info!("Creating directory for {PATH}: {:#?}", parent);
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create parent directory for {PATH}:\n{:#?}", e))?;
        }

        fs::write(PATH, "").map_err(|e| format!("Filed to write to {PATH}\n{:#?}", e))?;
        Ok(PathBuf::from(path))
    }

    match get_export_path() {
        Ok(path) => Some(path),
        Err(e) => {
            error!("Failed to get tiled_export_path: {e}");
            None
        },
    }
}
