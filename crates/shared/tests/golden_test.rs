//! Golden tests: ensure serialization output is stable

use igame_shared::{EntityData, MapDependencies, MapManifest, MapMeta, SceneData, TransformData};

#[test]
fn manifest_roundtrip() {
    let manifest = MapManifest {
        schema_version: 1,
        map: MapMeta {
            id: "golden_map".into(),
            name: "Golden Map".into(),
            version: "1.0.0".into(),
            author: "tester".into(),
            engine_min_version: "0.1.0".into(),
            entry_scene: "scene/main.json".into(),
            description: "golden test map".into(),
            preview_image: None,
        },
        dependencies: MapDependencies::default(),
    };

    let toml_str = manifest.to_toml_string().expect("should serialize");
    let parsed: MapManifest = MapManifest::from_toml_str(&toml_str).expect("should parse");
    assert_eq!(manifest, parsed, "roundtrip should be lossless");
}

#[test]
fn scene_roundtrip() {
    let scene = SceneData {
        schema_version: 1,
        entities: vec![EntityData {
            id: "e1".into(),
            name: Some("Entity One".into()),
            transform: TransformData {
                translation: [1.0, 2.0, 3.0],
                scale: [1.0, 1.0, 1.0],
                rotation: [0.0, 0.0, 0.0, 1.0],
            },
            components: Default::default(),
        }],
    };

    let json_str = scene.to_json_string().expect("should serialize");
    let parsed: SceneData = SceneData::from_json_str(&json_str).expect("should parse");
    assert_eq!(scene, parsed, "roundtrip should be lossless");
}

#[test]
fn transform_defaults_are_identity() {
    let t = TransformData::default();
    assert_eq!(t.translation, [0.0, 0.0, 0.0]);
    assert_eq!(t.scale, [1.0, 1.0, 1.0]);
    assert_eq!(t.rotation, [0.0, 0.0, 0.0, 1.0]);
}
