// Player info

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlayerSide {
    Auto,
    X,
    O,
}

pub struct Player {
    player_name: String,
    side: PlayerSide,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            player_name: "Croagunk".to_string(),
            side: PlayerSide::Auto,
        }
    }
}