use eframe::emath::{Align, Rect, RectTransform};
use egui::epaint::CircleShape;
use egui::{Color32, Image, Layout, Pos2, Response, Rounding, Sense, Sense, Shape, Stroke, Widget};

use crate::pages::types::PageType;
use crate::types::icons::*;
use crate::types::screen_grabber::ScreenGrabber;
use crate::types::Annotation;

pub fn medium_hover_button(ui: &mut egui::Ui, image: &Image<'_>) -> egui::Response {
    let size_points = egui::Vec2::splat(24.0);

    let (id, rect) = ui.allocate_space(size_points);
    let response = ui.interact(rect, id, Sense::click());
    let tint = if response.hovered() {
        ui.painter().rect_filled(
            rect,
            Rounding::same(4.0),
            ui.style().visuals.extreme_bg_color,
        );
        ui.visuals().widgets.active.fg_stroke.color
    } else {
        ui.visuals().widgets.inactive.fg_stroke.color
    };
    let image = image.clone().tint(tint);
    image.paint_at(ui, rect);
    response
    // ui.add(image.clone().fit_to_exact_size(size_points))
}

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
        egui::SidePanel::left("left-panel-toolbox")
            .exact_width(40f32)
            .show_inside(ui, |ui| {
                ui.vertical(|ui| {
                    // egui::ImageButton::new(ERASER_SVG.clone()).ui(ui);
                    medium_hover_button(ui, &CURSOR_SVG);
                    medium_hover_button(ui, &ELLIPSE_SVG);
                    medium_hover_button(ui, &ERASER_SVG);
                    medium_hover_button(ui, &LINE_SVG);
                    medium_hover_button(ui, &MOVE_SVG);
                    medium_hover_button(ui, &RECTANGLE_SVG);
                    medium_hover_button(ui, &SELECT_SVG);
                    medium_hover_button(ui, &TEXT_SVG);
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
                println!("{:?}", image_res.rect);
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
