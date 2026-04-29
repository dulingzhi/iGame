//! iGame runtime library.
//!
//! Re-exports key types used by the binary and integration tests.

pub mod camera;
pub mod map_loader;
pub mod scene_spawner;
pub mod state;
pub mod world;

use bevy::prelude::Resource;
use igame_shared::map_package::MapPackage;

/// Bevy resource holding the currently loaded [`MapPackage`].
#[derive(Resource)]
pub struct LoadedMap(pub MapPackage);

// Keep backwards-compatible re-exports used by integration tests.
pub use map_loader::SpawnedEntities;
