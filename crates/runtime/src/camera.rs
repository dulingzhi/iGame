//! RTS camera controller.
//!
//! Controls:
//! - **WASD / Arrow keys** — pan
//! - **Q / E** — rotate around Y axis
//! - **Scroll wheel** — zoom (move camera along its forward axis)
//! - **Middle-mouse drag** — pan
//! - **Right-mouse drag** — orbit

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

/// Marker component for the RTS camera entity.
#[derive(Component)]
pub struct RtsCamera;

/// Plugin that sets up and drives the RTS camera.
pub struct RtsCameraPlugin;

impl Plugin for RtsCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, rts_camera_system);
    }
}

/// Camera movement parameters (can be tweaked).
#[derive(Component)]
pub struct RtsCameraSettings {
    pub pan_speed: f32,
    pub zoom_speed: f32,
    pub rotate_speed: f32,
    /// Minimum distance (zoom in limit).
    pub min_zoom: f32,
    /// Maximum distance (zoom out limit).
    pub max_zoom: f32,
}

impl Default for RtsCameraSettings {
    fn default() -> Self {
        Self {
            pan_speed: 15.0,
            zoom_speed: 2.5,
            rotate_speed: 1.5,
            min_zoom: 3.0,
            max_zoom: 80.0,
        }
    }
}

/// Spawns the RTS camera.  Call this from a startup system.
pub fn spawn_rts_camera(commands: &mut Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
        RtsCamera,
        RtsCameraSettings::default(),
    ));
}

/// Main camera control system.
fn rts_camera_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mut scroll_events: EventReader<MouseWheel>,
    mut motion_events: EventReader<MouseMotion>,
    mut query: Query<(&mut Transform, &RtsCameraSettings), With<RtsCamera>>,
) {
    let Ok((mut transform, settings)) = query.get_single_mut() else {
        return;
    };

    let dt = time.delta_secs();

    // ── Pan (WASD / Arrows) ──────────────────────────────────────────────────
    let forward = {
        let f = transform.forward();
        Vec3::new(f.x, 0.0, f.z).normalize_or_zero()
    };
    let right = {
        let r = transform.right();
        Vec3::new(r.x, 0.0, r.z).normalize_or_zero()
    };

    let mut pan = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) || keyboard.pressed(KeyCode::ArrowUp) {
        pan += forward;
    }
    if keyboard.pressed(KeyCode::KeyS) || keyboard.pressed(KeyCode::ArrowDown) {
        pan -= forward;
    }
    if keyboard.pressed(KeyCode::KeyA) || keyboard.pressed(KeyCode::ArrowLeft) {
        pan -= right;
    }
    if keyboard.pressed(KeyCode::KeyD) || keyboard.pressed(KeyCode::ArrowRight) {
        pan += right;
    }
    transform.translation += pan * settings.pan_speed * dt;

    // ── Rotate (Q / E) ──────────────────────────────────────────────────────
    let pivot = transform.translation - transform.back() * 15.0;
    if keyboard.pressed(KeyCode::KeyQ) {
        let rotation = Quat::from_rotation_y(settings.rotate_speed * dt);
        transform.translation = pivot + rotation * (transform.translation - pivot);
        transform.rotate_y(settings.rotate_speed * dt);
    }
    if keyboard.pressed(KeyCode::KeyE) {
        let rotation = Quat::from_rotation_y(-settings.rotate_speed * dt);
        transform.translation = pivot + rotation * (transform.translation - pivot);
        transform.rotate_y(-settings.rotate_speed * dt);
    }

    // ── Zoom (Scroll wheel) ──────────────────────────────────────────────────
    let scroll: f32 = scroll_events.read().map(|e| e.y).sum();
    if scroll.abs() > 1e-4 {
        let forward_dir = transform.forward().as_vec3();
        let new_pos = transform.translation + forward_dir * scroll * settings.zoom_speed;
        // Clamp by height as a proxy for distance.
        let clamped_y = new_pos.y.clamp(settings.min_zoom, settings.max_zoom);
        let scale = if (new_pos.y).abs() > 1e-4 {
            clamped_y / new_pos.y
        } else {
            1.0
        };
        transform.translation = new_pos * Vec3::new(scale, 1.0, scale);
        transform.translation.y = clamped_y;
    }

    // ── Middle-mouse drag pan ────────────────────────────────────────────────
    if mouse_buttons.pressed(MouseButton::Middle) {
        for event in motion_events.read() {
            transform.translation -= right * event.delta.x * 0.05;
            transform.translation += forward * event.delta.y * 0.05;
        }
    } else {
        // Consume events so they don't pile up.
        motion_events.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn build_camera_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(bevy::input::InputPlugin);
        app.add_plugins(RtsCameraPlugin);
        // Manually spawn camera so we don't need the full window stack.
        app.world_mut().spawn((
            Transform::from_xyz(0.0, 15.0, 15.0).looking_at(Vec3::ZERO, Vec3::Y),
            RtsCamera,
            RtsCameraSettings::default(),
        ));
        app
    }

    #[test]
    fn camera_spawns() {
        let mut app = build_camera_app();
        app.update();
        let mut q = app.world_mut().query::<&RtsCamera>();
        let count = q.iter(app.world()).count();
        assert_eq!(count, 1);
    }

    #[test]
    fn camera_settings_default() {
        let settings = RtsCameraSettings::default();
        assert!(settings.pan_speed > 0.0);
        assert!(settings.min_zoom < settings.max_zoom);
    }
}
