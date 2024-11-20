mod states;
mod components;
mod systems;
mod ui;

use bevy::prelude::*;
use states::GameState;
use systems::gameplay::setup_gameplay;
use ui::{button_system, cleanup_title_screen, setup_title_screen};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        // Title Screen Systems
        .add_systems(OnEnter(GameState::TitleScreen), setup_title_screen)
        .add_systems(
            Update,
            button_system.run_if(in_state(GameState::TitleScreen)),
        )
        .add_systems(OnExit(GameState::TitleScreen), cleanup_title_screen)
        // Gameplay Systems
        .add_systems(OnEnter(GameState::Playing), setup_gameplay)
        .run();
}

