//! Map package loader — reads `MapPackage` data and spawns Bevy entities.

use bevy::prelude::*;
use igame_shared::{MapPackage, SceneData};

/// Resource holding the fully parsed map package.
#[derive(Resource)]
pub struct LoadedMap {
    pub package: MapPackage,
}

/// Resource populated after entities have been spawned.
#[derive(Resource, Default)]
pub struct SpawnedEntities {
    pub entities: Vec<Entity>,
}

/// Plugin that registers the map-loader systems.
pub struct MapLoaderPlugin {
    /// Raw TOML text of `manifest.toml`.
    pub manifest_toml: String,
    /// Raw JSON text of `scene.json`.
    pub scene_json: String,
}

impl Plugin for MapLoaderPlugin {
    fn build(&self, app: &mut App) {
        let package = MapPackage::from_strings(&self.manifest_toml, &self.scene_json)
            .expect("failed to load map package");

        app.insert_resource(LoadedMap { package })
            .init_resource::<SpawnedEntities>()
            .add_systems(Startup, spawn_scene_entities);
    }
}

/// Spawns entities described in `scene.json`.
fn spawn_scene_entities(
    mut commands: Commands,
    loaded: Res<LoadedMap>,
    mut spawned: ResMut<SpawnedEntities>,
) {
    let scene: &SceneData = &loaded.package.scene;

    for descriptor in &scene.entities {
        let t = &descriptor.transform;
        let translation = Vec3::from_array(t.translation);
        let rotation = Quat::from_array(t.rotation);
        let scale = Vec3::from_array(t.scale);

        let entity = commands
            .spawn((
                Name::new(descriptor.name.clone()),
                Transform {
                    translation,
                    rotation,
                    scale,
                },
                Visibility::default(),
            ))
            .id();

        spawned.entities.push(entity);
    }

    info!(
        "MapLoader: spawned {} entities from scene '{}'",
        spawned.entities.len(),
        loaded.package.manifest.name
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const MANIFEST: &str = r#"
name        = "Unit Test Map"
version     = "0.1.0"
author      = "Tests"
"#;

    const SCENE: &str = r#"{
        "entities": [
            {
                "name": "Alpha",
                "transform": { "translation": [1.0, 0.0, 0.0] }
            },
            {
                "name": "Beta",
                "transform": { "translation": [0.0, 2.0, 0.0] }
            }
        ]
    }"#;

    fn build_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(MapLoaderPlugin {
            manifest_toml: MANIFEST.to_string(),
            scene_json: SCENE.to_string(),
        });
        app
    }

    #[test]
    fn entities_are_spawned() {
        let mut app = build_test_app();
        app.update();

        let spawned = app.world().resource::<SpawnedEntities>();
        assert_eq!(spawned.entities.len(), 2);
    }

    #[test]
    fn entity_names_match() {
        let mut app = build_test_app();
        app.update();

        let mut q = app.world_mut().query::<&Name>();
        let names: Vec<String> = q
            .iter(app.world())
            .map(|n| n.as_str().to_string())
            .collect();

        assert!(names.contains(&"Alpha".to_string()));
        assert!(names.contains(&"Beta".to_string()));
    }

    #[test]
    fn entity_transforms_match() {
        let mut app = build_test_app();
        app.update();

        let mut q = app.world_mut().query::<(&Name, &Transform)>();
        let transforms: Vec<(String, Vec3)> = q
            .iter(app.world())
            .map(|(n, t)| (n.as_str().to_string(), t.translation))
            .collect();

        let alpha = transforms.iter().find(|(n, _)| n == "Alpha").unwrap();
        assert!((alpha.1.x - 1.0).abs() < 1e-6);

        let beta = transforms.iter().find(|(n, _)| n == "Beta").unwrap();
        assert!((beta.1.y - 2.0).abs() < 1e-6);
    }

    #[test]
    fn loaded_map_manifest_accessible() {
        let mut app = build_test_app();
        app.update();

        let loaded = app.world().resource::<LoadedMap>();
        assert_eq!(loaded.package.manifest.name, "Unit Test Map");
    }
}
