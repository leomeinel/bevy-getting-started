/*
 * File: text_input.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Text input
//!
//! Heavily inspired by: https://github.com/ickshonpe/bevy_ui_text_input/blob/master/examples/multiple_inputs.rs

use bevy::{color::palettes::tailwind, input_focus::InputFocus, prelude::*};
use bevy_ui_text_input::SubmitText;

/// Plugin
pub(super) fn plugin(app: &mut App) {
    // Add messages
    app.add_message::<TextInputError>();
    app.add_message::<TextInputSuccess>();

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
pub(crate) const OUTLINE_COLOR_ACTIVE: Srgba = tailwind::CYAN_500;
pub(crate) const OUTLINE_COLOR_ERROR: Srgba = tailwind::RED_500;
pub(crate) const OUTLINE_COLOR_INACTIVE: Srgba = tailwind::CYAN_100;

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
fn update_focus(mut query: Query<(Entity, &mut Outline)>, input_focus: Res<InputFocus>) {
    // Exit early if input focus has not changed
    if !input_focus.is_changed() {
        return;
    }

    // Change outline color based on focus and input
    for (entity, mut outline) in query.iter_mut() {
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
    mut query: Query<(Entity, &mut Outline)>,
) {
    for message in messages.read() {
        for (entity, mut outline) in query.iter_mut() {
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
    mut query: Query<(Entity, &mut Outline)>,
) {
    for message in messages.read() {
        for (entity, mut outline) in query.iter_mut() {
            // Continue if the entity of message does not match entity
            if message.entity != entity {
                continue;
            }

            outline.color = OUTLINE_COLOR_ACTIVE.into();
        }
    }
}
