use bevy::prelude::*;

use crate::game::{
    events::{
        player::SpawnPlayerEvent
    },
    player::{
        components::Player, 
        components::Velocity, 
    },
    config::{
        player::PlayerConfig,
    }
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
        Velocity::default(),
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
            Velocity::default(),
        ));
    }
}

pub fn player_movement_input_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in query.iter_mut() {
        let mut dir = Vec2::ZERO;

        if keyboard.pressed(KeyCode::W) {
            dir.y += 1.0;
        }
        if keyboard.pressed(KeyCode::S) {
            dir.y -= 1.0;
        }
        if keyboard.pressed(KeyCode::A) {
            dir.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::D) {
            dir.x += 1.0;
        }

        velocity.xz = dir.normalize_or_zero() * 2.0; // скорость движения
    }
}

pub fn player_apply_velocity_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Velocity), With<Player>>,
) {
    for (mut transform, velocity) in query.iter_mut() {
        let delta = Vec3::new(velocity.xz.x, 0.0, velocity.xz.y) * time.delta_seconds();
        transform.translation += delta;
    }
}

