use bevy_math::{Ray3d, Vec3};

/// Returns the world-space hit point where `ray` intersects the infinite plane
/// defined by `plane_origin` and `plane_normal`, or `None` if the ray is
/// parallel to the plane or the intersection is behind the ray.
pub fn ray_plane_intersection(ray: Ray3d, plane_origin: Vec3, plane_normal: Vec3) -> Option<Vec3> {
    let normal = plane_normal.normalize();
    let dir: Vec3 = *ray.direction;
    let denom = normal.dot(dir);
    if denom.abs() < 1e-6 {
        return None;
    }
    let t = normal.dot(plane_origin - ray.origin) / denom;
    if t < 0.0 {
        return None;
    }
    Some(ray.origin + dir * t)
}

/// Returns the ray parameter `t` of the nearest intersection with the
/// axis-aligned bounding box `[aabb_min, aabb_max]`, or `None` if there is no
/// intersection.
pub fn ray_aabb_intersection(ray: Ray3d, aabb_min: Vec3, aabb_max: Vec3) -> Option<f32> {
    let dir: Vec3 = *ray.direction;
    let inv_dir = Vec3::new(
        if dir.x.abs() > 1e-8 {
            1.0 / dir.x
        } else {
            f32::INFINITY * dir.x.signum()
        },
        if dir.y.abs() > 1e-8 {
            1.0 / dir.y
        } else {
            f32::INFINITY * dir.y.signum()
        },
        if dir.z.abs() > 1e-8 {
            1.0 / dir.z
        } else {
            f32::INFINITY * dir.z.signum()
        },
    );

    let t1 = (aabb_min - ray.origin) * inv_dir;
    let t2 = (aabb_max - ray.origin) * inv_dir;

    let t_min = t1.min(t2);
    let t_max = t1.max(t2);

    let t_enter = t_min.x.max(t_min.y).max(t_min.z);
    let t_exit = t_max.x.min(t_max.y).min(t_max.z);

    if t_enter > t_exit || t_exit < 0.0 {
        return None;
    }

    let t = if t_enter >= 0.0 { t_enter } else { t_exit };
    Some(t)
}

/// Returns the shortest distance from `ray` to the finite line segment that
/// starts at `seg_origin`, goes in direction `seg_dir` (must be unit length)
/// and has length `seg_length`.
pub fn ray_segment_distance(ray: Ray3d, seg_origin: Vec3, seg_dir: Vec3, seg_length: f32) -> f32 {
    let ray_dir: Vec3 = *ray.direction;
    let w0 = ray.origin - seg_origin;

    let a = ray_dir.dot(ray_dir); // always 1 for unit dir
    let b = ray_dir.dot(seg_dir);
    let c = seg_dir.dot(seg_dir); // always 1 for unit dir
    let d = ray_dir.dot(w0);
    let e = seg_dir.dot(w0);

    let denom = a * c - b * b;

    let (sc, tc) = if denom < 1e-8 {
        // Lines nearly parallel
        (0.0_f32, (e / c).clamp(0.0, seg_length))
    } else {
        let sc_raw = (b * e - c * d) / denom;
        let tc_raw = (a * e - b * d) / denom;
        let tc_clamped = tc_raw.clamp(0.0, seg_length);
        // Recompute sc for clamped tc
        let sc_adj = if (tc_raw - tc_clamped).abs() > 1e-8 {
            (b * tc_clamped - d) / a
        } else {
            sc_raw
        };
        (sc_adj.max(0.0), tc_clamped)
    };

    let closest_on_ray = ray.origin + ray_dir * sc;
    let closest_on_seg = seg_origin + seg_dir * tc;
    (closest_on_ray - closest_on_seg).length()
}

