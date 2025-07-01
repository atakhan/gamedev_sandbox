use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

pub fn show_ui(mut contexts: EguiContexts) {
    egui::Window::new("Debug UI").show(contexts.ctx_mut(), |ui| {
        ui.label("Добро пожаловать в редактор!");
    });
}
