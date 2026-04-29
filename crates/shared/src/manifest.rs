use serde::{Deserialize, Serialize};

/// Metadata for a map package.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Manifest {
    /// Human-readable map name.
    pub name: String,

    /// Semantic version string, e.g. "0.1.0".
    pub version: String,

    /// Optional author name or contact.
    pub author: Option<String>,

    /// Optional short description.
    pub description: Option<String>,

    /// Minimum iGame engine version required to run this map.
    pub engine_version_min: Option<String>,

    /// Relative path inside the package to the entry scene file.
    pub entry_scene: String,

    /// Optional relative path to a preview image.
    pub preview_image: Option<String>,
}
