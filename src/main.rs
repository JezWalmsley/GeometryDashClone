// main.rs

use bevy::prelude::*;

mod components;
mod levels;
mod states;
mod systems;
mod ui;

use components::SelectedLevel;
use states::GameState;
use systems::gameplay::{cleanup_gameplay, setup_gameplay};
use ui::{
    button_system, cleanup_level_selection, cleanup_title_screen, level_button_system,
    setup_level_selection, setup_title_screen,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
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
        .add_systems(OnEnter(GameState::Playing), setup_gameplay)
        .add_systems(OnExit(GameState::Playing), cleanup_gameplay)
        // Add your gameplay systems here
        .run();
}

