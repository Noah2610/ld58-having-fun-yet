use bevy::prelude::*;
use bevy_yoleck::{vpeol::VpeolCameraState, vpeol_2d::Vpeol2dCameraControl};

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_camera);
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("Camera"),
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
        VpeolCameraState::default(),
        Vpeol2dCameraControl::default(),
    ));
}
