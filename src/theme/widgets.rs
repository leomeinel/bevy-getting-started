/*
 * File: widgets.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Widgets

pub(crate) mod grid;
pub(crate) mod text_input;

use bevy::prelude::*;

/// Plugin
pub(crate) fn plugin(app: &mut App) {
    // Add plugins
    app.add_plugins((grid::plugin, text_input::plugin));
}
