use crate::pages::types::PageType;
use crate::types::screen_grabber::ScreenGrabber;

pub fn capture_page(app: &mut ScreenGrabber, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.heading("Capture page");
        if ui.button("Launcher").clicked() {
            app.set_page(PageType::Launcher)
        }
    });
}