/// Integration tests for pure logic in igame_editor.
/// These run headlessly without any rendering or window systems.
use bevy::prelude::*;

// Re-export from igame_runtime for convenience.
use igame_runtime::{
    gizmo_axis_project, ray_aabb_intersection, ray_plane_intersection, ray_segment_distance,
};

fn make_ray(origin: Vec3, dir: Vec3) -> Ray3d {
    Ray3d::new(origin, Dir3::new(dir).unwrap())
}

// -----------------------------------------------------------------------
// Selection picking via ray_aabb_intersection
// -----------------------------------------------------------------------

#[test]
fn pick_nearest_of_two_objects() {
    // Two cubes on the Z axis; ray shoots from +Z toward -Z.
    let ray = make_ray(Vec3::new(0.0, 0.0, 20.0), Vec3::NEG_Z);

    let cube_near = Vec3::new(0.0, 0.0, 5.0); // center at z=5
    let cube_far = Vec3::new(0.0, 0.0, 2.0); // center at z=2

    let half = Vec3::splat(0.6);
    let t_near = ray_aabb_intersection(ray, cube_near - half, cube_near + half);
    let t_far = ray_aabb_intersection(ray, cube_far - half, cube_far + half);

    assert!(t_near.is_some());
    assert!(t_far.is_some());
    assert!(
        t_near.unwrap() < t_far.unwrap(),
        "nearer cube should have smaller t"
    );
}

#[test]
fn pick_miss_when_ray_is_off_axis() {
    let ray = make_ray(Vec3::new(10.0, 0.0, 20.0), Vec3::NEG_Z);
    let cube = Vec3::ZERO;
    let half = Vec3::splat(0.6);
    let t = ray_aabb_intersection(ray, cube - half, cube + half);
    assert!(t.is_none(), "ray is far off to the side");
}

// -----------------------------------------------------------------------
// Gizmo translate logic
// -----------------------------------------------------------------------

#[test]
fn gizmo_x_axis_drag() {
    // Simulate dragging along +X axis.
    let entity_start = Vec3::new(1.0, 0.5, 1.0);

    // Click starts at x=1, ends at x=3 (both on XZ plane y≈0.5)
    let drag_start = Vec3::new(1.0, 0.5, 1.0);
    let drag_now = Vec3::new(3.0, 0.5, 1.0);

    let proj_start = gizmo_axis_project(drag_start, Vec3::X, entity_start);
    let proj_now = gizmo_axis_project(drag_now, Vec3::X, entity_start);
    let delta = proj_now - proj_start;

    let new_pos = entity_start + delta;
    assert!(
        (new_pos.x - 3.0).abs() < 1e-5,
        "x should be 3, got {}",
        new_pos.x
    );
    assert!(
        (new_pos.y - entity_start.y).abs() < 1e-5,
        "y should not change"
    );
    assert!(
        (new_pos.z - entity_start.z).abs() < 1e-5,
        "z should not change"
    );
}

#[test]
fn gizmo_y_axis_drag() {
    let entity_start = Vec3::new(2.0, 0.0, -1.0);
    let drag_start = Vec3::new(2.0, 0.0, -1.0);
    let drag_now = Vec3::new(2.0, 4.0, -1.0);

    let proj_start = gizmo_axis_project(drag_start, Vec3::Y, entity_start);
    let proj_now = gizmo_axis_project(drag_now, Vec3::Y, entity_start);
    let delta = proj_now - proj_start;

    let new_pos = entity_start + delta;
    assert!(
        (new_pos.y - 4.0).abs() < 1e-5,
        "y should be 4, got {}",
        new_pos.y
    );
    assert!((new_pos.x - entity_start.x).abs() < 1e-5);
    assert!((new_pos.z - entity_start.z).abs() < 1e-5);
}

#[test]
fn gizmo_z_axis_drag() {
    let entity_start = Vec3::ZERO;
    let drag_start = Vec3::ZERO;
    let drag_now = Vec3::new(0.0, 0.0, -5.0);

    let proj_start = gizmo_axis_project(drag_start, Vec3::Z, entity_start);
    let proj_now = gizmo_axis_project(drag_now, Vec3::Z, entity_start);
    let delta = proj_now - proj_start;

    let new_pos = entity_start + delta;
    assert!((new_pos.z - -5.0).abs() < 1e-5);
    assert!((new_pos.x).abs() < 1e-5);
    assert!((new_pos.y).abs() < 1e-5);
}

// -----------------------------------------------------------------------
// Plane intersection for drag plane
// -----------------------------------------------------------------------

#[test]
fn drag_plane_hit_on_xz() {
    // Camera ray from above; drag plane is XZ at y=0.5
    let ray = make_ray(
        Vec3::new(0.0, 10.0, 5.0),
        Vec3::new(0.0, -1.0, -0.5).normalize(),
    );
    let plane_origin = Vec3::new(0.0, 0.5, 0.0);
    let hit = ray_plane_intersection(ray, plane_origin, Vec3::Y);
    assert!(hit.is_some(), "should hit XZ drag plane");
    let hit = hit.unwrap();
    assert!(
        (hit.y - 0.5).abs() < 1e-4,
        "hit.y should be 0.5, got {}",
        hit.y
    );
}

// -----------------------------------------------------------------------
// ray_segment_distance (used for gizmo screen-distance approximation)
// -----------------------------------------------------------------------

#[test]
fn segment_dist_close_to_arrow() {
    // Ray passing very close to a gizmo arrow along X
    let ray = make_ray(Vec3::new(0.0, 0.05, 0.0), Vec3::Z); // almost at y=0
    let seg_origin = Vec3::new(0.0, 0.0, 5.0); // arrow perpendicular to ray
    let dist = ray_segment_distance(ray, seg_origin, Vec3::X, 3.0);
    // Closest point on seg is (0, 0, 5), distance from ray (0, 0.05, 5) ≈ 0.05
    assert!(dist < 0.1, "should be close, got {}", dist);
}

// -----------------------------------------------------------------------
// Minimal headless bevy app smoke-test
// -----------------------------------------------------------------------

#[test]
fn minimal_app_startup() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.update(); // should not panic
}
