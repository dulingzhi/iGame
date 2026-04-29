//! RTS-style camera: WASD pan, scroll-wheel zoom, middle-mouse drag.

use bevy::{
    input::mouse::{MouseScrollUnit, MouseWheel},
    prelude::*,
};

pub struct RtsCameraPlugin;

impl Plugin for RtsCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera)
            .add_systems(Update, camera_movement);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, Transform::from_xyz(0.0, 0.0, 999.9)));
}

fn camera_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut scroll_events: EventReader<MouseWheel>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
    time: Res<Time>,
) {
    const PAN_SPEED: f32 = 400.0;
    const ZOOM_SPEED: f32 = 0.15;
    const ZOOM_MIN: f32 = 0.1;
    const ZOOM_MAX: f32 = 10.0;

    for (mut transform, mut projection) in &mut query {
        // --- Pan ---
        let mut direction = Vec3::ZERO;
        if keys.pressed(KeyCode::KeyW) || keys.pressed(KeyCode::ArrowUp) {
            direction.y += 1.0;
        }
        if keys.pressed(KeyCode::KeyS) || keys.pressed(KeyCode::ArrowDown) {
            direction.y -= 1.0;
        }
        if keys.pressed(KeyCode::KeyA) || keys.pressed(KeyCode::ArrowLeft) {
            direction.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) || keys.pressed(KeyCode::ArrowRight) {
            direction.x += 1.0;
        }
        transform.translation += direction * PAN_SPEED * time.delta_secs();

        // --- Zoom ---
        for event in scroll_events.read() {
            let scroll_delta = match event.unit {
                MouseScrollUnit::Line => event.y,
                MouseScrollUnit::Pixel => event.y / 53.0,
            };
            projection.scale *= 1.0 - scroll_delta * ZOOM_SPEED;
            projection.scale = projection.scale.clamp(ZOOM_MIN, ZOOM_MAX);
        }
    }
}
