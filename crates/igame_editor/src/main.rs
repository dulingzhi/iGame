mod camera;
mod gizmo;
mod scene;
mod selection;
mod ui;
mod viewport;

use bevy::prelude::*;
use bevy_egui::EguiPlugin;

use camera::OrbitCamera;
use gizmo::GizmoDrag;
use selection::Selection;
use viewport::ViewportState;

/// System set for UI: runs first so other systems can check `wants_pointer_input`.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
struct UiSet;

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ViewportState>()
            .init_resource::<Selection>()
            .init_resource::<OrbitCamera>()
            .init_resource::<GizmoDrag>()
            .add_systems(
                Startup,
                (viewport::setup_viewport, scene::setup_scene).chain(),
            )
            .add_systems(Update, ui::editor_ui.in_set(UiSet))
            .add_systems(
                Update,
                (
                    camera::camera_controller,
                    selection::handle_selection,
                    gizmo::draw_gizmo,
                    gizmo::handle_gizmo_drag,
                    scene::draw_grid,
                )
                    .after(UiSet),
            );
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "iGame Editor".into(),
                resolution: (1280_u32, 720_u32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_plugins(EditorPlugin)
        .run();
}
