use eframe::egui;

use crate::app::ViewerApp;

impl ViewerApp {
    /// floating box in the top-right corner: slice slider and zoom/offset readout
    pub(super) fn show_controls(&mut self, ui: &mut egui::Ui) {
        egui::Area::new(egui::Id::unique("zoom_and_offset"))
            .anchor(egui::Align2::RIGHT_TOP, egui::vec2(-12.0, 12.0))
            .show(ui.ctx(), |ui| {
                egui::Frame::popup(ui.style()).show(ui, |ui| {
                    ui.style_mut().wrap_mode = Some(egui::TextWrapMode::Extend);
                    ui.set_min_width(240.0);
                    if let Some(v) = &self.view {
                        ui.monospace(format!("zoom {:.2}x   offset ({:.0}, {:.0})", v.zoom, v.offset.x, v.offset.y));
                    }
                });
            });
    }
}
