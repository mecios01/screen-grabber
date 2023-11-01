use eframe::emath::{Align, Rect, RectTransform};
use egui::{Layout, Pos2, Response, Sense, Shape, Widget};

use crate::pages::types::PageType;
use crate::types::annotation::Annotation;
use crate::types::screen_grabber::ScreenGrabber;

pub fn capture_page(app: &mut ScreenGrabber, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    // epaint::TessellationOptions::default().debug_paint_clip_rects = false;
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
        egui::SidePanel::left("left-panel-toolbox").show_inside(ui, |ui| {
            ui.vertical_centered_justified(|ui| {
                app.editor.show_tool_buttons(ui);
            })
        });
        if app.has_captured_image() {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                let image_res = egui::Image::new(&app.texture_image.clone().unwrap())
                    .max_size(ui.available_size())
                    .maintain_aspect_ratio(true)
                    .ui(ui);

                let original_rect = Rect::from_min_size(
                    Pos2::ZERO,
                    (app.texture_image.clone().unwrap().size_vec2()),
                );
                let to_screen = RectTransform::from_to(original_rect, image_res.rect);
                let scaling = to_screen.scale()[0]; //res.rect.size().x / app.texture_image.clone().unwrap().size()[0] as f32;
                                                    //ctx is an Arc so clone === copy pointer
                let painter = egui::Painter::new(ctx.clone(), image_res.layer_id, image_res.rect);

                let input_res = ui.interact(image_res.rect, image_res.id, Sense::click_and_drag());
                manage_input(app, input_res, to_screen.inverse());
                if app.editor.cur_annotation.is_some() {
                    painter.add(
                        app.editor
                            .cur_annotation
                            .as_mut()
                            .unwrap()
                            .render(scaling, to_screen),
                    );
                }

                let shapes: Vec<Shape> = app
                    .editor
                    .annotations
                    .iter()
                    .map(|a| a.render(scaling, to_screen))
                    .collect();
                painter.extend(shapes);
            });
        }
    });
}

fn manage_input(app: &mut ScreenGrabber, input_response: Response, to_orginal: RectTransform) {
    if input_response.drag_started() {
        app.editor.cur_annotation = Some(Annotation::segment(
            to_orginal.transform_pos_clamped(input_response.interact_pointer_pos().unwrap()),
        ));
        return;
    }
    if input_response.drag_released() {
        //TODO
        app.editor
            .annotations
            .push(app.editor.cur_annotation.clone().unwrap());
        app.editor.cur_annotation = None;
        return;
    }
    if app.editor.cur_annotation.is_some() {
        app.editor.cur_annotation.as_mut().unwrap().update(
            to_orginal.transform_pos_clamped(input_response.interact_pointer_pos().unwrap()),
        );
        return;
    }
}
