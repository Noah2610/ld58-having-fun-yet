use crate::{
    game::{player::Player, solid::Solid},
    screens::Screen,
};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_yoleck::{
    prelude::*,
    vpeol_2d::{Vpeol2dPluginForEditor, Vpeol2dPluginForGame, Vpeol2dPosition},
};

pub fn plugin(app: &mut App) {
    let is_editor = cfg!(feature = "editor") && std::env::args().any(|arg| arg == "--editor");

    if is_editor {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin::default());
        }

        app.add_plugins((
            YoleckPluginForEditor,
            Vpeol2dPluginForEditor,
            YoleckSyncWithEditorState {
                when_editor: Screen::Editor,
                when_game:   Screen::Gameplay,
            },
        ));
    } else {
        app.add_plugins((YoleckPluginForGame, Vpeol2dPluginForGame));
    }

    app.add_yoleck_entity_type(
        YoleckEntityType::new("Player")
            .with::<Player>()
            .with::<Vpeol2dPosition>(),
    );
    app.add_yoleck_entity_type(
        YoleckEntityType::new("Ground")
            .with::<Solid>()
            .with::<Vpeol2dPosition>(),
    );

    // app.add_systems(
    //     YoleckSchedule::Populate,
    //     (populate_player, populate_ground),
    // );
}

// fn populate_player(
//     mut populate: YoleckPopulate<&Player>,
//     assets: Res<PlayerAssets>,
// ) {
//     populate.populate(|_ctx, mut commands, _player| {
//         commands.insert(player(&assets));
//     });
// }

// fn populate_ground(mut populate: YoleckPopulate<&Solid>) {
//     populate.populate(|_ctx, mut commands, _player| {
//         commands.insert(solid());
//     });
// }
