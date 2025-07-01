use bevy::prelude::*;
use super::{components::FollowCamera, config::CameraConfig};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-5.0, 5.0, 10.0)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FollowCamera,
    ));
}

pub fn camera_follow_system(
    config: Res<CameraConfig>,
    mut query: Query<&mut Transform, With<FollowCamera>>,
) {
    for mut transform in query.iter_mut() {
        // Плавно двигаем камеру к config.position
        transform.translation = transform.translation.lerp(config.position, config.smoothness);

        // Плавно смотрим на цель через look_at (без интерполяции поворота)
        transform.look_at(config.target, Vec3::Y);
    }
}