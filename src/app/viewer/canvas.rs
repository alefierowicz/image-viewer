use eframe::egui;

use super::View;
use crate::app::ViewerApp;
use crate::volume::Volume;

const MIN_LABEL_ZOOM: f32 = 30.0;

impl ViewerApp {
    /// central panel with the image
    pub(super) fn show_canvas(&mut self, ui: &mut egui::Ui) {
        let Some(file) = &self.file else { return };

        egui::CentralPanel::default().show(ui, |ui| {
            let Some(texture) = &self.texture else {
                ui.label("cannot display this image");
                return;
            };
            Self::show_image(ui, texture, &mut self.view, &file.volume);
        });
    }

    fn show_image(ui: &mut egui::Ui, texture: &egui::TextureHandle, view: &mut Option<View>, volume: &Volume) {
        // reserving space to paint image
        // rect is the size of the free space, response - what we did just now (ie. click drag zoom whatever)
        let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
        let image_size = texture.size_vec2();

        // fitting the image on screen
        if view.is_none() || response.double_clicked() {
            let zoom = (rect.width() / image_size.x).min(rect.height() / image_size.y);
            let offset = (rect.size() - image_size * zoom) / 2.0;
            // ^ measured from left to right, divided by two so the image is centered

            *view = Some(View { zoom, offset });
        }
        let Some(view) = view.as_mut() else { return };

        // dragging the image
        view.offset += response.drag_delta();

        // zoom
        if let Some(cursor) = response.hover_pos() {
            let scroll = ui.input(|i| i.smooth_scroll_delta.y);

            if scroll != 0.0 {
                let factor = (scroll * 0.002).exp();
                let cursor_in_panel = cursor - rect.min; // all cursor calculations are based off the top-left corner as default
                view.offset = cursor_in_panel - (cursor_in_panel - view.offset) * factor;
                view.zoom *= factor;
            }
        }

        let image_rect = egui::Rect::from_min_size(rect.min + view.offset, image_size * view.zoom);
        let uv = egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0));

        let painter = ui.painter_at(rect);
        painter.image(texture.id(), image_rect, uv, egui::Color32::WHITE);

        if view.zoom >= MIN_LABEL_ZOOM {
            Self::draw_pixel_values(&painter, rect, image_rect, view.zoom, image_size, volume);
        }
    }

    fn draw_pixel_values(
        painter: &egui::Painter,
        rect: egui::Rect,
        image_rect: egui::Rect,
        zoom: f32,
        image_size: egui::Vec2,
        volume: &Volume,
    ) {
        let first = ((rect.min - image_rect.min) / zoom).floor().max(egui::Vec2::ZERO);

        let last = ((rect.max - image_rect.min) / zoom).ceil().min(image_size);
        let font = egui::FontId::monospace(10.0);

        for y in first.y as usize..last.y as usize {
            for x in first.x as usize..last.x as usize {
                let Some(val) = volume.pixel_value(x, y) else {
                    continue;
                };

                let text = format!("{}\n{}\n{}", val[0], val[1], val[2]);
                let center = image_rect.min + egui::vec2(x as f32 + 0.5, y as f32 + 0.5) * zoom;

                painter.text(
                    center + egui::vec2(1.0, 1.0),
                    egui::Align2::CENTER_CENTER,
                    &text,
                    font.clone(),
                    egui::Color32::BLACK,
                );
                painter.text(center, egui::Align2::CENTER_CENTER, &text, font.clone(), egui::Color32::WHITE);
            }
        }
    }
}
