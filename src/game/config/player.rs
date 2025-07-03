use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Debug, Serialize, Deserialize, Clone)]
pub struct PlayerConfig {
    pub spawn_position: Vec3,
    pub position: Vec3,
    pub speed: f32,
}

impl Default for PlayerConfig {
    fn default() -> Self {
        Self {
            spawn_position: Vec3::new(0.0, 1.0, 0.0),
            position: Vec3::new(0.0, 1.0, 0.0),
            speed: 5.0,
        }
    }
}
