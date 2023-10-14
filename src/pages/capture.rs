use crate::pages::types::PageType;
use crate::types::screen_grabber::ScreenGrabber;

pub fn capture_page(app: &mut ScreenGrabber, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    if app.captured_image.is_none() {
        _frame.set_minimized(true);
        _frame.set_always_on_top(false);
        app.is_minimized=true;
    }
    if !app.has_captured_image() {
        app.capture(ctx);
    }
    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button("Launcher").clicked() {
            app.set_page(PageType::Launcher);
        }
        if app.has_captured_image() {
            ui.image(&app.captured_image.clone().unwrap());
        }
    });
}