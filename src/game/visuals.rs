//! Funny animated visual effects like color shifting and camera zooming
//! (so i don't have to make graphics or learn shaders)

use crate::AppSystems;
use bevy::prelude::*;
use bevy_ecs_tiled::prelude::{TileColor, TilePos};
use std::f32::consts::PI;

pub fn plugin(app: &mut App) {
    app.insert_resource(ClearColor(Color::hsl(0.0, 0.3, 0.075)));

    app.insert_resource(BackgroundHueAnimation(VisualAnimation {
        range: Some((110.0, 220.0)),
        period: 10.0,
        direction: AnimationDirection::Boomerang,
        ..default()
    }))
    .init_resource::<BackgroundHueAnimationState>();

    // app.insert_resource(BackgroundSaturationAnimation(VisualAnimation {
    //     ..default()
    // }))
    // .init_resource::<BackgroundSaturationAnimationState>();

    // app.insert_resource(BackgroundLightnessAnimation(VisualAnimation {
    //     ..default()
    // }))
    // .init_resource::<BackgroundLightnessAnimationState>();

    app.insert_resource(GlobalAnimationsEnabled(true));
    app.insert_resource(GlobalColorAnimationsEnabled(true));
    app.insert_resource(GlobalTransformAnimationsEnabled(true));
    app.insert_resource(GlobalCameraAnimationsEnabled(true));
    app.insert_resource(BackgroundVisualAnimationEnabled(true));

    app.add_systems(
        Update,
        ((
            (
                handle_set_sprite_color,
                handle_store_positions_for_position_animations,
            ),
            (
                (
                    (
                        animate_background_hue.run_if(has_background_hue_animation),
                        animate_background_saturation.run_if(has_background_saturation_animation),
                        animate_background_lightness.run_if(has_background_lightness_animation),
                    )
                        .run_if(resource_exists_and_equals(
                            BackgroundVisualAnimationEnabled(true),
                        )),
                    update_hue_animations,
                    update_saturation_animations,
                    update_lightness_animations,
                )
                    .run_if(global_color_animations_enabled),
                (
                    (
                        update_rotation_animations,
                        update_projection_scale_animations,
                        update_position_x_animations,
                        update_position_y_animations,
                    )
                        .run_if(global_camera_animations_enabled),
                    update_transform_scale_x_animations,
                    update_transform_scale_y_animations,
                )
                    .run_if(global_transform_animations_enabled),
            ),
            (render_color_animations, render_position_animations),
        )
            .chain()
            .run_if(global_animations_enabled))
        .in_set(AppSystems::Update), // .in_set(GameplaySet),
    );
}

fn global_animations_enabled(enabled: Option<Res<GlobalAnimationsEnabled>>) -> bool {
    enabled.map(|e| e.0).unwrap_or_default()
}

fn global_color_animations_enabled(enabled: Option<Res<GlobalColorAnimationsEnabled>>) -> bool {
    enabled.map(|e| e.0).unwrap_or_default()
}

fn global_camera_animations_enabled(enabled: Option<Res<GlobalCameraAnimationsEnabled>>) -> bool {
    enabled.map(|e| e.0).unwrap_or_default()
}

fn global_transform_animations_enabled(
    enabled: Option<Res<GlobalTransformAnimationsEnabled>>,
) -> bool {
    enabled.map(|e| e.0).unwrap_or_default()
}

fn has_background_hue_animation(
    animation: Option<Res<BackgroundHueAnimation>>,
    state: Option<Res<BackgroundHueAnimationState>>,
) -> bool {
    animation.is_some() && state.is_some()
}

fn has_background_saturation_animation(
    animation: Option<Res<BackgroundSaturationAnimation>>,
    state: Option<Res<BackgroundSaturationAnimationState>>,
) -> bool {
    animation.is_some() && state.is_some()
}

fn has_background_lightness_animation(
    animation: Option<Res<BackgroundLightnessAnimation>>,
    state: Option<Res<BackgroundLightnessAnimationState>>,
) -> bool {
    animation.is_some() && state.is_some()
}

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct GlobalAnimationsEnabled(bool);
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct GlobalColorAnimationsEnabled(bool);
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct GlobalCameraAnimationsEnabled(bool);
#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct GlobalTransformAnimationsEnabled(bool);

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct AnimationsDisabled;

