//! iGame runtime binary entry point.
//!
//! Usage: `igame [path/to/map/package]`
//!
//! Defaults to `assets/maps/demo` if no argument is given.

use bevy::prelude::*;
use igame_runtime::{camera::RtsCameraPlugin, scene_spawner::SceneSpawnerPlugin, state::AppState};
use igame_shared::map_package::MapPackage;
use std::path::PathBuf;

fn main() {
    let map_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "assets/maps/demo".to_string());

    let package = MapPackage::load(&PathBuf::from(&map_path))
        .unwrap_or_else(|e| panic!("Failed to load map package at '{map_path}': {e}"));

    let window_title = format!("iGame — {}", package.manifest.name);

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: window_title,
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(igame_runtime::LoadedMap(package))
        .init_state::<AppState>()
        .add_plugins((RtsCameraPlugin, SceneSpawnerPlugin))
        .run();
}
