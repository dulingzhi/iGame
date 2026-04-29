use bevy::input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll};
use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::scene::EditorCamera;
use crate::viewport::ViewportState;

/// Persistent orbit-camera parameters.
#[derive(Resource)]
pub struct OrbitCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            focus: Vec3::ZERO,
            radius: 14.0,
            yaw: 0.4,
            pitch: 0.6,
        }
    }
}

impl OrbitCamera {
    /// Returns the camera world position for the current orbit parameters.
    pub fn eye_position(&self) -> Vec3 {
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        self.focus + self.radius * Vec3::new(sin_yaw * cos_pitch, sin_pitch, cos_yaw * cos_pitch)
    }
}

pub fn camera_controller(
    mut orbit: ResMut<OrbitCamera>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    viewport_state: Res<ViewportState>,
    mut egui_ctx: EguiContexts,
    mut camera_q: Query<&mut Transform, With<EditorCamera>>,
) {
    // Don't process input when egui wants it.
    let wants_input = egui_ctx
        .ctx_mut()
        .map(|ctx| ctx.wants_pointer_input())
        .unwrap_or(false);
    if wants_input {
        return;
    }

    // Check mouse is over the viewport panel.
    let mouse_in_viewport = viewport_state
        .panel_rect
        .map(|rect| {
            egui_ctx
                .ctx_mut()
                .map(|ctx| {
                    let pos = ctx.input(|i| i.pointer.hover_pos());
                    pos.map(|p| rect.contains(p)).unwrap_or(false)
                })
                .unwrap_or(false)
        })
        .unwrap_or(false);

    if !mouse_in_viewport {
        return;
    }

    let delta = mouse_motion.delta;

    // Right mouse drag → orbit
    if mouse_buttons.pressed(MouseButton::Right) {
        orbit.yaw -= delta.x * 0.005;
        orbit.pitch = (orbit.pitch - delta.y * 0.005).clamp(-1.4, 1.4);
    }

    // Middle mouse drag → pan
    if mouse_buttons.pressed(MouseButton::Middle) {
        let right = Vec3::new(orbit.yaw.cos(), 0.0, -orbit.yaw.sin());
        let up = Vec3::Y;
        let radius = orbit.radius;
        orbit.focus += right * (-delta.x * radius * 0.002);
        orbit.focus += up * (delta.y * radius * 0.002);
    }

    // Scroll wheel → zoom
    let scroll_y = mouse_scroll.delta.y;
    if scroll_y.abs() > 0.001 {
        orbit.radius = (orbit.radius - scroll_y * 0.5).clamp(0.5, 200.0);
    }

    // Apply to camera transform.
    for mut transform in &mut camera_q {
        let eye = orbit.eye_position();
        *transform = Transform::from_translation(eye).looking_at(orbit.focus, Vec3::Y);
    }
}
