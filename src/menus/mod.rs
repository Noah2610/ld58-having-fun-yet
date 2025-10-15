use crate::{
    input::{MenuAction, action_just_pressed},
    state_history::{InitStateHistory, StateHistory},
};

mod main;
mod pause;
mod settings;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state_with_history::<Menu>();
    app.add_plugins((main::plugin, settings::plugin, pause::plugin));

    app.add_systems(
        Update,
        pop_menu.run_if(in_settings_state.and(action_just_pressed(MenuAction::Cancel))),
    );
}

fn in_settings_state(menu: Res<State<Menu>>) -> bool {
    menu.is_settings()
}

#[derive(States, Reflect, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[reflect(State)]
pub enum Menu {
    #[default]
    None,
    Pop,
    Main,
    Pause,
    Settings,
    AudioSettings,
    VideoSettings,
}

impl Menu {
    fn is_settings(&self) -> bool {
        use Menu::*;
        matches!(self, Settings | AudioSettings | VideoSettings)
    }
}

impl StateHistory for Menu {
    const POP: Self = Self::Pop;
}

fn pop_menu_on_click(_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pop);
}

fn pop_menu(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Pop);
}

fn to_menu_on_click(menu: Menu) -> impl FnMut(On<Pointer<Click>>, ResMut<NextState<Menu>>) {
    move |_: On<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>| next_menu.set(menu)
}

#[allow(dead_code)]
fn to_menu(menu: Menu) -> impl FnMut(ResMut<NextState<Menu>>) {
    move |mut next_menu: ResMut<NextState<Menu>>| next_menu.set(menu)
}
