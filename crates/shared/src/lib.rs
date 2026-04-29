//! igame-shared — data types, serialization, and validation for iGame map packages.
//!
//! This crate has **no Bevy dependency** so it can be used in build tools, editors,
//! validators, and tests without pulling in the full engine.

pub mod map_package;
pub mod scene;
pub mod trigger;
pub mod validation;

pub use map_package::{MapManifest, MapPackage};
pub use scene::{ComponentRef, EntityDescriptor, SceneData, TransformDescriptor};
pub use trigger::{TriggerAction, TriggerCondition, TriggerEvent, TriggerGraph, TriggerNode};
pub use validation::{ValidationError, ValidationResult, Validator};
