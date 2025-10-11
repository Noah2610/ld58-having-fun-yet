use crate::{camera::MainCamera, game::mouse_aim::MouseAimEnabled, input::*};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<MouseAction>::default());
    // app.init_resource::<ActionState<MouseAction>>();

    app.add_systems(OnEnter(MouseAimEnabled(true)), add_input_map);
    app.add_systems(OnExit(MouseAimEnabled(false)), remove_input_map);
}

#[derive(Actionlike, Reflect, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[actionlike(DualAxis)]
pub enum MouseAction {
    Move,
}

impl MouseAction {
    fn default_input_map() -> InputMap<MouseAction> {
        InputMap::<MouseAction>::default().with_dual_axis(MouseAction::Move, MouseMove::default())
    }
}

fn add_input_map(
    mut commands: Commands,
    camera: Single<(Entity, Has<InputMap<MouseAction>>), With<MainCamera>>,
) {
    if camera.1 {
        return;
    }
    commands.entity(camera.0).insert((
        MouseAction::default_input_map(),
        ActionState::<MouseAction>::default(),
    ));
}

fn remove_input_map(
    mut commands: Commands,
    camera: Single<(Entity, Has<InputMap<MouseAction>>), With<MainCamera>>,
) {
    if !camera.1 {
        return;
    }

    commands
        .entity(camera.0)
        .remove::<InputMap<MouseAction>>()
        .remove::<ActionState<MouseAction>>();
}
