use bevy::ecs::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpawnCameraEvent;

impl Event for SpawnCameraEvent {}