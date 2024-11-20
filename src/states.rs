// states.rs

use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum GameState {
    #[default]
    TitleScreen,
    LevelSelection,
    Playing,
}
