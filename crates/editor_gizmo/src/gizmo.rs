//! War3-style translate gizmo state machine.
//!
//! ## Design overview
//!
//! The gizmo works exclusively in **world space**.  The caller is responsible
//! for converting screen-space mouse deltas into world-space rays and passing
//! them here; this module never touches any rendering or input API.
//!
//! ### Interaction model (War3 conventions)
//!
//! | Action | Effect |
//! |--------|--------|
//! | Drag the entity freely | Move on the **XZ ground plane** (Y unchanged) |
//! | Drag while holding **Shift** | Constrain to **single axis** (X or Z), chosen by largest projected delta |
//! | Drag the **X-axis handle** | Lock to **X axis** only |
//! | Drag the **Z-axis handle** | Lock to **Z axis** only |
//! | Drag the **XZ-plane handle** | Lock to **XZ plane** (same as free drag) |
//! | Release modifier / handle | Revert to free XZ-plane mode |
//!
//! ### Grid snapping
//!
//! A [`crate::snap::SnapConfig`] can be attached.  When `enabled == true` the
//! final world-space position is snapped to the grid after every drag step.

use crate::math::{ray_plane_intersect, Plane, Ray, RayPlaneHit, Vec3};
use crate::snap::SnapConfig;

// ─────────────────────────────────────────────────────────────────────────────
// Public types
// ─────────────────────────────────────────────────────────────────────────────

/// Which movement constraint is currently active for the gizmo.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum DragMode {
    /// Default: move freely on the XZ ground plane.
    XzPlane,
    /// Constrained to the world X axis only.
    AxisX,
    /// Constrained to the world Z axis only.
    AxisZ,
    /// No drag in progress (gizmo is idle).
    #[default]
    Idle,
}

/// The visible part of the gizmo a user can grab.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GizmoHandle {
    /// The centre "free-move" disc — moves on XZ plane.
    XzPlane,
    /// The X-axis arrow/handle.
    AxisX,
    /// The Z-axis arrow/handle.
    AxisZ,
}

/// Highlight state used by the renderer to tint gizmo handles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightState {
    /// The handle is not interacted with.
    Normal,
    /// The cursor hovers over this handle.
    Hovered,
    /// The handle is being dragged.
    Active,
}

// ─────────────────────────────────────────────────────────────────────────────
// GizmoState
// ─────────────────────────────────────────────────────────────────────────────

/// All runtime state for a single translate gizmo instance.
///
/// Create one per selected entity.  Pass mouse events through
/// [`GizmoState::begin_drag`], [`GizmoState::update_drag`], and
/// [`GizmoState::end_drag`].
#[derive(Debug, Clone)]
pub struct GizmoState {
    /// Current drag mode.
    pub mode: DragMode,
    /// World-space position of the entity when dragging started.
    drag_start_entity_pos: Vec3,
    /// World-space hit point on the constraint plane when dragging started.
    drag_start_hit: Vec3,
    /// Snapping configuration.
    pub snap: SnapConfig,
    /// Highlight states for each handle (XZ, AxisX, AxisZ).
    pub highlight: [HighlightState; 3],
    /// Whether a camera-control input is in progress.  When `true`, all gizmo
    /// input is suppressed (camera and gizmo are mutually exclusive).
    pub camera_active: bool,
}

impl GizmoState {
    /// Create a new idle gizmo with snapping disabled.
    pub fn new() -> Self {
        Self {
            mode: DragMode::Idle,
            drag_start_entity_pos: Vec3::ZERO,
            drag_start_hit: Vec3::ZERO,
            snap: SnapConfig::DISABLED,
            highlight: [HighlightState::Normal; 3],
            camera_active: false,
        }
    }

    // ── drag lifecycle ────────────────────────────────────────────────────

    /// Begin a drag interaction.
    ///
    /// * `handle` — which gizmo part was grabbed.
    /// * `shift_held` — if `true`, Shift-key single-axis mode is requested
    ///   (override by handle-based mode when a specific axis handle is used).
    /// * `entity_pos` — world-space position of the entity at drag start.
    /// * `pick_ray` — the picking ray at the mouse-down position.
    ///
    /// Returns the new drag mode, or `None` if the camera is active or the
    /// ray misses the constraint plane.
    pub fn begin_drag(
        &mut self,
        handle: GizmoHandle,
        shift_held: bool,
        entity_pos: Vec3,
        pick_ray: Ray,
    ) -> Option<DragMode> {
        if self.camera_active {
            return None;
        }

        // Determine mode from handle + modifier.
        let mode = match handle {
            GizmoHandle::AxisX => DragMode::AxisX,
            GizmoHandle::AxisZ => DragMode::AxisZ,
            GizmoHandle::XzPlane => {
                if shift_held {
                    // Shift + free-plane handle: we'll choose the dominant axis
                    // during the first update; start with XzPlane until then.
                    DragMode::XzPlane
                } else {
                    DragMode::XzPlane
                }
            }
        };

        // Compute the initial hit on the constraint plane/axis.
        let hit = self.hit_for_mode(mode, entity_pos, pick_ray)?;

        self.mode = mode;
        self.drag_start_entity_pos = entity_pos;
        self.drag_start_hit = hit;
        self.set_handle_highlight(handle, HighlightState::Active);

        Some(mode)
    }

