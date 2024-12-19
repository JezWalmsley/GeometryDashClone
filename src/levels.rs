use bevy::log::warn;
use bevy::math::*;

pub struct Level {
    pub level_id: usize,
    pub obstacles: Vec<ObstacleData>,
}

pub struct ObstacleData {
    pub position: Vec2,
    pub size: Vec2,
}

// Function to retrieve level data
pub fn get_level(level_id: usize) -> Option<Level> {
    match level_id {
        1 => Some(Level {
            level_id: 1,
            obstacles: vec![
                ObstacleData {
                    position: Vec2::new(500.0, 0.0),
                    size: Vec2::new(100.0, 100.0),
                },
                ObstacleData {
                    position: Vec2::new(200.0, 200.0),
                    size: Vec2::new(50.0, 50.0),
                },
            ],
        }),
        2 => Some(Level {
            level_id: 2,
            obstacles: vec![
                ObstacleData {
                    position: Vec2::new(0.0, 0.0),
                    size: Vec2::new(100.0, 100.0),
                },
                ObstacleData {
                    position: Vec2::new(200.0, 200.0),
                    size: Vec2::new(50.0, 50.0),
                },
                ObstacleData {
                    position: Vec2::new(400.0, 400.0),
                    size: Vec2::new(75.0, 75.0),
                },
            ],
        }),
        3 => Some(Level {
            level_id: 3,
            obstacles: vec![
                ObstacleData {
                    position: Vec2::new(0.0, 0.0),
                    size: Vec2::new(100.0, 100.0),
                },
                ObstacleData {
                    position: Vec2::new(200.0, 200.0),
                    size: Vec2::new(50.0, 50.0),
                },
                ObstacleData {
                    position: Vec2::new(400.0, 400.0),
                    size: Vec2::new(75.0, 75.0),
                },
                ObstacleData {
                    position: Vec2::new(600.0, 600.0),
                    size: Vec2::new(25.0, 25.0),
                },
            ],
        }),
        _ => {
            warn!("Requested level {} not found.", level_id);
            None
        }
    }
}
