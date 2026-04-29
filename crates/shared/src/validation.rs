//! Validation logic for map packages.

use thiserror::Error;

use crate::{MapManifest, SceneData};

/// Validation error variants.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ValidationError {
    #[error("manifest field '{field}' is empty")]
    EmptyField { field: &'static str },
    #[error("manifest version '{version}' is not valid semver")]
    InvalidVersion { version: String },
    #[error("entity #{index} has an empty name")]
    EntityEmptyName { index: usize },
    #[error("entity '{name}' has non-positive scale on axis {axis}")]
    InvalidScale { name: String, axis: usize },
}

/// Result type for validation.
pub type ValidationResult = Result<(), Vec<ValidationError>>;

/// Validator that checks manifest and scene data.
pub struct Validator;

impl Validator {
    /// Validate a [`MapManifest`], collecting all errors.
    pub fn validate_manifest(manifest: &MapManifest) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        if manifest.name.trim().is_empty() {
            errors.push(ValidationError::EmptyField { field: "name" });
        }
        if manifest.version.trim().is_empty() {
            errors.push(ValidationError::EmptyField { field: "version" });
        } else if !is_semver_like(&manifest.version) {
            errors.push(ValidationError::InvalidVersion {
                version: manifest.version.clone(),
            });
        }
        if manifest.author.trim().is_empty() {
            errors.push(ValidationError::EmptyField { field: "author" });
        }
        errors
    }

    /// Validate a [`SceneData`], collecting all errors.
    pub fn validate_scene(scene: &SceneData) -> Vec<ValidationError> {
        let mut errors = Vec::new();
        for (i, entity) in scene.entities.iter().enumerate() {
            if entity.name.trim().is_empty() {
                errors.push(ValidationError::EntityEmptyName { index: i });
            }
            for (axis, &s) in entity.transform.scale.iter().enumerate() {
                if s <= 0.0 {
                    errors.push(ValidationError::InvalidScale {
                        name: entity.name.clone(),
                        axis,
                    });
                }
            }
        }
        errors
    }
}

/// Very lenient semver-like check: `MAJOR.MINOR.PATCH` with optional pre-release.
fn is_semver_like(s: &str) -> bool {
    let parts: Vec<&str> = s.splitn(2, '-').collect();
    let version = parts[0];
    let nums: Vec<&str> = version.split('.').collect();
    if nums.len() != 3 {
        return false;
    }
    nums.iter().all(|n| n.parse::<u64>().is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::{EntityDescriptor, TransformDescriptor};
    use crate::MapManifest;

    fn valid_manifest() -> MapManifest {
        MapManifest {
            name: "Test".to_string(),
            version: "0.1.0".to_string(),
            author: "Dev".to_string(),
            description: String::new(),
            engine_min: "0.1.0".to_string(),
            entry_scene: "scene.json".to_string(),
        }
    }

    #[test]
    fn valid_manifest_passes() {
        assert!(Validator::validate_manifest(&valid_manifest()).is_empty());
    }

    #[test]
    fn empty_name_fails() {
        let mut m = valid_manifest();
        m.name = "  ".to_string();
        let errs = Validator::validate_manifest(&m);
        assert!(!errs.is_empty());
    }

    #[test]
    fn invalid_version_fails() {
        let mut m = valid_manifest();
        m.version = "not-semver".to_string();
        let errs = Validator::validate_manifest(&m);
        assert!(errs
            .iter()
            .any(|e| matches!(e, ValidationError::InvalidVersion { .. })));
    }

    #[test]
    fn entity_zero_scale_fails() {
        let scene = SceneData {
            entities: vec![EntityDescriptor {
                name: "Bad".to_string(),
                transform: TransformDescriptor {
                    scale: [0.0, 1.0, 1.0],
                    ..Default::default()
                },
                components: vec![],
            }],
        };
        let errs = Validator::validate_scene(&scene);
        assert!(!errs.is_empty());
    }

    #[test]
    fn valid_scene_passes() {
        let scene = SceneData {
            entities: vec![EntityDescriptor {
                name: "Ground".to_string(),
                transform: TransformDescriptor::default(),
                components: vec![],
            }],
        };
        assert!(Validator::validate_scene(&scene).is_empty());
    }
}
