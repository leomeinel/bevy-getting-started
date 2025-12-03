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
    grid::{self, GridMarker0, GridMarker1},
    text_input::{self, TextInputError, TextInputSuccess},
};

/// Plugin
pub(super) fn plugin(app: &mut App) {
    // Insert resources
    app.insert_resource(NameInputMap::default());
    app.insert_resource(NameMap::default());

    // Add startup systems
    app.add_systems(Startup, setup.after(grid::setup));

    // Add update systems
    app.add_systems(
        Update,
        (
            create_npc_on_input,
            create_npc_rename_inputs,
            rename_npc_on_input,
        ),
    );
}

/// Npc
#[derive(Component)]
struct Npc;

/// Name
#[derive(Component)]
struct Name(String);

/// Marker component for any name input that is used for renaming
#[derive(Component)]
struct RenameInputMarker;

/// Name input map
///
/// It is structured like this:
/// k: [`Npc`] [`Entity`] -> v: 'name input [`Entity`]'
#[derive(Resource, Deref, Default)]
struct NameInputMap(HashMap<Entity, Entity>);

/// Name map
///
/// It is structured like this:
/// k: [`Npc`] [`Entity`] -> v: 'name [`Entity`]'
#[derive(Resource, Deref, Default)]
struct NameMap(HashMap<Entity, Entity>);

/// Spawn name input for creating a new [`Npc`]
fn setup(
    grid_single: Single<Entity, With<GridMarker0>>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let grid_entity = grid_single.entity();

    commands.entity(grid_entity).with_children(|commands| {
        commands.spawn(Text::new("Enter name"));
        commands
            .spawn(name_input(&assets, "Create Npc"))
            .insert(name_input_filter());
    });
}

/// Add objects of type [`Npc`] from [`TextInputSuccess`]
///
/// This adds a bundle of [`Npc`] and [`Name`] from [`TextInputSuccess`]
fn create_npc_on_input(
    mut messages: MessageReader<TextInputSuccess>,
    mut error_writer: MessageWriter<TextInputError>,
    npc_query: Query<&Name, With<Npc>>,
    rename_query: Query<Entity, With<RenameInputMarker>>,
    mut commands: Commands,
) {
    for message in messages.read() {
        // Continue if targeting a rename entity
        if rename_query.contains(message.entity) {
            continue;
        }

        let name = message.text.clone();

        // Continue if an Npc with the same name already exists
        if npc_query.iter().any(|npc_name| npc_name.0 == name) {
            error_writer.write(TextInputError {
                entity: message.entity,
            });
            continue;
        }

        commands.spawn((Npc, Name(name)));
    }
}

/// Renames objects of type [`Npc`] from [`TextInputSuccess`]
fn rename_npc_on_input(
    mut messages: MessageReader<TextInputSuccess>,
    mut error_writer: MessageWriter<TextInputError>,
    mut npc_query: Query<(Entity, &mut Name), With<Npc>>,
    mut text_query: Query<&mut Text>,
    rename_query: Query<Entity, With<RenameInputMarker>>,
    name_input_map: Res<NameInputMap>,
    name_map: Res<NameMap>,
) {
    for message in messages.read() {
        // Continue if not targeting a rename entity
        if !rename_query.contains(message.entity) {
            continue;
        }

        let name = message.text.clone();

        // Continue if an Npc with the same name already exists
        if npc_query.iter().any(|(_, npc_name)| npc_name.0 == name) {
            error_writer.write(TextInputError {
                entity: message.entity,
            });
            continue;
        }

        for (npc_entity, mut npc_name) in &mut npc_query {
            let name_input_entity = name_input_map[&npc_entity];
            if name_input_entity != message.entity {
                continue;
            }
            npc_name.0 = name.clone();

            let name_entity = name_map[&npc_entity];
            if let Ok(mut text) = text_query.get_mut(name_entity) {
                text.0 = name.clone();
            }
        }
    }
}

/// Create the name inputs for renaming every [`Npc`]
fn create_npc_rename_inputs(
    grid_single: Single<Entity, With<GridMarker1>>,
    npc_query: Query<(Entity, &Name), With<Npc>>,
    mut commands: Commands,
    mut name_input_map: ResMut<NameInputMap>,
    mut name_map: ResMut<NameMap>,
    assets: Res<AssetServer>,
) {
    for (npc_entity, name) in &npc_query {
        // Continue if NameInputMap already contains key of npc_entity
        if name_input_map.0.contains_key(&npc_entity) {
            continue;
        }

        let grid_entity = grid_single.entity();

        commands.entity(grid_entity).with_children(|commands| {
            let name_input_entity = commands
                .spawn((name_input(&assets, "Rename Npc"), RenameInputMarker))
                .insert(name_input_filter())
                .id();
            let name_entity = commands.spawn(Text::new(name.0.as_str())).id();

            name_input_map.0.insert(npc_entity, name_input_entity);
            name_map.0.insert(npc_entity, name_entity);
        });
    }
}

/// [`Bundle`] containing name input
fn name_input(assets: &Res<AssetServer>, prompt: &str) -> impl Bundle {
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
            width: Val::Px(200.0),
            height: Val::Px(30.0),
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

/// Name input filter
///
/// This filters for anything that is alphanumeric or whitespace.
fn name_input_filter() -> TextInputFilter {
    TextInputFilter::custom(is_alphanumeric_or_whitespace)
}

/// Check if text is alphanumeric or whitespace
fn is_alphanumeric_or_whitespace(text: &str) -> bool {
    text.chars()
        .all(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace())
}
