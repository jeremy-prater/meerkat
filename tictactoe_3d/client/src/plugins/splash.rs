use crate::states::game_state::GameState;
use crate::systems::splash;
use bevy::prelude::*;
use iyes_progress::prelude::*;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add plugin for the splash screen
            .add_plugins(
                ProgressPlugin::<GameState>::new()
                    .with_asset_tracking()
                    .with_state_transition(GameState::Splash, GameState::MainMenu),
            )
            // Load our UI assets during our splash screen
            .add_systems(
                OnEnter(GameState::Splash),
                (splash::setup_splash_ui, splash::load_game_assets),
            )
            .add_systems(OnExit(GameState::Splash), splash::teardown_splash_ui);
    }
}
