use bevy::prelude::Reflect;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, Reflect, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Action {
    #[actionlike(Axis)]
    Move,
    #[actionlike(Button)]
    Jump,
}
