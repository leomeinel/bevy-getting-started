/*
 * File: getting_started_plugins.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Plugins
//!
//! Source: https://bevy.org/learn/quick-start/getting-started/plugins/

use crate::getting_started_ecs;
use bevy::prelude::*;

pub struct HelloPlugin;

impl Plugin for HelloPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, getting_started_ecs::add_people);
        app.add_systems(
            Update,
            (
                getting_started_ecs::print_hello_world,
                (
                    getting_started_ecs::update_people,
                    getting_started_ecs::greet_people,
                )
                    .chain(),
            ),
        );
    }
}
