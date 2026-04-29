//! iGame runtime library.
//!
//! Re-exports key types used by the binary and integration tests.

pub mod camera;
pub mod scene_spawner;
pub mod state;

use bevy::prelude::Resource;
use igame_shared::map_package::MapPackage;

/// Bevy resource holding the currently loaded [`MapPackage`].
#[derive(Resource)]
pub struct LoadedMap(pub MapPackage);
