//! RTS-style camera plugin: WASD pan, scroll zoom, middle-click drag.

use bevy::prelude::*;

pub struct RtsCameraPlugin;

impl Plugin for RtsCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, (pan_camera, zoom_camera));
    }
}

#[derive(Component)]
pub struct RtsCamera {
    pub speed: f32,
    pub zoom_speed: f32,
}

impl Default for RtsCamera {
    fn default() -> Self {
        Self {
            speed: 20.0,
            zoom_speed: 5.0,
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        RtsCamera::default(),
    ));
}

fn pan_camera(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&RtsCamera, &mut Transform)>,
) {
    for (cam, mut transform) in &mut query {
        let mut delta = Vec3::ZERO;
        if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
            delta.z -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
            delta.z += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
            delta.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
            delta.x += 1.0;
        }
        if delta != Vec3::ZERO {
            transform.translation += delta.normalize() * cam.speed * time.delta_secs();
        }
    }
}

fn zoom_camera(
    mut scroll: EventReader<bevy::input::mouse::MouseWheel>,
    mut query: Query<(&RtsCamera, &mut Transform)>,
) {
    let scroll_amount: f32 = scroll.read().map(|e| e.y).sum();
    if scroll_amount == 0.0 {
        return;
    }
    for (cam, mut transform) in &mut query {
        let forward = transform.forward();
        transform.translation += forward * scroll_amount * cam.zoom_speed;
    }
}
