use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .init_resource::<ActionState<PlayerAction>>()
        .insert_resource(PlayerAction::default_input_map());
}

#[derive(Actionlike, Reflect, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PlayerAction {
    #[actionlike(Axis)]
    Move,
    #[actionlike(Button)]
    Jump,
}

impl PlayerAction {
    fn default_input_map() -> InputMap<PlayerAction> {
        use PlayerAction::*;
        InputMap::default()
            .with_axis(Move, VirtualAxis::ad())
            .with_axis(Move, VirtualAxis::horizontal_arrow_keys())
            .with_axis(Move, GamepadControlAxis::LEFT_X)
            .with_axis(Move, VirtualAxis::dpad_x())
            .with(Jump, KeyCode::Space)
            .with(Jump, KeyCode::KeyK)
            .with(Jump, GamepadButton::South)
    }
}
