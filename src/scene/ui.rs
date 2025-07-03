// use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

// use crate::game::camera::config::CameraConfig;
// use crate::game::player::config::PlayerConfig;
// use crate::scene::manager::{save_scene, load_scene};

pub fn show_scene_panel(
    mut contexts: EguiContexts,
    // camera_config: Res<CameraConfig>,
    // mut player_config: ResMut<PlayerConfig>,
) {
    egui::Window::new("Scenes").show(
        contexts.ctx_mut(), 
        |ui
        | {
        // if ui.button("Save Scene").clicked() {
        //     ui_state.show_save_scene_dialog = true;
        // }

        // if ui.button("Load Scene").clicked() {
        //     if let Some(scene) = load_scene("scenes/default.json") {
        //         player_config.spawn_position = scene.player_position;
        //         // ❗️ камера не пересоздаётся, но мы можем:
        //         // camera_config.target = scene.camera_target;
        //         // в будущем — через события
        //     }
        // }

    });
}
