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
    /// Expects `<path>/manifest.toml` and the scene file referenced by
    /// `manifest.entry_scene`.
    pub fn load(path: &Path) -> Result<Self, MapPackageError> {
        // --- manifest ---
        let manifest_path = path.join("manifest.toml");
        let manifest_str = std::fs::read_to_string(&manifest_path)?;
        let manifest: Manifest = toml::from_str(&manifest_str)?;

        // --- scene ---
        let scene_path = path.join(&manifest.entry_scene);
        let scene_str = std::fs::read_to_string(&scene_path)?;
        let scene: MapScene =
            ron::from_str(&scene_str).map_err(|e| MapPackageError::Ron(e.to_string()))?;

        Ok(MapPackage { manifest, scene })
    }
}
