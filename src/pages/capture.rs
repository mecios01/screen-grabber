use egui::{Direction, Layout, Widget};

use crate::pages::types::PageType;
use crate::types::screen_grabber::ScreenGrabber;

pub fn capture_page(app: &mut ScreenGrabber, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    if app.captured_image.is_none() {
        _frame.set_minimized(true);
        _frame.set_always_on_top(false);
        app.is_minimized = true;
    }
    if !app.has_captured_image() {
        app.capture(ctx);
    }
    egui::CentralPanel::default().show(ctx, |ui| {
        if ui.button("Launcher").clicked() {
            app.set_page(PageType::Launcher);
        }
        if app.has_captured_image() {
            ui.with_layout(Layout::centered_and_justified(Direction::TopDown), |ui| {
                egui::Image::new(&app.captured_image.clone().unwrap())
                    .max_size(ctx.screen_rect().size())
                    .maintain_aspect_ratio(true).ui(ui);
            });
        }
    });
}