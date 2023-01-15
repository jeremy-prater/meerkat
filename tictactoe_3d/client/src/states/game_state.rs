// Our Game State

use bevy::prelude::*;
use iyes_loopless::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    Splash,
    MainMenu,
    Connecting,
    InGame,
    GameResults,
}

pub fn debug_game_state_changes(state: Res<CurrentState<GameState>>) {
    if state.is_changed() {
        info!("GameState :: Game state change to {:?}!", state);
    }
}
