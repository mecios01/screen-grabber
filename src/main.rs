use types::screen_grabber::ScreenGrabber;

pub mod pages;
pub mod types;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        // changed a bit after upgrading to egui 0.24.0
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 400.0])
            .with_min_inner_size([500.0, 400.0])
            .with_icon(types::icons::app_icon()),
        ..Default::default()
    };

    eframe::run_native(
        "screen-grabber",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            let mut app = ScreenGrabber::new(cc);
            app.spawn_threads();
            Box::new(app)
        }),
    )
}
