use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use crate::{
    game::{
        config::{
            camera::CameraConfig,
            player::PlayerConfig,
        },
        events::{
            player as player_events,
            camera as camera_events,
        },
        camera::systems as camera_systems,
        ground::systems as ground_systems,
        player::systems as player_systems,
    },
    ui::{
        show_ui,
        show_camera_panel, 
        show_player_panel,
        UiVisibility,
        show_save_scene_panel,
    },
    scene::{
        ui::show_scene_panel
    },
};


pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .insert_resource(CameraConfig::default())
        .insert_resource(PlayerConfig::default())
        .insert_resource(UiVisibility::default())

        // Регистрируем события
        .add_event::<player_events::SpawnPlayerEvent>()
        .add_event::<camera_events::SpawnCameraEvent>()

        // Системы запуска — выполняются один раз при старте
        .add_systems(Startup, (
            camera_systems::spawn_camera,
            ground_systems::spawn_ground,
            player_systems::spawn_player,
        ))
        
        // Основные обновляющие системы
        .add_systems(Update, (
            player_systems::respawn_player_system,
            player_systems::player_movement_input_system,
            player_systems::player_apply_velocity_system,
            camera_systems::camera_follow_system,
        ))

        // UI Системы
        .add_systems(Update, (
            show_ui,
            show_camera_panel,
            show_player_panel,
            show_scene_panel,
            show_save_scene_panel,
        ))

        .run();
}
