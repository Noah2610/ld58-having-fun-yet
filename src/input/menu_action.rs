use crate::input::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<MenuAction>::default())
        .init_resource::<ActionState<MenuAction>>()
        .insert_resource(MenuAction::default_input_map());
}

#[derive(Actionlike, Reflect, Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum MenuAction {
    Pause,
    Cancel,
    QuitGame,
    ToggleFullscreen,
    ToggleMute,
}

impl MenuAction {
    fn default_input_map() -> InputMap<MenuAction> {
        use MenuAction::*;
        InputMap::default()
            .with(Pause, KeyCode::KeyP)
            .with(Pause, KeyCode::Escape)
            .with(Pause, GamepadButton::Start)
            .with(Cancel, KeyCode::Escape)
            .with(Cancel, GamepadButton::Start)
            .with(QuitGame, ModifierKey::Control.with(KeyCode::KeyQ))
            .with(ToggleFullscreen, KeyCode::KeyF)
            .with(ToggleMute, KeyCode::KeyM)
    }
}
