use anyhow::Result;
use bevy::input::system::exit_on_esc_system;
use bevy::{log::LogPlugin, prelude::*};
use iyes_loopless::prelude::*;

pub mod components;
pub mod resources;
pub mod states;
pub mod ui;

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

        // Main menu
        // menu setup (state enter) systems
        .add_enter_system(GameState::MainMenu, ui::main_menu::setup_menu)
        .add_enter_system(GameState::MainMenu, ui::camera::setup_ui_camera)        
        .add_system_set(
            ConditionSet::new()
            .run_in_state(GameState::MainMenu)
            .with_system(exit_on_esc_system)
            .with_system(ui::main_menu::main_menu_ui_system)
            .into(),
        )
        // menu cleanup (state exit) systems
        .add_exit_system(
            GameState::MainMenu,
            meerkat_common::common::despawn::despawn_with::<components::main_menu::MainMenu>,
        )

        // Connecting
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Connecting)
                .with_system(ui::connecting::connecting_ui_system)
                .into(),
        )

        // In game
        // game setup (state enter) systems
        // .add_enter_system(GameState::InGame, ui::camera::setup_ui_camera)
        // game cleanup (state exit) systems

        // our other various systems:
        .add_system(states::game_state::debug_game_state_changes)
        .run();

    Ok(())
}
