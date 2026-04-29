use bevy::prelude::*;

/// Marker component for the editor scene camera.
#[derive(Component)]
pub struct EditorCamera;

/// Marker component for objects that can be selected in the editor.
#[derive(Component)]
pub struct SceneObject;

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Floor plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(20.0, 20.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.3, 0.35),
            ..default()
        })),
        Transform::default(),
        Name::new("Floor"),
    ));

    // Red cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.3, 0.3),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.5, 0.0),
        Name::new("Cube"),
        SceneObject,
    ));

    // Green cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.8, 1.5, 0.8))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.8, 0.3),
            ..default()
        })),
        Transform::from_xyz(3.0, 0.75, -2.0),
        Name::new("Tall Cube"),
        SceneObject,
    ));

    // Blue sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.6))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.3, 0.4, 0.9),
            ..default()
        })),
        Transform::from_xyz(-2.5, 0.6, 1.5),
        Name::new("Sphere"),
        SceneObject,
    ));

    // Directional light
    commands.spawn((
        DirectionalLight {
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.9, 0.5, 0.0)),
    ));

    commands.insert_resource(GlobalAmbientLight {
        brightness: 300.0,
        ..default()
    });
}

pub fn draw_grid(mut gizmos: Gizmos) {
    let grid_size = 20_i32;
    let grid_step = 1.0_f32;
    let color = Color::srgba(0.5, 0.5, 0.5, 0.5);
    for i in -grid_size..=grid_size {
        let x = i as f32 * grid_step;
        let extent = grid_size as f32 * grid_step;
        gizmos.line(Vec3::new(x, 0.0, -extent), Vec3::new(x, 0.0, extent), color);
        gizmos.line(Vec3::new(-extent, 0.0, x), Vec3::new(extent, 0.0, x), color);
    }
    // World origin axes
    gizmos.arrow(Vec3::ZERO, Vec3::X * 2.0, Color::srgb(1.0, 0.2, 0.2));
    gizmos.arrow(Vec3::ZERO, Vec3::Y * 2.0, Color::srgb(0.2, 1.0, 0.2));
    gizmos.arrow(Vec3::ZERO, Vec3::Z * 2.0, Color::srgb(0.2, 0.2, 1.0));
}