    /// Update an active drag with the current mouse-ray.
    ///
    /// Returns the new **world-space entity position** after applying the drag
    /// delta and optional snapping, or `None` if the drag ray misses the
    /// constraint plane.
    ///
    /// * `current_ray` — picking ray under the current mouse position.
    /// * `shift_held` — allows the caller to notify that Shift was pressed
    ///   mid-drag; when `true` and the current mode is `XzPlane`, the
    ///   dominant axis is selected from the accumulated delta.
    pub fn update_drag(
        &mut self,
        current_ray: Ray,
        shift_held: bool,
        entity_pos_before_drag: Vec3,
    ) -> Option<Vec3> {
        if self.mode == DragMode::Idle || self.camera_active {
            return None;
        }

        // Resolve hit on the constraint plane.
        let hit = self.hit_for_mode(self.mode, self.drag_start_entity_pos, current_ray)?;
        let mut delta = hit - self.drag_start_hit;

        // Shift: choose dominant single axis from the total accumulated delta.
        if shift_held && self.mode == DragMode::XzPlane {
            let total_delta = (entity_pos_before_drag - self.drag_start_entity_pos) + delta;
            self.mode = dominant_axis_mode(total_delta);
            // Recalculate delta with new single-axis mode.
            let hit2 =
                self.hit_for_mode(self.mode, self.drag_start_entity_pos, current_ray)?;
            delta = hit2 - self.drag_start_hit;
        }

        // Apply delta to the entity start position.
        let new_pos = apply_delta(self.drag_start_entity_pos, delta, self.mode);

        // Snap.
        let (sx, sy, sz) =
            crate::snap::snap_position(new_pos.x, new_pos.y, new_pos.z, &self.snap);
        Some(Vec3::new(sx, sy, sz))
    }

    /// End the current drag and return the gizmo to idle.
    pub fn end_drag(&mut self) {
        self.mode = DragMode::Idle;
        self.highlight = [HighlightState::Normal; 3];
    }

    // ── hover ─────────────────────────────────────────────────────────────

    /// Notify the gizmo that the cursor is hovering over `handle` (pass `None`
    /// when no handle is under the cursor).  Only updates highlight; does not
    /// change drag state.
    pub fn set_hover(&mut self, handle: Option<GizmoHandle>) {
        if self.mode != DragMode::Idle {
            // Do not change highlights during active drag.
            return;
        }
        self.highlight = [HighlightState::Normal; 3];
        if let Some(h) = handle {
            self.set_handle_highlight(h, HighlightState::Hovered);
        }
    }

    // ── camera exclusion ──────────────────────────────────────────────────

    /// Inform the gizmo that camera control has started.  Cancels any ongoing
    /// drag.
    pub fn notify_camera_start(&mut self) {
        self.camera_active = true;
        self.end_drag();
    }

    /// Inform the gizmo that camera control has ended.
    pub fn notify_camera_end(&mut self) {
        self.camera_active = false;
    }

    // ── private helpers ───────────────────────────────────────────────────

