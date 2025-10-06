use crate::{AppSystems, GameplaySet};
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_resource::<Score>();

    app.add_systems(
        Update,
        render_score
            .run_if(resource_changed::<Score>)
            .in_set(AppSystems::Update)
            .in_set(GameplaySet),
    );
}

#[derive(Resource, Reflect, Clone, Default, Debug)]
#[reflect(Resource)]
pub struct Score(pub u32);

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct ScoreValueUi;

fn render_score(score: Res<Score>, query: Query<&mut TextSpan, With<ScoreValueUi>>) {
    for mut ui_text in query {
        ui_text.0 = score.0.to_string();
    }
}
