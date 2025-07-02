use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use super::state::UiVisibility;

pub fn show_ui(mut contexts: EguiContexts, mut ui_state: ResMut<UiVisibility>) {
    egui::TopBottomPanel::top("menu_bar").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            if ui.button("Hide").clicked() {
                ui_state.show_camera = false;
                ui_state.show_player = false;
            }

            if ui.button("Camera").clicked() {
                ui_state.show_camera = true;
                ui_state.show_player = false;
            }

            if ui.button("Player").clicked() {
                ui_state.show_camera = false;
                ui_state.show_player = true;
            }
        });
    });
}
