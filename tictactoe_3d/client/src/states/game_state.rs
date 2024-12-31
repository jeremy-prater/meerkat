// Our Game State
use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    #[default]
    Splash,
    MainMenu,
    Connecting,
    InGame,
    GameResults,
}

pub fn debug_game_state_changes(state: Res<State<GameState>>) {
    if state.is_changed() {
        info!("GameState :: Game state changed to {:?}!", state);
    }
}

pub fn debug_gltf_asset_events(mut events: EventReader<AssetEvent<Gltf>>) {
    for event in  events.read() {
        info!("GLTF Event : {:?}", event);
    }
}
pub fn debug_font_asset_events(mut events: EventReader<AssetEvent<Font>>) {
    for event in  events.read() {
        info!("Font Event : {:?}", event);
    }
}

pub fn debug_image_asset_events(mut events: EventReader<AssetEvent<Image>>) {
    for event in  events.read() {
        info!("Image Event : {:?}", event);
    }
}