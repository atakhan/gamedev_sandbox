use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    game::{
        player::components::Player, 
        CameraConfig, 
        PlayerConfig
    }, 
    scene::{
        data::SceneData,
        manager::save_scene,
    }, 
    ui::UiVisibility
};

pub fn show_camera_panel(
    mut contexts: EguiContexts, 
    mut camera_config: ResMut<CameraConfig>,
    player_config: ResMut<PlayerConfig>,
    player_query: Query<&Transform, With<Player>>,
    ui_state: Res<UiVisibility>,
) {
    if !ui_state.show_camera {
        return;
    }
    egui::Window::new("Camera Settings")
        .fixed_pos(egui::pos2(10.0, 30.0))
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Position:");
            ui.add(egui::Slider::new(&mut camera_config.position.x, -50.0..=50.0).text("X"));
            ui.add(egui::Slider::new(&mut camera_config.position.y, -50.0..=50.0).text("Y"));
            ui.add(egui::Slider::new(&mut camera_config.position.z, -50.0..=50.0).text("Z"));

            ui.separator();

            ui.label("Target:");
            ui.add(egui::Slider::new(&mut camera_config.target.x, -50.0..=50.0).text("X"));
            ui.add(egui::Slider::new(&mut camera_config.target.y, -50.0..=50.0).text("Y"));
            ui.add(egui::Slider::new(&mut camera_config.target.z, -50.0..=50.0).text("Z"));

            ui.separator();
            
            ui.add(egui::Slider::new(&mut camera_config.smoothness, 0.01..=1.0).text("Smoothness"));
            
            ui.separator();

            if ui.button("Save Scene").clicked() {
                let scene_data = SceneData::from_configs(
                    "quick_save",
                    &camera_config,
                    &player_config,
                    player_query.get_single().ok(), // ❗ важно
                );
                let _ = save_scene(&scene_data);
            }
        });
}
