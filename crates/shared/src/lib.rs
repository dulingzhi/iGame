//! Shared data structures for iGame: MapPackage format, serialization, and validation.

pub mod manifest;
pub mod map_package;
pub mod scene;
pub mod validation;

pub use manifest::{MapDependencies, MapManifest, MapMeta};
pub use map_package::MapPackage;
pub use scene::{EntityData, SceneData, TransformData};
pub use validation::{validate_manifest, validate_scene, ValidationError};
