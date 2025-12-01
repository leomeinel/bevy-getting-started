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
use bevy_simple_text_input::TextInputSystem;

use crate::ui::text_input::{self, InputCache};

/// [`Plugin`] that prints greeting messages
pub struct GreetPlugin;

impl Plugin for GreetPlugin {
    /// Insert needed resources and add systems
    fn build(&self, app: &mut App) {
        // Insert resources
        app.insert_resource(GreetTimer(Timer::from_seconds(10.0, TimerMode::Repeating)));
        app.insert_resource(InputCache::default());

        // Add startup systems
        app.add_systems(Startup, print_hello_world);
        app.add_systems(Startup, text_input::spawn_ui);

        // Add update systems
        app.add_systems(Update, text_input::on_name_input.after(TextInputSystem));
        app.add_systems(
            Update,
            text_input::border_update.after(text_input::on_name_input),
        );
        app.add_systems(Update, greet_people.after(text_input::on_name_input));
    }
}

/// Add a [`Person`]
///
/// This adds a [`Person`] to [`World`] with [`Name`]
pub fn add_person(world: &mut World, name: String) {
    world.spawn((Person, Name(name)));
}

/// Person
#[derive(Component)]
struct Person;

/// Name
#[derive(Component)]
struct Name(String);

/// [`Timer`] that controls the delay between greeting messages
#[derive(Resource)]
struct GreetTimer(pub Timer);

/// Greet people
///
/// This prints a greeting message for each [`Person`], greeting them with their [`Name`]
fn greet_people(query: Query<&Name, With<Person>>, mut timer: ResMut<GreetTimer>, time: Res<Time>) {
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

// TODO: Accumulate added persons as text boxes and add ability to rename each of them.
//       This should use a scrollable list.
