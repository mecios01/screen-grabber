use egui::Align::Center;
use egui::Direction::LeftToRight;
use egui::ImageData::Color;
use egui::{Color32, FontId, Layout};
use egui::panel::{Side, TopBottomSide};
use egui::WidgetText::RichText;
use crate::pages::types::PageType;
use crate::types::screen_grabber::ScreenGrabber;

pub fn settings_page(app: &mut ScreenGrabber, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    _frame.set_decorations(true);
    let window_size = ctx.screen_rect().width();

    egui::containers::CentralPanel::default().show(ctx, |ui| {
        egui::TopBottomPanel::new(TopBottomSide::Top, "header").show(ctx, |ui| {
            // ui.add(egui::Label::new())
            ui.label(egui::RichText::new("Settings").color(Color32::DARK_GREEN).font(FontId::proportional(40.0)));

        });

        egui::SidePanel::new(Side::Left, "settings-sections").resizable(false).exact_width(window_size * 0.2).show(ctx, |ui| {
            ui.label("Keybindings");
            ui.label("Save options");
            ui.label("Theme");
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

        egui::TopBottomPanel::new(TopBottomSide::Bottom, "footer").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add_space(ui.available_width() - 210.0);
                if ui.button("Cancel").clicked() {
                    app.set_page(PageType::Launcher);
                }
                ui.button("Reset").clicked();
                ui.button("Save").clicked();
            })
        });
    });
}