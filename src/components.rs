// components.rs

use bevy::prelude::*;

// Marker components for UI buttons
#[derive(Component)]
pub struct StartButton;

#[derive(Component)]
pub struct QuitButton;

#[derive(Component)]
pub struct BackButton;

#[derive(Component)]
pub struct LevelButton {
    pub level_id: usize,
}

// Marker components for cameras
#[derive(Component)]
pub struct UICamera;

#[derive(Component)]
pub struct GameplayCamera;

// Resource to store the selected level
#[derive(Resource, Default)]
pub struct SelectedLevel {
    pub level_id: usize,
}
