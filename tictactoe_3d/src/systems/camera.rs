// Setup UI camera
use bevy::prelude::*;
use log::info;

pub fn setup_ui_camera(mut commands: Commands) {
    info!("Creating UI camera");
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(crate::components::main_menu::UiCamera);
}
