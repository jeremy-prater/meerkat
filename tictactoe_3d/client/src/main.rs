use anyhow::Result;
use bevy::prelude::*;
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;

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
        .add_plugin(plugins::splash::SplashPlugin)
        .add_plugin(plugins::main_menu::MainMenuPlugin)
        .init_resource::<resources::player::Player>()
        .init_resource::<resources::cloud::CloudClient>()
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 0.1,
        })
        .add_system_set(
            ConditionSet::new()
                .run_in_state(GameState::Connecting)
                .with_system(systems::connecting::connecting_ui_system)
                .into(),
        )
        .add_system(states::game_state::debug_game_state_changes)
        .run();

    Ok(())
}