/// Projects `hit_world` onto the `axis` through `gizmo_origin`, returning the
/// world-space point that lies on that axis and is closest to `hit_world`.
pub fn gizmo_axis_project(hit_world: Vec3, axis: Vec3, gizmo_origin: Vec3) -> Vec3 {
    let axis_norm = axis.normalize();
    let t = axis_norm.dot(hit_world - gizmo_origin);
    gizmo_origin + axis_norm * t
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use bevy_math::{Dir3, Vec3};

    fn make_ray(origin: Vec3, dir: Vec3) -> Ray3d {
        Ray3d::new(origin, Dir3::new(dir).unwrap())
    }

    // --- ray_plane_intersection ---

    #[test]
    fn plane_hit_xz() {
        // Ray pointing straight down, XZ plane at y=0
        let ray = make_ray(Vec3::new(3.0, 5.0, -2.0), Vec3::NEG_Y);
        let hit = ray_plane_intersection(ray, Vec3::ZERO, Vec3::Y);
        let hit = hit.expect("should hit XZ plane");
        assert!((hit.y).abs() < 1e-5, "hit y={}", hit.y);
        assert!((hit.x - 3.0).abs() < 1e-5);
        assert!((hit.z - -2.0).abs() < 1e-5);
    }

    #[test]
    fn plane_parallel_returns_none() {
        // Ray parallel to XZ plane
        let ray = make_ray(Vec3::new(0.0, 1.0, 0.0), Vec3::X);
        let result = ray_plane_intersection(ray, Vec3::ZERO, Vec3::Y);
        assert!(result.is_none());
    }

    #[test]
    fn plane_behind_ray_returns_none() {
        // Plane is behind the ray origin
        let ray = make_ray(Vec3::new(0.0, -1.0, 0.0), Vec3::NEG_Y);
        let result = ray_plane_intersection(ray, Vec3::ZERO, Vec3::Y);
        assert!(result.is_none());
    }

    #[test]
    fn plane_oblique_hit() {
        // Ray at 45° hitting y=0 plane
        let dir = Vec3::new(1.0, -1.0, 0.0).normalize();
        let ray = make_ray(Vec3::new(0.0, 2.0, 0.0), dir);
        let hit = ray_plane_intersection(ray, Vec3::ZERO, Vec3::Y).unwrap();
        assert!((hit.y).abs() < 1e-5);
        assert!((hit.x - 2.0).abs() < 1e-4, "x={}", hit.x);
    }

    // --- ray_aabb_intersection ---

    #[test]
    fn aabb_direct_hit() {
        let ray = make_ray(Vec3::new(0.0, 0.0, 10.0), Vec3::NEG_Z);
        let t = ray_aabb_intersection(ray, Vec3::splat(-1.0), Vec3::splat(1.0));
        assert!(t.is_some(), "should hit unit cube");
        let t = t.unwrap();
        assert!((t - 9.0).abs() < 1e-4, "t={}", t);
    }

    #[test]
    fn aabb_miss() {
        let ray = make_ray(Vec3::new(5.0, 0.0, 10.0), Vec3::NEG_Z);
        let t = ray_aabb_intersection(ray, Vec3::splat(-1.0), Vec3::splat(1.0));
        assert!(t.is_none());
    }

    #[test]
    fn aabb_ray_inside() {
        // Ray origin is inside the box
        let ray = make_ray(Vec3::ZERO, Vec3::X);
        let t = ray_aabb_intersection(ray, Vec3::splat(-1.0), Vec3::splat(1.0));
        assert!(t.is_some(), "ray inside box should exit");
    }

    #[test]
    fn aabb_behind_ray() {
        let ray = make_ray(Vec3::new(0.0, 0.0, 5.0), Vec3::Z);
        let t = ray_aabb_intersection(ray, Vec3::splat(-1.0), Vec3::splat(1.0));
        assert!(t.is_none(), "box is behind ray");
    }

    // --- ray_segment_distance ---

    #[test]
    fn segment_distance_perpendicular() {
        // Ray along +Z, segment along +X offset by 1 unit in Y
        let ray = make_ray(Vec3::new(0.0, 0.0, 0.0), Vec3::Z);
        let dist = ray_segment_distance(ray, Vec3::new(-2.0, 1.0, 5.0), Vec3::X, 4.0);
        // Closest point on segment to ray is (0, 1, 5); distance should be 1
        assert!((dist - 1.0).abs() < 1e-4, "dist={}", dist);
    }

    #[test]
    fn segment_distance_intersecting() {
        // Ray along +Z, segment along +X both at y=0, crossing at z=3
        let ray = make_ray(Vec3::new(0.0, 0.0, 0.0), Vec3::Z);
        let dist = ray_segment_distance(ray, Vec3::new(-2.0, 0.0, 3.0), Vec3::X, 4.0);
        assert!(dist < 1e-4, "should be nearly 0, got {}", dist);
    }

    #[test]
    fn segment_distance_beyond_segment_end() {
        // Segment ends before closest point — clamps to endpoint
        let ray = make_ray(Vec3::ZERO, Vec3::Z);
        // Segment along +X from x=5, only 1 unit long, at z=3
        let dist = ray_segment_distance(ray, Vec3::new(5.0, 0.0, 3.0), Vec3::X, 1.0);
        // Closest to ray (0,0,3): distance >= 5
        assert!(dist >= 4.9, "dist={}", dist);
    }

    // --- gizmo_axis_project ---

    #[test]
    fn axis_project_x_axis() {
        let hit = Vec3::new(3.0, 2.0, 1.0);
        let projected = gizmo_axis_project(hit, Vec3::X, Vec3::ZERO);
        assert!((projected - Vec3::new(3.0, 0.0, 0.0)).length() < 1e-5);
    }

    #[test]
    fn axis_project_y_axis_offset() {
        let hit = Vec3::new(1.0, 4.0, 2.0);
        let origin = Vec3::new(1.0, 1.0, 2.0);
        let projected = gizmo_axis_project(hit, Vec3::Y, origin);
        assert!((projected - Vec3::new(1.0, 4.0, 2.0)).length() < 1e-5);
    }

    #[test]
    fn axis_project_arbitrary() {
        let origin = Vec3::new(1.0, 1.0, 1.0);
        let axis = Vec3::new(1.0, 1.0, 0.0); // 45° in XY
        let hit = Vec3::new(3.0, 1.0, 5.0); // only X contributes beyond origin
        let projected = gizmo_axis_project(hit, axis, origin);
        // t = axis_norm . (hit - origin) = (2,0,4) . (0.707,0.707,0) = 1.414
        // projected = origin + axis_norm * 1.414
        let axis_norm = axis.normalize();
        let t = axis_norm.dot(hit - origin);
        let expected = origin + axis_norm * t;
        assert!((projected - expected).length() < 1e-5);
    }
}
