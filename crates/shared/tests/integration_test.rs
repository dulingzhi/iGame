//! Integration tests: load example map → validate → check entity count

use igame_shared::{validate_scene, MapPackage};

const MANIFEST_TOML: &str = include_str!("../../../maps/example_map/manifest.toml");
const SCENE_JSON: &str = include_str!("../../../maps/example_map/scene/main.json");

#[test]
fn load_example_map_succeeds() {
    let pkg = MapPackage::from_str(MANIFEST_TOML, SCENE_JSON)
        .expect("example map should parse without error");

    let errors = pkg.validate();
    assert!(
        errors.is_empty(),
        "example map should have no validation errors: {:?}",
        errors
    );
}

#[test]
fn example_map_has_correct_entity_count() {
    let pkg = MapPackage::from_str(MANIFEST_TOML, SCENE_JSON).unwrap();
    assert_eq!(
        pkg.scene.entities.len(),
        3,
        "example map should have 3 entities"
    );
}

#[test]
fn example_map_entity_ids_are_unique() {
    let pkg = MapPackage::from_str(MANIFEST_TOML, SCENE_JSON).unwrap();
    let errors = validate_scene(&pkg.scene);
    assert!(
        errors.is_empty(),
        "no validation errors expected: {:?}",
        errors
    );
}

#[test]
fn example_map_manifest_id_is_correct() {
    let pkg = MapPackage::from_str(MANIFEST_TOML, SCENE_JSON).unwrap();
    assert_eq!(pkg.manifest.map.id, "example_map");
    assert_eq!(pkg.manifest.map.author, "dulingzhi");
}

#[test]
fn invalid_json_returns_error() {
    let result = igame_shared::SceneData::from_json_str("not json at all");
    assert!(result.is_err());
}

#[test]
fn invalid_toml_returns_error() {
    let result = igame_shared::MapManifest::from_toml_str("[invalid toml ###");
    assert!(result.is_err());
}
