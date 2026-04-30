//! Scene setup: spawns ground plane, lighting, and entities from the loaded MapPackage.

use crate::map_loader::LoadedMap;
use bevy::prelude::*;

pub struct SceneSetupPlugin;

impl Plugin for SceneSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_lighting, setup_ground).chain())
            .add_systems(PostStartup, spawn_map_entities);
    }
}

fn setup_lighting(mut commands: Commands) {
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.5, -0.5, 0.0)),
    ));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 200.0,
    });
}

fn setup_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(50.0, 50.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.5, 0.3),
            ..default()
        })),
        Name::new("Ground"),
    ));
}

fn spawn_map_entities(
    mut commands: Commands,
    loaded_map: Res<LoadedMap>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let Some(pkg) = &loaded_map.package else {
        return;
    };

    for entity in &pkg.scene.entities {
        let [tx, ty, tz] = entity.transform.translation;
        let [sx, sy, sz] = entity.transform.scale;
        let [rx, ry, rz, rw] = entity.transform.rotation;

        let transform = Transform {
            translation: Vec3::new(tx, ty, tz),
            scale: Vec3::new(sx, sy, sz),
            rotation: Quat::from_xyzw(rx, ry, rz, rw),
        };

        let name = entity.name.clone().unwrap_or_else(|| entity.id.clone());

        commands.spawn((
            Mesh3d(meshes.add(Cuboid::default())),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgb(0.8, 0.3, 0.3),
                ..default()
            })),
            transform,
            Name::new(name),
        ));

        info!("Spawned entity '{}'", entity.id);
    }
}
