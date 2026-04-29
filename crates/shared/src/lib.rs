//! Core shared types for iGame: map packages, scenes, manifests, and validation.

pub mod error;
pub mod manifest;
pub mod map_package;
pub mod scene;
pub mod validate;

pub use error::MapPackageError;
pub use manifest::Manifest;
pub use map_package::MapPackage;
pub use scene::{EntityData, MapScene, SpriteData, TransformData};
pub use validate::validate;
