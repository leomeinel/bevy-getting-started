/*
 * File: npc.rs
 * Author: Leopold Johannes Meinel (leo@meinel.dev)
 * -----
 * Copyright (c) 2025 Leopold Johannes Meinel & contributors
 * SPDX ID: Apache-2.0
 * URL: https://www.apache.org/licenses/LICENSE-2.0
 */

//! Npc handler
//!
//! Heavily inspired by: https://bevy.org/learn/quick-start/getting-started

use bevy::{color::palettes::tailwind, platform::collections::HashMap, prelude::*};
use bevy_ui_text_input::{TextInputFilter, TextInputMode, TextInputNode, TextInputPrompt};

use crate::ui::{
    grid::{self, GridNodeMarker0, GridNodeMarker1},
    text_input::{self, TextInputError, TextInputSuccess},
};

/// Plugin
pub(super) fn plugin(app: &mut App) {
    // Insert resources
    app.insert_resource(TextInputMap::default());

    // Add startup systems
    app.add_systems(Startup, setup.after(grid::setup));

    // Add update systems
    app.add_systems(Update, (create_npc_on_input, create_npc_text_inputs));
}

/// Npc
#[derive(Component)]
struct Npc;

/// Name
#[derive(Component)]
struct Name(String);

/// Text input map
///
/// This contains any text input that is mapped to an [`Npc`].
/// These text inputs are meant as a way to change the [`Name`] of an [`Npc`].
#[derive(Resource, Default)]
struct TextInputMap(HashMap<Entity, Entity>);

/// Spawn text input for creating a new [`Npc`]
fn setup(
    grid_node_single: Single<Entity, With<GridNodeMarker0>>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let grid_entity = grid_node_single.entity();

    commands.entity(grid_entity).with_children(|commands| {
        commands
            .spawn(text_input(&assets, "Create Npc"))
            .insert(input_filter());
    });
}

/// Add objects of type [`Npc`] from [`TextInputSuccess`]
///
/// This adds a bundle of [`Npc`] and [`Name`] from [`TextInputSuccess`]
fn create_npc_on_input(
    mut messages: MessageReader<TextInputSuccess>,
    mut error_writer: MessageWriter<TextInputError>,
    npc_query: Query<&Name, With<Npc>>,
    mut commands: Commands,
) {
    for message in messages.read() {
        let text = message.text.clone();
        // Exit early if a character with the same name already exists
        for name in &npc_query {
            if name.0 == text {
                error_writer.write(TextInputError {
                    entity: message.entity,
                });
                return;
            }
        }
        commands.spawn((Npc, Name(text.clone())));
    }
}

/// Create the text inputs for renaming every [`Npc`]
fn create_npc_text_inputs(
    grid_node_single: Single<Entity, With<GridNodeMarker1>>,
    npc_query: Query<(Entity, &Name), With<Npc>>,
    mut commands: Commands,
    mut map: ResMut<TextInputMap>,
    assets: Res<AssetServer>,
) {
    for (npc_entity, name) in &npc_query {
        // Continue if TextInputs already contains Name
        if map.0.contains_key(&npc_entity) {
            continue;
        }

        let grid_entity = grid_node_single.entity();

        let prompt = format!("Rename {}", name.0);
        commands.entity(grid_entity).with_children(|commands| {
            let entity = commands
                .spawn(text_input(&assets, prompt.as_str()))
                .insert(input_filter())
                .id();
            map.0.insert(npc_entity, entity);
        });
    }
}

/// [`Bundle`] containing input [`Node`]
fn text_input(assets: &Res<AssetServer>, prompt: &str) -> impl Bundle {
    (
        TextInputNode {
            mode: TextInputMode::SingleLine,
            max_chars: Some(20),
            ..default()
        },
        TextFont {
            font: assets.load("fonts/Fira_Mono/FiraMono-Medium.ttf"),
            font_size: 20.,
            ..default()
        },
        TextInputPrompt::new(prompt),
        TextColor(tailwind::NEUTRAL_100.into()),
        Node {
            width: Val::Px(300.0),
            height: Val::Px(30.0),
            margin: UiRect::all(Val::Px(10.)),
            ..default()
        },
        BackgroundColor(tailwind::NEUTRAL_800.into()),
        Outline {
            width: Val::Px(2.0),
            offset: Val::Px(2.0),
            color: text_input::OUTLINE_COLOR_INACTIVE.into(),
        },
    )
}

/// Input filter
///
/// This filters for anything that is alphanumeric or whitespace.
fn input_filter() -> TextInputFilter {
    TextInputFilter::custom(is_alphanumeric_or_whitespace)
}

/// Check if text is alphanumeric or whitespace
fn is_alphanumeric_or_whitespace(text: &str) -> bool {
    text.chars()
        .all(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace())
}

// TODO: Accumulate added npcs as text boxes and add ability to rename each of them.
//       This should use a scrollable list.
