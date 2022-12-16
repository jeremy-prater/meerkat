use anyhow::Result;
use bevy::{prelude::*, window::close_on_esc};
use iyes_loopless::prelude::*;

pub mod cloud;
pub mod components;
pub mod resources;
pub mod states;
pub mod systems;
pub mod plugins;

use states::game_state::GameState;

fn main() -> Result<()> {
    // meerkat_common::logging::init_logging()?;
    App::new()
        // .add_plugins_with(DefaultPlugins, |group| group.disable::<LogPlugin>())
        .add_plugins(DefaultPlugins)
        .add_loopless_state(GameState::MainMenu)
        .init_resource::<resources::player::Player>()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.1,
        })
        .insert_resource(resources::cloud::CloudClient::default())
        // Main menu
        // menu setup (state enter) systems
        .add_enter_system(GameState::MainMenu, systems::main_menu::setup_menu)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::MainMenu)
                .with_system(close_on_esc)
                .with_system(systems::main_menu::main_menu_ui_system)
                .with_system(systems::main_menu::name_input)
                .into(),
        )
        // menu cleanup (state exit) systems
        .add_exit_system(
            GameState::MainMenu,
            meerkat_common::common::despawn::despawn_with::<components::main_menu::MainMenu>,
        )
        // Falling XO States
        .add_enter_system(GameState::MainMenu, systems::falling_xo::setup_falling_xo)
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::MainMenu)
                .with_system(systems::falling_xo::falling_xo_system_manager)
                .with_system(systems::falling_xo::falling_xo_system_movement)
                .into(),
        )
        .add_exit_system(
            GameState::MainMenu,
            meerkat_common::common::despawn::despawn_with::<components::main_menu::OModel>,
        )
        // Falling XO cleanup (state exit) systems
        // .add_exit_system(
        //     GameState::MainMenu,
        //     meerkat_common::common::despawn::despawn_with::<components::main_menu::MainMenu>,
        // )
        // Connecting
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Connecting)
                .with_system(systems::connecting::connecting_ui_system)
                .into(),
        )
        // In game
        // game setup (state enter) systems
        // .add_enter_system(GameState::InGame, systems::camera::setup_3d_camera)
        // game cleanup (state exit) systems
        // our other various systems:
        .add_system(states::game_state::debug_game_state_changes)
        .run();

    Ok(())
}
