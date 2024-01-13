use egui::panel::{Side, TopBottomSide};
use egui::{Align, FontId, Key, Layout, ModifierNames, ScrollArea, Vec2};
use egui_keybind::{Bind, Keybind};
use egui_modal::Modal;
use lazy_static::lazy_static;
use regex::Regex;

use crate::pages::types::{PageType, SettingType};
use crate::types::config::{Config, Status};
use crate::types::screen_grabber::ScreenGrabber;
use crate::types::sync::MasterSignal;
use crate::types::utils::{set_min_inner_size, set_theme};

lazy_static! {
    static ref INVALID_CHARS_REGEX: Regex = Regex::new(r#"[\/\?%\*:|"<>\. ]"#).unwrap();
}
const SETTINGS_SECTIONS: [(&str, SettingType); 4] = [
    ("General", SettingType::General),
    ("Keybindings", SettingType::Keybindings),
    ("Appearance", SettingType::Appearance),
    ("About", SettingType::About),
];
pub fn settings_page(app: &mut ScreenGrabber, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    set_min_inner_size(ctx);
    // _frame.set_decorations(true);
    let window_size = ctx.screen_rect().width();
    let modal = Modal::new(ctx, "to_confirm");

    egui::containers::CentralPanel::default().show(ctx, |ui| {
        egui::TopBottomPanel::new(TopBottomSide::Top, "header").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Settings").font(FontId::proportional(30.0)));
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("Back").clicked() {
                        if !app.prev_config.eq(&app.config) {
                            app.config.status = Status::ToGoBack
                        } else {
                            app.set_page(PageType::Launcher)
                        }
                    }
                })
            });
        });

        ui.add_space(30.0);
        egui::SidePanel::new(Side::Left, "settings-sections")
            .resizable(false)
            .exact_width(window_size * 0.2)
            .show_inside(ui, |ui| {
                for (t, a) in SETTINGS_SECTIONS.iter() {
                    if ui.add(egui::SelectableLabel::new(false, *t)).clicked() {
                        app.set_active_section(*a);
                    }
                    ui.add_space(6.0);
                }
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            let max = ui.available_size();
            ui.allocate_ui(max, |ui| match app.active_section {
                SettingType::General => {
                    egui::Grid::new("grid")
                        .striped(true)
                        .spacing(Vec2::new(10.0, 20.0))
                        .num_columns(2)
                        .min_col_width(300.0)
                        .show(ui, |ui| {
                            ui.label("Start Minimized");
                            ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                                ui.checkbox(&mut app.config.start_minimized, "");
                            });
                            ui.end_row();

                            ui.label("Default path");
                            ui.with_layout(Layout::left_to_right(Align::Max), |ui| {
                                let mut path = app
                                    .config
                                    .default_path
                                    .clone()
                                    .unwrap_or_default()
                                    .into_os_string()
                                    .into_string()
                                    .unwrap_or_else(|_| String::new());
                                ui.horizontal(|ui| {
                                    ui.add_enabled(false, egui::TextEdit::singleline(&mut path));
                                    ui.add_space(10.0);
                                    if ui.button("Select folder").clicked() {
                                        let path = app.choose_folder_dialog();
                                        app.config.default_path = Some(path);
                                    }
                                });
                            });
                            ui.end_row();
                            ui.label("Default filename");
                            ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                                {
                                    ui.label("Prefix");
                                    let mut prev = app.config.default_filename.prefix.clone();
                                    if ui
                                        .add(
                                            egui::TextEdit::singleline(&mut prev)
                                                .char_limit(10)
                                                .desired_width(100.0),
                                        )
                                        .changed()
                                    {
                                        let next =
                                            INVALID_CHARS_REGEX.replace_all(&prev, "").to_string();
                                        app.config.default_filename.prefix = String::from(next);
                                    };
                                }
                                ui.add_space(8.0);
                                ui.add(egui::Checkbox::new(
                                    &mut app.config.default_filename.timestamp,
                                    "timestamp",
                                ));
                                ui.add_space(8.0);

                                {
                                    ui.label("Postfix");
                                    let mut prev = app.config.default_filename.postfix.clone();
                                    if ui
                                        .add(
                                            egui::TextEdit::singleline(&mut prev)
                                                .char_limit(10)
                                                .desired_width(100.0),
                                        )
                                        .changed()
                                    {
                                        let next =
                                            INVALID_CHARS_REGEX.replace_all(&prev, "").to_string();
                                        app.config.default_filename.postfix = String::from(next);
                                    }
                                }
                            })
                        });
                }
                SettingType::Keybindings => {
                    // ui.label("KEYBINDINGS");
                    ScrollArea::vertical().show(ui, |ui| {
                        let old_binds: Vec<String> = app
                            .config
                            .hotkeys
                            .iter()
                            .chain(app.config.in_app_hotkeys.iter())
                            .map(|b| b.key_bind.clone())
                            .collect();

                        ui.add_space(10.0);

                        //Global Bindings
                        ui.label("Global Hotkeys").highlight();
                        ui.add_space(10.0);
                        for g in app.config.hotkeys.iter_mut() {
                            let prev_shortcut = g.shortcut;
                            let res = ui.add(
                                Keybind::new(&mut g.shortcut, g.action.to_string())
                                    .with_text(&g.action.to_string())
                                    .with_reset_key(Some(Key::Escape)),
                            );
                            if res.clicked() {
                                app.is_binding = true;
                            }
                            if res.changed() {
                                let new_str = g
                                    .shortcut
                                    .format(&ModifierNames::NAMES, cfg!(target_os = "macos"));
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
                            let res = ui.add(
                                Keybind::new(&mut a.shortcut, a.action.to_string())
                                    .with_text(&a.action.to_string())
                                    .with_reset_key(Some(Key::Escape)),
                            );
                            if res.clicked() {
                                app.is_binding = true;
                            }
                            if res.changed() {
                                let new_str = a
                                    .shortcut
                                    .format(&ModifierNames::NAMES, cfg!(target_os = "macos"));
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
                    ui.add_space(10.0);
                    ui.horizontal(|ui| {
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
                    Layout::left_to_right(Align::Max).with_cross_align(Align::Center),
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
                            let _ = app
                                .hotkey_channel
                                .sender
                                .send(MasterSignal::SetHotkey(app.config.hotkeys.clone()));
                            ctx.set_visuals(set_theme(app.config.is_dark));
                        }
                        Status::ToGoBack => {
                            app.config = app.prev_config.clone();
                            let _ = app
                                .hotkey_channel
                                .sender
                                .send(MasterSignal::SetHotkey(app.config.hotkeys.clone()));
                            ctx.set_visuals(set_theme(app.config.is_dark));
                            app.set_page(PageType::Launcher);
                        }
                        Status::ToReset => {
                            app.config = Config::default();
                            app.prev_config = Config::default();
                            app.store_config().unwrap_or_default();
                            let _ = app
                                .hotkey_channel
                                .sender
                                .send(MasterSignal::SetHotkey(app.config.hotkeys.clone()));
                            ctx.set_visuals(set_theme(app.config.is_dark));
                        }
                        Status::ToSave => {
                            app.prev_config.clone_from(&app.config);
                            app.store_config().unwrap_or_default();
                            let _ = app
                                .hotkey_channel
                                .sender
                                .send(MasterSignal::SetHotkey(app.config.hotkeys.clone()));
                            ctx.set_visuals(set_theme(app.config.is_dark));
                            app.set_page(PageType::Launcher);
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
