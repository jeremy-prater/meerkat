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
        .init_state::<GameState>()
        .add_plugins(plugins::splash::SplashPlugin)
        // Main menu
        .add_plugins(plugins::main_menu::MainMenuPlugin)
        .init_resource::<resources::player::Player>()
        .init_resource::<resources::cloud::CloudClient>()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.1,
        })
        // Connecting
        .add_systems(
            Update,
            systems::connecting::connecting_ui_system.run_if(in_state(GameState::Connecting)),
        )
        // In game
        // game setup (state enter) systems
        .add_systems(OnEnter(GameState::InGame), systems::camera::setup_3d_camera)
        // Log state changes
        .add_systems(Update, states::game_state::debug_game_state_changes)
        .add_systems(Update, states::game_state::debug_gltf_asset_events)
        .add_systems(Update, states::game_state::debug_font_asset_events)
        .add_systems(Update, states::game_state::debug_image_asset_events)
        .run();

    Ok(())
}
