use bevy::prelude::*;

#[derive(States, Clone, Copy, Eq, PartialEq, Hash, Debug, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Playing,
    Paused,
    LevelUp,
    GameOver,
}

#[derive(Message, Default)]
pub struct ResetGameMessage;
