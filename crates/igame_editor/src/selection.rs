use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::scene::{EditorCamera, SceneObject};
use crate::viewport::ViewportState;
use igame_runtime::ray_aabb_intersection;

/// Holds the currently selected entity (if any).
#[derive(Resource, Default)]
pub struct Selection {
    pub entity: Option<Entity>,
}

/// Estimated half-extent used for AABB picking when the entity has no computed
/// AABB component.
const PICK_HALF_EXTENT: f32 = 0.6;

pub fn handle_selection(
    mut selection: ResMut<Selection>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    viewport_state: Res<ViewportState>,
    mut egui_ctx: EguiContexts,
    camera_q: Query<(&Camera, &GlobalTransform), With<EditorCamera>>,
    objects: Query<(Entity, &GlobalTransform, Option<&Name>), With<SceneObject>>,
    windows: Query<&Window>,
) {
    // Only act on a fresh left click.
    if !mouse_buttons.just_pressed(MouseButton::Left) {
        return;
    }

    // Don't steal clicks that egui is consuming.
    let wants_input = egui_ctx
        .ctx_mut()
        .map(|ctx| ctx.wants_pointer_input())
        .unwrap_or(false);
    if wants_input {
        return;
    }

    let panel_rect = match viewport_state.panel_rect {
        Some(r) => r,
        None => return,
    };

    // Cursor position in window pixels.
    let cursor_window = match windows.single().ok().and_then(|w| w.cursor_position()) {
        Some(p) => p,
        None => return,
    };

    // Check cursor is inside the viewport panel.
    let egui_pos = bevy_egui::egui::pos2(cursor_window.x, cursor_window.y);
    if !panel_rect.contains(egui_pos) {
        return;
    }
    let panel_size = panel_rect.size();
    let rel = egui_pos - panel_rect.min;
    let vp_x = rel.x / panel_size.x * crate::viewport::VIEWPORT_WIDTH as f32;
    let vp_y = rel.y / panel_size.y * crate::viewport::VIEWPORT_HEIGHT as f32;

    let Ok((camera, cam_transform)): Result<(&Camera, &GlobalTransform), _> = camera_q.single()
    else {
        return;
    };

    let viewport_pos = Vec2::new(vp_x, vp_y);
    let Ok(ray) = camera.viewport_to_world(cam_transform, viewport_pos) else {
        return;
    };

    // Pick nearest entity whose estimated AABB the ray hits.
    let mut best_t = f32::MAX;
    let mut best_entity: Option<Entity> = None;

    for (entity, global_transform, _name) in &objects {
        let center = global_transform.translation();
        let aabb_min = center - Vec3::splat(PICK_HALF_EXTENT);
        let aabb_max = center + Vec3::splat(PICK_HALF_EXTENT);
        if let Some(t) = ray_aabb_intersection(ray, aabb_min, aabb_max) {
            if t < best_t {
                best_t = t;
                best_entity = Some(entity);
            }
        }
    }

    selection.entity = best_entity;
}
