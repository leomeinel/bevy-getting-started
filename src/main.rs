/*
 * File: main.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use bevy::{input_focus::InputDispatchPlugin, prelude::*};
use bevy_simple_text_input::TextInputPlugin;

mod greet;
mod ui;

/// Main function
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputDispatchPlugin)
        .add_plugins(TextInputPlugin)
        .add_plugins(greet::GreetPlugin)
        .run();
}
