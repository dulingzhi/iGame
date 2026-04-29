//! Headless integration tests for the runtime crate.
//! Uses Bevy's `MinimalPlugins` to run systems without a window.

use bevy::prelude::*;
use bevy::state::app::StatesPlugin;
use igame_runtime::{scene_spawner::SceneSpawnerPlugin, state::AppState, LoadedMap};
use igame_shared::{
    manifest::Manifest,
    map_package::MapPackage,
    scene::{EntityData, MapScene, SpriteData, TransformData},
};

fn make_test_package(entities: Vec<EntityData>) -> MapPackage {
    MapPackage {
        manifest: Manifest {
            name: "Test Map".into(),
            version: "0.1.0".into(),
            author: None,
            description: None,
            engine_version_min: None,
            entry_scene: "scene.ron".into(),
            preview_image: None,
        },
        scene: MapScene { entities },
    }
}

fn make_test_entity(name: &str, x: f32, y: f32) -> EntityData {
    EntityData {
        name: Some(name.into()),
        transform: TransformData {
            translation: [x, y, 0.0],
            rotation: [0.0, 0.0, 0.0, 1.0],
            scale: [1.0, 1.0, 1.0],
        },
        sprite: Some(SpriteData {
            color: [1.0, 0.0, 0.0, 1.0],
            custom_size: Some([32.0, 32.0]),
        }),
        tags: vec!["unit".into()],
    }
}

/// Build a minimal headless Bevy app suitable for testing.
fn build_test_app(package: MapPackage) -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(StatesPlugin)
        .insert_resource(LoadedMap(package))
        .init_state::<AppState>()
        .add_plugins(SceneSpawnerPlugin);
    app
}

#[test]
fn test_entities_are_spawned() {
    let package = make_test_package(vec![
        make_test_entity("Ground", 0.0, 0.0),
        make_test_entity("Unit1", -50.0, 0.0),
        make_test_entity("Unit2", 50.0, 0.0),
    ]);

    let mut app = build_test_app(package);

    // Run two updates: first transitions Loading→Playing, second spawns entities
    app.update();
    app.update();

    // Count entities that have a Name component (the ones we spawned)
    let world = app.world_mut();
    let named_count = world.query::<&Name>().iter(world).count();
    assert_eq!(named_count, 3, "Expected 3 named entities");
}

#[test]
fn test_entity_transforms_are_correct() {
    let package = make_test_package(vec![make_test_entity("Marker", 42.0, -7.0)]);
    let mut app = build_test_app(package);
    app.update();
    app.update();

    let world = app.world_mut();
    let mut q = world.query::<(&Name, &Transform)>();
    let (_, transform) = q
        .iter(world)
        .find(|(name, _)| name.as_str() == "Marker")
        .expect("Marker entity not found");

    assert_eq!(transform.translation.x, 42.0);
    assert_eq!(transform.translation.y, -7.0);
}

#[test]
fn test_empty_scene_spawns_no_named_entities() {
    let package = make_test_package(vec![]);
    let mut app = build_test_app(package);
    app.update();
    app.update();

    let world = app.world_mut();
    let named_count = world.query::<&Name>().iter(world).count();
    assert_eq!(named_count, 0);
}

#[test]
fn test_state_transitions_to_playing() {
    let package = make_test_package(vec![]);
    let mut app = build_test_app(package);

    // Before first update, state should be Loading
    {
        let state = app.world().resource::<State<AppState>>();
        assert_eq!(*state.get(), AppState::Loading);
    }

    app.update(); // OnEnter(Loading) → transition_to_playing
    app.update(); // OnEnter(Playing) fires

    let state = app.world().resource::<State<AppState>>();
    assert_eq!(*state.get(), AppState::Playing);
}
