use igame_shared::map_package::MapPackage;
use std::path::Path;

fn fixtures_path() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("demo")
}

#[test]
fn test_load_demo_fixture() {
    let path = fixtures_path();
    let package = MapPackage::load(&path).expect("failed to load demo fixture");
    assert_eq!(package.manifest.name, "Demo Map");
    assert_eq!(package.manifest.version, "0.1.0");
    assert_eq!(package.manifest.entry_scene, "scene.ron");
    assert!(!package.scene.entities.is_empty());
}

#[test]
fn test_entity_count_in_fixture() {
    let path = fixtures_path();
    let package = MapPackage::load(&path).expect("failed to load demo fixture");
    // The demo fixture has at least 3 entities: ground + 2 units
    assert!(package.scene.entities.len() >= 3);
}

#[test]
fn test_ground_entity_tags() {
    let path = fixtures_path();
    let package = MapPackage::load(&path).expect("failed to load demo fixture");
    let ground = package
        .scene
        .entities
        .iter()
        .find(|e| e.name.as_deref() == Some("Ground"))
        .expect("ground entity not found");
    assert!(ground.tags.contains(&"ground".to_string()));
}

#[test]
fn test_load_nonexistent_path_returns_error() {
    let result = MapPackage::load(Path::new("/nonexistent/path/to/map"));
    assert!(result.is_err());
}
