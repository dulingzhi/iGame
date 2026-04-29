//! igame-shared – shared data structures, serialization, and validation.
//!
//! This crate is `no_std`-friendly (no direct OS I/O) and compiles to
//! `wasm32-unknown-unknown` without additional feature flags.

pub mod map;
pub mod trigger;

pub use map::{MapManifest, SceneData};
pub use trigger::TriggerGraph;
