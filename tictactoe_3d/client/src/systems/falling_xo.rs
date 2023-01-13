use bevy::prelude::*;

use log::info;
use rand::random;

use crate::components::main_menu::OModel;

pub fn setup_falling_xo(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    _player: ResMut<crate::resources::player::Player>,
) {
    info!("setup falling xo!");

    let _camera = commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(15.0, 0.0, 0.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        })
        .insert(crate::components::main_menu::Camera)
        .id();

    const HALF_SIZE: f32 = 1.0;
    let _light = commands
        .spawn(DirectionalLightBundle {
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

    // commands.entity(camera).push_children(&[o_model, light]);
}

fn generate_o() -> crate::components::main_menu::OModel {
    let o_model = crate::components::main_menu::OModel {
        rotate_deg_s: Vec3::new(
            random::<f32>() - 0.5,
            random::<f32>() - 0.5,
            random::<f32>() - 0.5,
        ),
        fall_s: (random::<f32>() * 0.025) + 0.025,
    };

    info!("Creating O Model {:?}", o_model);

    o_model
}

pub fn falling_xo_system_manager(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<&crate::components::main_menu::OModel>,
) {
    if query.iter().count() == 20 {
        return;
    }

    let pos_z = (random::<f32>() - 0.5) * 20.0;
    let pos_x = (random::<f32>() - 0.5) * 1.0;

    let x_or_o = random::<bool>();

    let my_gltf = match x_or_o {
        true => asset_server.load("o.gltf#Scene0"),
        false => asset_server.load("x.gltf#Scene0"),
    };

    let _o_model = commands
        .spawn(SceneBundle {
            scene: my_gltf,
            transform: Transform::from_translation(Vec3::new(pos_x, 5.0, pos_z)),
            ..default()
        })
        .insert(generate_o())
        .id();
}

pub fn falling_xo_system_movement(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &crate::components::main_menu::OModel,
            &mut Transform,
        ),
        (With<OModel>),
    >,
) {
    for (entity, model, mut transform) in &mut query {
        transform.rotation *= Quat::from_rotation_x(time.delta_seconds() * model.rotate_deg_s.x);
        transform.rotation *= Quat::from_rotation_z(time.delta_seconds() * model.rotate_deg_s.z);
        transform.rotation *= Quat::from_rotation_y(time.delta_seconds() * model.rotate_deg_s.y);
        transform.translation.y -= model.fall_s;

        if transform.translation.y < -5.0 {
            // transform.translation.y = 5.0;
            info!("despawning : {}", entity.index());
            commands.entity(entity).despawn_recursive();
        }
    }
}
