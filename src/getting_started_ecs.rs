/*
 * File: getting_started_ecs.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! ECS
//!
//! This also uses the [`Resource`] object
//!
//! ### Sources
//! - https://bevy.org/learn/quick-start/getting-started/ecs/
//! - https://bevy.org/learn/quick-start/getting-started/resources/

use bevy::{
    ecs::{
        component::Component,
        query::With,
        resource::Resource,
        system::{Commands, Query, Res, ResMut},
    },
    time::{Time, Timer},
};

/// Person
#[derive(Component)]
pub struct Person;

/// Name
#[derive(Component)]
pub struct Name(String);

/// [`Timer`] that controls the delay between greeting messages
#[derive(Resource)]
pub struct GreetTimer(pub Timer);

/// Add people
///
/// This adds multiple objects of type [`Person`] with a [`Name`] to the [`World`]
pub fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Elaina Proctor".to_string())));
    commands.spawn((Person, Name("Renzo Hume".to_string())));
    commands.spawn((Person, Name("Zayna Nieves".to_string())));
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

/// Greet people
///
/// This prints a greeting message for each [`Person`], greeting them with their [`Name`]
pub fn greet_people(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    query: Query<&Name, With<Person>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("Hello {}", name.0);
        }
    }
}

/// Print Hello World
pub fn print_hello_world() {
    println!("Hello World");
}
