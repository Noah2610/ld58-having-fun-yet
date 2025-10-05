use crate::{AppSystems, GameplaySet, game::player::Player};
use bevy::prelude::*;
use bevy_yoleck::{vpeol::VpeolCameraState, vpeol_2d::Vpeol2dCameraControl};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera).add_systems(
        Update,
        camera_follow_player
            .in_set(AppSystems::Update)
            .in_set(GameplaySet),
    );
}

#[derive(Component, Reflect, Clone, Copy, Debug, Default)]
#[reflect(Component)]
#[require(Name::new("Camera"), Camera2d)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
        VpeolCameraState::default(),
        Vpeol2dCameraControl::default(),
    ));
}

fn camera_follow_player(
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    camera.translation = player.translation;
}
