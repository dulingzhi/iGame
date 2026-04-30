//! Map package data structures: manifest and scene.

use serde::{Deserialize, Serialize};

/// Top-level metadata for a map package (`manifest.toml`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MapManifest {
    /// Human-readable map name.
    pub name: String,
    /// Semantic version string (e.g. `"0.1.0"`).
    pub version: String,
    /// Author display name.
    pub author: String,
    /// Minimum engine version required to load this map.
    pub min_engine_version: String,
    /// Path to the entry scene file inside the package (relative).
    pub entry_scene: String,
    /// Optional short description.
    #[serde(default)]
    pub description: String,
}

impl MapManifest {
    /// Validate that required fields are non-empty.
    ///
    /// Returns `Ok(())` on success or an error string describing the problem.
    pub fn validate(&self) -> Result<(), String> {
        if self.name.is_empty() {
            return Err("manifest.name must not be empty".to_string());
        }
        if self.version.is_empty() {
            return Err("manifest.version must not be empty".to_string());
        }
        if self.entry_scene.is_empty() {
            return Err("manifest.entry_scene must not be empty".to_string());
        }
        Ok(())
    }
}

/// Serialized scene data (`scene/<name>.json`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct SceneData {
    /// List of entities in the scene.
    pub entities: Vec<EntityData>,
}

/// A single entity in the scene.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EntityData {
    /// Logical identifier (unique within the scene).
    pub id: String,
    /// Optional human-readable label.
    #[serde(default)]
    pub label: String,
    /// Component values encoded as JSON objects keyed by component type name.
    pub components: serde_json::Value,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn valid_manifest() -> MapManifest {
        MapManifest {
            name: "Test Map".to_string(),
            version: "0.1.0".to_string(),
            author: "tester".to_string(),
            min_engine_version: "0.1.0".to_string(),
            entry_scene: "scene/main.json".to_string(),
            description: String::new(),
        }
    }

    #[test]
    fn valid_manifest_passes_validation() {
        assert!(valid_manifest().validate().is_ok());
    }

    #[test]
    fn empty_name_fails_validation() {
        let mut m = valid_manifest();
        m.name = String::new();
        assert!(m.validate().is_err());
    }

    #[test]
    fn empty_version_fails_validation() {
        let mut m = valid_manifest();
        m.version = String::new();
        assert!(m.validate().is_err());
    }

    #[test]
    fn empty_entry_scene_fails_validation() {
        let mut m = valid_manifest();
        m.entry_scene = String::new();
        assert!(m.validate().is_err());
    }

    #[test]
    fn manifest_round_trips_toml() {
        let m = valid_manifest();
        let toml_str = toml::to_string(&m).expect("serialise to TOML");
        let m2: MapManifest = toml::from_str(&toml_str).expect("deserialise from TOML");
        assert_eq!(m, m2);
    }

    #[test]
    fn scene_data_round_trips_json() {
        let scene = SceneData {
            entities: vec![EntityData {
                id: "unit_0".to_string(),
                label: "Hero".to_string(),
                components: serde_json::json!({
                    "Transform": { "x": 0.0, "y": 0.0, "z": 0.0 }
                }),
            }],
        };
        let json = serde_json::to_string(&scene).expect("serialise to JSON");
        let scene2: SceneData = serde_json::from_str(&json).expect("deserialise from JSON");
        assert_eq!(scene, scene2);
    }
}
