//! MapPackage scene (scene/main.json) data structures.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Current schema version for scene JSON files
pub const SCHEMA_VERSION: u32 = 1;

/// Root structure of a scene JSON file
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SceneData {
    /// Schema version for compat checks
    pub schema_version: u32,
    /// List of entities in this scene
    #[serde(default)]
    pub entities: Vec<EntityData>,
}

/// An entity in the scene
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EntityData {
    /// Unique identifier within this scene
    pub id: String,
    /// Optional human-readable name
    #[serde(default)]
    pub name: Option<String>,
    /// World-space transform
    #[serde(default)]
    pub transform: TransformData,
    /// Additional components as arbitrary JSON
    #[serde(default)]
    pub components: HashMap<String, serde_json::Value>,
}

/// World-space transform for an entity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransformData {
    /// XYZ translation
    #[serde(default)]
    pub translation: [f32; 3],
    /// Uniform/non-uniform scale
    #[serde(default = "default_scale")]
    pub scale: [f32; 3],
    /// Rotation as XYZW quaternion
    #[serde(default = "default_rotation")]
    pub rotation: [f32; 4],
}

impl Default for TransformData {
    fn default() -> Self {
        Self {
            translation: [0.0, 0.0, 0.0],
            scale: [1.0, 1.0, 1.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
        }
    }
}

fn default_scale() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

fn default_rotation() -> [f32; 4] {
    [0.0, 0.0, 0.0, 1.0]
}

impl SceneData {
    /// Parse a scene from JSON text
    pub fn from_json_str(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }

    /// Serialize to pretty JSON text
    pub fn to_json_string(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
