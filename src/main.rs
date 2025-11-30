/*
 * File: main.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

use bevy::prelude::*;

fn main() {
    let hello_world = || println!("Hello World");
    // https://docs.rs/bevy/latest/bevy/app/struct.App.html
    App::new().add_systems(Update, hello_world).run();
}
