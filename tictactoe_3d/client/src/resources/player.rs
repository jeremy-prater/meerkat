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

impl Player {
    pub fn check_name(&self) -> bool {
        true
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            name: "Jones".to_string(),
            side: PlayerSide::Auto,
        }
    }
}