use bevy::prelude::*;
use super::components::Ground;

pub fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Mesh::from(shape::Plane {
        size: 20.0,
        subdivisions: 0,
    }));

    let material = materials.add(StandardMaterial {
        base_color: Color::rgb(0.2, 0.2, 0.2),
        ..default()
    });

    commands.spawn((
        PbrBundle {
            mesh,
            material,
            ..default()
        },
        Ground,
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}
