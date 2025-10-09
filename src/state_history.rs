use bevy::{prelude::*, state::state::FreelyMutableState};

pub trait InitStateHistory {
    /// Init given state (App::init_state) and setup its state history.
    fn init_state_with_history<S: StateHistory + FreelyMutableState + Default>(
        &mut self,
    ) -> &mut Self;
    /// Init state history for given state. State must init separately.
    fn init_state_history<S: StateHistory>(&mut self) -> &mut Self;
}

impl InitStateHistory for App {
    fn init_state_with_history<S: StateHistory + FreelyMutableState + FromWorld>(
        &mut self,
    ) -> &mut Self {
        self.init_state::<S>();
        self.init_state_history::<S>()
    }

    fn init_state_history<S: StateHistory>(&mut self) -> &mut Self {
        self.init_resource::<StateHistoryList<S>>();
        self.add_systems(StateTransition, S::push_history);
        self
    }
}

pub trait StateHistory: States + FreelyMutableState {
    const POP: Self;

    fn push_history(
        mut history: ResMut<StateHistoryList<Self>>,
        mut reader: MessageReader<StateTransitionEvent<Self>>,
        mut next_state: ResMut<NextState<Self>>,
    ) {
        for event in reader.read() {
            match (event.entered.as_ref(), event.exited.as_ref()) {
                (Some(p), _) if p == &Self::POP => {
                    if let Some(popped) = history.pop() {
                        next_state.set(popped)
                    }
                },
                (_, Some(p)) if p == &Self::POP => {},
                (_, Some(new_state))
                    if history.0.last().map(|s| s != new_state).unwrap_or(true) =>
                {
                    history.0.push(new_state.clone())
                },
                _ => {},
            }
        }
    }
}

#[derive(Resource, Reflect, Debug)]
#[reflect(Resource)]
pub struct StateHistoryList<S: StateHistory>(Vec<S>);

impl<S: StateHistory> StateHistoryList<S> {
    fn pop(&mut self) -> Option<S> {
        self.0.pop()
    }
}

impl<S: StateHistory> Default for StateHistoryList<S> {
    fn default() -> Self {
        Self(Vec::new())
    }
}
