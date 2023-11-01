use egui::panel::Side;

use crate::pages::types::PageType;
use crate::types::screen_grabber::ScreenGrabber;

pub fn settings_page(app: &mut ScreenGrabber, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    let window_size = ctx.screen_rect().width();
    egui::containers::CentralPanel::default().show(ctx, |_ui| {
        egui::SidePanel::new(Side::Left, "settings-sections")
            .resizable(false)
            .exact_width(window_size * 0.3)
            .show(ctx, |ui| {
                ui.label("Keybindings");
                ui.label("Save options");
                ui.label("Theme")
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            let max = ui.max_rect();
            ui.allocate_ui_at_rect(max, |ui| {
                ui.label("Settings page");
                if ui.button("Capture").clicked() {
                    app.set_page(PageType::Capture)
                }
                if ui.button("Launcher").clicked() {
                    app.set_page(PageType::Launcher)
                }
            });
        });
    });
}
