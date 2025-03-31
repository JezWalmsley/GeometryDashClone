use std::{fs, io};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

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

#[derive(Resource, Default)]
pub struct LevelProgress {
    pub current_percentage: f32,
}

#[derive(Component)]
pub struct ProgressText;
#[derive(Component)]
pub struct DeathSound;
#[derive(Component)]
pub struct VictorySound;

#[derive(Resource)]
pub struct GameAudio {
    pub death_sound: Handle<AudioSource>,
    pub victory_sound: Handle<AudioSource>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProgressEntry {
    pub date: String,
    pub percentage: f32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ProgressHistory {
    pub entries: Vec<ProgressEntry>,
}

impl ProgressHistory {

    pub fn load(file_path: &str) -> Result<Self, io::Error> {
    match fs::read_to_string(file_path) {
        Ok(data) => match serde_json::from_str::<Self>(&data) {
            Ok(history) => Ok(history),
            Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "Failed to parse JSON")),
        },
        Err(e) => Err(e),
    }
}


    pub fn save(&self, file_path: &str) {
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::write(file_path, json);
        }
    }

    pub fn add_entry(&mut self, new_entry: ProgressEntry, file_path: &str) {
        if let Some(existing_entry) = self.entries.iter_mut().find(|e| e.date == new_entry.date) {
            if new_entry.percentage > existing_entry.percentage {
                existing_entry.percentage = new_entry.percentage;
            }
        } else {
            self.entries.push(new_entry);
        }

        self.entries.sort_by(|a, b| {
            b.percentage
                .partial_cmp(&a.percentage)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| b.date.cmp(&a.date))
        });

        self.save(file_path);
    }
}

#[derive(Component)]
pub struct LeaderboardButton;