use bevy::prelude::*;
use bevy_egui::{
    egui,
    egui::{Color32, RichText},
    EguiContext,
};
use iyes_loopless::prelude::*;
use log::info;

pub fn connecting_ui_system(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    player: Res<crate::resources::player::Player>,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.label(RichText::new("3D Tic Tac Toe").color(Color32::LIGHT_BLUE));
        ui.label(RichText::new(&player.name).color(Color32::LIGHT_BLUE));
        ui.label(RichText::new("Connecting...").color(Color32::LIGHT_BLUE));

        if ui.button("Cancel").clicked() {
            info!("Connecting cancelled");
            commands.insert_resource(NextState(crate::GameState::MainMenu));
        }
    });
}
