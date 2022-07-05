use anyhow::Result;
use bevy::app::AppExit;
use bevy::input::system::exit_on_esc_system;
use bevy::{log::LogPlugin, prelude::*};
use iyes_loopless::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

pub mod components;
pub mod states;
pub mod resources;
pub mod ui;

use states::game_state::GameState;

fn main() -> Result<()> {
    meerkat_common::logging::init_logging()?;
    App::new()
        .add_plugins_with(DefaultPlugins, |group| group.disable::<LogPlugin>())
        .add_plugin(EguiPlugin)
        .add_loopless_state(GameState::MainMenu)
        .init_resource::<crate::resources::player::Player>()
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
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::MainMenu)
                .with_system(exit_on_esc_system)
                .with_system(ui::main_menu::main_menu_ui_system)
                // our menu button handlers
                // .with_system(butt_exit.run_if(on_butt_interact::<ExitButt>))
                // .with_system(butt_game.run_if(on_butt_interact::<EnterButt>))
                .into(),
        )
        // our other various systems:
        .add_system(states::game_state::debug_game_state_changes)
        .run();

    Ok(())
}
