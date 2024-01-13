use crate::pages::types::PageType;
use crate::types::screen_grabber::ScreenGrabber;
use crate::types::utils::set_min_inner_size;

pub fn capture_page(app: &mut ScreenGrabber, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    if !app.has_captured_image() {
        app.set_page(PageType::Launcher)
    }
    set_min_inner_size(ctx);
    egui::CentralPanel::default().show(ctx, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Launcher").clicked() {
                app.set_page(PageType::Launcher);
            }
            if ui
                .add_enabled(!app.is_saving, egui::Button::new("Save as"))
                .clicked()
            {
                app.save_as();
            }
            if ui
                .add_enabled(!app.is_saving, egui::Button::new("Save default"))
                .clicked()
            {
                app.save_default();
            }
            if ui
                .add_enabled(!app.is_saving, egui::Button::new("Copy to clipboard"))
                .clicked()
            {
                app.save_clipboard();
            }
            app.editor.show_fill_dropdown(ui);
        });
        egui::SidePanel::left("left-panel-toolbox")
            .resizable(false)
            .max_width(22f32)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    app.editor.show_tool_buttons(ui);
                    app.editor.show_color_picker(ui);
                    app.editor.show_fill_color_picker(ui);
                })
            });
        if app.has_captured_image() {
            ui.vertical_centered(|ui| {
                app.editor.manage(ui);
            });
        }
    });
}
