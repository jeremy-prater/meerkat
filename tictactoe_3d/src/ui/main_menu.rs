use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use log::info;

pub fn setup_menu(
    mut commands: Commands,
    ass: Res<AssetServer>,
    player: ResMut<crate::resources::player::Player>,
    mut egui_ctx: ResMut<EguiContext>,
) {
    info!("setup menu!");

    // player.

    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

pub fn main_menu_ui_system(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Hello").show(egui_context.ctx_mut(), |ui| {
        ui.label("world");
    });
}
