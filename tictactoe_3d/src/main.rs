use anyhow::Result;
use bevy::input::system::exit_on_esc_system;
use bevy::{log::LogPlugin, prelude::*};
use bevy_egui::EguiPlugin;
use iyes_loopless::prelude::*;

pub mod components;
pub mod resources;
pub mod states;
pub mod ui;

use states::game_state::GameState;

fn main() -> Result<()> {
    meerkat_common::logging::init_logging()?;
    App::new()
        .add_plugins_with(DefaultPlugins, |group| group.disable::<LogPlugin>())
        .add_plugin(EguiPlugin)
        .add_loopless_state(GameState::MainMenu)
        .init_resource::<resources::player::Player>()
        // menu setup (state enter) systems
        .add_enter_system(GameState::MainMenu, ui::main_menu::setup_menu)
        // menu cleanup (state exit) systems
        .add_exit_system(
            GameState::MainMenu,
            meerkat_common::common::despawn::despawn_with::<components::main_menu::MainMenu>,
        )
        // game setup (state enter) systems
        // .add_enter_system(GameState::InGame, setup_game_camera)
        // game cleanup (state exit) systems
        // Main menu
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::MainMenu)
                .with_system(exit_on_esc_system)
                .with_system(ui::main_menu::main_menu_ui_system)
                .into(),
        )
        // Connecting
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Connecting)
                .with_system(ui::connecting::connecting_ui_system)
                .into(),
        )
        // our other various systems:
        .add_system(states::game_state::debug_game_state_changes)
        .run();

    Ok(())
}
