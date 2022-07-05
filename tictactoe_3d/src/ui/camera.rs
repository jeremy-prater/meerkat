// Setup UI camera

use bevy::prelude::*;

pub fn setup_ui_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}