use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use crate::{
    game::{
        camera::{
            config::CameraConfig, 
            systems as camera_systems},
        ground::systems as ground_systems,
        player::{
            config::PlayerConfig, 
            systems as player_systems,
            events as player_events,
        },
    },
    ui::{
        show_ui,
        show_camera_panel, 
        show_player_panel,
        UiVisibility,
    },
};


pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .insert_resource(CameraConfig::default())
        .insert_resource(PlayerConfig::default())
        .insert_resource(UiVisibility::default())
        
        // Системы запуска — выполняются один раз при старте
        .add_systems(Startup, (
            camera_systems::spawn_camera,
            ground_systems::spawn_ground,
            player_systems::spawn_player,
        ))
        
        // Основные обновляющие системы
        .add_systems(Update, (
            player_systems::respawn_player_system,
            camera_systems::camera_follow_system,
        ))

        // UI Системы
        .add_systems(Update, (
            show_ui,
            show_camera_panel,
            show_player_panel,
        ))
        
        // Регистрируем события
        .add_event::<player_events::SpawnPlayerEvent>()
        
        .run();
}
