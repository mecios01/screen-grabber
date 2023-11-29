use types::screen_grabber::ScreenGrabber;

pub mod pages;
pub mod types;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 400.0])
            .with_min_inner_size([500.0, 400.0])
            .with_transparent(true)
            .with_mouse_passthrough(true),
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
