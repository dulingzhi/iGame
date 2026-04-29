use bevy::prelude::*;
use bevy_egui::EguiContexts;

use crate::scene::EditorCamera;
use crate::selection::Selection;
use crate::viewport::ViewportState;
use igame_runtime::gizmo_axis_project;

/// Length (in world units) of each gizmo arrow.
const ARROW_LEN: f32 = 1.5;
/// Screen-space hover radius in pixels.
const HOVER_RADIUS_PX: f32 = 15.0;

/// Which gizmo axis (if any) is currently being dragged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GizmoAxis {
    X,
    Y,
    Z,
}

/// Runtime drag state for the translate gizmo.
#[derive(Resource, Default)]
pub struct GizmoDrag {
    pub active_axis: Option<GizmoAxis>,
    pub drag_start_world: Vec3,
    pub entity_start_pos: Vec3,
}

/// Draw selection bounding box + translate gizmo arrows.
pub fn draw_gizmo(
    selection: Res<Selection>,
    transforms: Query<&GlobalTransform>,
    mut gizmos: Gizmos,
    drag: Res<GizmoDrag>,
) {
    let Some(entity) = selection.entity else {
        return;
    };
    let Ok(gt) = transforms.get(entity) else {
        return;
    };
    let pos = gt.translation();

    // Bounding box
    let half = Vec3::splat(0.6);
    let min = pos - half;
    let max = pos + half;
    let corners = [
        Vec3::new(min.x, min.y, min.z),
        Vec3::new(max.x, min.y, min.z),
        Vec3::new(max.x, min.y, max.z),
        Vec3::new(min.x, min.y, max.z),
        Vec3::new(min.x, max.y, min.z),
        Vec3::new(max.x, max.y, min.z),
        Vec3::new(max.x, max.y, max.z),
        Vec3::new(min.x, max.y, max.z),
    ];
    let edges = [
        (0, 1),
        (1, 2),
        (2, 3),
        (3, 0), // bottom
        (4, 5),
        (5, 6),
        (6, 7),
        (7, 4), // top
        (0, 4),
        (1, 5),
        (2, 6),
        (3, 7), // verticals
    ];
    let box_color = Color::srgb(1.0, 1.0, 0.0);
    for (a, b) in edges {
        gizmos.line(corners[a], corners[b], box_color);
    }

    // Translate gizmo arrows
    let x_color = if drag.active_axis == Some(GizmoAxis::X) {
        Color::srgb(1.0, 1.0, 0.0)
    } else {
        Color::srgb(1.0, 0.2, 0.2)
    };
    let y_color = if drag.active_axis == Some(GizmoAxis::Y) {
        Color::srgb(1.0, 1.0, 0.0)
    } else {
        Color::srgb(0.2, 1.0, 0.2)
    };
    let z_color = if drag.active_axis == Some(GizmoAxis::Z) {
        Color::srgb(1.0, 1.0, 0.0)
    } else {
        Color::srgb(0.2, 0.2, 1.0)
    };

    gizmos.arrow(pos, pos + Vec3::X * ARROW_LEN, x_color);
    gizmos.arrow(pos, pos + Vec3::Y * ARROW_LEN, y_color);
    gizmos.arrow(pos, pos + Vec3::Z * ARROW_LEN, z_color);
}

