use igame_shared::{
    manifest::Manifest,
    map_package::MapPackage,
    scene::{EntityData, MapScene, SpriteData, TransformData},
    validate,
};

fn make_valid_package() -> MapPackage {
    MapPackage {
        manifest: Manifest {
            name: "Valid Map".into(),
            version: "0.1.0".into(),
            author: None,
            description: None,
            engine_version_min: None,
            entry_scene: "scene.ron".into(),
            preview_image: None,
        },
        scene: MapScene {
            entities: vec![EntityData {
                name: Some("Ground".into()),
                transform: TransformData::default(),
                sprite: Some(SpriteData {
                    color: [0.0, 1.0, 0.0, 1.0],
                    custom_size: Some([100.0, 100.0]),
                }),
                tags: vec!["ground".into()],
            }],
        },
    }
}

#[test]
fn test_valid_package_passes() {
    let pkg = make_valid_package();
    assert!(validate(&pkg).is_ok());
}

#[test]
fn test_empty_name_fails() {
    let mut pkg = make_valid_package();
    pkg.manifest.name = "".into();
    let err = validate(&pkg).unwrap_err();
    assert!(err.to_string().contains("name"));
}

#[test]
fn test_whitespace_name_fails() {
    let mut pkg = make_valid_package();
    pkg.manifest.name = "   ".into();
    let err = validate(&pkg).unwrap_err();
    assert!(err.to_string().contains("name"));
}

#[test]
fn test_empty_version_fails() {
    let mut pkg = make_valid_package();
    pkg.manifest.version = "".into();
    let err = validate(&pkg).unwrap_err();
    assert!(err.to_string().contains("version"));
}

#[test]
fn test_empty_entry_scene_fails() {
    let mut pkg = make_valid_package();
    pkg.manifest.entry_scene = "".into();
    let err = validate(&pkg).unwrap_err();
    assert!(err.to_string().contains("entry_scene"));
}

#[test]
fn test_zero_scale_entity_fails() {
    let mut pkg = make_valid_package();
    pkg.scene.entities[0].transform.scale = [0.0, 1.0, 1.0];
    let err = validate(&pkg).unwrap_err();
    assert!(err.to_string().contains("scale"));
}

#[test]
fn test_empty_scene_passes() {
    let mut pkg = make_valid_package();
    pkg.scene.entities.clear();
    assert!(validate(&pkg).is_ok());
}
