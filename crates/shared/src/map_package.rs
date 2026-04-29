//! Map package manifest and top-level package type.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::scene::SceneData;

/// Top-level map package.  Owns a parsed manifest and scene.
#[derive(Debug, Clone)]
pub struct MapPackage {
    pub manifest: MapManifest,
    pub scene: SceneData,
}

impl MapPackage {
    /// Load a `MapPackage` from a manifest TOML string and a scene JSON string.
    pub fn from_strings(manifest_toml: &str, scene_json: &str) -> Result<Self, MapPackageError> {
        let manifest: MapManifest = toml::from_str(manifest_toml)
            .map_err(|e| MapPackageError::ManifestParse(e.to_string()))?;
        let scene: SceneData = serde_json::from_str(scene_json)
            .map_err(|e| MapPackageError::SceneParse(e.to_string()))?;
        Ok(Self { manifest, scene })
    }
}

/// Manifest stored in `manifest.toml` at the root of a map package.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MapManifest {
    /// Human-readable map name.
    pub name: String,
    /// SemVer string, e.g. `"0.1.0"`.
    pub version: String,
    /// Author display name.
    pub author: String,
    /// Description shown in map selection UI.
    #[serde(default)]
    pub description: String,
    /// Minimum engine version required to load this map.
    #[serde(default = "default_engine_min")]
    pub engine_min: String,
    /// Relative path to the entry scene file (JSON).
    #[serde(default = "default_entry_scene")]
    pub entry_scene: String,
}

fn default_engine_min() -> String {
    "0.1.0".to_string()
}

fn default_entry_scene() -> String {
    "scene.json".to_string()
}

/// Errors that can arise when loading a [`MapPackage`].
#[derive(Debug, Error)]
pub enum MapPackageError {
    #[error("failed to parse manifest.toml: {0}")]
    ManifestParse(String),
    #[error("failed to parse scene.json: {0}")]
    SceneParse(String),
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;

    const MANIFEST_TOML: &str = r#"
name        = "Test Map"
version     = "0.1.0"
author      = "Test Author"
description = "A map for unit tests"
engine_min  = "0.1.0"
entry_scene = "scene.json"
"#;

    #[test]
    fn parse_valid_manifest() {
        let m: MapManifest = toml::from_str(MANIFEST_TOML).unwrap();
        assert_eq!(m.name, "Test Map");
        assert_eq!(m.version, "0.1.0");
        assert_eq!(m.author, "Test Author");
    }

    #[test]
    fn manifest_defaults() {
        let minimal = "name = \"X\"\nversion = \"0.1.0\"\nauthor = \"A\"";
        let m: MapManifest = toml::from_str(minimal).unwrap();
        assert_eq!(m.entry_scene, "scene.json");
        assert_eq!(m.engine_min, "0.1.0");
    }

    #[test]
    fn map_package_from_strings() {
        let scene_json = r#"{"entities":[]}"#;
        let pkg = MapPackage::from_strings(MANIFEST_TOML, scene_json).unwrap();
        assert_eq!(pkg.manifest.name, "Test Map");
        assert!(pkg.scene.entities.is_empty());
    }

    #[test]
    fn bad_manifest_returns_error() {
        let err = MapPackage::from_strings("NOT TOML {{{{", r#"{"entities":[]}"#);
        assert!(err.is_err());
    }
}
