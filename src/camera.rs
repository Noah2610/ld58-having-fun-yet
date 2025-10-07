use crate::{
    AppSystems, GameplaySet,
    game::{
        player::Player,
        visuals::{
            AnimationDirection, ProjectionScaleAnimation, RotationAnimation, ScaleXAnimation,
            ScaleYAnimation, VisualAnimation,
        },
    },
    screens::Screen,
};
use bevy::prelude::*;
use rand::Rng;

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
        period:      rand::rng().random_range(1.0 .. 4.0),
        // range:       Some((-0.003, 0.003)),
        range:       Some((rand::rng().random_range(-1.0 .. 0.0), rand::rng().random_range(0.0 .. 1.0))),
        ..default()
    }),
    ProjectionScaleAnimation(VisualAnimation {
        direction:   AnimationDirection::Boomerang,
        period:      rand::rng().random_range(1.0 .. 8.0),
        // range:       Some((0.20, 0.21)),
        range:       Some((0.05, 0.5)),
        ..default()
    }),
    ScaleXAnimation(VisualAnimation{
        direction: AnimationDirection::Boomerang,
        period: rand::rng().random_range(1.0 .. 8.0),
        range: Some((rand::rng().random_range(0.0 .. 0.5), rand::rng().random_range(0.5 .. 2.0))),
        time_offset: 0.0,
    }),
    ScaleYAnimation(VisualAnimation{
        direction: AnimationDirection::Boomerang,
        period: rand::rng().random_range(1.0 .. 8.0),
        range: Some((rand::rng().random_range(0.0 .. 0.5), rand::rng().random_range(0.5 .. 2.0))),
        time_offset: 0.0,
    }),
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
