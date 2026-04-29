//! MapPackage: a loaded map consisting of manifest + scene data.

use crate::manifest::MapManifest;
use crate::scene::SceneData;
use crate::validation::{validate_manifest, validate_scene, ValidationError};
use anyhow::{Context, Result};

/// A fully loaded and validated map package
#[derive(Debug, Clone)]
pub struct MapPackage {
    pub manifest: MapManifest,
    pub scene: SceneData,
}

impl MapPackage {
    /// Load a MapPackage from manifest TOML text and scene JSON text.
    pub fn from_str(manifest_toml: &str, scene_json: &str) -> Result<Self> {
        let manifest =
            MapManifest::from_toml_str(manifest_toml).context("failed to parse manifest.toml")?;
        let scene =
            SceneData::from_json_str(scene_json).context("failed to parse scene JSON")?;
        Ok(Self { manifest, scene })
    }

    /// Validate both manifest and scene, returning all errors.
    pub fn validate(&self) -> Vec<ValidationError> {
        let mut errors = validate_manifest(&self.manifest);
        errors.extend(validate_scene(&self.scene));
        errors
    }
}
