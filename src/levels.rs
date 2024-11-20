// levels.rs

pub struct Level {
    pub level_id: usize,
    // Add other properties as needed
}

// Function to retrieve level data
pub fn get_level(level_id: usize) -> Option<Level> {
    match level_id {
        1 => Some(Level {
            level_id: 1,
            // Define level properties
        }),
        2 => Some(Level {
            level_id: 2,
            // Define level properties
        }),
        3 => Some(Level {
            level_id: 3,
            // Define level properties
        }),
        4 => Some(Level {
            level_id: 4,
            // Define level properties
        }),
        5 => Some(Level {
            level_id: 5,
            // Define level properties
        }),
        _ => None,
    }
}
