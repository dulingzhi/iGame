//! Pure math helpers for the editor gizmo.
//!
//! All types use plain `f32` and standard library only — no external crates
//! required, so these functions are fully testable without a graphics context.

/// A 3-component vector (x, y, z).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    /// Zero vector.
    pub const ZERO: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

    /// Unit vector along X.
    pub const X: Vec3 = Vec3 { x: 1.0, y: 0.0, z: 0.0 };

    /// Unit vector along Y.
    pub const Y: Vec3 = Vec3 { x: 0.0, y: 1.0, z: 0.0 };

    /// Unit vector along Z.
    pub const Z: Vec3 = Vec3 { x: 0.0, y: 0.0, z: 1.0 };

    /// Create a new vector.
    #[inline]
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Dot product.
    #[inline]
    pub fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Scale by a scalar.
    #[inline]
    pub fn scale(self, s: f32) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }

    /// Project this vector onto `axis` (axis must be a unit vector).
    #[inline]
    pub fn project_onto_axis(self, axis: Vec3) -> Vec3 {
        axis.scale(self.dot(axis))
    }
}

impl std::fmt::Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    #[inline]
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    #[inline]
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

/// A ray defined by an origin and a (unit) direction.
#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    /// Create a ray; `direction` should be normalised by the caller.
    #[inline]
    pub const fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Evaluate the point at parameter `t`: `origin + t * direction`.
    #[inline]
    pub fn at(self, t: f32) -> Vec3 {
        self.origin + self.direction.scale(t)
    }
}

/// An infinite plane defined by a point on the plane and its normal.
#[derive(Debug, Clone, Copy)]
pub struct Plane {
    /// A point that lies on the plane.
    pub point: Vec3,
    /// The unit normal of the plane.
    pub normal: Vec3,
}

impl Plane {
    /// Create a plane.
    #[inline]
    pub const fn new(point: Vec3, normal: Vec3) -> Self {
        Self { point, normal }
    }

    /// The XZ ground plane (y = 0, normal = +Y).
    pub const XZ: Plane = Plane {
        point: Vec3::ZERO,
        normal: Vec3::Y,
    };
}

/// Result of a ray–plane intersection test.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RayPlaneHit {
    /// The ray intersects the plane at world-space position `hit` with ray
    /// parameter `t`.
    Hit { t: f32, point: Vec3 },
    /// The ray is parallel to (or lies in) the plane — no single intersection.
    Parallel,
}

/// Intersect a ray with an infinite plane.
///
/// Returns [`RayPlaneHit::Hit`] when `|denom| > epsilon`, otherwise
/// [`RayPlaneHit::Parallel`].
///
/// # Formula
/// ```text
/// t = dot(plane.point - ray.origin, plane.normal)
///     ─────────────────────────────────────────
///              dot(ray.direction, plane.normal)
/// hit = ray.origin + t * ray.direction
/// ```
pub fn ray_plane_intersect(ray: Ray, plane: Plane) -> RayPlaneHit {
    const EPSILON: f32 = 1e-6;
    let denom = ray.direction.dot(plane.normal);
    if denom.abs() < EPSILON {
        return RayPlaneHit::Parallel;
    }
    let t = (plane.point - ray.origin).dot(plane.normal) / denom;
    RayPlaneHit::Hit {
        t,
        point: ray.at(t),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── ray_plane_intersect ──────────────────────────────────────────────────

    /// Ray pointing straight down from above hits the XZ plane at the origin.
    #[test]
    fn ray_hits_xz_plane_at_origin() {
        let ray = Ray::new(Vec3::new(0.0, 5.0, 0.0), Vec3::new(0.0, -1.0, 0.0));
        match ray_plane_intersect(ray, Plane::XZ) {
            RayPlaneHit::Hit { t, point } => {
                assert!((t - 5.0).abs() < 1e-5, "t={t}");
                assert!(point.y.abs() < 1e-5, "y={}", point.y);
                assert!(point.x.abs() < 1e-5);
                assert!(point.z.abs() < 1e-5);
            }
            RayPlaneHit::Parallel => panic!("expected hit, got parallel"),
        }
    }

    /// Diagonal ray from camera position hits at a predictable XZ point.
    #[test]
    fn ray_hits_xz_plane_offset() {
        // Camera at (10, 10, 10), looking toward (10, 0, 10)
        let ray = Ray::new(Vec3::new(10.0, 10.0, 10.0), Vec3::new(0.0, -1.0, 0.0));
        match ray_plane_intersect(ray, Plane::XZ) {
            RayPlaneHit::Hit { t, point } => {
                assert!((t - 10.0).abs() < 1e-5);
                assert!((point.x - 10.0).abs() < 1e-5);
                assert!(point.y.abs() < 1e-5);
                assert!((point.z - 10.0).abs() < 1e-5);
            }
            RayPlaneHit::Parallel => panic!("expected hit"),
        }
    }

    /// Ray that is perfectly horizontal (parallel to XZ) should return Parallel.
    #[test]
    fn horizontal_ray_is_parallel_to_xz() {
        let ray = Ray::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        assert_eq!(ray_plane_intersect(ray, Plane::XZ), RayPlaneHit::Parallel);
    }

    /// Ray hitting a tilted plane (normal = +X, plane through origin).
    #[test]
    fn ray_hits_yz_plane() {
        let plane = Plane::new(Vec3::ZERO, Vec3::X); // YZ plane
        let ray = Ray::new(Vec3::new(-5.0, 3.0, 2.0), Vec3::new(1.0, 0.0, 0.0));
        match ray_plane_intersect(ray, plane) {
            RayPlaneHit::Hit { t, point } => {
                assert!((t - 5.0).abs() < 1e-5);
                assert!(point.x.abs() < 1e-5);
                assert!((point.y - 3.0).abs() < 1e-5);
                assert!((point.z - 2.0).abs() < 1e-5);
            }
            RayPlaneHit::Parallel => panic!("expected hit"),
        }
    }

    /// Ray pointing away from the plane — t should be negative (behind origin).
    #[test]
    fn ray_pointing_away_gives_negative_t() {
        // Ray pointing upward from y=0, plane is XZ at y=0.
        // Intersection is behind the origin (t < 0).
        let ray = Ray::new(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 0.0));
        match ray_plane_intersect(ray, Plane::XZ) {
            RayPlaneHit::Hit { t, .. } => {
                assert!(t < 0.0, "expected t<0, got t={t}");
            }
            RayPlaneHit::Parallel => panic!("expected hit"),
        }
    }

    // ── Vec3 helpers ─────────────────────────────────────────────────────────

    #[test]
    fn vec3_dot() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(4.0, 5.0, 6.0);
        assert!((a.dot(b) - 32.0).abs() < 1e-6);
    }

    #[test]
    fn vec3_project_onto_axis() {
        let v = Vec3::new(3.0, 7.0, 2.0);
        let axis = Vec3::X;
        let projected = v.project_onto_axis(axis);
        assert!((projected.x - 3.0).abs() < 1e-6);
        assert!(projected.y.abs() < 1e-6);
        assert!(projected.z.abs() < 1e-6);
    }
}
