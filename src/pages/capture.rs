use eframe::emath::{Align, Rect, RectTransform};
use egui::{Image, Layout, Pos2, Widget};

use crate::pages::types::PageType;
use crate::types::screen_grabber::ScreenGrabber;

pub fn capture_page(app: &mut ScreenGrabber, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    if app.texture_image.is_none() {
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
        if ui.button("Save as").clicked() {
            app.save_as();
        }
        egui::SidePanel::left("left-panel-toolbox")
            .resizable(false)
            .max_width(22f32)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    app.editor.show_tool_buttons(ui);
                })
            });
        if app.has_captured_image() {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                app.editor.manage(ui);
            });
        }
    });
}
