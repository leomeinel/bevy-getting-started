/*
 * File: npc.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Npc handler
//!
//! Heavily inspired by: https://bevy.org/learn/quick-start/getting-started

use bevy::prelude::*;

use crate::ui::text_input::{TextInputError, TextInputSuccess};

/// Plugin
pub(super) fn plugin(app: &mut App) {
    // Insert resources
    app.insert_resource(GreetTimer(Timer::from_seconds(10.0, TimerMode::Repeating)));

    // Add startup systems
    app.add_systems(Startup, print_hello_world);

    // Add update systems
    app.add_systems(Update, (on_submit_output, greet_npcs).chain());
}

/// Npc
#[derive(Component)]
struct Npc;

/// Name
#[derive(Component)]
struct Name(String);

/// [`Timer`] that controls the delay between greeting messages
#[derive(Resource)]
struct GreetTimer(Timer);

/// Add objects of type [`Npc`] from [`TextInputSuccess`]
///
/// This adds a bundle of [`Npc`] and [`Name`] from [`TextInputSuccess`]
fn on_submit_output(
    npc_query: Query<&Name, With<Npc>>,
    mut messages: MessageReader<TextInputSuccess>,
    mut error_writer: MessageWriter<TextInputError>,
    mut commands: Commands,
) {
    for message in messages.read() {
        let text = message.text.clone();
        // Exit early if a character with the same name already exists
        for name in &npc_query {
            if name.0 == text {
                error_writer.write(TextInputError {
                    entity: message.entity,
                });
                return;
            }
        }
        commands.spawn((Npc, Name(text)));
    }
}

/// Greet npcs
///
/// This prints a greeting message for each [`Npc`], greeting them with the [`Name`] they are bundled with
fn greet_npcs(npc_query: Query<&Name, With<Npc>>, mut timer: ResMut<GreetTimer>, time: Res<Time>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &npc_query {
            println!("Hello {}", name.0);
        }
    }
}

/// Print Hello World
fn print_hello_world() {
    println!("Hello World");
}

// TODO: Accumulate added npcs as text boxes and add ability to rename each of them.
//       This should use a scrollable list.
