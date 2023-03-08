use crate::states::game_state::GameState;
use crate::systems::splash;
use bevy::prelude::*;
use iyes_progress::prelude::*;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            // Add plugin for the splash screen
            .add_plugin(
                ProgressPlugin::new(GameState::Splash)
                    .continue_to(GameState::MainMenu)
                    .track_assets(),
            )
            // Load our UI assets during our splash screen
            .add_systems(
                (splash::setup_splash_ui, splash::load_game_assets)
                    .chain()
                    .in_schedule(OnEnter(GameState::Splash)),
            )
            .add_system(splash::teardown_splash_ui.in_schedule(OnExit(GameState::Splash)));
    }
}
