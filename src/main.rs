use types::screen_grabber::ScreenGrabber;
use crate::types::utils::load_icon;

pub mod pages;
pub mod types;

fn main() -> eframe::Result<()> {
    let icon_path = "src/assets/icons/screengrabber.png";
    let icon = load_icon(icon_path).unwrap();

    let native_options = eframe::NativeOptions {
        // changed a bit after upgrading to egui 0.24.0
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 400.0])
            .with_min_inner_size([500.0, 400.0])
            .with_icon(icon),
        // .with_window_level(WindowLevel::AlwaysOnTop),
        ..Default::default()
    };

    eframe::run_native(
        "Screengrabber",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let mut app = ScreenGrabber::new(cc);
            app.spawn_threads();
            Box::new(app)
        }),
    )
}
