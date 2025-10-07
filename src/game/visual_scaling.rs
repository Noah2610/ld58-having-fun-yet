use crate::{
    AppSystems, GameplaySet,
    camera::MainCamera,
    game::{
        survival_timer::SurvivalTimer,
        visuals::{ProjectionScaleAnimation, RotationAnimation},
    },
    screens::Screen,
};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<LastScaleAtSec>();
    app.add_systems(OnEnter(Screen::Gameplay), |mut commands: Commands| {
        commands.insert_resource(LastScaleAtSec::default())
    });
    app.add_systems(
        Update,
        handle_visual_scaling
            .in_set(AppSystems::Update)
            .in_set(GameplaySet),
    );
}

#[derive(Resource, Default)]
struct LastScaleAtSec(u32);

const ROTATION_INCREASE: f32 = 0.0005;
const SCALE_INCREASE: f32 = 0.001;
const SCALE_EVERY_N_SECS: u32 = 5;

fn handle_visual_scaling(
    time: Res<SurvivalTimer>,
    mut last_scale: ResMut<LastScaleAtSec>,
    camera_query: Query<(&mut RotationAnimation, &mut ProjectionScaleAnimation), With<MainCamera>>,
) {
    let secs = time.0.elapsed().as_secs() as u32;
    if last_scale.0 == secs || !secs.is_multiple_of(SCALE_EVERY_N_SECS) {
        return;
    }

    last_scale.0 = secs;

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
