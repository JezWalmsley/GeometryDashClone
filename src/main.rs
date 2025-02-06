use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy::log::*;
use editor::EditorPlugin;

mod components;
mod levels;
mod states;
mod systems;
mod ui;
mod editor;

use components::SelectedLevel;
use states::GameState;
use systems::gameplay::{cleanup_gameplay, setup_gameplay};
use ui::{
    button_system, cleanup_level_selection, cleanup_title_screen, level_button_system,
    setup_level_selection, setup_title_screen,
};
use crate::systems::gameplay::{collision_event_system, continuous_floor_system, exit_level_system, level_scrolling_system, player_movement_system, spawn_floor};
use bevy::log::LogPlugin;
use crate::ui::{cleanup_game_over_menu, game_over_menu_buttons, setup_game_over_menu};

fn main() {
    info!("Starting the application...");
    App::new()
        .add_plugins(DefaultPlugins.set(
            LogPlugin {
            level: Level::DEBUG,
            filter: "wgpu=error,bevy_render=info,bevy_ecs=trace".to_string(),
            custom_layer: |_| None,
        }).set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Geometry Dash".to_string(),
                    ..default()
                }),
                ..default()
            }
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(EditorPlugin)
        .init_state::<GameState>()
        .insert_resource(SelectedLevel::default())
        // Title Screen Systems
        .add_systems(OnEnter(GameState::TitleScreen), setup_title_screen)
        .add_systems(Update, button_system.run_if(in_state(GameState::TitleScreen)))
        .add_systems(OnExit(GameState::TitleScreen), cleanup_title_screen)
        // Level Selection Systems
        .add_systems(OnEnter(GameState::LevelSelection), setup_level_selection)
        .add_systems(Update, level_button_system.run_if(in_state(GameState::LevelSelection)))
        .add_systems(OnExit(GameState::LevelSelection), cleanup_level_selection)
        // Gameplay Systems
        .add_systems(OnEnter(GameState::Playing), (setup_gameplay, spawn_floor))
        .add_systems(OnExit(GameState::Playing), cleanup_gameplay)
        .add_systems(
            Update,
            (
                player_movement_system,
                collision_event_system,
                level_scrolling_system,
                continuous_floor_system,
                exit_level_system,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnEnter(GameState::GameOver), setup_game_over_menu)
        .add_systems(Update, game_over_menu_buttons.run_if(in_state(GameState::GameOver)))
        .add_systems(OnExit(GameState::GameOver), cleanup_game_over_menu)
        // Run the app
        .run();
    info!("Application has stopped running.");
}
