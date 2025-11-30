/*
 * File: getting_started_ecs.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! ECS examples
//!
//! Source: https://bevy.org/learn/quick-start/getting-started/ecs/

use bevy::prelude::*;

/// Print Hello World
pub fn print_hello_world() {
    println!("Hello World");
}

/// Add people
///
/// This adds multiple objects of type [`Person`] with a [`Name`] to the [`World`]
pub fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
}

/// Greet people
///
/// This prints a welcome message for each [`Person`], greeting them with their [`Name`]
pub fn greet_people(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("Hello {}", name.0);
    }
}

/// Update people
///
/// This uses a [`Query`] to change the [`Name`] of a [`Person`]
pub fn update_people(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Elaina Proctor" {
            name.0 = "Elaina Hume".to_string();
            break; // We don't need to change any other names.
        }
    }
}

#[derive(Component)]
/// Person
pub struct Person;

#[derive(Component)]
/// Name
pub struct Name(String);
