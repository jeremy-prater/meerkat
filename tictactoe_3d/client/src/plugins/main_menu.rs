use crate::components;
use crate::states::game_state::GameState;
use crate::systems::{falling_xo, main_menu};
use bevy::prelude::*;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), main_menu::setup_menu)
            .add_systems(
                Update,
                (
                    // close_on_esc,
                    main_menu::main_menu_ui_system,
                    main_menu::name_input,
                )
                    .run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(
                OnExit(GameState::MainMenu),
                meerkat_common::common::despawn::despawn_with::<components::main_menu::MainMenu>,
            )
            // Falling XO Systems
            .add_systems(
                OnEnter(GameState::MainMenu),
                falling_xo::setup_falling_xo.run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(
                Update,
                (
                    falling_xo::falling_xo_system_manager,
                    falling_xo::falling_xo_system_movement,
                )
                    .run_if(in_state(GameState::MainMenu)),
            )
            .add_systems(
                OnExit(GameState::MainMenu),
                meerkat_common::common::despawn::despawn_with::<components::main_menu::OModel>,
            );
    }
}
