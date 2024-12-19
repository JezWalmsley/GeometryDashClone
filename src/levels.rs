use bevy::prelude::*;
use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Level {
    pub level_id: usize,
    pub obstacles: Vec<ObstacleData>,
}

#[derive(Deserialize)]
pub struct ObstacleData {
    pub position: Vec2,
    pub vertices: [[f32; 2]; 4], // Define vertices of the triangle
    pub non_lethal: Option<bool>,
}

// Example level loading function
pub fn load_level(level_id: usize) -> Option<Level> {
    let path = format!("assets/levels/level_{}.json", level_id);
    info!("Attempting to load level file: {}", path);

    let data = match fs::read_to_string(&path) {
        Ok(content) => content,
        Err(e) => {
            warn!("Failed to read level file: {}. Error: {}", path, e);
            return None;
        }
    };

    match serde_json::from_str::<Level>(&data) {
        Ok(level) => Some(level),
        Err(e) => {
            warn!("Failed to parse level file: {}. Error: {}", path, e);
            None
        }
    }
}

