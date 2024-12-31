// Player info

use bevy::prelude::*;

#[derive(Resource)]
pub struct ArcadeFont(pub Handle<Font>);

#[derive(Resource)]
pub struct XGltf(pub Handle<Scene>);

#[derive(Resource)]
pub struct OGltf(pub Handle<Scene>);
