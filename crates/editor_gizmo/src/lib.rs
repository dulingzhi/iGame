//! `editor_gizmo` — War3-style translate gizmo logic.
//!
//! This crate contains **pure logic** with no external dependencies.  It can
//! be compiled and tested in any environment, including CI without a GPU.
//!
//! ## Modules
//!
//! | Module | Contents |
//! |--------|----------|
//! [`math`] | [`Vec3`], [`Ray`], [`Plane`], [`ray_plane_intersect`] |
//! [`snap`] | [`SnapConfig`], [`snap_value`], [`snap_position`] |
//! [`gizmo`] | [`GizmoState`], [`DragMode`], [`GizmoHandle`], [`HighlightState`] |
//!
//! ## Quick-start
//!
//! ```rust
//! use editor_gizmo::gizmo::{GizmoHandle, GizmoState};
//! use editor_gizmo::math::{Ray, Vec3};
//! use editor_gizmo::snap::SnapConfig;
//!
//! let mut gizmo = GizmoState::new();
//! gizmo.snap = SnapConfig::new(1.0); // 1-unit grid
//!
//! let entity_pos = Vec3::new(5.0, 0.0, 3.0);
//!
//! // Mouse button down — begin dragging on the XZ plane.
//! let pick_ray = Ray::new(Vec3::new(5.0, 10.0, 3.0), Vec3::new(0.0, -1.0, 0.0));
//! gizmo.begin_drag(GizmoHandle::XzPlane, false, entity_pos, pick_ray);
//!
//! // Mouse moved — compute new entity position.
//! let current_ray = Ray::new(Vec3::new(7.4, 10.0, 5.6), Vec3::new(0.0, -1.0, 0.0));
//! let new_pos = gizmo.update_drag(current_ray, false, entity_pos);
//! // new_pos ≈ Some(Vec3 { x: 7.0, y: 0.0, z: 6.0 }) — snapped to 1-unit grid
//!
//! // Mouse button up — end drag.
//! gizmo.end_drag();
//! ```

pub mod gizmo;
pub mod math;
pub mod snap;

// Re-export the most common types at the crate root for convenience.
pub use gizmo::{DragMode, GizmoHandle, GizmoState, HighlightState};
pub use math::{ray_plane_intersect, Plane, Ray, RayPlaneHit, Vec3};
pub use snap::{snap_position, snap_value, SnapConfig};
