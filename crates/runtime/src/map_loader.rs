//! `MapLoaderPlugin` — loads a [`MapPackage`] from strings and populates
//! `SpawnedEntities`.  Used primarily by integration tests and the editor
//! where the package content is already in memory.

use bevy::prelude::*;
use igame_shared::map_package::MapPackage;

use crate::LoadedMap;

/// Resource populated after entities have been spawned from the scene.
#[derive(Resource, Default)]
pub struct SpawnedEntities {
    pub entities: Vec<Entity>,
}

/// Plugin that loads a map package from in-memory strings and spawns entities.
///
/// Useful for headless integration tests and the editor where files are
/// already in memory.
pub struct MapLoaderPlugin {
    /// TOML text of `manifest.toml`.
    pub manifest_toml: String,
    /// RON text of the scene file.
    pub scene_ron: String,
}

impl Plugin for MapLoaderPlugin {
    fn build(&self, app: &mut App) {
        let package = MapPackage::from_strings(&self.manifest_toml, &self.scene_ron)
            .expect("MapLoaderPlugin: failed to parse map package");

        app.insert_resource(LoadedMap(package))
            .init_resource::<SpawnedEntities>()
            .add_systems(Startup, spawn_scene_entities);
    }
}

/// Spawns entities from the loaded [`MapPackage`] scene.
fn spawn_scene_entities(
    mut commands: Commands,
    loaded: Res<LoadedMap>,
    mut spawned: ResMut<SpawnedEntities>,
) {
    for descriptor in &loaded.0.scene.entities {
        let t = &descriptor.transform;
        let mut entity_cmds = commands.spawn((
            Transform {
                translation: Vec3::from_array(t.translation),
                rotation: Quat::from_array(t.rotation),
                scale: Vec3::from_array(t.scale),
            },
            Visibility::default(),
        ));

        if let Some(ref name) = descriptor.name {
            entity_cmds.insert(Name::new(name.clone()));
        }

        if let Some(ref sprite_data) = descriptor.sprite {
            let [r, g, b, a] = sprite_data.color;
            let color = Color::srgba(r, g, b, a);
            let custom_size = sprite_data.custom_size.map(|[w, h]| Vec2::new(w, h));
            entity_cmds.insert(Sprite {
                color,
                custom_size,
                ..default()
            });
        }

        spawned.entities.push(entity_cmds.id());
    }

    info!(
        "MapLoader: spawned {} entities from '{}'",
        spawned.entities.len(),
        loaded.0.manifest.name
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    const MANIFEST: &str = r#"
name        = "Unit Test Map"
version     = "0.1.0"
author      = "Tests"
entry_scene = "scene.ron"
"#;

    // RON scene with two simple entities.
    const SCENE: &str = r#"(
        entities: [
            (
                name: Some("Alpha"),
                transform: (
                    translation: (1.0, 0.0, 0.0),
                    rotation:    (0.0, 0.0, 0.0, 1.0),
                    scale:       (1.0, 1.0, 1.0),
                ),
                sprite: None,
                tags: [],
            ),
            (
                name: Some("Beta"),
                transform: (
                    translation: (0.0, 2.0, 0.0),
                    rotation:    (0.0, 0.0, 0.0, 1.0),
                    scale:       (1.0, 1.0, 1.0),
                ),
                sprite: None,
                tags: [],
            ),
        ],
    )"#;

    fn build_test_app() -> App {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(MapLoaderPlugin {
            manifest_toml: MANIFEST.to_string(),
            scene_ron: SCENE.to_string(),
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
        assert_eq!(loaded.0.manifest.name, "Unit Test Map");
    }
}
