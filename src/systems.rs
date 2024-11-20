// systems.rs

use bevy::prelude::*;

pub mod gameplay {
    use super::*;
    use crate::components::{GameplayCamera, SelectedLevel};
    use crate::levels::get_level;

    pub fn setup_gameplay(
        mut commands: Commands,
        selected_level: Res<SelectedLevel>,
        asset_server: Res<AssetServer>,
    ) {
        // Spawn the gameplay camera
        commands.spawn((
            Camera2dBundle::default(),
            GameplayCamera,
        ));

        // Load and set up the selected level
        if let Some(level) = get_level(selected_level.level_id) {
            info!("Loading Level {}", level.level_id);
            // Add your level setup code here
            // e.g., spawn player, obstacles, etc.
        } else {
            error!("Level {} not found!", selected_level.level_id);
        }
    }

    pub fn cleanup_gameplay(
        mut commands: Commands,
        entities: Query<Entity, Without<Camera>>,
        camera_entities: Query<Entity, With<GameplayCamera>>,
    ) {
        // Despawn all entities except cameras
        for entity in entities.iter() {
            commands.entity(entity).despawn_recursive();
        }
        // Despawn the gameplay camera
        for entity in camera_entities.iter() {
            commands.entity(entity).despawn();
        }
    }
}
