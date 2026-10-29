mod canvas;
mod controls;
mod sidebar;

use eframe::egui;

use super::ViewerApp;

pub(super) struct View {
    zoom: f32,
    offset: egui::Vec2,
}

impl ViewerApp {
    pub(super) fn show_main(&mut self, ui: &mut egui::Ui) {
        self.show_sidebar(ui);
        self.show_controls(ui);
        self.update_texture(ui);
        self.show_canvas(ui);
    }

    fn update_texture(&mut self, ui: &mut egui::Ui) {
        let Some(file) = &self.file else { return };
        if self.texture.is_some() {
            return;
        }

        let pixels = file.volume.render();

        let size = [file.volume.width(), file.volume.height()];
        let image = egui::ColorImage::from_rgb(size, &pixels);

        if let Some(texture) = &mut self.texture {
            texture.set(image, egui::TextureOptions::NEAREST);
        } else {
            self.texture = Some(ui.ctx().load_texture("frame", image, egui::TextureOptions::NEAREST));
        }
    }
}
