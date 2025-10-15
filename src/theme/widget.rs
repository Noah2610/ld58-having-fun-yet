//! Helper functions for creating common widgets.

use crate::theme::{
    interaction::{InteractionPalette, checkbox::CheckedDefault},
    palette::*,
};
pub use bevy::ui_widgets::*;
use bevy::{
    ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    input_focus::tab_navigation::{TabGroup, TabIndex},
    prelude::*,
};
use std::borrow::Cow;

/// A root UI node that fills the window and centers its content.
pub fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(24),
            ..default()
        },
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
        TabGroup::default(),
    )
}

pub fn h1(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Header"),
        Node {
            padding: UiRect::vertical(px(24)),
            ..default()
        },
        Text(text.into()),
        TextFont::from_font_size(60.0),
        TextColor(HEADER_TEXT),
    )
}

pub fn h2(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Header"),
        Text(text.into()),
        TextFont::from_font_size(40.0),
        TextColor(HEADER_TEXT),
    )
}

pub fn h3(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Header"),
        Text(text.into()),
        TextFont::from_font_size(32.0),
        TextColor(HEADER_TEXT),
    )
}

/// A simple text label.
pub fn label(text: impl Into<String>) -> impl Bundle {
    let text: String = text.into();
    (
        Name::new(format!("{} Label", text.as_str())),
        Text(text),
        TextFont::from_font_size(24.0),
        TextColor(LABEL_TEXT),
    )
}

/// A large rounded button with text and an action defined as an [`Observer`].
pub fn button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        action,
        (
            Node {
                width: px(380),
                height: px(64),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderRadius::all(px(8)),
        ),
    )
}

/// A small square button with text and an action defined as an [`Observer`].
pub fn button_small<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(text, action, Node {
        width: px(30),
        height: px(30),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..default()
    })
}

/// A simple button with text and an action defined as an [`Observer`]. The
/// button's layout is provided by `button_bundle`.
fn button_base<E, B, M, I>(
    text: impl Into<String>,
    action: I,
    button_bundle: impl Bundle,
) -> impl Bundle
where
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let text = text.into();
    let action = IntoObserverSystem::into_system(action);
    (
        Name::new(format!("{} Button", text.as_str())),
        Node::default(),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Name::new("Button Inner"),
                    bevy::prelude::Button,
                    BackgroundColor(BUTTON_BACKGROUND),
                    InteractionPalette {
                        none:    BUTTON_BACKGROUND,
                        hovered: BUTTON_HOVERED_BACKGROUND,
                        pressed: BUTTON_PRESSED_BACKGROUND,
                    },
                    children![(
                        Name::new("Button Text"),
                        Text(text),
                        TextFont::from_font_size(24.0),
                        TextColor(BUTTON_TEXT),
                        // Don't bubble picking events from the text up to the button.
                        Pickable::IGNORE,
                    )],
                ))
                .insert(button_bundle)
                .observe(action);
        })),
    )
}

pub fn checkbox<C, E, B, M, I>(
    marker: C,
    caption: impl Into<String>,
    checked: bool,
    action: I,
) -> impl Bundle
where
    C: Component,
    E: EntityEvent,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    const CHECKBOX_OUTLINE: Color = Color::srgb(0.45, 0.45, 0.45);

    let caption: String = caption.into();
    let action = IntoObserverSystem::into_system(action);

    (
        Name::new(format!("{} Checkbox", caption.as_str())),
        marker,
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            align_content: AlignContent::Center,
            column_gap: px(4),
            ..default()
        },
        InteractionPalette {
            none:    BUTTON_BACKGROUND,
            hovered: BUTTON_HOVERED_BACKGROUND,
            pressed: BUTTON_PRESSED_BACKGROUND,
        },
        // Hovered::default(),
        Checkbox,
        CheckedDefault(checked),
        TabIndex(0),
        Children::spawn((
            Spawn((label(caption), self_end())),
            SpawnWith(|parent: &mut ChildSpawner| {
                parent.spawn((
                    // Checkbox outer
                    Node {
                        display: Display::Flex,
                        width: px(24),
                        height: px(24),
                        border: UiRect::all(px(2)),
                        justify_self: JustifySelf::Start,
                        ..default()
                    },
                    BorderColor::all(CHECKBOX_OUTLINE), // Border color for the checkbox
                    BorderRadius::all(px(2)),
                    children![
                        // Checkbox inner
                        (
                            Node {
                                display: Display::Flex,
                                width: px(12),
                                height: px(12),
                                position_type: PositionType::Absolute,
                                left: px(4),
                                top: px(4),
                                ..default()
                            },
                            BackgroundColor(Srgba::NONE.into()),
                        ),
                    ],
                ));
                // .observe(checkbox_self_update);
                // .observe(action);
            }),
        )),
        observe(checkbox_self_update),
        observe(action),
    )

    // (
    //     // Node {
    //     //     width: px(64),
    //     //     height: px(64),
    //     //     ..default()
    //     // },
    //     BackgroundColor(Color::BLACK),
    //     ui_widgets::Checkbox,
    //     Pickable::default(),
    // )
}

// fn if_component<C: Component>(enabled: bool, comp: C) -> impl Bundle {
//     if enabled { comp } else { () }
// }

/// Bundle for a vertical flex layout
pub fn settings_list() -> impl Bundle {
    Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        row_gap: px(24),
        ..default()
    }
}

/// Bundle for a 2-column grid layout
pub fn settings_grid_2x() -> impl Bundle {
    Node {
        display: Display::Grid,
        row_gap: px(8),
        column_gap: px(24),
        grid_template_columns: RepeatedGridTrack::px(2, 600.0),
        ..default()
    }
}

#[inline]
pub fn self_start() -> impl Bundle {
    Node {
        justify_self: JustifySelf::Start,
        ..default()
    }
}

#[inline]
pub fn self_end() -> impl Bundle {
    Node {
        justify_self: JustifySelf::End,
        ..default()
    }
}

pub fn analog_slider<
    C: Component,
    LS: IntoObserverSystem<E, B, M>,
    RS: IntoObserverSystem<E, B, M>,
    E: EntityEvent,
    B: Bundle,
    M,
>(
    marker: C,
    on_decrease: LS,
    on_increase: RS,
) -> impl Bundle {
    (self_start(), children![
        button_small("-", on_decrease),
        (
            Node {
                padding: UiRect::horizontal(px(10)),
                justify_content: JustifyContent::Center,
                ..default()
            },
            children![(label(""), marker)],
        ),
        button_small("+", on_increase),
    ])
}
