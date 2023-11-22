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
                // let x = app.captured_image.clone().unwrap().region(
                //     &Rect::from_two_pos(Pos2::new(0.0, 0.0), Pos2::new(100.0, 1000.0)),
                //     None,
                // );
                // let y = ctx.load_texture("screenshot", x, TextureOptions::default());

                let image_res = Image::new(&app.texture_image.clone().unwrap())
                    //let image_res = Image::new(&y.clone())
                    .maintain_aspect_ratio(true)
                    .max_size(ui.available_size())
                    .ui(ui);
                let original_rect =
                    Rect::from_min_size(Pos2::ZERO, app.texture_image.clone().unwrap().size_vec2());
                let to_screen = RectTransform::from_to(original_rect, image_res.rect);
                let painter = egui::Painter::new(ctx.clone(), image_res.layer_id, image_res.rect);
                app.editor
                    .manage_input(&ctx, ui, to_screen.inverse(), &painter);
                app.editor.manage_render(&painter, to_screen);
            });
        }
    });
}
