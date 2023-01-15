use crate::states::game_state::GameState;
use crate::systems::splash;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::Splash)
            // Add plugin for the splash screen
            .add_plugin(
                ProgressPlugin::new(GameState::Splash)
                    .continue_to(GameState::MainMenu)
                    .track_assets(),
            )
            // Load our UI assets during our splash screen
            .add_enter_system(GameState::Splash, splash::load_game_assets)
            .add_enter_system(GameState::Splash, splash::setup_splash_ui)
            .add_exit_system(GameState::Splash, splash::teardown_splash_ui);
    }
}
