//! iGame Runtime: entry point for the desktop game/editor application.

use bevy::prelude::*;

mod camera;
mod map_loader;
mod scene_setup;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "iGame".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins((
            camera::RtsCameraPlugin,
            map_loader::MapLoaderPlugin,
            scene_setup::SceneSetupPlugin,
        ))
        .run();
}
