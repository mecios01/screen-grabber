use eframe::epaint::FontId;
use egui::{Align, Layout, Vec2};

use crate::pages::types::PageType;
use crate::types::screen_grabber::ScreenGrabber;
use crate::types::utils::set_max_inner_size;

pub fn launcher_page(app: &mut ScreenGrabber, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    set_max_inner_size(ctx);
    let pad_maxw = ctx.available_rect().width() - 20.0;
    egui::containers::CentralPanel::default().show(ctx, |ui| {
        let layout = Layout::top_down(Align::Center).with_cross_align(Align::Center);
        ui.label(egui::RichText::new("Launcher").font(FontId::proportional(30.0)));
        ui.with_layout(layout, |ui| {
            ui.add_space(40.0);
            egui::Grid::new("buttons_grid")
                .spacing((0.0, 6.0))
                .num_columns(1)
                .show(ui, |ui| {
                    let ms = Vec2::new(pad_maxw, 40.0);
                    if ui.add(egui::Button::new("Capture").min_size(ms)).clicked() {
                        app.capture();
                    }
                    ui.end_row();
                    if ui
                        .add_enabled(
                            app.has_captured_image(),
                            egui::Button::new("Edit").min_size(ms),
                        )
                        .clicked()
                    {
                        app.set_page(PageType::Capture);
                    }
                    ui.end_row();

                    if ui.add(egui::Button::new("Settings").min_size(ms)).clicked() {
                        app.set_page(PageType::Settings)
                    }
                    ui.end_row();

                    ui.group(|ui| {
                        ui.add(egui::Label::new("Delay"));
                        ui.add(
                            egui::Slider::new(&mut app.capture_delay_s, 0.3..=6.0)
                                .step_by(0.1)
                                .custom_formatter(|n, _r| format!("{:.1} s", n)),
                        )
                            .on_hover_text("Capture delay")
                    });
                    ui.end_row();
                });
            egui::TopBottomPanel::bottom("launcher_footer")
                .show_separator_line(false)
                .show_inside(ui, |ui| {
                    ui.label(egui::RichText::new("Shortcuts").font(FontId::proportional(18.0)));
                    ui.add_space(5.0);
                    egui::Grid::new("helper")
                        .show(ui, |ui| {
                            for h in app
                                .config
                                .hotkeys
                                .iter()
                                .chain(app.config.in_app_hotkeys.iter())
                            {
                                ui.label(h.action.to_string());
                                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                                    ui.label(&h.key_bind);
                                });
                                ui.end_row()
                            }
                        });
                    ui.add_space(10.0)
                });
        });
    });
}
