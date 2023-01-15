use crate::states::game_state::GameState;
use crate::systems::{falling_xo, main_menu};
use bevy::prelude::*;
use bevy::window::close_on_esc;
use iyes_loopless::prelude::*;
use crate::components;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::MainMenu, main_menu::setup_menu)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::MainMenu)
                    .with_system(close_on_esc)
                    .with_system(main_menu::main_menu_ui_system)
                    .with_system(main_menu::name_input)
                    .into(),
            )
            // menu cleanup (state exit) systems
            .add_exit_system(
                GameState::MainMenu,
                meerkat_common::common::despawn::despawn_with::<components::main_menu::MainMenu>,
            )
            // Falling XO States
            .add_enter_system(GameState::MainMenu, falling_xo::setup_falling_xo)
            .add_system_set(
                ConditionSet::new()
                    .run_in_state(GameState::MainMenu)
                    .with_system(falling_xo::falling_xo_system_manager)
                    .with_system(falling_xo::falling_xo_system_movement)
                    .into(),
            )
            .add_exit_system(
                GameState::MainMenu,
                meerkat_common::common::despawn::despawn_with::<components::main_menu::OModel>,
            );
    }
}
