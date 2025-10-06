use crate::{AppSystems, GameplaySet, Paused, screens::Screen};
use bevy::{prelude::*, time::Stopwatch};
use std::time::Duration;

pub fn plugin(app: &mut App) {
    app.init_resource::<SurvivalTimer>();

    app.add_systems(OnEnter(Screen::Gameplay), start_timer);
    app.add_systems(OnExit(Screen::Gameplay), stop_timer);
    app.add_systems(
        OnEnter(Paused(true)),
        pause_timer.run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(
        OnEnter(Paused(false)),
        resume_timer.run_if(in_state(Screen::Gameplay)),
    );

    app.add_systems(
        Update,
        (
            tick_timer
                .in_set(AppSystems::TickTimers)
                .in_set(GameplaySet),
            render_timer.in_set(AppSystems::Update).in_set(GameplaySet),
        ),
    );
}

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct SurvivalTimer(pub Stopwatch);

#[derive(Component, Reflect, Clone, Copy, Default)]
#[reflect(Component)]
pub struct TimeSurvivedValueUi;

fn tick_timer(time: Res<Time>, mut timer: ResMut<SurvivalTimer>) {
    timer.0.tick(time.delta());
}

fn start_timer(mut timer: ResMut<SurvivalTimer>) {
    timer.0.reset();
    timer.0.unpause();
}

fn stop_timer(mut timer: ResMut<SurvivalTimer>) {
    timer.0.pause();
    timer.0.reset();
}

fn pause_timer(mut timer: ResMut<SurvivalTimer>) {
    timer.0.pause();
}

fn resume_timer(mut timer: ResMut<SurvivalTimer>) {
    timer.0.unpause();
}

fn render_timer(timer: Res<SurvivalTimer>, query: Query<&mut TextSpan, With<TimeSurvivedValueUi>>) {
    for mut ui_text in query {
        ui_text.0 = format_time(timer.0.elapsed());
    }
}

fn format_time(time: Duration) -> String {
    let total_seconds = time.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}
