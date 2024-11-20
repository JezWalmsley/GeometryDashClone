// systems.rs

use bevy::prelude::*;

pub mod gameplay {
    use super::*;
    use crate::components::GameplayCamera;

    pub fn setup_gameplay(mut commands: Commands) {
        // Spawn the gameplay camera and tag it
        commands.spawn((
            Camera2dBundle::default(),
            GameplayCamera, // Tag the camera
        ));

        // Add your gameplay setup here
    }
}
