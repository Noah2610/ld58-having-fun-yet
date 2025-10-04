use crate::{
    demo::player::{player, Player, PlayerAssets},
    screens::Screen,
};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_yoleck::prelude::*;

pub fn plugin(app: &mut App) {
    let is_editor = cfg!(feature = "editor")
        && std::env::args().any(|arg| arg == "--editor");

    if is_editor {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin::default());
        }

        app.add_plugins((YoleckPluginForEditor, YoleckSyncWithEditorState {
            when_editor: Screen::Editor,
            when_game:   Screen::Gameplay,
        }));
    } else {
        app.add_plugins(YoleckPluginForGame);
    }

    app.add_yoleck_entity_type({
        YoleckEntityType::new("Player").with::<Player>()
    });

    app.add_systems(YoleckSchedule::Populate, populate_player);

    // app.add_yoleck_edit_system(edit_rectangle);
    // app.add_systems(YoleckSchedule::Populate, populate_rectangle);
}

fn populate_player(
    mut populate: YoleckPopulate<&Player>,
    assets: Res<PlayerAssets>,
) {
    populate.populate(|_ctx, mut commands, _player| {
        commands.insert(player(&assets));
    });
}
