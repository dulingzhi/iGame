//! MapPackage loader plugin: loads manifest.toml + scene JSON into Bevy ECS.

use bevy::prelude::*;
use igame_shared::MapPackage;

pub struct MapLoaderPlugin;

impl Plugin for MapLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LoadedMap::default())
            .add_systems(Startup, load_example_map);
    }
}

/// Resource holding the currently loaded map package
#[derive(Resource, Default)]
pub struct LoadedMap {
    pub package: Option<MapPackage>,
}

fn load_example_map(mut loaded_map: ResMut<LoadedMap>) {
    let manifest_toml = include_str!("../../../maps/example_map/manifest.toml");
    let scene_json = include_str!("../../../maps/example_map/scene/main.json");

    match MapPackage::from_str(manifest_toml, scene_json) {
        Ok(pkg) => {
            let errors = pkg.validate();
            if errors.is_empty() {
                info!(
                    "Loaded map: {} v{}",
                    pkg.manifest.map.name, pkg.manifest.map.version
                );
                info!("  entities: {}", pkg.scene.entities.len());
            } else {
                warn!("Map loaded with {} validation error(s):", errors.len());
                for e in &errors {
                    warn!("  - {}", e);
                }
            }
            loaded_map.package = Some(pkg);
        }
        Err(e) => {
            error!("Failed to load example map: {}", e);
        }
    }
}
