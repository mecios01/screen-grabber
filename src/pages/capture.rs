use eframe::emath::Align;
use egui::epaint::CircleShape;
use egui::{epaint, Color32, Layout, Pos2, Stroke, Widget};

use crate::pages::types::PageType;
use crate::types::screen_grabber::ScreenGrabber;

pub fn capture_page(app: &mut ScreenGrabber, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    epaint::TessellationOptions::default().debug_paint_clip_rects = true;
    if app.texture_image.is_none() {
        _frame.set_minimized(true);
        _frame.set_always_on_top(false);
        app.is_minimized = true;
    }
    if !app.has_captured_image() {
        app.capture(ctx);
    }
    egui::CentralPanel::default().show(ctx, |mut ui| {
        if ui.button("Launcher").clicked() {
            app.set_page(PageType::Launcher);
        }
        if app.has_captured_image() {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                let res = egui::Image::new(&app.texture_image.clone().unwrap())
                    .max_size(ctx.screen_rect().size())
                    .maintain_aspect_ratio(true)
                    // .shrink_to_fit()
                    .ui(ui);
                let scale = res.rect.size().x / app.captured_image.clone().unwrap().size[0] as f32;
                //ctx is an Arc so clone === copy pointer
                let painter = egui::Painter::new(ctx.clone(), res.layer_id, res.rect);
                let (x, y) = (res.rect.width(), res.rect.height());
                println!("x:{x} y:{y}");

                let offset = (ctx.screen_rect().width() - x) / 2.0;
                let input = (Pos2::new(500.0, 400.0), 100.0);

                let circle = egui::Shape::Circle(CircleShape::stroke(
                    Pos2::new((input.0.x * scale) + offset, input.0.y * scale),
                    input.1 * scale,
                    Stroke::new(10.0 * scale, Color32::RED),
                ));
                painter.add(circle);
            });
        }
    });
}