    /// Find the world-space point on the constraint geometry for `mode`.
    fn hit_for_mode(&self, mode: DragMode, entity_pos: Vec3, ray: Ray) -> Option<Vec3> {
        match mode {
            DragMode::XzPlane | DragMode::Idle => {
                // Intersect with the horizontal XZ plane at the entity's Y.
                let plane = Plane::new(entity_pos, Vec3::Y);
                match ray_plane_intersect(ray, plane) {
                    RayPlaneHit::Hit { point, .. } => Some(point),
                    RayPlaneHit::Parallel => None,
                }
            }
            DragMode::AxisX => {
                // Project onto a vertical plane whose normal is Z, then extract X.
                // We use the plane containing the axis line.
                let plane = Plane::new(entity_pos, Vec3::Z);
                match ray_plane_intersect(ray, plane) {
                    RayPlaneHit::Hit { point, .. } => Some(point),
                    RayPlaneHit::Parallel => {
                        // Fallback: try the XY plane.
                        let plane2 = Plane::new(entity_pos, Vec3::Y);
                        match ray_plane_intersect(ray, plane2) {
                            RayPlaneHit::Hit { point, .. } => Some(point),
                            RayPlaneHit::Parallel => None,
                        }
                    }
                }
            }
            DragMode::AxisZ => {
                // Use a plane with normal X.
                let plane = Plane::new(entity_pos, Vec3::X);
                match ray_plane_intersect(ray, plane) {
                    RayPlaneHit::Hit { point, .. } => Some(point),
                    RayPlaneHit::Parallel => {
                        let plane2 = Plane::new(entity_pos, Vec3::Y);
                        match ray_plane_intersect(ray, plane2) {
                            RayPlaneHit::Hit { point, .. } => Some(point),
                            RayPlaneHit::Parallel => None,
                        }
                    }
                }
            }
        }
    }

    fn set_handle_highlight(&mut self, handle: GizmoHandle, state: HighlightState) {
        let idx = handle_index(handle);
        self.highlight[idx] = state;
    }
}

