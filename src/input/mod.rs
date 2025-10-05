use bevy::prelude::*;
pub use leafwing_input_manager::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .init_resource::<ActionState<PlayerAction>>()
        .insert_resource(PlayerAction::default_input_map());
}

#[derive(Actionlike, Reflect, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum PlayerAction {
    #[actionlike(DualAxis)]
    Move,
    #[actionlike(DualAxis)]
    Aim,
    #[actionlike(Button)]
    Shoot,
}

impl PlayerAction {
    fn default_input_map() -> InputMap<PlayerAction> {
        use PlayerAction::*;
        InputMap::default()
            .with_dual_axis(Move, VirtualDPad::wasd())
            .with_dual_axis(Move, VirtualDPad::dpad())
            .with_dual_axis(Move, GamepadStick::LEFT)
            .with_dual_axis(Aim, VirtualDPad::arrow_keys())
            .with_dual_axis(Aim, VirtualDPad::action_pad())
            .with_dual_axis(Aim, GamepadStick::RIGHT)
            .with(Shoot, KeyCode::Space)
            .with(Shoot, GamepadButton::RightTrigger)
            .with(Shoot, GamepadButton::RightTrigger2)
            .with(Shoot, GamepadButton::LeftTrigger)
            .with(Shoot, GamepadButton::LeftTrigger2)
    }
}
