//! iGame runtime — demo entry point.
//!
//! Run with: `cargo run -p igame-runtime`
//!
//! Controls:
//!   WASD / Arrow keys — pan camera
//!   Q / E             — rotate camera
//!   Scroll wheel      — zoom
//!   Esc               — quit

use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};
use igame_runtime::camera::spawn_rts_camera;
use igame_runtime::{GameState, MapLoaderPlugin, RtsCameraPlugin};

const MANIFEST_TOML: &str = include_str!("../../../examples/demo_map/manifest.toml");
const SCENE_JSON: &str = include_str!("../../../examples/demo_map/scene.json");

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "iGame — Demo".to_string(),
                        resolution: WindowResolution::new(1280.0, 720.0),
                        present_mode: PresentMode::AutoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    file_path: "../../".to_string(),
                    ..default()
                }),
        )
        .init_state::<GameState>()
        .add_plugins(MapLoaderPlugin {
            manifest_toml: MANIFEST_TOML.to_string(),
            scene_json: SCENE_JSON.to_string(),
        })
        .add_plugins(RtsCameraPlugin)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, quit_on_esc)
        .run();
}

/// Set up lighting and spawn the RTS camera.
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // RTS camera
    spawn_rts_camera(&mut commands);

    // Ambient light
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.3,
    });

    // Directional sun light
    commands.spawn((
        DirectionalLight {
            illuminance: 15_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Ground plane (visual only — map entities are spawned by MapLoader)
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.25, 0.45, 0.2),
            perceptual_roughness: 0.9,
            ..default()
        })),
        Name::new("_GroundPlane"),
    ));

    info!("Demo scene ready.");
}

fn quit_on_esc(keyboard: Res<ButtonInput<KeyCode>>, mut app_exit: EventWriter<AppExit>) {
    if keyboard.just_pressed(KeyCode::Escape) {
        app_exit.send(AppExit::Success);
    }
}
