use eframe::egui;

use crate::app::ViewerApp;

impl ViewerApp {
    /// top bar: file name and parameters on the first row, file controls and errors on the second
    pub(super) fn show_header(&mut self, ui: &mut egui::Ui) {
        egui::Panel::top("header").resizable(false).show(ui, |ui| {
            if let Some(file) = &self.file {
                ui.horizontal(|ui| {
                    ui.heading(&file.name);
                    ui.label(format!(
                        "{} x {}, max: {}",
                        file.volume.width(),
                        file.volume.height(),
                        file.volume.maxval()
                    ));
                });
                ui.separator();
            }
            ui.horizontal(|ui| {
                if ui.button("open file").clicked() {
                    self.open_dialog();
                }
                if ui.button("export to JPEG").clicked() {
                    self.save_dialog();
                }
                ui.add(egui::Slider::new(&mut self.jpeg_quality, 1..=100).text("JPEG quality"));

                if let Some(msg) = &self.error {
                    ui.separator();
                    ui.colored_label(ui.visuals().error_fg_color, msg);
                }
            });
        });
    }
}
