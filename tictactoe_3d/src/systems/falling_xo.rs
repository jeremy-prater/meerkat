use bevy::{app::AppExit, prelude::*};
use iyes_loopless::prelude::*;
use log::info;

pub fn setup_falling_xo(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player: ResMut<crate::resources::player::Player>,
) {
    info!("setup falling xo!");

    commands
        .spawn_bundle(Camera3dBundle {
            transform: Transform::from_xyz(5.0, 0.0, 0.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        })
        .insert(crate::components::main_menu::Camera);
    // note that we have to include the `Scene0` label
    let my_gltf = asset_server.load("o.gltf#Scene0");

    // to be able to position our 3d model:
    // spawn a parent entity with a TransformBundle
    // and spawn our gltf as a scene under it
    let o_model = commands
        .spawn_bundle(TransformBundle {
            local: Transform::identity(),
            global: GlobalTransform::identity(),
        })
        .with_children(|parent| {
            parent.spawn_bundle(SceneBundle {
                scene: my_gltf,
                ..default()
            });
        })
        .insert(crate::components::main_menu::OModel);

    const HALF_SIZE: f32 = 1.0;
    let light = commands
        .spawn_bundle(DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadow_projection: OrthographicProjection {
                    left: -HALF_SIZE,
                    right: HALF_SIZE,
                    bottom: -HALF_SIZE,
                    top: HALF_SIZE,
                    near: -10.0 * HALF_SIZE,
                    far: 10.0 * HALF_SIZE,
                    ..default()
                },
                shadows_enabled: true,
                ..default()
            },
            ..default()
        })
        .insert(crate::components::main_menu::Light)
        .id();
}

pub fn falling_xo_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
}
