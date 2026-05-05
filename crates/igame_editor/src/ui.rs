use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::selection::Selection;
use crate::viewport::ViewportState;

/// Main egui layout: side panel + central viewport panel.
pub fn editor_ui(
    mut egui_ctx: EguiContexts,
    mut viewport_state: ResMut<ViewportState>,
    selection: Res<Selection>,
    names: Query<(Entity, Option<&Name>)>,
    transforms: Query<&Transform>,
) {
    let Ok(ctx_ref) = egui_ctx.ctx_mut() else {
        return;
    };
    let ctx = ctx_ref.clone();

    // ---- Left panel: entity list + properties ----
    egui::SidePanel::left("entity_panel")
        .resizable(true)
        .default_width(220.0)
        .show(&ctx, |ui| {
            ui.heading("Scene");
            ui.separator();

            for (entity, name) in &names {
                let label = name
                    .map(|n| n.as_str().to_owned())
                    .unwrap_or_else(|| format!("Entity {:?}", entity));
                let selected = selection.entity == Some(entity);
                if selected {
                    ui.colored_label(egui::Color32::YELLOW, &label);
                } else {
                    ui.label(&label);
                }
            }

            ui.add_space(12.0);
            ui.separator();
            ui.heading("Properties");

            if let Some(ent) = selection.entity {
                if let Ok(transform) = transforms.get(ent) {
                    let t = transform.translation;
                    ui.label(format!("Position:  ({:.2}, {:.2}, {:.2})", t.x, t.y, t.z));
                    let r = transform.rotation.to_euler(EulerRot::XYZ);
                    ui.label(format!(
                        "Rotation:  ({:.1}°, {:.1}°, {:.1}°)",
                        r.0.to_degrees(),
                        r.1.to_degrees(),
                        r.2.to_degrees()
                    ));
                    let s = transform.scale;
                    ui.label(format!("Scale:     ({:.2}, {:.2}, {:.2})", s.x, s.y, s.z));
                }
            } else {
                ui.label("(nothing selected)");
            }
        });

    // ---- Central viewport panel ----
    egui::CentralPanel::default().show(&ctx, |ui| {
        ui.heading("Viewport");
        ui.separator();

        if let Some(tex_id) = viewport_state.egui_texture_id {
            let available = ui.available_size();
            // Keep aspect ratio of the render texture.
            let aspect =
                crate::viewport::VIEWPORT_WIDTH as f32 / crate::viewport::VIEWPORT_HEIGHT as f32;
            let width = available.x.min(available.y * aspect);
            let height = width / aspect;
            let viewport_size = [width, height];

            let response = ui.add(egui::Image::new(egui::load::SizedTexture::new(
                tex_id,
                viewport_size,
            )));

            // Store the rect so other systems can map mouse coordinates.
            viewport_state.panel_rect = Some(response.rect);
        } else {
            ui.label("Render target not ready.");
        }
    });
}