#[derive(Reflect)]
pub struct VisualAnimation {
    pub direction:   AnimationDirection,
    pub period:      f32,
    pub range:       Option<(f32, f32)>,
    pub time_offset: f32,
}

impl Default for VisualAnimation {
    fn default() -> Self {
        Self {
            direction:   AnimationDirection::Linear,
            period:      5.0,
            range:       None,
            time_offset: 0.0,
        }
    }
}

#[derive(Reflect, Default)]
pub enum AnimationDirection {
    #[default]
    Linear,
    Boomerang,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct SetSpriteColor(pub Color);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(HueAnimationState)]
pub struct HueAnimation(pub VisualAnimation);
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct HueAnimationState(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(SaturationAnimationState)]
pub struct SaturationAnimation(pub VisualAnimation);
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SaturationAnimationState(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(LightnessAnimationState)]
pub struct LightnessAnimation(pub VisualAnimation);
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct LightnessAnimationState(pub f32);

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct BackgroundHueAnimation(VisualAnimation);
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct BackgroundHueAnimationState(f32);

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct BackgroundSaturationAnimation(VisualAnimation);
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct BackgroundSaturationAnimationState(f32);

#[derive(Resource, Reflect)]
#[reflect(Resource)]
struct BackgroundLightnessAnimation(VisualAnimation);
#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
struct BackgroundLightnessAnimationState(f32);

#[derive(Resource, Reflect, Clone, PartialEq, Eq)]
#[reflect(Resource)]
struct BackgroundVisualAnimationEnabled(bool);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(RotationAnimationState)]
pub struct RotationAnimation(pub VisualAnimation);
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct RotationAnimationState(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(ProjectionScaleAnimationState)]
pub struct ProjectionScaleAnimation(pub VisualAnimation);
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ProjectionScaleAnimationState(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(ScaleXAnimationState)]
pub struct ScaleXAnimation(pub VisualAnimation);
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ScaleXAnimationState(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(ScaleYAnimationState)]
pub struct ScaleYAnimation(pub VisualAnimation);
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct ScaleYAnimationState(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(PositionXAnimationState, StoredPositionX)]
pub struct PositionXAnimation(pub VisualAnimation);
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct PositionXAnimationState(pub f32);
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct StoredPositionX(pub f32);

#[derive(Component, Reflect)]
#[reflect(Component)]
#[require(PositionYAnimationState, StoredPositionY)]
pub struct PositionYAnimation(pub VisualAnimation);
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct PositionYAnimationState(pub f32);
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct StoredPositionY(pub f32);

const DEFAULT_HUE_RANGE: (f32, f32) = (0.0, 360.0);
const DEFAULT_SATURATION_RANGE: (f32, f32) = (0.0, 1.0);
const DEFAULT_LIGHTNESS_RANGE: (f32, f32) = (0.0, 1.0);
const DEFAULT_CAMERA_ROTATION_RANGE: (f32, f32) = (-0.05, 0.05);
const DEFAULT_CAMERA_SCALE_RANGE: (f32, f32) = (0.2, 0.3);
const DEFAULT_SCALE_RANGE: (f32, f32) = (0.5, 1.5);
const DEFAULT_POSITION_RANGE: (f32, f32) = (-20.0, 20.0);

fn handle_set_sprite_color(
    mut commands: Commands,
    query: Query<
        (
            Entity,
            &SetSpriteColor,
            Option<&mut Sprite>,
            Option<&mut TileColor>,
        ),
        (Added<SetSpriteColor>, Or<(With<Sprite>, With<TileColor>)>),
    >,
) {
    for (entity, set_color, mut sprite, mut tile) in query {
        let color = sprite
            .as_mut()
            .map(|s| &mut s.color)
            .or_else(|| tile.as_mut().map(|t| &mut t.0));

        if let Some(color) = color {
            *color = set_color.0;
        }

        commands.entity(entity).remove::<SetSpriteColor>();
    }
}

