/*
 * File: text_input.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Text input widget
//!
//! Heavily inspired by: https://github.com/ickshonpe/bevy_ui_text_input/blob/master/examples/multiple_inputs.rs

use bevy::{color::palettes::tailwind, input_focus::InputFocus, prelude::*};
use bevy_ui_text_input::SubmitText;

/// Plugin
pub(super) fn plugin(app: &mut App) {
    // Add messages
    app.add_message::<InputError>();
    app.add_message::<InputSuccess>();

    // Add update systems
    app.add_systems(Update, (on_submit, focus, on_error, on_success));
}

pub(crate) const OUTLINE_COLOR_ACTIVE: Srgba = tailwind::CYAN_500;
pub(crate) const OUTLINE_COLOR_ERROR: Srgba = tailwind::RED_500;
pub(crate) const OUTLINE_COLOR_INACTIVE: Srgba = tailwind::CYAN_100;

/// Message for unsuccessful input submission
#[derive(Message)]
pub(crate) struct InputError(pub(crate) Entity);

/// Message for successful input submission
#[derive(Message)]
pub(crate) struct InputSuccess {
    /// Entity of successful input
    pub(crate) entity: Entity,
    /// Text from input submission
    pub(crate) text: String,
}

/// Read messages of type [`SubmitText`]
///
/// This also writes a [`Message`] [`InputSuccess`] on successful input submission or a [`InputError`] on error.
fn on_submit(
    mut msgs: MessageReader<SubmitText>,
    mut error_msg: MessageWriter<InputError>,
    mut success_msg: MessageWriter<InputSuccess>,
) {
    for msg in msgs.read() {
        let entity = msg.entity;
        let text = msg.text.trim();

        if text.is_empty() {
            // Write InputError
            error_msg.write(InputError(entity));
            continue;
        }

        // Write InputSuccess
        success_msg.write(InputSuccess {
            entity,
            text: text.to_string(),
        });
    }
}

/// Update outline color based on focus
fn focus(mut outline_q: Query<(Entity, &mut Outline)>, input_focus: Res<InputFocus>) {
    // Return if input focus has not changed
    if !input_focus.is_changed() {
        return;
    }

    // Change outline color based on focus and input
    for (entity, mut outline) in outline_q.iter_mut() {
        if input_focus.0.is_some_and(|active| active == entity) {
            outline.color = OUTLINE_COLOR_ACTIVE.into();
        } else {
            outline.color = OUTLINE_COLOR_INACTIVE.into();
        }
    }
}

/// Update outline color based on input error
fn on_error(mut msgs: MessageReader<InputError>, mut outline_q: Query<(Entity, &mut Outline)>) {
    for msg in msgs.read() {
        for (outline_e, mut outline) in outline_q.iter_mut() {
            // Continue if the entity of msg does not match entity
            if msg.0 != outline_e {
                continue;
            }

            outline.color = OUTLINE_COLOR_ERROR.into();
        }
    }
}

/// Update outline color based on input success
fn on_success(mut msgs: MessageReader<InputSuccess>, mut outline_q: Query<(Entity, &mut Outline)>) {
    for msg in msgs.read() {
        for (outline_e, mut outline) in outline_q.iter_mut() {
            // Continue if the entity of msg does not match entity
            if msg.entity != outline_e {
                continue;
            }

            outline.color = OUTLINE_COLOR_ACTIVE.into();
        }
    }
}
