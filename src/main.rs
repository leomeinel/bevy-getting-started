/*
 * File: main.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

mod getting_started_app;

use bevy::prelude::*;

/**
 * Main function
 */
fn main() {
    // https://docs.rs/bevy/latest/bevy/app/struct.App.html
    App::new()
        .add_systems(Update, getting_started_app::print_hello_world)
        .run();
}
