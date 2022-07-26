// Setup UI camera
use log::info;
use bevy::prelude::*;

pub fn setup_ui_camera(mut commands: Commands) {
    info!("Creating UI camera");
    commands.spawn_bundle(UiCameraBundle::default());
}