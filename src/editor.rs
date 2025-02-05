use bevy::prelude::*;
use std::fs;
use bevy::window::{PrimaryWindow, Window};
use crate::components::{Obstacle, Floor};
use crate::levels::{Level, ObstacleData};
use serde::Serialize;
use crate::states::GameState;

#[derive(Default)]
pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Editor), setup_editor)
            .add_systems(Update, editor_system.run_if(in_state(GameState::Editor)))
            .add_systems(OnExit(GameState::Editor), cleanup_editor);
    }
}

#[derive(Component)]
pub struct EditorCamera;

pub fn setup_editor(mut commands: Commands) {
    // add camera
    commands.spawn((
        Camera2dBundle::default(),
        EditorCamera,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.8, 0.8, 0.8),
                custom_size: Some(Vec2::new(1000.0, 10.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -36.0, 0.0),
            ..default()
        },
        Floor,
    ));
}

pub fn editor_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    mut query: Query<&Transform, With<Obstacle>>,
) {
    // Get the primary window
    let window = windows.get_single().unwrap();

    if mouse_button_input.just_pressed(MouseButton::Left) {
        debug!("Mouse location x: {}, y: {}", window.cursor_position().unwrap().x, window.cursor_position().unwrap().y);
        if let Some(cursor_position) = window.cursor_position() {
            let world_position = Vec3::new(
                cursor_position.x - window.width() / 2.0,
                cursor_position.y - window.height() / 2.0,
                0.0,
            );

            // Spawn a new obstacle at the cursor position
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::srgb(1.0, 0.0, 0.0),
                        custom_size: Some(Vec2::new(30.0, 30.0)),
                        ..default()
                    },
                    transform: Transform::from_translation(world_position),
                    ..default()
                },
                Obstacle,
            ));
        }
    }

    if keyboard_input.just_pressed(KeyCode::KeyS) {
        save_level(&query);
    }
}

fn save_level(query: &Query<&Transform, With<Obstacle>>) {
    let mut obstacles = Vec::new();

    for transform in query.iter() {
        obstacles.push(ObstacleData {
            position: Vec2::new(transform.translation.x, transform.translation.y),
            vertices: [
                [0.0, 0.0],
                [0.0, 30.0],
                [30.0, 30.0],
                [30.0, 0.0],
            ],
            non_lethal: Some(false)
        })
    }

    let level = Level {
        level_id: 2,
        obstacles,
    };

    let serialized_level = serde_json::to_string_pretty(&level).expect("Failed to serialize level");
    fs::write("assets/levels/level_edited.json", serialized_level).expect("Failed to write level to file");
}

pub fn cleanup_editor(
    mut commands: Commands,
    editor_entities: Query<Entity, With<EditorCamera>>,
    obstacle_entities: Query<Entity, With<Obstacle>>,
) {
    // Cleanup editor camera
    for entity in editor_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }

    // Cleanup obstacles
    for entity in obstacle_entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
