use crate::{asset_tracking::LoadResource, audio::sound_effect};
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(checkbox::plugin);

    app.add_systems(Update, apply_interaction_palette);

    app.load_resource::<InteractionAssets>();
    app.add_observer(play_on_hover_sound_effect);
    app.add_observer(play_on_click_sound_effect);
}

/// Palette for widget interactions. Add this to an entity that supports
/// [`Interaction`]s, such as a button, to change its [`BackgroundColor`] based
/// on the current interaction state.
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct InteractionPalette {
    pub none:    Color,
    pub hovered: Color,
    pub pressed: Color,
}

fn apply_interaction_palette(
    mut palette_query: Query<
        (&Interaction, &InteractionPalette, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (interaction, palette, mut background) in &mut palette_query {
        *background = match interaction {
            Interaction::None => palette.none,
            Interaction::Hovered => palette.hovered,
            Interaction::Pressed => palette.pressed,
        }
        .into();
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct InteractionAssets {
    #[dependency]
    hover: Handle<AudioSource>,
    #[dependency]
    click: Handle<AudioSource>,
}

impl FromWorld for InteractionAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            hover: assets.load("audio/ui/button_hover.ogg"),
            click: assets.load("audio/ui/button_click.ogg"),
        }
    }
}

fn play_on_hover_sound_effect(
    trigger: On<Pointer<Over>>,
    mut commands: Commands,
    interaction_assets: Option<Res<InteractionAssets>>,
    interaction_query: Query<(), With<Interaction>>,
) {
    let Some(interaction_assets) = interaction_assets else {
        return;
    };

    if interaction_query.contains(trigger.entity) {
        commands.spawn(sound_effect(interaction_assets.hover.clone()));
    }
}

fn play_on_click_sound_effect(
    trigger: On<Pointer<Click>>,
    mut commands: Commands,
    interaction_assets: Option<Res<InteractionAssets>>,
    interaction_query: Query<(), With<Interaction>>,
) {
    let Some(interaction_assets) = interaction_assets else {
        return;
    };

    if interaction_query.contains(trigger.entity) {
        commands.spawn(sound_effect(interaction_assets.click.clone()));
    }
}

pub mod checkbox {
    //! source:
    //! https://github.com/bevyengine/bevy/blob/main/examples/ui/standard_widgets_observers.rs

    use bevy::{
        input_focus::{
            InputDispatchPlugin,
            tab_navigation::{TabGroup, TabIndex, TabNavigationPlugin},
        },
        picking::hover::Hovered,
        prelude::*,
        reflect::Is,
        ui::{Checked, InteractionDisabled, Pressed},
        ui_widgets::{
            Activate, Button, Checkbox, Slider, SliderRange, SliderThumb, SliderValue,
            UiWidgetsPlugins, ValueChange, checkbox_self_update, observe,
        },
    };

    #[derive(Component, Default)]
    pub struct CheckedDefault(pub bool);

    pub(super) fn plugin(app: &mut App) {
        app.add_plugins((InputDispatchPlugin, TabNavigationPlugin))
            // .add_observer(button_on_interaction::<Add, Pressed>)
            // .add_observer(button_on_interaction::<Remove, Pressed>)
            // .add_observer(button_on_interaction::<Add, InteractionDisabled>)
            // .add_observer(button_on_interaction::<Remove, InteractionDisabled>)
            // .add_observer(button_on_interaction::<Insert, Hovered>)
            // .add_observer(slider_on_interaction::<Add, InteractionDisabled>)
            // .add_observer(slider_on_interaction::<Remove, InteractionDisabled>)
            // .add_observer(slider_on_interaction::<Insert, Hovered>)
            // .add_observer(slider_on_change_value::<SliderValue>)
            // .add_observer(slider_on_change_value::<SliderRange>)
            .add_observer(checkbox_on_interaction::<Add, InteractionDisabled>)
            .add_observer(checkbox_on_interaction::<Remove, InteractionDisabled>)
            .add_observer(checkbox_on_interaction::<Insert, Hovered>)
            .add_observer(checkbox_on_interaction::<Add, Checked>)
            .add_observer(checkbox_on_interaction::<Remove, Checked>)
            .add_systems(Update, add_checked);
    }

    fn add_checked(
        mut commands: Commands,
        query: Query<(Entity, &CheckedDefault), (Added<CheckedDefault>, With<Checkbox>)>,
    ) {
        for (entity, checked_default) in query {
            if checked_default.0 {
                commands
                    .entity(entity)
                    .insert(Checked)
                    .remove::<CheckedDefault>();
            }
        }
    }

    const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
    const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
    const SLIDER_TRACK: Color = Color::srgb(0.05, 0.05, 0.05);
    const SLIDER_THUMB: Color = Color::srgb(0.35, 0.75, 0.35);
    const CHECKBOX_OUTLINE: Color = Color::srgb(0.45, 0.45, 0.45);
    const CHECKBOX_CHECK: Color = Color::srgb(0.35, 0.75, 0.35);

    fn checkbox_on_interaction<E: EntityEvent, C: Component>(
        event: On<E, C>,
        checkboxes: Query<
            (
                Option<&Hovered>,
                Has<InteractionDisabled>,
                Has<Checked>,
                &Children,
            ),
            With<Checkbox>,
        >,
        mut borders: Query<(&mut BorderColor, &mut Children), Without<Checkbox>>,
        mut marks: Query<&mut BackgroundColor, (Without<Checkbox>, Without<Children>)>,
    ) {
        if let Ok((hovered, disabled, checked, children)) = checkboxes.get(event.event_target()) {
            let hovered = hovered.map(|h| h.get()).unwrap_or_default();
            // These "removal event checks" exist because the `Remove` event is triggered _before_
            // the component is actually removed, meaning it still shows up in the
            // query. We're investigating the best way to improve this scenario.
            let checked = checked && !(E::is::<Remove>() && C::is::<Checked>());
            let disabled = disabled && !(E::is::<Remove>() && C::is::<InteractionDisabled>());

            // let Some(border_id) = children.first() else {
            //     return;
            // };

            let Some(border_id) = children.get(1) else {
                return;
            };

            let Ok((mut border_color, border_children)) = borders.get_mut(*border_id) else {
                return;
            };

            let Some(mark_id) = border_children.first() else {
                warn!("Checkbox does not have a mark entity.");
                return;
            };

            let Ok(mut mark_bg) = marks.get_mut(*mark_id) else {
                warn!("Checkbox mark entity lacking a background color.");
                return;
            };

            let color: Color = if disabled {
                // If the checkbox is disabled, use a lighter color
                CHECKBOX_OUTLINE.with_alpha(0.2)
            } else if hovered {
                // If hovering, use a lighter color
                CHECKBOX_OUTLINE.lighter(0.2)
            } else {
                // Default color for the checkbox
                CHECKBOX_OUTLINE
            };

            // Update the background color of the check mark
            border_color.set_all(color);

            let mark_color: Color = match (disabled, checked) {
                (true, true) => CHECKBOX_CHECK.with_alpha(0.5),
                (false, true) => CHECKBOX_CHECK,
                (_, false) => Srgba::NONE.into(),
            };

            if mark_bg.0 != mark_color {
                // Update the color of the check mark
                mark_bg.0 = mark_color;
            }
        }
    }
}
