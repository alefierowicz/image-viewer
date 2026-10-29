mod landing;
mod viewer;

use crate::loader::{LoadedFile, load_file};
use eframe::egui;

#[derive(Default)]
pub struct ViewerApp {
    file: Option<LoadedFile>,
    texture: Option<egui::TextureHandle>,
    error: Option<String>,
    view: Option<viewer::View>,
}

impl ViewerApp {
    fn open_dialog(&mut self) {
        let Some(path) = rfd::FileDialog::new()
            .add_filter("Images", &["ppm", "jpg", "jpeg"])
            .add_filter("All files", &["*"])
            .pick_file()
        else {
            return;
        };

        match load_file(&path) {
            Ok(file) => self.set_file(file),
            Err(msg) => self.error = Some(msg),
        }
    }

    fn set_file(&mut self, file: LoadedFile) {
        self.file = Some(file);
        self.texture = None;
        self.error = None;
        self.view = None;
    }
}

impl eframe::App for ViewerApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.file.is_none() {
            self.show_landing(ui);
        } else {
            self.show_main(ui);
        }
    }
}
