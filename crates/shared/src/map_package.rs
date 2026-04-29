//! Map package — top-level container for manifest + scene data.

use std::path::Path;

use crate::{error::MapPackageError, manifest::Manifest, scene::MapScene};

/// A fully-loaded map package, ready to be instantiated by the runtime.
#[derive(Debug, Clone)]
pub struct MapPackage {
    /// Package metadata.
    pub manifest: Manifest,

    /// Scene entities and their components.
    pub scene: MapScene,
}

impl MapPackage {
    /// Load a map package from a directory on disk.
    ///
    /// Expects `<dir>/manifest.toml` and the scene file referenced by
    /// `manifest.entry_scene` (RON format).
    pub fn load(path: &Path) -> Result<Self, MapPackageError> {
        let manifest_str = std::fs::read_to_string(path.join("manifest.toml"))?;
        let manifest: Manifest = toml::from_str(&manifest_str)?;

        let scene_str = std::fs::read_to_string(path.join(&manifest.entry_scene))?;
        let scene: MapScene =
            ron::from_str(&scene_str).map_err(|e| MapPackageError::Ron(e.to_string()))?;

        Ok(MapPackage { manifest, scene })
    }

    /// Load a map package from in-memory strings (useful for tests and embedded maps).
    ///
    /// `manifest_toml` is TOML text, `scene_ron` is RON text.
    pub fn from_strings(manifest_toml: &str, scene_ron: &str) -> Result<Self, MapPackageError> {
        let manifest: Manifest = toml::from_str(manifest_toml)?;
        let scene: MapScene =
            ron::from_str(scene_ron).map_err(|e| MapPackageError::Ron(e.to_string()))?;
        Ok(MapPackage { manifest, scene })
    }
}
