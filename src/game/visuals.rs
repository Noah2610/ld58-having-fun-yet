//! Funny animated visual effects like color shifting and camera zooming
//! (so i don't have to make graphics or learn shaders)

use crate::{AppSystems, GameplaySet, camera::MainCamera};
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn plugin(app: &mut App) {
    app.init_resource::<BackgroundColorAnimation>();
    app.init_resource::<BackgroundColorAnimationState>();
    app.init_resource::<BackgroundColorAnimationEnabled>();
    app.insert_resource(GlobalColorAnimationsEnabled(true));

    app.add_systems(
        Update,
        (
            animate_background.run_if(global_animations_enabled().and(resource_exists_and_equals(
                BackgroundColorAnimationEnabled(true),
            ))),
            update_hue_animations,
            animate_camera,
        ), // .in_set(AppSystems::Update), // .in_set(GameplaySet),
    );
}

fn global_animations_enabled() -> impl SystemCondition<()> {
    IntoSystem::into_system(|enabled: Option<Res<GlobalColorAnimationsEnabled>>| enabled.is_some())
}

#[derive(Resource, Reflect, Clone, PartialEq, Eq)]
#[reflect(Resource)]
pub struct GlobalColorAnimationsEnabled(bool);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(ColorAnimationState)]
pub struct ColorAnimation {
    pub range:       (f32, f32),
    pub period:      f32,
    pub direction:   AnimationDirection,
    pub time_offset: f32,
}

#[derive(Reflect, Default)]
pub enum AnimationDirection {
    #[default]
    Linear,
    Boomerang,
}

impl Default for ColorAnimation {
    fn default() -> Self {
        Self {
            range:       (0.0, 360.0),
            period:      5.0,
            direction:   AnimationDirection::Linear,
            time_offset: 0.0,
        }
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ColorAnimationState(pub Color);

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct ColorAnimationDisabled;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct BackgroundColorAnimation(ColorAnimation);

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct BackgroundColorAnimationState(ColorAnimationState);

#[derive(Resource, Reflect, Clone, PartialEq, Eq)]
#[reflect(Resource)]
struct BackgroundColorAnimationEnabled(bool);

impl Default for BackgroundColorAnimationEnabled {
    fn default() -> Self {
        Self(true)
    }
}

impl Default for BackgroundColorAnimation {
    fn default() -> Self {
        Self(ColorAnimation {
            range: (110.0, 220.0),
            period: 10.0,
            direction: AnimationDirection::Boomerang,
            ..default()
        })
    }
}

impl Default for BackgroundColorAnimationState {
    fn default() -> Self {
        Self(ColorAnimationState(Color::hsl(0.0, 0.2, 0.05)))
    }
}

fn animate_background(
    time: Res<Time>,
    visuals: Res<BackgroundColorAnimation>,
    mut state: ResMut<BackgroundColorAnimationState>,
    mut bg: ResMut<ClearColor>,
) {
    state.0.0 = Color::hsla(
        animate(&visuals.0, time.elapsed_secs()),
        state.0.0.saturation(),
        state.0.0.luminance(),
        state.0.0.alpha(),
    );
    bg.0 = state.0.0;
}

fn update_hue_animations(
    time: Res<Time>,
    mut animations: Query<
        (&ColorAnimation, &mut ColorAnimationState, &mut Sprite),
        Without<ColorAnimationDisabled>,
    >,
) {
    for (config, mut state, mut sprite) in &mut animations {
        state.0 = Color::hsla(
            animate(config, time.elapsed_secs()),
            state.0.saturation(),
            state.0.luminance(),
            state.0.alpha(),
        );
        sprite.color = state.0;
    }
}

fn animate_camera(
    time: Res<Time>,
    mut camera: Single<(&mut Transform, &mut Projection), With<MainCamera>>,
) {
    camera.0.rotation.z = animate(
        &ColorAnimation {
            range:       (-0.05, 0.05),
            period:      6.0,
            direction:   AnimationDirection::Boomerang,
            time_offset: 0.0,
        },
        time.elapsed_secs(),
    );

    if let Projection::Orthographic(ortho) = camera.1.as_mut() {
        ortho.scale = animate(
            &ColorAnimation {
                range:       (0.20, 0.30),
                period:      4.0,
                direction:   AnimationDirection::Boomerang,
                time_offset: 0.0,
            },
            time.elapsed_secs(),
        );
    }
}

fn animate(
    ColorAnimation {
        range: (min, max),
        period,
        direction,
        time_offset,
    }: &ColorAnimation,
    elapsed: f32,
) -> f32 {
    let t = elapsed + time_offset;
    match direction {
        AnimationDirection::Linear => min + ((max - min) * ((t / period) % 1.0)),
        AnimationDirection::Boomerang => {
            min + (max - min) * 0.5 * (1.0 - ((2.0 * PI * t) / period).cos())
        },
    }
}

// fn color_anim(
//     time: Res<Time>,
//     mut player: Single<&mut Sprite, With<Player>>,
//     mut bg: ResMut<ClearColor>,
//     mut proj: Single<&mut Projection, With<Camera2d>>,
// ) {
//     let delta = time.delta_secs();

//     {
//         let hue = (player.color.hue() + 60.0 * delta) % 360.0;
//         player.color = Color::hsl(hue, 1.0, 0.5);
//     }

//     {
//         let hue = (bg.0.hue() + 30.0 * delta) % 360.0;
//         bg.0 = Color::hsl(hue, 0.3, 0.1);
//     }

//     {
//         if let Projection::Orthographic(ortho) = proj.as_mut() {
//             ortho.scale = 0.25 + (time.elapsed_secs().sin() * 0.05);
//         }
//     }
// }
