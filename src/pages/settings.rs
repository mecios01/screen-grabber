use crate::pages::types::PageType;
use crate::pages::types::SettingSection;
use crate::types::screen_grabber::ScreenGrabber;
use egui::panel::{Side, TopBottomSide};
use egui::{Color32, FontId};

pub fn settings_page(app: &mut ScreenGrabber, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // _frame.set_decorations(true);
    let window_size = ctx.screen_rect().width();

    egui::containers::CentralPanel::default().show(ctx, |_| {
        egui::TopBottomPanel::new(TopBottomSide::Top, "header").show(ctx, |ui| {
            ui.label(
                egui::RichText::new("Settings")
                    .color(Color32::DARK_GREEN)
                    .font(FontId::proportional(40.0)),
            );
        });

        egui::SidePanel::new(Side::Left, "settings-sections")
            .resizable(false)
            .exact_width(window_size * 0.2)
            .show(ctx, |ui| {
                if ui.selectable_label(false, "General").clicked() {
                    app.set_active_section(SettingSection::General);
                }
                if ui.selectable_label(false, "Keybindings").clicked() {
                    app.set_active_section(SettingSection::Keybindings)
                }
                if ui.selectable_label(false, "Appearance").clicked() {
                    app.set_active_section(SettingSection::Appearance)
                }
                if ui.selectable_label(false, "About").clicked() {
                    app.set_active_section(SettingSection::About)
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let max = ui.max_rect();
            ui.allocate_ui_at_rect(max, |ui| match app.active_section {
                SettingSection::General => {
                    ui.label("GENERAL");
                    ui.checkbox(&mut app.config.start_minimized, "Start minimized");
                    let mut text = app.config.get_example_test().to_owned();
                    let response = ui.add(egui::TextEdit::singleline(&mut text));
                    if response.changed() {
                        app.config.set_example_test(text);
                    }
                }
                SettingSection::Keybindings => {
                    ui.label("KEYBINDINGS");
                }
                SettingSection::Appearance => {
                    ui.label("APPEARANCE");
                    egui::widgets::global_dark_light_mode_buttons(ui);
                }
                SettingSection::About => {
                    ui.label("ABOUT");
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
                if ui.button("Save").clicked() {
                    app.store_config().unwrap_or_default();
                }
            })
        });
    });
}
