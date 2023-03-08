use crate::components;
use crate::states::game_state::GameState;
use crate::systems::{falling_xo, main_menu};
use bevy::prelude::*;
use bevy::window::close_on_esc;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(main_menu::setup_menu.in_schedule(OnEnter(GameState::MainMenu)))
            .add_systems(
                (
                    close_on_esc,
                    main_menu::main_menu_ui_system,
                    main_menu::name_input,
                )
                    .chain()
                    .in_set(GameState::MainMenu),
            )
            .add_system(
                meerkat_common::common::despawn::despawn_with::<components::main_menu::MainMenu>
                    .in_schedule(OnExit(GameState::MainMenu)),
            )

            // Falling XO Systems
            .add_system(falling_xo::setup_falling_xo.in_schedule(OnEnter(GameState::MainMenu)))
            .add_systems(
                (
                    falling_xo::falling_xo_system_manager,
                    falling_xo::falling_xo_system_movement,
                )
                    .chain()
                    .in_set(GameState::MainMenu),
            )
            .add_system(
                meerkat_common::common::despawn::despawn_with::<components::main_menu::OModel>
                    .in_schedule(OnExit(GameState::MainMenu)),
            );
    }
}
