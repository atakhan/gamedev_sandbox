use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Debug, Clone, Copy)]
pub struct Velocity {
    pub xz: Vec2,
}

impl Default for Velocity {
    fn default() -> Self {
        Self {
            xz: Vec2::ZERO,
        }
    }
}