fn handle_store_positions_for_position_animations(
    query: Query<
        (
            &Transform,
            Option<&mut StoredPositionX>,
            Option<&mut StoredPositionY>,
        ),
        (
            Added<Transform>,
            Or<(With<StoredPositionX>, With<StoredPositionY>)>,
        ),
    >,
) {
    for (transform, stored_x, stored_y) in query {
        if let Some(mut stored_x) = stored_x {
            stored_x.0 = transform.translation.x;
        }
        if let Some(mut stored_y) = stored_y {
            stored_y.0 = transform.translation.y;
        }
    }
}

fn render_color_animations(
    query: Query<
        (
            Option<&mut Sprite>,
            Option<&mut TileColor>,
            Option<&HueAnimationState>,
            Option<&SaturationAnimationState>,
            Option<&LightnessAnimationState>,
        ),
        (
            Without<AnimationsDisabled>,
            Or<(
                With<HueAnimationState>,
                With<SaturationAnimationState>,
                With<LightnessAnimationState>,
            )>,
            Or<(With<Sprite>, With<TileColor>)>,
        ),
    >,
) {
    for (mut sprite, mut tile_color, hue, saturation, lightness) in query {
        let color = sprite
            .as_mut()
            .map(|s| &mut s.color)
            .or_else(|| tile_color.as_mut().map(|t| &mut t.0));
        if let Some(color) = color {
            if let Some(hue) = hue {
                color.set_hue(hue.0);
            }
            if let Some(saturation) = saturation {
                color.set_saturation(saturation.0);
            }
            if let Some(lightness) = lightness {
                *color = Color::hsl(color.hue(), color.saturation(), lightness.0);
            }
        }
    }
}

fn render_position_animations(
    query: Query<
        (
            &mut Transform,
            Option<(&PositionXAnimationState, &StoredPositionX)>,
            Option<(&PositionYAnimationState, &StoredPositionY)>,
        ),
        (
            Without<AnimationsDisabled>,
            Or<(With<PositionXAnimationState>, With<PositionYAnimationState>)>,
        ),
    >,
) {
    for (mut transform, anim_x, anim_y) in query {
        if let Some((state_x, stored_x)) = anim_x {
            transform.translation.x = state_x.0 + stored_x.0;
        }
        if let Some((state_y, stored_y)) = anim_y {
            transform.translation.y = state_y.0 + stored_y.0;
        }
    }
}

fn animate_background_hue(
    time: Res<Time>,
    visuals: Res<BackgroundHueAnimation>,
    mut state: ResMut<BackgroundHueAnimationState>,
    mut bg: ResMut<ClearColor>,
) {
    state.0 = animate(time.elapsed_secs(), &visuals.0, DEFAULT_HUE_RANGE);
    bg.0.set_hue(state.0);
}

fn animate_background_saturation(
    time: Res<Time>,
    visuals: Res<BackgroundSaturationAnimation>,
    mut state: ResMut<BackgroundSaturationAnimationState>,
    mut bg: ResMut<ClearColor>,
) {
    state.0 = animate(time.elapsed_secs(), &visuals.0, DEFAULT_SATURATION_RANGE);
    bg.0.set_saturation(state.0);
}

fn animate_background_lightness(
    time: Res<Time>,
    visuals: Res<BackgroundLightnessAnimation>,
    mut state: ResMut<BackgroundLightnessAnimationState>,
    mut bg: ResMut<ClearColor>,
) {
    state.0 = animate(time.elapsed_secs(), &visuals.0, DEFAULT_LIGHTNESS_RANGE);
    bg.0 = Color::hsl(bg.0.hue(), bg.0.saturation(), state.0);
}

fn update_hue_animations(
    time: Res<Time>,
    mut animations: Query<(&HueAnimation, &mut HueAnimationState), Without<AnimationsDisabled>>,
) {
    for (anim, mut state) in &mut animations {
        state.0 = animate(time.elapsed_secs(), &anim.0, DEFAULT_HUE_RANGE);
    }
}

