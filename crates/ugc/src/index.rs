//! Package index — a list of available map packages.

use igame_shared::Manifest;
use serde::{Deserialize, Serialize};

/// A single entry in the package index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapEntry {
    /// Unique identifier (e.g. `"author/map-name"`).
    pub id: String,
    /// Parsed manifest for display / compatibility checks.
    pub manifest: Manifest,
    /// Download URL or local filesystem path.
    pub source: String,
}

/// In-memory index of available map packages.
#[derive(Debug, Default)]
pub struct PackageIndex {
    entries: Vec<MapEntry>,
}

impl PackageIndex {
    /// Create an empty index.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a map entry.
    pub fn add(&mut self, entry: MapEntry) {
        self.entries.push(entry);
    }

    /// Find an entry by ID.
    pub fn find(&self, id: &str) -> Option<&MapEntry> {
        self.entries.iter().find(|e| e.id == id)
    }

    /// All entries.
    pub fn all(&self) -> &[MapEntry] {
        &self.entries
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_manifest(name: &str) -> Manifest {
        Manifest {
            name: name.to_string(),
            version: "0.1.0".to_string(),
            author: Some("Test".to_string()),
            description: None,
            engine_version_min: Some("0.1.0".to_string()),
            entry_scene: "scene.ron".to_string(),
            preview_image: None,
        }
    }

    #[test]
    fn find_existing_entry() {
        let mut index = PackageIndex::new();
        index.add(MapEntry {
            id: "test/demo".to_string(),
            manifest: make_manifest("Demo"),
            source: "/tmp/demo".to_string(),
        });
        assert!(index.find("test/demo").is_some());
    }

    #[test]
    fn find_missing_entry_returns_none() {
        let index = PackageIndex::new();
        assert!(index.find("does/not/exist").is_none());
    }

    #[test]
    fn all_returns_all_entries() {
        let mut index = PackageIndex::new();
        index.add(MapEntry {
            id: "a".to_string(),
            manifest: make_manifest("A"),
            source: "/a".to_string(),
        });
        index.add(MapEntry {
            id: "b".to_string(),
            manifest: make_manifest("B"),
            source: "/b".to_string(),
        });
        assert_eq!(index.all().len(), 2);
    }
}
