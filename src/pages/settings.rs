use crate::pages::types::{PageType, SettingType};
use crate::types::config::{Config, Status};
use crate::types::screen_grabber::ScreenGrabber;
use egui::panel::{Side, TopBottomSide};
use egui::{Align, FontId, Key, Layout, ModifierNames, ScrollArea};
use egui_keybind::{Bind, Keybind};
use egui_modal::Modal;
use crate::types::sync::MasterSignal;
use crate::types::utils::set_theme;

pub fn settings_page(app: &mut ScreenGrabber, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // _frame.set_decorations(true);
    let window_size = ctx.screen_rect().width();
    let modal = Modal::new(ctx, "to_confirm");

    egui::containers::CentralPanel::default().show(ctx, |ui| {
        egui::TopBottomPanel::new(TopBottomSide::Top, "header").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(
                    egui::RichText::new("Settings")
                        .font(FontId::proportional(40.0)),
                );
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("Back").clicked() {
                        if !app.prev_config.eq(&app.config){
                            app.config.status = Status::ToGoBack
                        }else { app.set_page(PageType::Launcher) }
                    }
                })
            });
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
                    // ui.label("GENERAL");
                    egui::Grid::new("grid").striped(true).min_col_width(ui.available_size().x/2.0).show(ui, |ui|{
                        ui.label("Start Minimized");
                        ui.with_layout(Layout::right_to_left(Align::Max), |ui|{
                            ui.checkbox(&mut app.config.start_minimized, "");
                        });
                        ui.end_row();
                        ui.label("Example Text");
                        ui.with_layout(Layout::right_to_left(Align::Max), |ui|{
                            let mut text = app.config.get_example_test().to_owned();
                            let response = ui.add(egui::TextEdit::singleline(&mut text));
                            if response.changed() {
                                app.config.set_example_test(text);
                            }
                        });
                        ui.end_row()
                    });
                }
                SettingType::Keybindings => {
                    // ui.label("KEYBINDINGS");
                    ScrollArea::vertical().show(ui, |ui| {

                        let old_binds : Vec<String> = app.config.hotkeys.iter().chain(app.config.in_app_hotkeys.iter())
                            .map(|b| b.key_bind.clone())
                            .collect();

                        ui.add_space(10.0);

                        //Global Bindings
                        ui.label("Global Hotkeys").highlight();
                        ui.add_space(10.0);
                        for g in app.config.hotkeys.iter_mut() {
                            let prev_shortcut = g.shortcut;
                            let res = ui.add(Keybind::new(&mut g.shortcut, g.action.to_string()).with_text(&g.action.to_string()).with_reset_key(Some(Key::Escape)));
                            if res.clicked() {
                                app.is_binding = true;
                            }
                            if res.changed() {
                                let new_str = g.shortcut.format(&ModifierNames::NAMES, cfg!(target_os = "macos"));
                                if old_binds.contains(&new_str) {
                                    g.shortcut = prev_shortcut;
                                } else {
                                    g.key_bind = new_str.to_string();
                                    println!("Global Rebinded!");
                                }
                                app.is_binding = false;
                            }
                        }

                        //In App Bindings
                        ui.add_space(20.0);
                        ui.label("In App Hotkeys").highlight();
                        ui.add_space(10.0);
                        for a in app.config.in_app_hotkeys.iter_mut() {
                            let prev_shortcut = a.shortcut;
                            let res = ui.add(Keybind::new(&mut a.shortcut, a.action.to_string()).with_text(&a.action.to_string()).with_reset_key(Some(Key::Escape)));
                            if res.clicked() {
                                app.is_binding = true;
                            }
                            if res.changed() {
                                let new_str = a.shortcut.format(&ModifierNames::NAMES, cfg!(target_os = "macos"));
                                if old_binds.contains(&new_str) {
                                    a.shortcut = prev_shortcut;
                                } else {
                                    a.key_bind = new_str.to_string();
                                    println!("In App Rebinded!");
                                }
                                app.is_binding = false;
                            }
                        }
                    });
                }
                SettingType::Appearance => {
                    // ui.label("APPEARANCE");
                    ui.add_space(10.0);
                    ui.horizontal(|ui|{
                        ui.label("Theme");
                        egui::widgets::global_dark_light_mode_buttons(ui);
                    });
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
                                    !app.prev_config.eq(&Config::default()),
                                    egui::Button::new("Reset"),
                                )
                                .clicked()
                            {
                                app.config.status = Status::ToReset;
                            }
                            if ui
                                .add_enabled(
                                    !app.config.eq(&app.prev_config),
                                    egui::Button::new("Discard"),
                                )
                                .clicked()
                            {
                                app.config.status = Status::ToDiscard;
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
                Status::ToDiscard => {
                    modal.title(ui, "Discard Changes");
                    modal.frame(ui, |ui| {
                        modal.body(ui, "Do you want to discard current changes?")
                    });
                }
                Status::ToGoBack => {
                    modal.title(ui, "Discard Changes");
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
                        Status::ToDiscard => {
                            app.config = app.prev_config.clone();
                            let _ = app.hotkey_channel.sender.send(MasterSignal::SetHotkey(app.config.hotkeys.clone()));
                            ctx.set_visuals(set_theme(app.config.is_dark));
                        }
                        Status::ToGoBack => {
                            app.config = app.prev_config.clone();
                            let _ = app.hotkey_channel.sender.send(MasterSignal::SetHotkey(app.config.hotkeys.clone()));
                            ctx.set_visuals(set_theme(app.config.is_dark));
                            app.set_page(PageType::Launcher);
                        }
                        Status::ToReset => {
                            app.config = Config::default();
                            app.prev_config = Config::default();
                            app.store_config().unwrap_or_default();
                            let _ = app.hotkey_channel.sender.send(MasterSignal::SetHotkey(app.config.hotkeys.clone()));
                            ctx.set_visuals(set_theme(app.config.is_dark));
                        }
                        Status::ToSave => {
                            app.prev_config.clone_from(&app.config);
                            app.store_config().unwrap_or_default();
                            let _ = app.hotkey_channel.sender.send(MasterSignal::SetHotkey(app.config.hotkeys.clone()));
                            ctx.set_visuals(set_theme(app.config.is_dark));
                        }
                        _ => {}
                    }
                    app.config.status = Status::Normal
                }
                if modal.caution_button(ui, "Cancel").clicked() {
                    app.config.status = Status::Normal
                }
            })
        });
    });
}
