/*
 * File: main.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

mod getting_started_ecs;

use bevy::prelude::*;

/// Main function
fn main() {
    App::new()
        .add_systems(Startup, getting_started_ecs::add_people)
        .add_systems(
            Update,
            (
                getting_started_ecs::print_hello_world,
                (
                    getting_started_ecs::update_people,
                    getting_started_ecs::greet_people,
                )
                    .chain(),
            ),
        )
        .run();
}
