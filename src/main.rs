mod app;
mod loader;
mod ppm;
mod theme;
mod volume;
fn main() -> eframe::Result {
    eframe::run_native(
        "image Viewer",
        eframe::NativeOptions::default(),
        Box::new(|cc| {
            theme::apply(&cc.egui_ctx);
            Ok(Box::new(app::ViewerApp::default()))
        }),
    )
}
