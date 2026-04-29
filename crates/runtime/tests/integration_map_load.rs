//! Integration tests — load the demo map package and assert expected state.

use bevy::prelude::*;
use igame_runtime::{LoadedMap, MapLoaderPlugin, SpawnedEntities};

/// Raw content embedded at compile time so tests are self-contained
/// and work regardless of the working directory.
const MANIFEST_TOML: &str = include_str!("../../../examples/demo_map/manifest.toml");
const SCENE_JSON: &str = include_str!("../../../examples/demo_map/scene.json");

fn build_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(MapLoaderPlugin {
        manifest_toml: MANIFEST_TOML.to_string(),
        scene_json: SCENE_JSON.to_string(),
    });
    app
}

// ── Manifest assertions ──────────────────────────────────────────────────────

#[test]
fn demo_map_manifest_name() {
    let mut app = build_app();
    app.update();
    let loaded = app.world().resource::<LoadedMap>();
    assert_eq!(loaded.package.manifest.name, "Demo Map");
}

#[test]
fn demo_map_manifest_version() {
    let mut app = build_app();
    app.update();
    let loaded = app.world().resource::<LoadedMap>();
    assert_eq!(loaded.package.manifest.version, "0.1.0");
}

#[test]
fn demo_map_manifest_author() {
    let mut app = build_app();
    app.update();
    let loaded = app.world().resource::<LoadedMap>();
    assert!(!loaded.package.manifest.author.is_empty());
}

// ── Scene entity assertions ──────────────────────────────────────────────────

#[test]
fn demo_map_has_entities() {
    let mut app = build_app();
    app.update();
    let spawned = app.world().resource::<SpawnedEntities>();
    assert!(
        !spawned.entities.is_empty(),
        "demo map must have at least one entity"
    );
}

#[test]
fn demo_map_entity_count_matches_scene() {
    let mut app = build_app();
    app.update();

    let expected = app
        .world()
        .resource::<LoadedMap>()
        .package
        .scene
        .entities
        .len();

    let spawned = app.world().resource::<SpawnedEntities>();
    assert_eq!(spawned.entities.len(), expected);
}

#[test]
fn all_spawned_entities_have_name() {
    let mut app = build_app();
    app.update();

    let spawned_ids: Vec<Entity> = app.world().resource::<SpawnedEntities>().entities.clone();

    for id in spawned_ids {
        assert!(
            app.world().get::<Name>(id).is_some(),
            "entity {id:?} is missing a Name component"
        );
    }
}

#[test]
fn all_spawned_entities_have_transform() {
    let mut app = build_app();
    app.update();

    let spawned_ids: Vec<Entity> = app.world().resource::<SpawnedEntities>().entities.clone();

    for id in spawned_ids {
        assert!(
            app.world().get::<Transform>(id).is_some(),
            "entity {id:?} is missing a Transform component"
        );
    }
}

#[test]
fn demo_map_contains_ground_entity() {
    let mut app = build_app();
    app.update();

    let mut q = app.world_mut().query::<&Name>();
    let names: Vec<String> = q
        .iter(app.world())
        .map(|n| n.as_str().to_string())
        .collect();

    assert!(
        names.iter().any(|n| n == "Ground"),
        "expected a 'Ground' entity in the demo map; got: {names:?}"
    );
}

#[test]
fn demo_map_contains_player_start() {
    let mut app = build_app();
    app.update();

    let mut q = app.world_mut().query::<&Name>();
    let names: Vec<String> = q
        .iter(app.world())
        .map(|n| n.as_str().to_string())
        .collect();

    assert!(
        names.iter().any(|n| n == "PlayerStart"),
        "expected a 'PlayerStart' entity in the demo map; got: {names:?}"
    );
}

// ── Multi-tick smoke test ────────────────────────────────────────────────────

#[test]
fn multiple_ticks_do_not_duplicate_entities() {
    let mut app = build_app();
    // Run several ticks — entity count must stay stable.
    for _ in 0..5 {
        app.update();
    }
    let spawned = app.world().resource::<SpawnedEntities>();
    let expected = app
        .world()
        .resource::<LoadedMap>()
        .package
        .scene
        .entities
        .len();
    assert_eq!(spawned.entities.len(), expected);
}