/// Handle gizmo mouse interaction: hover + drag to translate the selected entity.
#[allow(clippy::too_many_arguments)]
pub fn handle_gizmo_drag(
    selection: ResMut<Selection>,
    mut drag: ResMut<GizmoDrag>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    viewport_state: Res<ViewportState>,
    mut egui_ctx: EguiContexts,
    camera_q: Query<(&Camera, &GlobalTransform), With<EditorCamera>>,
    mut transforms: Query<&mut Transform>,
    windows: Query<&Window>,
) {
    let Some(entity) = selection.entity else {
        drag.active_axis = None;
        return;
    };

    let ctx_wants_input = egui_ctx
        .ctx_mut()
        .map(|ctx| ctx.wants_pointer_input())
        .unwrap_or(false);
    if ctx_wants_input {
        return;
    }

    let panel_rect = match viewport_state.panel_rect {
        Some(r) => r,
        None => return,
    };

    let cursor_window = match windows.single().ok().and_then(|w| w.cursor_position()) {
        Some(p) => p,
        None => return,
    };

    let egui_pos = bevy_egui::egui::pos2(cursor_window.x, cursor_window.y);
    if !panel_rect.contains(egui_pos) {
        if mouse_buttons.just_released(MouseButton::Left) {
            drag.active_axis = None;
        }
        return;
    }

    let panel_size = panel_rect.size();
    let rel = egui_pos - panel_rect.min;
    let vp_x = rel.x / panel_size.x * crate::viewport::VIEWPORT_WIDTH as f32;
    let vp_y = rel.y / panel_size.y * crate::viewport::VIEWPORT_HEIGHT as f32;
    let viewport_pos = Vec2::new(vp_x, vp_y);

    let Ok((camera, cam_transform)): Result<(&Camera, &GlobalTransform), _> = camera_q.single()
    else {
        return;
    };

    // Current entity world position (read-only first, then mutably below)
    let entity_pos = {
        if let Ok(t) = transforms.get(entity) {
            t.translation
        } else {
            return;
        }
    };

    let axis_tips = [
        (GizmoAxis::X, entity_pos + Vec3::X * ARROW_LEN),
        (GizmoAxis::Y, entity_pos + Vec3::Y * ARROW_LEN),
        (GizmoAxis::Z, entity_pos + Vec3::Z * ARROW_LEN),
    ];

    // Project gizmo handles to screen space for hover detection.
    let mut hovered: Option<GizmoAxis> = None;
    let mut best_screen_dist = HOVER_RADIUS_PX;

    for (axis, tip_world) in &axis_tips {
        let Some(screen_tip) = camera.world_to_viewport(cam_transform, *tip_world).ok() else {
            continue;
        };
        let Some(screen_origin) = camera.world_to_viewport(cam_transform, entity_pos).ok() else {
            continue;
        };
        // Check distance from mouse to the projected line segment origin→tip.
        let pt = Vec2::new(vp_x, vp_y);
        let dist = point_segment_distance_2d(pt, screen_origin, screen_tip);
        if dist < best_screen_dist {
            best_screen_dist = dist;
            hovered = Some(*axis);
        }
    }

    // Drag start
    if mouse_buttons.just_pressed(MouseButton::Left) {
        if let Some(axis) = hovered {
            drag.active_axis = Some(axis);
            drag.entity_start_pos = entity_pos;
            // Compute world hit on the appropriate drag plane.
            if let Ok(ray) = camera.viewport_to_world(cam_transform, viewport_pos) {
                let plane_normal = match axis {
                    GizmoAxis::X | GizmoAxis::Z => Vec3::Y,
                    GizmoAxis::Y => {
                        // Use plane facing camera for Y drag
                        let cam_dir: Vec3 = (cam_transform.translation() - entity_pos).normalize();
                        Vec3::new(cam_dir.x, 0.0, cam_dir.z).normalize()
                    }
                };
                if let Some(hit) =
                    igame_runtime::ray_plane_intersection(ray, entity_pos, plane_normal)
                {
                    drag.drag_start_world = hit;
                }
            }
        }
    }

    // Drag update
    if mouse_buttons.pressed(MouseButton::Left) {
        if let Some(axis) = drag.active_axis {
            if let Ok(ray) = camera.viewport_to_world(cam_transform, viewport_pos) {
                let plane_normal = match axis {
                    GizmoAxis::X | GizmoAxis::Z => Vec3::Y,
                    GizmoAxis::Y => {
                        let cam_dir: Vec3 =
                            (cam_transform.translation() - drag.entity_start_pos).normalize();
                        Vec3::new(cam_dir.x, 0.0, cam_dir.z).normalize()
                    }
                };
                if let Some(hit_world) =
                    igame_runtime::ray_plane_intersection(ray, drag.entity_start_pos, plane_normal)
                {
                    let axis_vec = match axis {
                        GizmoAxis::X => Vec3::X,
                        GizmoAxis::Y => Vec3::Y,
                        GizmoAxis::Z => Vec3::Z,
                    };
                    let projected_start =
                        gizmo_axis_project(drag.drag_start_world, axis_vec, drag.entity_start_pos);
                    let projected_now =
                        gizmo_axis_project(hit_world, axis_vec, drag.entity_start_pos);
                    let delta = projected_now - projected_start;

                    if let Ok(mut transform) = transforms.get_mut(entity) {
                        transform.translation = drag.entity_start_pos + delta;
                    }
                }
            }
        }
    }

    // Drag end
    if mouse_buttons.just_released(MouseButton::Left) {
        drag.active_axis = None;
    }
}

/// 2D point-to-segment distance.
fn point_segment_distance_2d(pt: Vec2, a: Vec2, b: Vec2) -> f32 {
    let ab = b - a;
    let len_sq = ab.length_squared();
    if len_sq < 1e-8 {
        return pt.distance(a);
    }
    let t = ((pt - a).dot(ab) / len_sq).clamp(0.0, 1.0);
    pt.distance(a + ab * t)
}
