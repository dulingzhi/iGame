use bevy::camera::RenderTarget;
use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy_egui::{EguiTextureHandle, EguiUserTextures};

use crate::scene::EditorCamera;

pub const VIEWPORT_WIDTH: u32 = 1024;
pub const VIEWPORT_HEIGHT: u32 = 768;

/// Holds persistent viewport state: the render target image handle and the
/// egui texture id used to display it.
#[derive(Resource, Default)]
pub struct ViewportState {
    pub image_handle: Handle<Image>,
    pub egui_texture_id: Option<bevy_egui::egui::TextureId>,
    /// Rect (in window pixels) that the viewport panel currently occupies.
    pub panel_rect: Option<bevy_egui::egui::Rect>,
}

pub fn setup_viewport(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut egui_user_textures: ResMut<EguiUserTextures>,
    mut viewport_state: ResMut<ViewportState>,
) {
    let size = Extent3d {
        width: VIEWPORT_WIDTH,
        height: VIEWPORT_HEIGHT,
        depth_or_array_layers: 1,
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };
    image.resize(size);

    let image_handle = images.add(image);

    let tex_id = egui_user_textures.add_image(EguiTextureHandle::Strong(image_handle.clone()));

    viewport_state.image_handle = image_handle.clone();
    viewport_state.egui_texture_id = Some(tex_id);

    // Spawn the editor camera that renders into this texture.
    commands.spawn((
        Camera3d::default(),
        Camera::default(),
        RenderTarget::Image(image_handle.into()),
        Transform::from_xyz(0.0, 8.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        EditorCamera,
    ));
}
