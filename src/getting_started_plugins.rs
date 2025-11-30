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

use bevy::{
    app::{App, Plugin, Startup, Update},
    ecs::schedule::IntoScheduleConfigs as _,
    time::{Timer, TimerMode},
};

use crate::getting_started_ecs::{self, GreetTimer};

/// [`Plugin`] that prints greeting messages
pub struct GreetPlugin;

impl Plugin for GreetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(10.0, TimerMode::Repeating)));
        app.add_systems(Startup, getting_started_ecs::add_people);
        app.add_systems(Startup, getting_started_ecs::print_hello_world);
        app.add_systems(
            Update,
            (
                getting_started_ecs::update_people,
                getting_started_ecs::greet_people,
            )
                .chain(),
        );
    }
}
