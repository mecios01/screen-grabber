use egui::{ColorImage, TextureOptions};
use screenshots::Screen;

use crate::pages::types::PageType;
use crate::types::screen_grabber::ScreenGrabber;

pub fn capture_page(app: &mut ScreenGrabber, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // _frame.set_maximized(true);
    // _frame.set_decorations(true);
    if !app.has_captured_image() {
        let screenshot = Screen::all().unwrap()[0].capture().unwrap();
        let size = [screenshot.width() as _, screenshot.height() as _];
        let pixels = screenshot.as_flat_samples();
        let image = ColorImage::from_rgba_unmultiplied(size, pixels.as_slice());
        let id = ctx.load_texture("screenshot", image, TextureOptions::default());
        app.set_new_captured_image(id);
    }
    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button("Launcher").clicked() {
            app.set_page(PageType::Launcher)
        }
        if app.has_captured_image() {
            ui.image(&app.captured_image.clone().unwrap());
        }
    });
}