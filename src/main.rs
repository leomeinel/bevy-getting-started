/*
 * File: main.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use bevy::{DefaultPlugins, app::App};

mod getting_started_ecs;
mod getting_started_plugins;

/// Main function
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(getting_started_plugins::GreetPlugin)
        .run();
}
