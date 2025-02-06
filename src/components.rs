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

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Obstacle;

#[derive(Component)]
pub struct Floor;

#[derive(Component)]
pub struct NonLethal;

#[derive(Component)]
pub struct JumpBuffer {
    pub timer: Timer,
}

#[derive(Component)]
pub struct GameOverText;

#[derive(Component)]
pub struct RestartButton;

#[derive(Component)]
pub struct ReturnToMenuButton;

#[derive(Component)]
pub struct NextLevelButton;

#[derive(Component)]
pub struct FinishLine;

