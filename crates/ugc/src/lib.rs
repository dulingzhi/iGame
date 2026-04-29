//! igame-ugc — UGC package management (stub).
//!
//! Planned features (M6 / M7):
//! - Map package index (local and remote)
//! - Dependency resolution and conflict detection
//! - Package download, caching, and integrity verification
//! - Version migration framework
//! - Publisher signing (optional)

pub mod index;
pub mod registry;

pub use index::{MapEntry, PackageIndex};
pub use registry::Registry;
