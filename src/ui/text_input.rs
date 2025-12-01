/*
 * File: text_input.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Text input [`Node`] with embedded handling of the input
//!
//! Heavily inspired by: https://github.com/rparrett/bevy_simple_text_input/blob/main/examples/focus.rs

use bevy::{color::palettes::tailwind, input_focus::InputFocus, prelude::*};
use bevy_simple_text_input::{
    TextInput, TextInputInactive, TextInputPlaceholder, TextInputSubmitMessage, TextInputTextColor,
    TextInputTextFont,
};

use crate::greet;

// Colors
const BORDER_COLOR_ACTIVE: Srgba = tailwind::CYAN_500;
const BORDER_COLOR_INACTIVE: Srgba = tailwind::CYAN_100;
const BORDER_COLOR_ERROR: Srgba = tailwind::RED_500;
const TEXT_COLOR: Srgba = tailwind::NEUTRAL_100;
const BACKGROUND_COLOR: Srgba = tailwind::NEUTRAL_800;

// Parent node
const PARENT_NODE_ROW_GAP: Val = Val::Px(10.);

// Input
const INPUT_NODE_WIDTH: Val = Val::Px(400.0);
const INPUT_NODE_BORDER: UiRect = UiRect::all(Val::Px(2.0));
const INPUT_NODE_PADDING: UiRect = INPUT_NODE_BORDER;
const INPUT_FONT_SIZE: f32 = 20.;
const INPUT_PLACEHOLDER: &str = "Enter name for person";

/// Cache that stores the associated entity
#[derive(Resource, Default)]
pub struct InputCache {
    entity: Option<Entity>,
}

/// Stores whether an error has occurred while processing input
#[derive(Component)]
pub struct InputError;

/// Spawn ui with parent [`Node`] and child input [`Node`]
pub fn spawn_ui(mut commands: Commands, mut cache: ResMut<InputCache>) {
    // Spawn Camera2d to show input node
    commands.spawn(Camera2d);

    // Spawn parent node that fills 100% of screen with child input node
    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            row_gap: PARENT_NODE_ROW_GAP,
            ..default()
        })
        .with_children(|parent| {
            let entity = parent
                .spawn(input_node_bundle())
                .observe(on_input_node_click)
                .id();
            cache.entity = Some(entity);
        })
        .observe(on_background_click);
}

/// Listener that adds a bundle of [`Person`] and [`Name`] to [`World`]
pub fn on_name_input(
    mut commands: Commands,
    mut events: MessageReader<TextInputSubmitMessage>,
    cache: Res<InputCache>,
) {
    // Assign entity
    let Some(entity) = cache.entity else {
        return;
    };

    for event in events.read() {
        let name = event.value.trim().to_string();

        // Exit early if name is empty and set InputError
        if name.is_empty() {
            commands.entity(entity).insert(InputError);
            continue;
        }

        // Remove InputError
        commands.entity(entity).remove::<InputError>();

        // Add a person to world with name on each submitted message
        commands.queue(|world: &mut World| {
            greet::add_person(world, name);
        });
    }
}

/// Update border
pub fn border_update(
    cache: Res<InputCache>,
    focus: Res<InputFocus>,
    input_error_query: Query<(), With<InputError>>,
    mut entity_query: Query<(&mut BorderColor, &mut TextInputInactive)>,
) {
    // Assign entity
    let Some(entity) = cache.entity else {
        return;
    };
    let Ok((mut border_color, mut inactive)) = entity_query.get_mut(entity) else {
        return;
    };
    let has_input_error = input_error_query.contains(entity);
    let is_focused = focus.0 == Some(entity);

    if has_input_error && is_focused {
        // Focus
        inactive.0 = false;
        // Set border_color to error
        *border_color = BORDER_COLOR_ERROR.into();
    } else if is_focused {
        // Focus
        inactive.0 = false;
        *border_color = BORDER_COLOR_ACTIVE.into();
    } else {
        // Unfocus
        inactive.0 = true;
        *border_color = BORDER_COLOR_INACTIVE.into();
    }
}

/// Bundle containing input [`Node`] and additional attributes
fn input_node_bundle() -> impl Bundle {
    (
        Node {
            width: INPUT_NODE_WIDTH,
            border: INPUT_NODE_BORDER,
            padding: INPUT_NODE_PADDING,
            ..default()
        },
        BorderColor::all(BORDER_COLOR_INACTIVE),
        BackgroundColor(BACKGROUND_COLOR.into()),
        TextInput,
        TextInputTextFont(TextFont {
            font_size: INPUT_FONT_SIZE,
            ..default()
        }),
        TextInputTextColor(TextColor(TEXT_COLOR.into())),
        TextInputPlaceholder {
            value: INPUT_PLACEHOLDER.to_string(),
            ..default()
        },
        TextInputInactive(true),
    )
}

/// Handle background [`Node`] click
fn on_background_click(mut trigger: On<Pointer<Click>>, mut focus: ResMut<InputFocus>) {
    // Disable focus
    focus.0 = None;
    trigger.propagate(false);
}

/// Handle input [`Node`] click
fn on_input_node_click(mut trigger: On<Pointer<Click>>, mut focus: ResMut<InputFocus>) {
    // Enable focus
    focus.0 = Some(trigger.event_target());
    trigger.propagate(false);
}
