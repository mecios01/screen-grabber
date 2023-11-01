use types::screen_grabber::ScreenGrabber;

pub mod pages;
pub mod types;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        decorated: true,
        transparent: true,
        mouse_passthrough: false, // Changing this to true makes window fully invisible
        min_window_size: Some(egui::vec2(500.0, 400.0)),
        initial_window_size: Some(egui::vec2(500.0, 400.0)),
        ..Default::default()
    };
    // native_options.transparent = true;
    eframe::run_native(
        "Screengrabber",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(ScreenGrabber::new(cc))
        }),
    )
}
