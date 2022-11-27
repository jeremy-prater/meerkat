// Setup UI camera
use bevy::prelude::*;
use log::info;

pub fn setup_3d_camera(mut commands: Commands) {
    info!("Creating 3D camera");
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(5.0, 5.0, 5.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        })
        .insert(crate::components::main_menu::Camera);
}
