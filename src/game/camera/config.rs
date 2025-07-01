use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct CameraConfig {
    pub position: Vec3,
    pub target: Vec3,
    pub smoothness: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            position: Vec3::new(-5.0, 5.0, 10.0),
            target: Vec3::ZERO,
            smoothness: 0.1,
        }
    }
}
