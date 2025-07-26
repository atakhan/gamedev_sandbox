// game/player/events.rs
use bevy::ecs::event::Event;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpawnPlayerEvent;

impl Event for SpawnPlayerEvent {}