impl Default for GizmoState {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Free helpers
// ─────────────────────────────────────────────────────────────────────────────

/// Choose the dominant axis (X or Z) based on which component of `delta` has
/// greater magnitude.
fn dominant_axis_mode(delta: Vec3) -> DragMode {
    if delta.x.abs() >= delta.z.abs() {
        DragMode::AxisX
    } else {
        DragMode::AxisZ
    }
}

/// Apply the world-space `delta` to `start` with constraints from `mode`.
fn apply_delta(start: Vec3, delta: Vec3, mode: DragMode) -> Vec3 {
    match mode {
        DragMode::XzPlane | DragMode::Idle => {
            // Move in XZ, preserve Y.
            Vec3::new(start.x + delta.x, start.y, start.z + delta.z)
        }
        DragMode::AxisX => {
            // Move only along X.
            Vec3::new(start.x + delta.x, start.y, start.z)
        }
        DragMode::AxisZ => {
            // Move only along Z.
            Vec3::new(start.x, start.y, start.z + delta.z)
        }
    }
}

fn handle_index(handle: GizmoHandle) -> usize {
    match handle {
        GizmoHandle::XzPlane => 0,
        GizmoHandle::AxisX => 1,
        GizmoHandle::AxisZ => 2,
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Tests
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: a ray pointing straight down from (ox, 10, oz).
    fn down_ray(ox: f32, oz: f32) -> Ray {
        Ray::new(Vec3::new(ox, 10.0, oz), Vec3::new(0.0, -1.0, 0.0))
    }

    // ── dominant_axis_mode ────────────────────────────────────────────────

    #[test]
    fn dominant_axis_prefers_x_when_equal() {
        let delta = Vec3::new(1.0, 0.0, 1.0);
        assert_eq!(dominant_axis_mode(delta), DragMode::AxisX);
    }

    #[test]
    fn dominant_axis_z_when_larger() {
        let delta = Vec3::new(0.5, 0.0, 2.0);
        assert_eq!(dominant_axis_mode(delta), DragMode::AxisZ);
    }

    #[test]
    fn dominant_axis_x_when_larger() {
        let delta = Vec3::new(3.0, 0.0, 0.1);
        assert_eq!(dominant_axis_mode(delta), DragMode::AxisX);
    }

    // ── apply_delta ───────────────────────────────────────────────────────

    #[test]
    fn apply_delta_xz_plane_preserves_y() {
        let start = Vec3::new(1.0, 5.0, 1.0);
        let delta = Vec3::new(2.0, 99.0, 3.0); // y delta should be ignored
        let result = apply_delta(start, delta, DragMode::XzPlane);
        assert!((result.x - 3.0).abs() < 1e-5);
        assert!((result.y - 5.0).abs() < 1e-5, "Y must not change");
        assert!((result.z - 4.0).abs() < 1e-5);
    }

    #[test]
    fn apply_delta_axis_x_only_changes_x() {
        let start = Vec3::new(1.0, 5.0, 1.0);
        let delta = Vec3::new(3.0, 99.0, 7.0);
        let result = apply_delta(start, delta, DragMode::AxisX);
        assert!((result.x - 4.0).abs() < 1e-5);
        assert!((result.y - 5.0).abs() < 1e-5);
        assert!((result.z - 1.0).abs() < 1e-5, "Z must not change");
    }

    #[test]
    fn apply_delta_axis_z_only_changes_z() {
        let start = Vec3::new(1.0, 5.0, 1.0);
        let delta = Vec3::new(7.0, 99.0, 3.0);
        let result = apply_delta(start, delta, DragMode::AxisZ);
        assert!((result.x - 1.0).abs() < 1e-5, "X must not change");
        assert!((result.y - 5.0).abs() < 1e-5);
        assert!((result.z - 4.0).abs() < 1e-5);
    }

    // ── GizmoState drag lifecycle ─────────────────────────────────────────

    #[test]
    fn begin_drag_returns_xz_plane_mode() {
        let mut g = GizmoState::new();
        let entity_pos = Vec3::new(0.0, 0.0, 0.0);
        let ray = down_ray(0.0, 0.0);
        let mode = g.begin_drag(GizmoHandle::XzPlane, false, entity_pos, ray);
        assert_eq!(mode, Some(DragMode::XzPlane));
        assert_eq!(g.mode, DragMode::XzPlane);
    }

    #[test]
    fn begin_drag_axis_x_handle_sets_axis_x_mode() {
        let mut g = GizmoState::new();
        let entity_pos = Vec3::new(5.0, 0.0, 5.0);
        let ray = down_ray(5.0, 5.0);
        let mode = g.begin_drag(GizmoHandle::AxisX, false, entity_pos, ray);
        assert_eq!(mode, Some(DragMode::AxisX));
    }

    #[test]
    fn begin_drag_axis_z_handle_sets_axis_z_mode() {
        let mut g = GizmoState::new();
        let entity_pos = Vec3::new(5.0, 0.0, 5.0);
        let ray = down_ray(5.0, 5.0);
        let mode = g.begin_drag(GizmoHandle::AxisZ, false, entity_pos, ray);
        assert_eq!(mode, Some(DragMode::AxisZ));
    }

    #[test]
    fn begin_drag_blocked_when_camera_active() {
        let mut g = GizmoState::new();
        g.camera_active = true;
        let mode = g.begin_drag(GizmoHandle::XzPlane, false, Vec3::ZERO, down_ray(0.0, 0.0));
        assert_eq!(mode, None);
    }

    /// Free drag on XZ plane moves entity position correctly.
    #[test]
    fn update_drag_xz_plane_displacement() {
        let mut g = GizmoState::new();
        let entity_pos = Vec3::new(0.0, 0.0, 0.0);

        // Start drag at (0,0) on ground.
        g.begin_drag(GizmoHandle::XzPlane, false, entity_pos, down_ray(0.0, 0.0));

        // Move mouse to (3, 0, 4) on ground.
        let new_ray = down_ray(3.0, 4.0);
        let new_pos = g.update_drag(new_ray, false, entity_pos).unwrap();

        assert!((new_pos.x - 3.0).abs() < 1e-4, "x={}", new_pos.x);
        assert!((new_pos.z - 4.0).abs() < 1e-4, "z={}", new_pos.z);
        assert!(new_pos.y.abs() < 1e-4, "y should stay 0");
    }

    /// X-axis drag must not move Z.
    #[test]
    fn update_drag_axis_x_ignores_z() {
        let mut g = GizmoState::new();
        let entity_pos = Vec3::new(0.0, 0.0, 0.0);

        // Drag X-axis handle, start ray over entity.
        g.begin_drag(GizmoHandle::AxisX, false, entity_pos, down_ray(0.0, 0.0));

        // Mouse moved to x=5, z=10 — Z movement should be ignored.
        // For AxisX mode we intersect the Z=const plane.
        // Use a ray that encodes x=5 movement.
        let moved_ray = Ray::new(
            Vec3::new(5.0, 10.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
        );
        let new_pos = g.update_drag(moved_ray, false, entity_pos).unwrap();

        assert!((new_pos.x - 5.0).abs() < 1e-4, "x={}", new_pos.x);
        assert!((new_pos.z - 0.0).abs() < 1e-4, "z must stay 0, got {}", new_pos.z);
    }

    /// Z-axis drag must not move X.
    #[test]
    fn update_drag_axis_z_ignores_x() {
        let mut g = GizmoState::new();
        let entity_pos = Vec3::new(0.0, 0.0, 0.0);

        g.begin_drag(GizmoHandle::AxisZ, false, entity_pos, down_ray(0.0, 0.0));

        let moved_ray = Ray::new(
            Vec3::new(0.0, 10.0, 7.0),
            Vec3::new(0.0, -1.0, 0.0),
        );
        let new_pos = g.update_drag(moved_ray, false, entity_pos).unwrap();

        assert!((new_pos.x - 0.0).abs() < 1e-4, "x must stay 0, got {}", new_pos.x);
        assert!((new_pos.z - 7.0).abs() < 1e-4, "z={}", new_pos.z);
    }

    /// Entity position Y is preserved throughout drag.
    #[test]
    fn update_drag_preserves_entity_y() {
        let mut g = GizmoState::new();
        let entity_pos = Vec3::new(0.0, 3.0, 0.0); // entity is elevated

        g.begin_drag(GizmoHandle::XzPlane, false, entity_pos, down_ray(0.0, 0.0));

        // The constraint plane is at y=3; the ray should hit at y=3.
        let new_ray = Ray::new(Vec3::new(2.0, 13.0, 2.0), Vec3::new(0.0, -1.0, 0.0));
        let new_pos = g.update_drag(new_ray, false, entity_pos).unwrap();

        assert!((new_pos.y - 3.0).abs() < 1e-4, "y should stay 3, got {}", new_pos.y);
    }

    /// Snapping rounds the final position to the nearest grid point.
    #[test]
    fn update_drag_with_snapping() {
        let mut g = GizmoState::new();
        g.snap = SnapConfig::new(1.0);

        let entity_pos = Vec3::new(0.0, 0.0, 0.0);
        g.begin_drag(GizmoHandle::XzPlane, false, entity_pos, down_ray(0.0, 0.0));

        // Move to (1.3, _, 2.7) — should snap to (1.0, _, 3.0).
        let new_pos = g.update_drag(down_ray(1.3, 2.7), false, entity_pos).unwrap();
        assert!((new_pos.x - 1.0).abs() < 1e-4, "x={}", new_pos.x);
        assert!((new_pos.z - 3.0).abs() < 1e-4, "z={}", new_pos.z);
    }

    /// end_drag resets mode to Idle and clears highlights.
    #[test]
    fn end_drag_resets_state() {
        let mut g = GizmoState::new();
        let entity_pos = Vec3::new(0.0, 0.0, 0.0);
        g.begin_drag(GizmoHandle::XzPlane, false, entity_pos, down_ray(0.0, 0.0));
        assert_ne!(g.mode, DragMode::Idle);
        g.end_drag();
        assert_eq!(g.mode, DragMode::Idle);
        assert!(g.highlight.iter().all(|h| *h == HighlightState::Normal));
    }

    // ── camera exclusion ──────────────────────────────────────────────────

    #[test]
    fn camera_start_cancels_drag() {
        let mut g = GizmoState::new();
        let entity_pos = Vec3::new(0.0, 0.0, 0.0);
        g.begin_drag(GizmoHandle::XzPlane, false, entity_pos, down_ray(0.0, 0.0));
        assert_eq!(g.mode, DragMode::XzPlane);

        g.notify_camera_start();
        assert_eq!(g.mode, DragMode::Idle);
        assert!(g.camera_active);
    }

    #[test]
    fn drag_blocked_while_camera_active() {
        let mut g = GizmoState::new();
        g.notify_camera_start();
        let result = g.begin_drag(GizmoHandle::XzPlane, false, Vec3::ZERO, down_ray(0.0, 0.0));
        assert_eq!(result, None);
    }

    #[test]
    fn drag_allowed_after_camera_end() {
        let mut g = GizmoState::new();
        g.notify_camera_start();
        g.notify_camera_end();
        let result = g.begin_drag(GizmoHandle::XzPlane, false, Vec3::ZERO, down_ray(0.0, 0.0));
        assert!(result.is_some());
    }

    // ── hover highlights ──────────────────────────────────────────────────

    #[test]
    fn set_hover_updates_correct_handle() {
        let mut g = GizmoState::new();
        g.set_hover(Some(GizmoHandle::AxisX));
        assert_eq!(g.highlight[0], HighlightState::Normal); // XzPlane
        assert_eq!(g.highlight[1], HighlightState::Hovered); // AxisX
        assert_eq!(g.highlight[2], HighlightState::Normal); // AxisZ
    }

    #[test]
    fn set_hover_none_clears_all() {
        let mut g = GizmoState::new();
        g.set_hover(Some(GizmoHandle::AxisZ));
        g.set_hover(None);
        assert!(g.highlight.iter().all(|h| *h == HighlightState::Normal));
    }

    #[test]
    fn hover_does_not_change_during_active_drag() {
        let mut g = GizmoState::new();
        let entity_pos = Vec3::new(0.0, 0.0, 0.0);
        g.begin_drag(GizmoHandle::AxisX, false, entity_pos, down_ray(0.0, 0.0));
        let before = g.highlight;
        g.set_hover(Some(GizmoHandle::AxisZ)); // should be ignored
        assert_eq!(g.highlight, before);
    }
}
