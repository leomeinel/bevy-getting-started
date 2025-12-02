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

use bevy::{color::palettes::tailwind, input_focus::InputFocus, prelude::*};
use bevy_ui_text_input::{
    SubmitText, TextInputFilter, TextInputMode, TextInputNode, TextInputPrompt,
};

/// Plugin
pub(super) fn plugin(app: &mut App) {
    // Add messages
    app.add_message::<TextInputError>();
    app.add_message::<TextInputSuccess>();

    // Add startup systems
    app.add_systems(Startup, setup);

    // Add update systems
    app.add_systems(
        Update,
        (
            on_submit_text,
            update_focus,
            update_on_error,
            update_on_success,
        ),
    );
}

// Outline colors
const OUTLINE_COLOR_ACTIVE: Srgba = tailwind::CYAN_500;
const OUTLINE_COLOR_ERROR: Srgba = tailwind::RED_500;
const OUTLINE_COLOR_INACTIVE: Srgba = tailwind::CYAN_100;

/// Message that gets written on successful input submission
#[derive(Message)]
pub(crate) struct TextInputError {
    pub(crate) entity: Entity,
}

/// Message that gets written on successful input submission
#[derive(Message)]
pub(crate) struct TextInputSuccess {
    pub(crate) entity: Entity,
    /// Text from input submission
    pub(crate) text: String,
}

/// Setup ui and [`InputVec`]
fn setup(mut commands: Commands, assets: Res<AssetServer>) {
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
                    }
                });
        });
}

/// Read messages of type [`SubmitText`]
///
/// This also writes a [`Message`] [`TextInputSuccess`] on successful input submission
fn on_submit_text(
    mut messages: MessageReader<SubmitText>,
    mut error_writer: MessageWriter<TextInputError>,
    mut success_writer: MessageWriter<TextInputSuccess>,
) {
    for message in messages.read() {
        let text = message.text.trim();

        if text.is_empty() {
            // Write TextInputError
            error_writer.write(TextInputError {
                entity: message.entity,
            });
            continue;
        }

        // Write TextInputSuccess
        success_writer.write(TextInputSuccess {
            entity: message.entity,
            text: text.to_string(),
        });
    }
}

/// Update outline color based on focus
fn update_focus(mut outline_query: Query<(Entity, &mut Outline)>, input_focus: Res<InputFocus>) {
    // Exit early if input focus has not changed
    if !input_focus.is_changed() {
        return;
    }

    // Change outline color based on focus and input
    for (entity, mut outline) in outline_query.iter_mut() {
        if input_focus.0.is_some_and(|active| active == entity) {
            outline.color = OUTLINE_COLOR_ACTIVE.into();
        } else {
            outline.color = OUTLINE_COLOR_INACTIVE.into();
        }
    }
}

/// Update outline color based on input error
fn update_on_error(
    mut messages: MessageReader<TextInputError>,
    mut outline_query: Query<(Entity, &mut Outline)>,
) {
    for message in messages.read() {
        for (entity, mut outline) in outline_query.iter_mut() {
            // Continue if the entity of message does not match entity
            if message.entity != entity {
                continue;
            }

            outline.color = OUTLINE_COLOR_ERROR.into();
        }
    }
}

/// Update outline color based on input success
fn update_on_success(
    mut messages: MessageReader<TextInputSuccess>,
    mut outline_query: Query<(Entity, &mut Outline)>,
) {
    for message in messages.read() {
        for (entity, mut outline) in outline_query.iter_mut() {
            // Continue if the entity of message does not match entity
            if message.entity != entity {
                continue;
            }

            outline.color = OUTLINE_COLOR_ACTIVE.into();
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