fn update_saturation_animations(
    time: Res<Time>,
    mut animations: Query<
        (&SaturationAnimation, &mut SaturationAnimationState),
        Without<AnimationsDisabled>,
    >,
) {
    for (anim, mut state) in &mut animations {
        state.0 = animate(time.elapsed_secs(), &anim.0, DEFAULT_SATURATION_RANGE);
    }
}

fn update_lightness_animations(
    time: Res<Time>,
    mut animations: Query<
        (&LightnessAnimation, &mut LightnessAnimationState),
        Without<AnimationsDisabled>,
    >,
) {
    for (anim, mut state) in &mut animations {
        state.0 = animate(time.elapsed_secs(), &anim.0, DEFAULT_LIGHTNESS_RANGE);
    }
}

fn update_rotation_animations(
    time: Res<Time>,
    mut query: Query<
        (
            &RotationAnimation,
            &mut RotationAnimationState,
            &mut Transform,
        ),
        Without<AnimationsDisabled>,
    >,
) {
    for (anim, mut state, mut transform) in &mut query {
        state.0 = animate(time.elapsed_secs(), &anim.0, DEFAULT_CAMERA_ROTATION_RANGE);
        transform.rotation.z = state.0;
    }
}

fn update_projection_scale_animations(
    time: Res<Time>,
    mut query: Query<
        (
            &ProjectionScaleAnimation,
            &mut ProjectionScaleAnimationState,
            &mut Projection,
        ),
        Without<AnimationsDisabled>,
    >,
) {
    for (anim, mut state, mut projection) in &mut query {
        if let Projection::Orthographic(ortho) = projection.as_mut() {
            state.0 = animate(time.elapsed_secs(), &anim.0, DEFAULT_CAMERA_SCALE_RANGE);
            ortho.scale = state.0;
        }
    }
}

fn update_transform_scale_x_animations(
    time: Res<Time>,
    mut query: Query<
        (&ScaleXAnimation, &mut ScaleXAnimationState, &mut Transform),
        Without<AnimationsDisabled>,
    >,
) {
    for (anim, mut state, mut transform) in &mut query {
        state.0 = animate(time.elapsed_secs(), &anim.0, DEFAULT_SCALE_RANGE);
        transform.scale = Vec3::new(state.0, transform.scale.y, transform.scale.z);
    }
}

fn update_transform_scale_y_animations(
    time: Res<Time>,
    mut query: Query<
        (&ScaleYAnimation, &mut ScaleYAnimationState, &mut Transform),
        Without<AnimationsDisabled>,
    >,
) {
    for (anim, mut state, mut transform) in &mut query {
        state.0 = animate(time.elapsed_secs(), &anim.0, DEFAULT_SCALE_RANGE);
        transform.scale = Vec3::new(transform.scale.x, state.0, transform.scale.z);
    }
}

fn update_position_x_animations(
    time: Res<Time>,
    query: Query<(&PositionXAnimation, &mut PositionXAnimationState), Without<AnimationsDisabled>>,
) {
    let secs = time.elapsed_secs();
    for (anim, mut state) in query {
        state.0 = animate(secs, &anim.0, DEFAULT_POSITION_RANGE);
    }
}

fn update_position_y_animations(
    time: Res<Time>,
    query: Query<(&PositionYAnimation, &mut PositionYAnimationState), Without<AnimationsDisabled>>,
) {
    let secs = time.elapsed_secs();
    for (anim, mut state) in query {
        state.0 = animate(secs, &anim.0, DEFAULT_POSITION_RANGE);
    }
}

fn animate(
    elapsed: f32,
    VisualAnimation {
        range,
        period,
        direction,
        time_offset,
    }: &VisualAnimation,
    default_range: (f32, f32),
) -> f32 {
    let t = elapsed + time_offset;
    let (min, max) = range.unwrap_or(default_range);
    match direction {
        AnimationDirection::Linear => min + ((max - min) * ((t / period) % 1.0)),
        AnimationDirection::Boomerang => {
            min + (max - min) * 0.5 * (1.0 - ((2.0 * PI * t) / period).cos())
        },
    }
}
