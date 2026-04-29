//! Validation for MapPackage manifest and scene data.

use crate::manifest::MapManifest;
use crate::scene::SceneData;
use thiserror::Error;

/// Validation error for manifest or scene data
#[derive(Debug, Error, PartialEq, Clone)]
pub enum ValidationError {
    #[error("schema_version mismatch: expected {expected}, got {actual}")]
    SchemaMismatch { expected: u32, actual: u32 },

    #[error("required field '{field}' is empty")]
    EmptyRequiredField { field: String },

    #[error("duplicate entity id '{id}'")]
    DuplicateEntityId { id: String },
}

/// Validate a MapManifest, returning a list of errors (empty = valid)
pub fn validate_manifest(manifest: &MapManifest) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    if manifest.schema_version != crate::manifest::SCHEMA_VERSION {
        errors.push(ValidationError::SchemaMismatch {
            expected: crate::manifest::SCHEMA_VERSION,
            actual: manifest.schema_version,
        });
    }

    let meta = &manifest.map;
    for (field, value) in [
        ("map.id", meta.id.as_str()),
        ("map.name", meta.name.as_str()),
        ("map.version", meta.version.as_str()),
        ("map.author", meta.author.as_str()),
        ("map.engine_min_version", meta.engine_min_version.as_str()),
        ("map.entry_scene", meta.entry_scene.as_str()),
    ] {
        if value.is_empty() {
            errors.push(ValidationError::EmptyRequiredField {
                field: field.to_string(),
            });
        }
    }

    errors
}

/// Validate SceneData, returning a list of errors (empty = valid)
pub fn validate_scene(scene: &SceneData) -> Vec<ValidationError> {
    let mut errors = Vec::new();

    if scene.schema_version != crate::scene::SCHEMA_VERSION {
        errors.push(ValidationError::SchemaMismatch {
            expected: crate::scene::SCHEMA_VERSION,
            actual: scene.schema_version,
        });
    }

    let mut seen_ids = std::collections::HashSet::new();
    for entity in &scene.entities {
        if !seen_ids.insert(entity.id.clone()) {
            errors.push(ValidationError::DuplicateEntityId {
                id: entity.id.clone(),
            });
        }
    }

    errors
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::manifest::{MapDependencies, MapManifest, MapMeta};
    use crate::scene::{EntityData, SceneData};

    fn valid_manifest() -> MapManifest {
        MapManifest {
            schema_version: 1,
            map: MapMeta {
                id: "test_map".into(),
                name: "Test Map".into(),
                version: "1.0.0".into(),
                author: "tester".into(),
                engine_min_version: "0.1.0".into(),
                entry_scene: "scene/main.json".into(),
                description: String::new(),
                preview_image: None,
            },
            dependencies: MapDependencies::default(),
        }
    }

    #[test]
    fn valid_manifest_has_no_errors() {
        assert!(validate_manifest(&valid_manifest()).is_empty());
    }

    #[test]
    fn empty_id_is_an_error() {
        let mut m = valid_manifest();
        m.map.id = String::new();
        let errs = validate_manifest(&m);
        assert!(!errs.is_empty());
        assert!(errs
            .iter()
            .any(|e| matches!(e, ValidationError::EmptyRequiredField { field } if field == "map.id")));
    }

    #[test]
    fn schema_mismatch_is_an_error() {
        let mut m = valid_manifest();
        m.schema_version = 99;
        let errs = validate_manifest(&m);
        assert!(errs
            .iter()
            .any(|e| matches!(e, ValidationError::SchemaMismatch { .. })));
    }

    #[test]
    fn duplicate_entity_id_detected() {
        let scene = SceneData {
            schema_version: 1,
            entities: vec![
                EntityData {
                    id: "ent_1".into(),
                    name: None,
                    transform: Default::default(),
                    components: Default::default(),
                },
                EntityData {
                    id: "ent_1".into(),
                    name: None,
                    transform: Default::default(),
                    components: Default::default(),
                },
            ],
        };
        let errs = validate_scene(&scene);
        assert!(errs
            .iter()
            .any(|e| matches!(e, ValidationError::DuplicateEntityId { id } if id == "ent_1")));
    }
}
