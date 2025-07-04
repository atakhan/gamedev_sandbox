use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use super::state::{ScenePanelsVisibility, Screens};

pub fn show_menu(
    mut contexts: EguiContexts, 
    mut ui_state: ResMut<ScenePanelsVisibility>,
    mut next_screen_state: ResMut<NextState<Screens>>,
) {
    egui::TopBottomPanel::top("menu_bar").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|ui| {

            if ui.button("Scenes").clicked() {
                next_screen_state.set(Screens::SceneManager);
                
                ui_state.show_camera = false;
                ui_state.show_player = false;
            }

            if ui.button("Camera").clicked() {
                next_screen_state.set(Screens::SceneView);
                ui_state.show_camera = !ui_state.show_camera;
                ui_state.show_player = false;
            }

            if ui.button("Player").clicked() {
                next_screen_state.set(Screens::SceneView);
                ui_state.show_player = !ui_state.show_player;
                ui_state.show_camera = false;
            }
            
            if ui.button("Hide").clicked() {
                next_screen_state.set(Screens::SceneView);
                ui_state.show_camera = false;
                ui_state.show_player = false;
            }

        });
    });
}
