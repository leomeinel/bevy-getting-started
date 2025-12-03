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

/// Marker component for the first grid.
///
/// Should only ever exist once.
#[derive(Component)]
pub(crate) struct GridMarker0;

/// Marker component for the second grid.
///
/// Should only ever exist once.
#[derive(Component)]
pub(crate) struct GridMarker1;

/// Spawn a parent [`Bundle`], a bottom [`grid`] and a top [`grid`]
pub(crate) fn setup(mut commands: Commands) {
    // Spawn grid_container bundle containing a child bundle with a grid.
    commands.spawn(grid_container()).with_children(|commands| {
        commands.spawn((grid(), GridMarker0));
        commands.spawn((grid(), GridMarker1));
    });
}

/// [`Bundle`] containing grid container
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

/// [`Bundle`] containing grid
fn grid() -> impl Bundle {
    Node {
        display: Display::Grid,
        grid_template_columns: RepeatedGridTrack::px(2, 200.),
        column_gap: Val::Px(20.),
        row_gap: Val::Px(20.),
        margin: UiRect::all(Val::Px(60.)),
        ..default()
    }
}
