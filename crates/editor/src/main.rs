//! igame-editor — map editor stub.
//!
//! This crate will grow into a full editor (M3) with:
//! - egui Viewport (scene view, Gizmos, entity hierarchy)
//! - Inspector (component field editing)
//! - Asset browser
//! - Trigger node graph editor
//! - Play-In-Editor (PIE)
//!
//! For now it opens the demo map in the runtime view and shows
//! a basic on-screen overlay via Bevy UI.

use bevy::prelude::*;
use igame_runtime::camera::spawn_rts_camera;
use igame_runtime::{GameState, MapLoaderPlugin, RtsCameraPlugin};

const MANIFEST_TOML: &str = include_str!("../../../examples/demo_map/manifest.toml");
const SCENE_JSON: &str = include_str!("../../../examples/demo_map/scene.json");

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "iGame Editor (Stub — M3 planned)".to_string(),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_plugins(MapLoaderPlugin {
            manifest_toml: MANIFEST_TOML.to_string(),
            scene_json: SCENE_JSON.to_string(),
        })
        .add_plugins(RtsCameraPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (quit_on_esc, spawn_overlay_text))
        .run();
}

fn setup(mut commands: Commands) {
    spawn_rts_camera(&mut commands);
    commands.insert_resource(AmbientLight {
        brightness: 0.4,
        ..default()
    });
    commands.spawn((
        DirectionalLight {
            illuminance: 15_000.0,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn spawn_overlay_text(
    mut commands: Commands,
    mut spawned: Local<bool>,
    entity_query: Query<&Name>,
) {
    if *spawned {
        return;
    }
    *spawned = true;

    let count = entity_query.iter().count();
    commands.spawn((
        Text::new(format!(
            "iGame Editor (stub)\nEntities: {count}\nM3: full egui editor planned"
        )),
        TextFont {
            font_size: 18.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));
}

fn quit_on_esc(keyboard: Res<ButtonInput<KeyCode>>, mut app_exit: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        app_exit.send(AppExit::Success);
    }
}
