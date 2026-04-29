//! igame-runtime library — re-exports for integration tests.

pub mod camera;
pub mod map_loader;
pub mod state;

pub use camera::RtsCameraPlugin;
pub use map_loader::{LoadedMap, MapLoaderPlugin, SpawnedEntities};
pub use state::GameState;
