/*
 * File: text_input.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Text input [`Node`]
//!
//! Heavily inspired by: https://github.com/ickshonpe/bevy_ui_text_input/blob/master/examples/multiple_inputs.rs

use bevy::{
    color::palettes::tailwind, input_focus::InputFocus, platform::collections::HashMap, prelude::*,
};
use bevy_ui_text_input::{
    SubmitText, TextInputFilter, TextInputMode, TextInputNode, TextInputPrompt,
};

/// Plugin
pub(super) fn plugin(app: &mut App) {
    // Add messages
    app.add_message::<SubmitOutput>();

    // Add startup systems
    app.add_systems(Startup, setup);

    // Add update systems
    app.add_systems(Update, (on_submit_text, update_outline_color).chain());
}

const OUTLINE_COLOR_INACTIVE: Srgba = tailwind::CYAN_100;

/// Map of input entities to output entities
#[derive(Resource, Deref, DerefMut, Default)]
struct InputMap(HashMap<Entity, Entity>);

/// Component that stores the output computed from an input submission
#[derive(Component, Default)]
struct Output {
    /// Text from input submission
    text: String,
}

/// Message that gets written on successful input submission
#[derive(Message)]
pub(crate) struct SubmitOutput {
    /// Text from input submission
    pub(crate) text: String,
}

/// Marker for empty inputs
#[derive(Component)]
struct EmptyInputMarker;

/// Setup ui and [`InputMap`]
fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let mut map = InputMap::default();
    let filters: [(Option<TextInputFilter>, &str); 1] = [(None, "Create Npc")];

    // Spawn parent node containing a child node with a grid. That grid also has child nodes containing the input.
    commands
        .spawn(parent_node_bundle())
        .with_children(|commands| {
            commands
                .spawn(grid_node_bundle())
                .with_children(|commands| {
                    // Spawn a child node for every filter
                    for (filter, prompt) in filters {
                        // Spawn input node and insert filter if it is Some
                        let mut input = commands.spawn(input_node_bundle(&assets, prompt));
                        if let Some(filter) = filter {
                            input.insert(filter);
                        }

                        // Insert into InputMap
                        let input_entity = input.id();
                        let output_entity = commands.spawn(Output::default()).id();
                        map.insert(input_entity, output_entity);
                    }
                });
        });
    // Insert map as resource
    commands.insert_resource(map);
}

/// Read messages of type [`SubmitText`]
///
/// This also writes a [`Message`] [`SubmitOutput`] on successful input submission
fn on_submit_text(
    mut output_query: Query<&mut Output>,
    mut commands: Commands,
    mut messages: MessageReader<SubmitText>,
    mut message_writer: MessageWriter<SubmitOutput>,
    map: Res<InputMap>,
) {
    for message in messages.read() {
        if let Some(&output_entity) = map.0.get(&message.entity) {
            let text = message.text.trim();

            // Exit early if text is empty and insert EmptyInputMarker
            if text.is_empty() {
                commands.entity(output_entity).insert(EmptyInputMarker);
                continue;
            }

            // Remove EmptyInputMarker
            commands.entity(output_entity).remove::<EmptyInputMarker>();

            // Set Output text and write message
            output_query.get_mut(output_entity).unwrap().text = text.to_string();
            message_writer.write(SubmitOutput {
                text: text.to_string(),
            });
        }
    }
}

/// Update outline color based on focus and input
fn update_outline_color(
    mut outline_query: Query<(Entity, &mut Outline)>,
    output_query: Query<Entity, With<Output>>,
    empty_query: Query<(), With<EmptyInputMarker>>,
    input_focus: Res<InputFocus>,
) {
    const OUTLINE_COLOR_ERROR: Srgba = tailwind::RED_500;
    const OUTLINE_COLOR_ACTIVE: Srgba = tailwind::CYAN_500;

    let mut is_empty_input = false;

    // Determine whether any empty input has been submitted
    for output_entity in output_query {
        if empty_query.contains(output_entity) {
            is_empty_input = empty_query.contains(output_entity);
            break;
        }
    }

    // Change outline color based on focus and input
    for (entity, mut outline) in outline_query.iter_mut() {
        let is_focused = input_focus.0.is_some_and(|active| active == entity);
        if is_focused && is_empty_input {
            outline.color = OUTLINE_COLOR_ERROR.into();
        } else if is_focused {
            outline.color = OUTLINE_COLOR_ACTIVE.into();
        } else {
            outline.color = OUTLINE_COLOR_INACTIVE.into();
        }
    }
}

/// [`Bundle`] containing parent [`Node`]
fn parent_node_bundle() -> impl Bundle {
    Node {
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Column,
        ..default()
    }
}

/// [`Bundle`] containing grid [`Node`]
fn grid_node_bundle() -> impl Bundle {
    const COLUMN_SIZE: f32 = 300.;
    const COLUMN_GAP: Val = Val::Px(20.);

    Node {
        display: Display::Grid,
        grid_template_columns: vec![GridTrack::auto(), GridTrack::px(COLUMN_SIZE)],
        column_gap: COLUMN_GAP,
        row_gap: COLUMN_GAP,
        ..default()
    }
}

/// [`Bundle`] containing input [`Node`]
fn input_node_bundle(assets: &Res<AssetServer>, prompt: &str) -> impl Bundle {
    const MAX_CHARS: Option<usize> = Some(20);
    const FONT_PATH: &str = "fonts/Fira_Mono/FiraMono-Medium.ttf";
    const FONT_SIZE: f32 = 20.;
    const TEXT_COLOR: Srgba = tailwind::NEUTRAL_100;
    const BACKGROUND_COLOR: Srgba = tailwind::NEUTRAL_800;
    const WIDTH: Val = Val::Px(300.0);
    const HEIGHT: Val = Val::Px(30.0);
    const OUTLINE_WIDTH: Val = Val::Px(2.0);

    (
        TextInputNode {
            mode: TextInputMode::SingleLine,
            max_chars: MAX_CHARS,
            ..default()
        },
        TextFont {
            font: assets.load(FONT_PATH),
            font_size: FONT_SIZE,
            ..default()
        },
        TextInputPrompt::new(prompt),
        TextColor(TEXT_COLOR.into()),
        Node {
            width: WIDTH,
            height: HEIGHT,
            ..default()
        },
        BackgroundColor(BACKGROUND_COLOR.into()),
        Outline {
            width: OUTLINE_WIDTH,
            offset: OUTLINE_WIDTH,
            color: OUTLINE_COLOR_INACTIVE.into(),
        },
    )
}
