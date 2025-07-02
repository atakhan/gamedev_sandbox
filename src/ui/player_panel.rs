use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::game::player::{
    config::PlayerConfig, 
    events::SpawnPlayerEvent,
    components::Player,
};
use crate::ui::UiVisibility;


pub fn show_player_panel(
    mut contexts: EguiContexts, 
    mut config: ResMut<PlayerConfig>,
    mut spawn_event_writer: EventWriter<SpawnPlayerEvent>,
    mut query: Query<&mut Transform, With<Player>>,
    ui_state: Res<UiVisibility>,
) {
    if !ui_state.show_player {
        return;
    }

    egui::Window::new("Player Settings")
        .fixed_pos(egui::pos2(10.0, 30.0))
        .fixed_size(egui::vec2(300.0, 180.0))
        .show(contexts.ctx_mut(), |ui| {
            ui.add(egui::Slider::new(&mut config.speed, 0.0..=20.0).text("Speed"));
            
            ui.separator();

            ui.label("Spawn Position:");
            ui.add(egui::Slider::new(&mut config.spawn_position.x, -50.0..=50.0).text("X"));
            ui.add(egui::Slider::new(&mut config.spawn_position.y, 0.0..=10.0).text("Y"));
            ui.add(egui::Slider::new(&mut config.spawn_position.z, -50.0..=50.0).text("Z"));

            if ui.button("Respawn Player").clicked() {
                spawn_event_writer.send(SpawnPlayerEvent);
            }

        });

    egui::Window::new("Player Controls")
        .fixed_pos(egui::pos2(10.0, 220.0))
        .fixed_size(egui::vec2(300.0, 180.0))
        .show(contexts.ctx_mut(), |ui| {
        if let Ok(mut transform) = query.get_single_mut() {
            let mut pos = transform.translation;

            ui.label("Player Position");
            ui.add(egui::DragValue::new(&mut pos.x).prefix("X: ").speed(0.1));
            ui.add(egui::DragValue::new(&mut pos.y).prefix("Y: ").speed(0.1));
            ui.add(egui::DragValue::new(&mut pos.z).prefix("Z: ").speed(0.1));

            // Применяем изменения
            transform.translation = pos;
        } else {
            ui.label("Игрок не найден");
        }
    });
}
