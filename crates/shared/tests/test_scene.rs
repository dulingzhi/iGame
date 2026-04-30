use igame_shared::scene::{EntityData, MapScene, SpriteData, TransformData};

#[test]
fn test_scene_deserialize_empty() {
    let ron_str = r#"(entities: [])"#;
    let scene: MapScene = ron::from_str(ron_str).expect("parse failed");
    assert!(scene.entities.is_empty());
}

#[test]
fn test_scene_deserialize_single_entity() {
    let ron_str = r#"
(
    entities: [
        (
            name: Some("Ground"),
            transform: (
                translation: (0.0, 0.0, 0.0),
                rotation: (0.0, 0.0, 0.0, 1.0),
                scale: (1.0, 1.0, 1.0),
            ),
            sprite: Some((
                color: (0.2, 0.6, 0.2, 1.0),
                custom_size: Some((800.0, 600.0)),
            )),
            tags: ["ground"],
        ),
    ],
)
"#;
    let scene: MapScene = ron::from_str(ron_str).expect("parse failed");
    assert_eq!(scene.entities.len(), 1);

    let entity = &scene.entities[0];
    assert_eq!(entity.name.as_deref(), Some("Ground"));
    assert_eq!(entity.tags, vec!["ground"]);
    assert!(entity.sprite.is_some());

    let sprite = entity.sprite.as_ref().unwrap();
    assert_eq!(sprite.custom_size, Some([800.0, 600.0]));
}

#[test]
fn test_scene_deserialize_no_sprite() {
    let ron_str = r#"
(
    entities: [
        (
            name: None,
            transform: (
                translation: (1.0, 2.0, 3.0),
                rotation: (0.0, 0.0, 0.0, 1.0),
                scale: (2.0, 2.0, 2.0),
            ),
            sprite: None,
            tags: [],
        ),
    ],
)
"#;
    let scene: MapScene = ron::from_str(ron_str).expect("parse failed");
    let entity = &scene.entities[0];
    assert!(entity.name.is_none());
    assert!(entity.sprite.is_none());
    assert_eq!(entity.transform.translation, [1.0, 2.0, 3.0]);
    assert_eq!(entity.transform.scale, [2.0, 2.0, 2.0]);
}

#[test]
fn test_transform_default() {
    let td = TransformData::default();
    assert_eq!(td.translation, [0.0, 0.0, 0.0]);
    assert_eq!(td.rotation, [0.0, 0.0, 0.0, 1.0]);
    assert_eq!(td.scale, [1.0, 1.0, 1.0]);
}

#[test]
fn test_scene_roundtrip() {
    let original = MapScene {
        entities: vec![EntityData {
            name: Some("TestEntity".into()),
            transform: TransformData {
                translation: [1.0, 2.0, 0.0],
                rotation: [0.0, 0.0, 0.0, 1.0],
                scale: [1.0, 1.0, 1.0],
            },
            sprite: Some(SpriteData {
                color: [1.0, 0.0, 0.0, 1.0],
                custom_size: Some([32.0, 32.0]),
            }),
            tags: vec!["unit".into()],
        }],
    };
    let serialized = ron::to_string(&original).expect("serialize failed");
    let deserialized: MapScene = ron::from_str(&serialized).expect("deserialize failed");
    assert_eq!(original, deserialized);
}
