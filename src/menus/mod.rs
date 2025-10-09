use crate::state_history::{InitStateHistory, StateHistory};

mod main;
mod pause;
mod settings;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state_with_history::<Menu>();
    app.add_plugins((main::plugin, settings::plugin, pause::plugin));
}

#[derive(States, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[reflect(State)]
pub enum Menu {
    #[default]
    None,
    Main,
    Settings,
    Pause,
    Pop,
}

impl StateHistory for Menu {
    const POP: Self = Self::Pop;
}
