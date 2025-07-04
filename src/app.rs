use bevy::prelude::*;
use bevy::ecs::system::IntoSystem; // 👈 обязательно импортируем это!
use bevy_egui::EguiPlugin;

use crate::{
    game::{
        config::{
            camera::CameraConfig, 
            player::PlayerConfig
        }, 
        events::{
            camera as camera_events, 
            player as player_events
        }, 
        camera::systems as camera_systems, 
        ground::systems as ground_systems, 
        player::systems as player_systems
    },
    ui::{
        show_camera_panel, 
        show_menu, 
        show_player_panel, 
        show_scene_manager, 
        ScenePanelsVisibility, 
        ScreenState, 
        Screens
    },
};

pub fn run() {
    App::new()
        // Плагины
        .add_plugins((DefaultPlugins, EguiPlugin))

        // Состояние экрана
        .add_state::<Screens>()

        // События
        .add_event::<player_events::SpawnPlayerEvent>()
        .add_event::<camera_events::SpawnCameraEvent>()

        // Ресурсы
        .insert_resource(CameraConfig::default())
        .insert_resource(PlayerConfig::default())
        .insert_resource(ScenePanelsVisibility::default())
        .insert_resource(ScreenState::default())

        // UI: Меню сцен
        .add_systems(
            Update,
            IntoSystem::into_system(show_scene_manager).run_if(in_state(Screens::SceneManager)),
        )

        // UI: Интерфейс сцены
        .add_systems(
            Update,
            (
                IntoSystem::into_system(show_menu),
                IntoSystem::into_system(show_camera_panel),
                IntoSystem::into_system(show_player_panel),
            )
            .run_if(in_state(Screens::SceneView)),
        )

        // Системы запуска
        .add_systems(
            Startup,
            (
                camera_systems::spawn_camera,
                ground_systems::spawn_ground,
                player_systems::spawn_player,
            ),
        )

        // Основные игровые системы
        .add_systems(
            Update,
            (
                player_systems::respawn_player_system,
                player_systems::player_movement_input_system,
                player_systems::player_apply_velocity_system,
                camera_systems::camera_follow_system,
            ),
        )

        .run();
}
