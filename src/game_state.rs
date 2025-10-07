use crate::screens::Screen;
use avian2d::prelude::PhysicsSystems;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<Paused>().init_state::<GameOver>();
    app.configure_sets(
        Update,
        GameplaySet.run_if(in_state(Screen::Gameplay).and(in_state(Paused(false)))),
    )
    .configure_sets(
        PreUpdate,
        GameplaySet.run_if(in_state(Screen::Gameplay).and(in_state(Paused(false)))),
    )
    .configure_sets(
        PostUpdate,
        GameplaySet.run_if(in_state(Screen::Gameplay).and(in_state(Paused(false)))),
    )
    .configure_sets(
        FixedPostUpdate,
        PhysicsSystems::StepSimulation
            .run_if(in_state(Screen::Gameplay).and(in_state(Paused(false)))),
    );

    app.add_systems(
        OnEnter(Screen::Gameplay),
        |mut next_state: ResMut<NextState<GameOver>>| next_state.set(GameOver(false)),
    );
}

#[derive(States, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[reflect(State)]
pub struct Paused(pub bool);

#[derive(States, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[reflect(State)]
pub struct GameOver(pub bool);

#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct GameplaySet;
