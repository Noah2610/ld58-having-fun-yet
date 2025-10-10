use crate::input::*;

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(
        InputMap::<MouseAction>::default().with_dual_axis(MouseAction::Pan, MouseMove::default()),
    )
    .init_resource::<ActionState<MouseAction>>();
}

#[derive(Actionlike, Reflect, Clone, Copy, PartialEq, Eq, Hash, Debug)]
#[actionlike(DualAxis)]
pub enum MouseAction {
    Pan,
}
