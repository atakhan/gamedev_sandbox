use bevy::prelude::*;

use crate::game::player::{
    components::Player, 
    config::PlayerConfig, 
    events::SpawnPlayerEvent
};


pub fn spawn_player(
    mut commands: Commands,
    config: Res<PlayerConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.5,
                depth: 1.5,
                ..default()
            })),
            material: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
            transform: Transform::from_translation(config.spawn_position),
            ..default()
        },
        Player,
    ));
}


pub fn respawn_player_system(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnPlayerEvent>,
    query: Query<Entity, With<Player>>,
    config: Res<PlayerConfig>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for _ in event_reader.iter() {
        // Удаляем всех игроков (в идеале их один)
        for entity in query.iter() {
            commands.entity(entity).despawn_recursive();
        }

        // Спавним нового игрока с актуальной позицией
        commands.spawn((
            PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Capsule {
                    radius: 0.5,
                    depth: 1.5,
                    ..default()
                })),
                material: materials.add(Color::rgb(0.8, 0.2, 0.2).into()),
                transform: Transform::from_translation(config.spawn_position),
                ..default()
            },
            Player,
        ));
    }
}

