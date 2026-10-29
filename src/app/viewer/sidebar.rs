use eframe::egui;

use crate::app::ViewerApp;

impl ViewerApp {
    /// Left panel with the file name. Returns which open button was clicked, if any.
    pub(super) fn show_sidebar(&mut self, ui: &mut egui::Ui) {
        egui::Panel::left("info").resizable(false).exact_size(500.0).show(ui, |ui| {
            if ui.button("open file").clicked() {
                self.open_dialog();
            }
            let Some(file) = &self.file else { return };
            if let Some(msg) = &self.error {
                let red = ui.visuals().error_fg_color;
                ui.colored_label(red, msg);
            }
            ui.separator();
            let params: (usize, usize, u16) = (file.volume.width(), file.volume.height(), file.volume.maxval());
            ui.heading(&file.name);
            ui.label(format!("{} x {}, max: {}", params.0, params.1, params.2));
        });
    }
}
