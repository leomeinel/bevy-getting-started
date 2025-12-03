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

use crate::widgets::{
    grid::{self, GridMarker0, GridMarker1},
    text_input::{self, InputError, InputSuccess},
};

/// Plugin
pub(super) fn plugin(app: &mut App) {
    // Add messages
    app.add_message::<Rename>();

    // Insert resources
    app.insert_resource(InputMap::default());
    app.insert_resource(OutputMap::default());

    // Add startup systems
    app.add_systems(Startup, setup.after(grid::setup));

    // Add update systems
    app.add_systems(
        Update,
        (spawn_rename, create_on_input, rename_on_input, on_rename),
    );
}

/// Message that gets written on successful renaming
#[derive(Message)]
struct Rename {
    /// [`Entity`] of the [`Npc`]
    npc_e: Entity,
    /// [`Entity`] of name output
    name_output_e: Entity,
    /// Text from input submission
    text: String,
}

/// Npc
#[derive(Component)]
struct Npc;

/// Name
#[derive(Component)]
struct Name(String);

/// Marker component for any name input that is used for renaming
#[derive(Component)]
struct RenameMarker;

/// Name input map
///
/// It is structured like this:
/// k: [`Npc`] [`Entity`] -> v: 'name input [`Entity`]'
#[derive(Resource, Deref, Default)]
struct InputMap(HashMap<Entity, Entity>);

/// Name output map
///
/// It is structured like this:
/// k: [`Npc`] [`Entity`] -> v: 'name [`Entity`]'
#[derive(Resource, Deref, Default)]
struct OutputMap(HashMap<Entity, Entity>);

/// Spawn name input for creating a new [`Npc`]
fn setup(
    grid_q: Single<Entity, With<GridMarker0>>,
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    let grid_e = grid_q.entity();

    // Spawn as child of grid_e
    commands.entity(grid_e).with_children(|commands| {
        commands.spawn(Text::new("Enter name"));
        commands
            .spawn(input(&assets, "Create Npc"))
            .insert(input_filter());
    });
}

/// Create the name inputs for renaming every [`Npc`]
fn spawn_rename(
    grid_q: Single<Entity, With<GridMarker1>>,
    npc_q: Query<(Entity, &Name), With<Npc>>,
    mut commands: Commands,
    mut input_map: ResMut<InputMap>,
    mut output_map: ResMut<OutputMap>,
    assets: Res<AssetServer>,
) {
    for (npc_e, name) in &npc_q {
        // Continue if InputMap already contains key of npc_e
        if input_map.0.contains_key(&npc_e) {
            continue;
        }

        let grid_e = grid_q.entity();

        // Spawn as child of grid_e
        commands.entity(grid_e).with_children(|commands| {
            let name_input_e = commands
                .spawn((input(&assets, "Rename Npc"), RenameMarker))
                .insert(input_filter())
                .id();
            let name_output_e = commands.spawn(Text::new(name.0.as_str())).id();

            input_map.0.insert(npc_e, name_input_e);
            output_map.0.insert(npc_e, name_output_e);
        });
    }
}

/// Add objects of type [`Npc`] from [`InputSuccess`]
///
/// This adds a bundle of [`Npc`] and [`Name`] from [`InputSuccess`]
fn create_on_input(
    mut msgs: MessageReader<InputSuccess>,
    mut error_msg: MessageWriter<InputError>,
    npc_q: Query<&Name, With<Npc>>,
    rename_q: Query<Entity, With<RenameMarker>>,
    mut commands: Commands,
) {
    for msg in msgs.read() {
        // Continue if targeting a rename entity
        let entity = msg.entity;
        if rename_q.contains(entity) {
            continue;
        }

        let name = msg.text.clone();

        // Continue if an Npc with the same name already exists
        if npc_q.iter().any(|npc_name| npc_name.0 == name) {
            error_msg.write(InputError(entity));
            continue;
        }

        commands.spawn((Npc, Name(name)));
    }
}

/// Rename objects of type [`Npc`] from [`InputSuccess`]
fn rename_on_input(
    mut msgs: MessageReader<InputSuccess>,
    mut error_msg: MessageWriter<InputError>,
    mut success_msg: MessageWriter<Rename>,
    mut npc_q: Query<(Entity, &mut Name), With<Npc>>,
    rename_q: Query<Entity, With<RenameMarker>>,
    input_map: Res<InputMap>,
    output_map: Res<OutputMap>,
) {
    for msg in msgs.read() {
        // Continue if not targeting a rename entity
        let entity = msg.entity;
        if !rename_q.contains(entity) {
            continue;
        }

        let name = msg.text.clone();

        for (npc_e, mut npc_name) in &mut npc_q {
            // Break if an Npc with the same name already exists and write InputError
            if npc_name.0 == name {
                error_msg.write(InputError(entity));
                break;
            }

            // Continue if we can't get the key npc_e from input_map or the contained entity is not our entity
            if input_map.get(&npc_e).is_none_or(|e| *e != entity) {
                continue;
            }

            // Set name associated to npc and write Rename message
            if let Some(name_output_e) = output_map.get(&npc_e) {
                npc_name.0 = name.clone();
                success_msg.write(Rename {
                    npc_e,
                    name_output_e: *name_output_e,
                    text: name.clone(),
                });
            }
        }
    }
}

/// Modify text of name output on rename
fn on_rename(
    mut msgs: MessageReader<Rename>,
    npc_q: Query<Entity, With<Npc>>,
    mut text_q: Query<&mut Text>,
) {
    for msg in msgs.read() {
        for npc_e in &npc_q {
            // Continue if not targeting the msg npc entity
            if npc_e != msg.npc_e {
                continue;
            }

            // Modify text of name output entity via text query
            if let Ok(mut text) = text_q.get_mut(msg.name_output_e) {
                text.0 = msg.text.clone();
            }
        }
    }
}

/// [`Bundle`] containing name input
fn input(assets: &Res<AssetServer>, prompt: &str) -> impl Bundle {
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
fn input_filter() -> TextInputFilter {
    TextInputFilter::custom(is_alphanumeric_or_whitespace)
}

/// Check if text is alphanumeric or whitespace
fn is_alphanumeric_or_whitespace(text: &str) -> bool {
    text.chars()
        .all(|c| c.is_ascii_alphanumeric() || c.is_ascii_whitespace())
}
