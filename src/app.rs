mod landing;
mod viewer;

use crate::{
    jpeg,
    loader::{LoadedFile, load_file},
};
use eframe::egui;
use std::path::Path;
pub struct ViewerApp {
    file: Option<LoadedFile>,
    texture: Option<egui::TextureHandle>,
    error: Option<String>,
    view: Option<viewer::View>,
    jpeg_quality: u8,
}

impl Default for ViewerApp {
    fn default() -> Self {
        Self { file: None, texture: None, error: None, view: None, jpeg_quality: 90 }
    }
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

    fn save_dialog(&mut self) {
        let Some(file) = &self.file else { return };
        let name_as_jpeg = Path::new(&file.name).with_extension("jpg");

        match rfd::FileDialog::new()
            .add_filter("JPEG", &["jpg"])
            .set_file_name(name_as_jpeg.to_string_lossy())
            .save_file()
        {
            None => (),
            Some(path) => match jpeg::save(&file.volume, &path, self.jpeg_quality) {
                Ok(_) => println!("file saved successfully: {}", &path.display()),
                Err(msg) => self.error = Some(msg),
            },
        };
    }

    fn set_file(&mut self, file: LoadedFile) {
        self.file = Some(file);
        self.texture = None;
        self.error = None;
        self.view = None;
        self.jpeg_quality = 90;
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
