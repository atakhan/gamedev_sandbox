use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::scene::manager::list_scenes;
use crate::ui::Screens;

pub fn show_scene_manager(
    screen_state: Res<State<Screens>>,
    mut contexts: EguiContexts,
    mut next_state: ResMut<NextState<Screens>>,
) {
    if *screen_state.get() != Screens::SceneManager {
        return;
    }
    
    egui::CentralPanel::default().show(contexts.ctx_mut(), |ui| {
        ui.heading("Scene Manager");

        let scenes = list_scenes();

        for scene_name in scenes {
            ui.horizontal(|ui| {
                ui.label(&scene_name);
                if ui.button("Go to scene").clicked() {
                    // Переключаемся в экран игры
                    next_state.set(Screens::SceneView);
                    // TODO: здесь вызвать загрузку сцены из файла
                }
            });
        }
    });
}
