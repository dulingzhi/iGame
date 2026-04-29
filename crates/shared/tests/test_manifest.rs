use igame_shared::manifest::Manifest;
use std::path::Path;

#[test]
fn test_manifest_deserialize_minimal() {
    let toml_str = r#"
name = "Test Map"
version = "0.1.0"
entry_scene = "scene.ron"
"#;
    let manifest: Manifest = toml::from_str(toml_str).expect("deserialize failed");
    assert_eq!(manifest.name, "Test Map");
    assert_eq!(manifest.version, "0.1.0");
    assert_eq!(manifest.entry_scene, "scene.ron");
    assert!(manifest.author.is_none());
    assert!(manifest.description.is_none());
}

#[test]
fn test_manifest_deserialize_full() {
    let toml_str = r#"
name = "Full Map"
version = "1.2.3"
author = "Alice"
description = "A complete map"
engine_version_min = "0.1.0"
entry_scene = "scene.ron"
preview_image = "preview.png"
"#;
    let manifest: Manifest = toml::from_str(toml_str).expect("deserialize failed");
    assert_eq!(manifest.name, "Full Map");
    assert_eq!(manifest.author.as_deref(), Some("Alice"));
    assert_eq!(manifest.description.as_deref(), Some("A complete map"));
    assert_eq!(manifest.engine_version_min.as_deref(), Some("0.1.0"));
    assert_eq!(manifest.preview_image.as_deref(), Some("preview.png"));
}

#[test]
fn test_manifest_roundtrip() {
    let original = Manifest {
        name: "Round-trip Map".into(),
        version: "0.2.0".into(),
        author: Some("Bob".into()),
        description: None,
        engine_version_min: Some("0.1.0".into()),
        entry_scene: "main.ron".into(),
        preview_image: None,
    };
    let serialized = toml::to_string(&original).expect("serialize failed");
    let deserialized: Manifest = toml::from_str(&serialized).expect("deserialize failed");
    assert_eq!(original, deserialized);
}

#[test]
fn test_manifest_golden() {
    let manifest = Manifest {
        name: "Golden Test Map".into(),
        version: "0.1.0".into(),
        author: None,
        description: None,
        engine_version_min: None,
        entry_scene: "scene.ron".into(),
        preview_image: None,
    };
    let serialized = toml::to_string_pretty(&manifest).expect("serialize failed");

    let golden_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("golden")
        .join("manifest_roundtrip.toml");

    if golden_path.exists() {
        let expected = std::fs::read_to_string(&golden_path).expect("read golden file");
        assert_eq!(
            serialized.trim(),
            expected.trim(),
            "Golden file mismatch! Update the golden file if the change is intentional."
        );
    } else {
        std::fs::create_dir_all(golden_path.parent().unwrap()).ok();
        std::fs::write(&golden_path, &serialized).expect("write golden file");
    }
}
