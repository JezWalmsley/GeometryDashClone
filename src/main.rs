use bevy::audio::AudioPlugin;
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
use crate::components::LevelProgress;
use crate::systems::{play_death_sound, play_victory_sound, progress_tracker_system, setup_audio_system};
use crate::ui::{cleanup_game_over_menu, cleanup_victory_screen, game_over_menu_buttons, setup_game_over_menu, setup_victory_screen, update_progress_ui, victory_screen_buttons};

fn main() {
    info!("Starting the application...");
    App::new()
        .add_plugins(DefaultPlugins
                         .set(LogPlugin {
                             level: Level::DEBUG,
                             filter: "wgpu=error,bevy_render=info,bevy_ecs=trace".to_string(),
                             custom_layer: |_| None,
                         })
                         .set(WindowPlugin {
                             primary_window: Some(Window {
                                 title: "Geometry Dash".to_string(),
                                 ..default()
                             }),
                             ..default()
                         })
                         .set(AudioPlugin::default()) // Added AudioPlugin here
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(EditorPlugin)
        .init_state::<GameState>()
        .insert_resource(SelectedLevel::default())
        .insert_resource(LevelProgress::default())
        // Audio Systems
        .add_systems(Startup, setup_audio_system)
        .add_systems(Update, (play_death_sound, play_victory_sound).run_if(in_state(GameState::Playing)))
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
        .add_systems(OnEnter(GameState::VictoryScreen), setup_victory_screen)
        .add_systems(Update, victory_screen_buttons.run_if(in_state(GameState::VictoryScreen)))
        .add_systems(OnExit(GameState::VictoryScreen), cleanup_victory_screen)
        .add_systems(Update, (progress_tracker_system, update_progress_ui))
        // Run the app
        .run();
    info!("Application has stopped running.");
}
