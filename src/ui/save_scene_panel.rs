use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
// use crate::scene::manager::save_scene;
// use crate::scene::data::SceneData;

// use crate::game::config::{
//     camera::CameraConfig,
//     player::PlayerConfig,
// };

use crate::ui::state::UiVisibility;

pub fn show_save_scene_panel(
    mut contexts: EguiContexts,
    // camera: Res<CameraConfig>,
    // player: Res<PlayerConfig>,
    mut ui_state: ResMut<UiVisibility>,
) {
    // Временные переменные, чтобы избежать двойного мутабельного заимствования
    let mut show_dialog = ui_state.show_save_scene_dialog;
    let mut scene_name = ui_state.new_scene_name.clone();

    egui::Window::new("Save Scene")
        .open(&mut show_dialog)
        .fixed_size(egui::vec2(300.0, 120.0))
        .show(contexts.ctx_mut(), |ui| {
            ui.label("Scene name:");
            ui.text_edit_singleline(&mut scene_name);

            // if ui.button("Save").clicked() {
            //     if !scene_name.trim().is_empty() {
            //         let data = SceneData::from_configs(&scene_name, &camera, &player);
            //         // save_scene(&data, &format!("scenes/{}.json", scene_name));
            //         info!("Prepare to save Scene '{}'", scene_name);
            //     }
            // }
        });

    // Вернём обратно значения
    ui_state.show_save_scene_dialog = show_dialog;
    ui_state.new_scene_name = scene_name;
}
