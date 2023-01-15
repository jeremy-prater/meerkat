use bevy::prelude::*;
use iyes_progress::prelude::*;

use log::info;

use crate::resources;

pub fn load_game_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // we need to add our handles here, to track their loading progress:
    mut loading: ResMut<AssetsLoading>,
) {
    info!("Loading game assets");

    let font: Handle<Font> = asset_server.load("ARCADE.TTF");
    let o_model: Handle<Scene> = asset_server.load("o.gltf#Scene0");
    let x_model: Handle<Scene> = asset_server.load("x.gltf#Scene0");

    loading.add(&font);
    loading.add(&o_model);
    loading.add(&x_model);
}

pub fn setup_splash_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera
    let camera = commands.spawn(Camera2dBundle::default()).id();

    // root node
    let background = commands.spawn(ImageBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            ..default()
        },
        image: asset_server.load("splash.png").into(),
        ..default()
    }).id();

    commands.insert_resource(resources::splash::SplashScreen {
        camera,
        background,
    })
}

pub fn teardown_splash_ui(mut commands: Commands, splash: Res<resources::splash::SplashScreen>) {
    commands.entity(splash.background).despawn_recursive();
    commands.entity(splash.camera).despawn_recursive();
    commands.remove_resource::<resources::splash::SplashScreen>();
}
