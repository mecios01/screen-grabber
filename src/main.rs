use types::screen_grabber::ScreenGrabber;

pub mod types;
pub mod pages;

fn main() -> eframe::Result<()> {
    let mut native_options = eframe::NativeOptions {
        decorated: true,
        transparent: true,
        mouse_passthrough: false, // Changing this to true makes window fully invisible
        min_window_size: Some(egui::vec2(320.0, 100.0)),
        initial_window_size: Some(egui::vec2(320.0, 240.0)),
        ..Default::default()
    };
    // native_options.transparent = true;
    eframe::run_native("App", native_options, Box::new(|cc| Box::new(ScreenGrabber::new(cc))))
}
