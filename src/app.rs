use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use crate::game::{camera_systems, ground_systems};
use crate::game::camera::config::CameraConfig;
use crate::ui::{show_ui, show_camera_panel};

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .insert_resource(CameraConfig::default())
        .add_startup_system(camera_systems::spawn_camera)
        .add_startup_system(ground_systems::spawn_ground)
        .add_system(show_ui)
        .add_system(camera_systems::camera_follow_system)
        .add_system(show_camera_panel)
        .run();
}
