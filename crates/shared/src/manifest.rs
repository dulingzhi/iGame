//! MapPackage manifest (manifest.toml) data structures.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Current schema version for manifest.toml
pub const SCHEMA_VERSION: u32 = 1;

/// Root structure of manifest.toml
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MapManifest {
    /// Schema version for forward/backward compat checks
    pub schema_version: u32,
    /// Map metadata
    pub map: MapMeta,
    /// Optional dependencies on other map packages
    #[serde(default)]
    pub dependencies: MapDependencies,
}

/// Core metadata about a map
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MapMeta {
    /// Unique machine-readable identifier (snake_case)
    pub id: String,
    /// Human-readable display name
    pub name: String,
    /// SemVer version string
    pub version: String,
    /// Author name or handle
    pub author: String,
    /// Minimum engine version required
    pub engine_min_version: String,
    /// Relative path to the entry scene JSON
    pub entry_scene: String,
    /// Optional human-readable description
    #[serde(default)]
    pub description: String,
    /// Optional relative path to preview image
    #[serde(default)]
    pub preview_image: Option<String>,
}

/// Package dependencies (map id -> version constraint)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MapDependencies {
    #[serde(flatten)]
    pub packages: HashMap<String, String>,
}

impl MapManifest {
    /// Parse a manifest from TOML text
    pub fn from_toml_str(s: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(s)
    }

    /// Serialize to TOML text
    pub fn to_toml_string(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }
}
