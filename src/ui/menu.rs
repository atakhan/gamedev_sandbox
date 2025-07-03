use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use super::state::UiVisibility;

pub fn show_ui(mut contexts: EguiContexts, mut ui_state: ResMut<UiVisibility>) {
    egui::TopBottomPanel::top("menu_bar").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {
            if ui.button("Hide").clicked() {
                // если хотя бы одна панель была включена — выключим всё
                // иначе включим всё
                let is_any_visible = ui_state.show_camera || ui_state.show_player;
                ui_state.show_camera = !is_any_visible;
                ui_state.show_player = !is_any_visible;
            }

            if ui.button("Camera").clicked() {
                // переключаем только панель камеры
                ui_state.show_camera = !ui_state.show_camera;
                ui_state.show_player = false;
            }

            if ui.button("Player").clicked() {
                // переключаем только панель игрока
                ui_state.show_player = !ui_state.show_player;
                ui_state.show_camera = false;
            }
        });
    });
}
