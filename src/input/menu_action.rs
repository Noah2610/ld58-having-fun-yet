use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<MenuAction>::default())
        .init_resource::<ActionState<MenuAction>>()
        .insert_resource(MenuAction::default_input_map());
}

#[derive(Actionlike, Reflect, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MenuAction {
    #[actionlike(Button)]
    Pause,
    #[actionlike(Button)]
    QuitGame,
}

impl MenuAction {
    fn default_input_map() -> InputMap<MenuAction> {
        use MenuAction::*;
        InputMap::default()
            .with(Pause, KeyCode::KeyP)
            .with(Pause, KeyCode::Escape)
            .with(Pause, GamepadButton::Start)
            .with(QuitGame, ModifierKey::Control.with(KeyCode::KeyQ))
    }
}
