// Player info

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerSide {
    Auto,
    X,
    O,
}

#[derive(Resource)]
pub struct Player {
    pub name: String,
    pub side: PlayerSide,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: "Croagunk".to_string(),
            side: PlayerSide::Auto,
        }
    }
}