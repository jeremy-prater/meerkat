use anyhow::Result;
use bevy::prelude::*;

pub mod cloud;
pub mod components;
pub mod plugins;
pub mod resources;
pub mod states;
pub mod systems;

use states::game_state::GameState;

fn main() -> Result<()> {
    // meerkat_common::logging::init_logging()?;
    App::new()
        // .add_plugins_with(DefaultPlugins, |group| group.disable::<LogPlugin>())
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_plugin(plugins::splash::SplashPlugin)
        // Main menu
        .add_plugin(plugins::main_menu::MainMenuPlugin)
        .init_resource::<resources::player::Player>()
        .init_resource::<resources::cloud::CloudClient>()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.1,
        })
        .insert_resource(resources::cloud::CloudClient::default())
        // Log state changes
        .add_system(states::game_state::debug_game_state_changes)
        // Connecting
        .add_system(systems::connecting::connecting_ui_system.in_set(GameState::Connecting))
        // In game
        // game setup (state enter) systems
        .add_system(systems::camera::setup_3d_camera.in_schedule(OnEnter(GameState::InGame)))
        .run();

    Ok(())
}
