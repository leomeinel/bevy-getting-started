/*
 * File: greet.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! ECS
//!
//! ### Sources
//! - https://bevy.org/learn/quick-start/getting-started/plugins/
//! - https://bevy.org/learn/quick-start/getting-started/ecs/
//! - https://bevy.org/learn/quick-start/getting-started/resources/

use bevy::prelude::*;

use crate::ui::text_input::{self, SubmitOutput};

/// [`Plugin`] that prints greeting messages
pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
    /// Insert needed resources and add systems
    fn build(&self, app: &mut App) {
        // Insert resources
        app.insert_resource(GreetTimer(Timer::from_seconds(10.0, TimerMode::Repeating)));

        // Add messages
        app.add_message::<SubmitOutput>();

        // Add startup systems
        app.add_systems(Startup, print_hello_world);
        app.add_systems(Startup, text_input::setup);

        // Add update systems
        app.add_systems(Update, text_input::on_submit_text);
        app.add_systems(
            Update,
            text_input::update_outline_color.after(text_input::on_submit_text),
        );
        app.add_systems(Update, on_submit_output);
        app.add_systems(Update, greet_characters.after(on_submit_output));
    }
}

/// Add objects of type [`Character`] from [`SubmitOutput`]
///
/// This adds a bundle of [`Character`] and [`Name`] from [`SubmitOutput`]
fn on_submit_output(mut messages: MessageReader<SubmitOutput>, mut commands: Commands) {
    for message in messages.read() {
        let text = message.text.clone();
        commands.spawn((Character, Name(text)));
    }
}

/// Character
#[derive(Component)]
struct Character;

/// Name
#[derive(Component)]
struct Name(String);

/// [`Timer`] that controls the delay between greeting messages
#[derive(Resource)]
struct GreetTimer(pub Timer);

/// Greet characters
///
/// This prints a greeting message for each [`Character`], greeting them with the [`Name`] they are bundled with
fn greet_characters(
    query: Query<&Name, With<Character>>,
    mut timer: ResMut<GreetTimer>,
    time: Res<Time>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello {}", name.0);
        }
    }
}

/// Print Hello World
fn print_hello_world() {
    println!("Hello World");
}

// TODO: Accumulate added characters as text boxes and add ability to rename each of them.
//       This should use a scrollable list.
