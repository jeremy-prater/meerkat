// Our Game State

use bevy::prelude::*;
use iyes_loopless::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameState {
    MainMenu,
    Connecting,
    InGame,
    GameResults,
}

pub fn debug_game_state_changes(state: Res<CurrentState<GameState>>) {
    if state.is_changed() {
        info!("GameState :: Detected state change to {:?}!", state);
    }
}
