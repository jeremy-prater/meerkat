use bevy::{app::AppExit, prelude::*};
use bevy_egui::{
    egui,
    egui::{Color32, RichText},
    EguiContext,
};
use iyes_loopless::prelude::*;
use log::info;

pub fn setup_menu(
    // mut commands: Commands,
    // ass: Res<AssetServer>,
    mut player: ResMut<crate::resources::player::Player>,
    mut egui_ctx: ResMut<EguiContext>,
) {
    info!("setup menu!");

    player.name = "Jones".to_string();

    egui_ctx.ctx_mut().set_visuals(egui::Visuals {
        window_rounding: 0.0.into(),
        ..Default::default()
    });
}

pub fn main_menu_ui_system(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    mut player: ResMut<crate::resources::player::Player>,
    mut ev: EventWriter<AppExit>,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        ui.label(RichText::new("3D Tic Tac Toe").color(Color32::LIGHT_BLUE));
        ui.text_edit_singleline(&mut player.name);
        if ui.button("Start Game").clicked() {
            info!("Start Game");
            commands.insert_resource(NextState(crate::GameState::Connecting));
        }
        if ui.button("Exit").clicked() {
            info!("Exiting!");
            ev.send(AppExit);
        }
    });
}
