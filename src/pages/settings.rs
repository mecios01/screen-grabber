use crate::pages::types::{PageType, SettingType};
use crate::types::config::{Config, Status};
use crate::types::screen_grabber::ScreenGrabber;
use egui::panel::{Side, TopBottomSide};
use egui::{Align, Color32, FontId, Layout};
use egui_modal::Modal;

pub fn settings_page(app: &mut ScreenGrabber, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // _frame.set_decorations(true);
    let window_size = ctx.screen_rect().width();
    let modal = Modal::new(ctx, "to_confirm");

    egui::containers::CentralPanel::default().show(ctx, |ui| {
        egui::TopBottomPanel::new(TopBottomSide::Top, "header").show_inside(ui, |ui| {
            ui.label(
                egui::RichText::new("Settings")
                    .color(Color32::DARK_GREEN)
                    .font(FontId::proportional(40.0)),
            );
        });

        egui::SidePanel::new(Side::Left, "settings-sections")
            .resizable(false)
            .exact_width(window_size * 0.2)
            .show_inside(ui, |ui| {
                if ui.selectable_label(false, "General").clicked() {
                    app.set_active_section(SettingType::General);
                }
                if ui.selectable_label(false, "Keybindings").clicked() {
                    app.set_active_section(SettingType::Keybindings)
                }
                if ui.selectable_label(false, "Appearance").clicked() {
                    app.set_active_section(SettingType::Appearance)
                }
                if ui.selectable_label(false, "About").clicked() {
                    app.set_active_section(SettingType::About)
                }
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            let max = ui.max_rect();
            ui.allocate_ui_at_rect(max, |ui| match app.active_section {
                SettingType::General => {
                    ui.label("GENERAL");
                    ui.checkbox(&mut app.config.start_minimized, "Start minimized");
                    let mut text = app.config.get_example_test().to_owned();
                    let response = ui.add(egui::TextEdit::singleline(&mut text));
                    if response.changed() {
                        app.config.set_example_test(text);
                    }
                }
                SettingType::Keybindings => {
                    ui.label("KEYBINDINGS");
                }
                SettingType::Appearance => {
                    ui.label("APPEARANCE");
                    egui::widgets::global_dark_light_mode_buttons(ui);
                }
                SettingType::About => {
                    ui.label("ABOUT");
                }
            });
        });

        egui::TopBottomPanel::new(TopBottomSide::Bottom, "footer")
            .max_height(30.0)
            .show_inside(ui, |ui| {
                ui.with_layout(
                    Layout::right_to_left(Align::Max).with_cross_align(Align::Center),
                    |ui| match app.config.status {
                        Status::Normal => {
                            if ui
                                .add_enabled(
                                    !app.config.eq(&app.prev_config),
                                    egui::Button::new("Save"),
                                )
                                .clicked()
                            {
                                app.config.status = Status::ToSave;
                            }
                            if ui
                                .add_enabled(
                                    !app.config.eq(&Config::default()),
                                    egui::Button::new("Reset"),
                                )
                                .clicked()
                            {
                                app.config.status = Status::ToReset;
                            }
                            if ui
                                .add_enabled(
                                    !app.config.eq(&app.prev_config),
                                    egui::Button::new("Cancel"),
                                )
                                .clicked()
                            {
                                app.config.status = Status::ToCancel;
                            }
                        }
                        _ => {
                            modal.open();
                        }
                    },
                )
            });

        modal.show(|ui| {
            ui.set_max_width(500.0);
            match app.config.status {
                Status::ToCancel => {
                    modal.title(ui, "Cancel");
                    modal.frame(ui, |ui| {
                        modal.body(ui, "Do you want to discard current changes?")
                    });
                }
                Status::ToReset => {
                    modal.title(ui, "Reset");
                    modal.frame(ui, |ui| {
                        modal.body(ui, "Do you want to reset default settings?")
                    });
                }
                Status::ToSave => {
                    modal.title(ui, "Save");
                    modal.frame(ui, |ui| {
                        modal.body(ui, "Do you want to save current changes?")
                    });
                }
                _ => {}
            }

            modal.buttons(ui, |ui| {
                if modal.button(ui, "Confirm").clicked() {
                    match app.config.status {
                        Status::ToCancel => app.config = app.prev_config.clone(),
                        Status::ToReset => {
                            app.config = Config::default();
                            app.prev_config = Config::default();
                            app.store_config().unwrap_or_default()
                        }
                        Status::ToSave => {
                            app.prev_config.clone_from(&app.config);
                            app.store_config().unwrap_or_default();
                            app.set_page(PageType::Launcher)
                        }
                        _ => {}
                    }
                    app.config.status = Status::Normal
                }
                if modal.caution_button(ui, "Back").clicked() {
                    app.config.status = Status::Normal
                }
            })
        });
    });
}
