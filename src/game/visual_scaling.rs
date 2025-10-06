use crate::{
    AppSystems, GameplaySet,
    camera::MainCamera,
    game::{
        survival_timer::SurvivalTimer,
        visuals::{ProjectionScaleAnimation, RotationAnimation},
    },
};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        handle_visual_scaling
            .in_set(AppSystems::Update)
            .in_set(GameplaySet),
    );
}

const ROTATION_INCREASE: f32 = 0.0005;
const SCALE_INCREASE: f32 = 0.0005;
const SCALE_EVERY_N_SECS: u32 = 5;

fn handle_visual_scaling(
    time: Res<SurvivalTimer>,
    camera_query: Query<(&mut RotationAnimation, &mut ProjectionScaleAnimation), With<MainCamera>>,
) {
    let secs = time.0.elapsed().as_secs() as u32;
    if secs > 0 && (secs % SCALE_EVERY_N_SECS) > 0 {
        return;
    }

    for (mut rot, mut scale) in camera_query {
        if let Some(range) = rot.0.range.as_mut() {
            range.0 -= ROTATION_INCREASE;
            range.1 += ROTATION_INCREASE;
        };
        if let Some(range) = scale.0.range.as_mut() {
            range.0 -= SCALE_INCREASE;
            range.1 += SCALE_INCREASE;
        };
    }
}
