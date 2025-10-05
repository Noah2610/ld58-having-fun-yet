use crate::screens::Screen;
use avian2d::prelude::PhysicsSystems;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<Paused>();
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
    );
    app.configure_sets(
        FixedPostUpdate,
        PhysicsSystems::StepSimulation
            .run_if(in_state(Screen::Gameplay).and(in_state(Paused(false)))),
    );
}

#[derive(States, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[reflect(State)]
pub struct Paused(pub bool);

#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct GameplaySet;
