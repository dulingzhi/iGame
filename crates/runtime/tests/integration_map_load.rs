//! Integration tests — load the demo map package and assert expected state.
//!
//! The demo map lives in `assets/maps/demo/` (uses RON scene format).

use bevy::prelude::*;
use igame_runtime::map_loader::MapLoaderPlugin;
use igame_runtime::{LoadedMap, SpawnedEntities};

const MANIFEST_TOML: &str = include_str!("../../../assets/maps/demo/manifest.toml");
const SCENE_RON: &str = include_str!("../../../assets/maps/demo/scene.ron");

fn build_app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(MapLoaderPlugin {
        manifest_toml: MANIFEST_TOML.to_string(),
        scene_ron: SCENE_RON.to_string(),
    });
    app
}

// ── Manifest assertions ──────────────────────────────────────────────────────

#[test]
fn demo_map_manifest_name() {
    let mut app = build_app();
    app.update();
    let loaded = app.world().resource::<LoadedMap>();
    assert_eq!(loaded.0.manifest.name, "Demo Map");
}

#[test]
fn demo_map_manifest_version() {
    let mut app = build_app();
    app.update();
    let loaded = app.world().resource::<LoadedMap>();
    assert_eq!(loaded.0.manifest.version, "0.1.0");
}

#[test]
fn demo_map_manifest_has_author() {
    let mut app = build_app();
    app.update();
    let loaded = app.world().resource::<LoadedMap>();
    assert!(loaded.0.manifest.author.is_some());
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

    let expected = app.world().resource::<LoadedMap>().0.scene.entities.len();

    let spawned = app.world().resource::<SpawnedEntities>();
    assert_eq!(spawned.entities.len(), expected);
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
        "expected a 'Ground' entity; got: {names:?}"
    );
}

#[test]
fn demo_map_contains_player_unit() {
    let mut app = build_app();
    app.update();

    let mut q = app.world_mut().query::<&Name>();
    let names: Vec<String> = q
        .iter(app.world())
        .map(|n| n.as_str().to_string())
        .collect();

    assert!(
        names.iter().any(|n| n == "Player Unit"),
        "expected a 'Player Unit' entity; got: {names:?}"
    );
}

// ── Multi-tick smoke test ────────────────────────────────────────────────────

#[test]
fn multiple_ticks_do_not_duplicate_entities() {
    let mut app = build_app();
    for _ in 0..5 {
        app.update();
    }
    let spawned = app.world().resource::<SpawnedEntities>();
    let expected = app.world().resource::<LoadedMap>().0.scene.entities.len();
    assert_eq!(spawned.entities.len(), expected);
}
