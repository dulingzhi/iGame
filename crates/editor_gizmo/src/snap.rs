//! Grid-snapping utility.
//!
//! A configurable step size is used to round world-space positions to the
//! nearest grid point.  When `step == 0.0` snapping is disabled and the
//! position is returned unchanged.

/// Snap a single `f32` value to the nearest multiple of `step`.
///
/// If `step` is zero or negative the value is returned unchanged.
///
/// ```
/// # use editor_gizmo::snap::snap_value;
/// assert_eq!(snap_value(3.7, 1.0), 4.0);
/// assert_eq!(snap_value(3.2, 1.0), 3.0);
/// assert_eq!(snap_value(3.5, 1.0), 4.0);   // ties round away from zero
/// assert_eq!(snap_value(1.6, 0.5), 1.5);
/// ```
pub fn snap_value(value: f32, step: f32) -> f32 {
    if step <= 0.0 {
        return value;
    }
    (value / step).round() * step
}

/// Configuration for the grid-snap feature.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SnapConfig {
    /// Grid cell size.  A value ≤ 0 disables snapping.
    pub step: f32,
    /// Whether snapping is currently active.
    pub enabled: bool,
}

impl SnapConfig {
    /// Snapping disabled.
    pub const DISABLED: SnapConfig = SnapConfig {
        step: 1.0,
        enabled: false,
    };

    /// Create a snap config with the given step, enabled by default.
    pub const fn new(step: f32) -> Self {
        Self { step, enabled: true }
    }
}

impl Default for SnapConfig {
    fn default() -> Self {
        Self::DISABLED
    }
}

/// Apply snapping to a 3-component position (X, Y, Z) when enabled.
///
/// Only the X and Z components are snapped (XZ ground plane); Y is left
/// untouched so that vertical placement remains free.
pub fn snap_position(x: f32, y: f32, z: f32, cfg: &SnapConfig) -> (f32, f32, f32) {
    if !cfg.enabled || cfg.step <= 0.0 {
        return (x, y, z);
    }
    (snap_value(x, cfg.step), y, snap_value(z, cfg.step))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── snap_value ───────────────────────────────────────────────────────────

    #[test]
    fn snap_rounds_to_nearest_integer_step() {
        assert_eq!(snap_value(0.4, 1.0), 0.0);
        assert_eq!(snap_value(0.5, 1.0), 1.0);
        assert_eq!(snap_value(0.6, 1.0), 1.0);
        assert_eq!(snap_value(1.4, 1.0), 1.0);
        assert_eq!(snap_value(1.5, 1.0), 2.0);
    }

    #[test]
    fn snap_with_half_step() {
        assert!((snap_value(1.74, 0.5) - 1.5).abs() < 1e-5);
        assert!((snap_value(1.75, 0.5) - 2.0).abs() < 1e-5);
        assert!((snap_value(0.24, 0.5) - 0.0).abs() < 1e-5);
    }

    #[test]
    fn snap_disabled_when_step_zero() {
        assert_eq!(snap_value(1.7, 0.0), 1.7);
        assert_eq!(snap_value(1.7, -1.0), 1.7);
    }

    #[test]
    fn snap_already_on_grid() {
        assert_eq!(snap_value(2.0, 1.0), 2.0);
        assert_eq!(snap_value(-3.0, 1.0), -3.0);
    }

    #[test]
    fn snap_negative_values() {
        assert!((snap_value(-0.6, 1.0) - -1.0).abs() < 1e-5);
        assert!((snap_value(-1.4, 1.0) - -1.0).abs() < 1e-5);
    }

    #[test]
    fn snap_large_step() {
        assert!((snap_value(7.0, 5.0) - 5.0).abs() < 1e-5);
        assert!((snap_value(8.0, 5.0) - 10.0).abs() < 1e-5);
    }

    // ── snap_position ────────────────────────────────────────────────────────

    #[test]
    fn snap_position_only_affects_xz() {
        let cfg = SnapConfig::new(1.0);
        let (sx, sy, sz) = snap_position(1.6, 2.3, 3.7, &cfg);
        assert!((sx - 2.0).abs() < 1e-5, "x={sx}");
        assert!((sy - 2.3).abs() < 1e-5, "y={sy}"); // Y unchanged
        assert!((sz - 4.0).abs() < 1e-5, "z={sz}");
    }

    #[test]
    fn snap_position_disabled() {
        let cfg = SnapConfig::DISABLED;
        let (sx, sy, sz) = snap_position(1.6, 2.3, 3.7, &cfg);
        assert!((sx - 1.6).abs() < 1e-5);
        assert!((sy - 2.3).abs() < 1e-5);
        assert!((sz - 3.7).abs() < 1e-5);
    }

    #[test]
    fn snap_position_half_step() {
        let cfg = SnapConfig::new(0.5);
        let (sx, _sy, sz) = snap_position(1.3, 0.0, 2.6, &cfg);
        assert!((sx - 1.5).abs() < 1e-5, "x={sx}");
        assert!((sz - 2.5).abs() < 1e-5, "z={sz}");
    }
}
