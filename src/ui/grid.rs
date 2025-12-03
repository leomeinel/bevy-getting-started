/*
 * File: grid.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Grid

use bevy::prelude::*;

/// Plugin
pub(super) fn plugin(app: &mut App) {
    // Add startup systems
    app.add_systems(Startup, setup);
}

/// Marker component for the first grid node.
///
/// Should only ever exist once.
#[derive(Component)]
pub struct GridNodeMarker0;

/// Marker component for the second grid node.
///
/// Should only ever exist once.
#[derive(Component)]
pub struct GridNodeMarker1;

/// Spawn a parent [`Bundle`], a bottom [`grid`] and a top [`grid`]
pub(crate) fn setup(mut commands: Commands) {
    // Spawn grid_container bundle containing a child bundle with a grid.
    commands.spawn(grid_container()).with_children(|commands| {
        commands.spawn((grid(), GridNodeMarker0));
        commands.spawn((grid(), GridNodeMarker1));
    });
}

/// [`Bundle`] containing parent [`Node`]
fn grid_container() -> impl Bundle {
    Node {
        display: Display::Flex,
        width: Val::Percent(100.),
        height: Val::Percent(100.),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        flex_direction: FlexDirection::Row,
        ..default()
    }
}

/// [`Bundle`] containing grid [`Node`]
fn grid() -> impl Bundle {
    Node {
        display: Display::Grid,
        width: Val::Px(300.0),
        margin: UiRect::all(Val::Px(20.)),
        ..default()
    }
}
