//! Core shared types for iGame: map packages, scenes, manifests, triggers, and validation.
//!
//! This crate has **no Bevy dependency** so it can be used in build tools, editors,
//! validators, and tests without pulling in the full engine.

pub mod error;
pub mod manifest;
pub mod map_package;
pub mod scene;
pub mod trigger;
pub mod validate;
// Legacy alias kept for backwards compat with existing tests
pub mod validation;

pub use error::MapPackageError;
pub use manifest::Manifest;
pub use map_package::MapPackage;
pub use scene::{EntityData, MapScene, SpriteData, TransformData};
pub use trigger::{TriggerEdge, TriggerGraph, TriggerNode, VarType, Variable};
pub use validate::validate;
pub use validation::{ValidationError, ValidationResult, Validator};
