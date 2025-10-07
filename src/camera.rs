use crate::{
    AppSystems, GameplaySet,
    game::{
        player::Player,
        visuals::{
            AnimationDirection, ProjectionScaleAnimation, RotationAnimation, VisualAnimation,
        },
    },
    screens::Screen,
};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Title), spawn_camera);
    app.add_systems(
        Update,
        camera_follow_player
            .in_set(AppSystems::Update)
            .in_set(GameplaySet),
    );
}

#[derive(Component, Reflect, Clone, Copy, Debug, Default)]
#[reflect(Component)]
#[require(
    Name::new("Camera"),
    Camera2d,
    RotationAnimation(VisualAnimation {
        direction:   AnimationDirection::Boomerang,
        period:      4.0,
        range:       Some((-0.003, 0.003)),
        ..default()
    }),
    ProjectionScaleAnimation(VisualAnimation {
        direction:   AnimationDirection::Boomerang,
        period:      8.0,
        range:       Some((0.20, 0.21)),
        ..default()
    })
)]
pub struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        DespawnOnEnter(Screen::Title),
        Projection::Orthographic(OrthographicProjection {
            scale: 0.25,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn camera_follow_player(
    mut camera: Single<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player: Single<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    camera.translation = player.translation;
}
