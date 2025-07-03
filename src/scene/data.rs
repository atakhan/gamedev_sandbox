use bevy::transform::components::Transform;
use serde::{Serialize, Deserialize};
use crate::game::config::{
    CameraConfig,
    PlayerConfig,
};

/// Главная структура сцены
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SceneData {
    pub name: String,
    pub camera: CameraConfig,
    pub player: PlayerConfig,
}

impl SceneData {
    pub fn from_configs(
        name: &str,
        camera: &CameraConfig,
        player: &PlayerConfig,
        player_transform: Option<&Transform>,
    ) -> Self {
        let current_position = player_transform
            .map(|t| t.translation)
            .unwrap_or(player.spawn_position);

        Self {
            name: name.to_string(),
            camera: CameraConfig { 
                position: camera.position, 
                target: camera.target, 
                smoothness: camera.smoothness,
            },
            player: PlayerConfig { 
                spawn_position: player.spawn_position, 
                position: current_position, 
                speed: player.speed,
            },
        }
    }
}


