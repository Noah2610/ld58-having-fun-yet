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
    );

    app.configure_sets(
        Update,
        ActiveGameplaySet.run_if(
            in_state(Screen::Gameplay).and(
                in_state(Paused(false))
                    .or(in_state(Paused(true)).and(in_state(ActiveGameplayForced(true)))),
            ),
        ),
    )
    .configure_sets(
        PreUpdate,
        ActiveGameplaySet.run_if(
            in_state(Screen::Gameplay).and(
                in_state(Paused(false))
                    .or(in_state(Paused(true)).and(in_state(ActiveGameplayForced(true)))),
            ),
        ),
    )
    .configure_sets(
        PostUpdate,
        ActiveGameplaySet.run_if(
            in_state(Screen::Gameplay).and(
                in_state(Paused(false))
                    .or(in_state(Paused(true)).and(in_state(ActiveGameplayForced(true)))),
            ),
        ),
    );

    app.configure_sets(
        FixedPostUpdate,
        PhysicsSystems::StepSimulation.run_if(
            in_state(Screen::Gameplay).and(
                in_state(Paused(false))
                    .or(in_state(Paused(true)).and(in_state(ActiveGameplayForced(true)))),
            ),
        ),
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

#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ActiveGameplaySet;

#[derive(States, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[reflect(State)]
pub struct ActiveGameplayForced(pub bool);

/// High-level groupings of systems for the app in the `Update` schedule.
/// When adding a new variant, make sure to order it in the `configure_sets`
/// call above.
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum AppSystems {
    /// Tick timers.
    TickTimers,
    /// Record player input.
    RecordInput,
    /// Do everything else (consider splitting this into further variants).
    Update,
}
