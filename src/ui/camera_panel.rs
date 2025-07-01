use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::game::camera::config::CameraConfig;

pub fn show_camera_panel(mut contexts: EguiContexts, mut config: ResMut<CameraConfig>) {
    egui::Window::new("Camera Settings").show(contexts.ctx_mut(), |ui| {
        ui.label("Position:");
        ui.add(egui::Slider::new(&mut config.position.x, -50.0..=50.0).text("X"));
        ui.add(egui::Slider::new(&mut config.position.y, -50.0..=50.0).text("Y"));
        ui.add(egui::Slider::new(&mut config.position.z, -50.0..=50.0).text("Z"));

        ui.separator();

        ui.label("Target:");
        ui.add(egui::Slider::new(&mut config.target.x, -50.0..=50.0).text("X"));
        ui.add(egui::Slider::new(&mut config.target.y, -50.0..=50.0).text("Y"));
        ui.add(egui::Slider::new(&mut config.target.z, -50.0..=50.0).text("Z"));

        ui.separator();

        ui.add(egui::Slider::new(&mut config.smoothness, 0.01..=1.0).text("Smoothness"));
    });
}
