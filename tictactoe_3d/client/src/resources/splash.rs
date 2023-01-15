// Player info

use bevy::prelude::*;

#[derive(Resource)]
pub struct SplashScreen {
    pub camera: Entity,
    pub background: Entity,
}
