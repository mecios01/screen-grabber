use eframe::emath::{Align, Rect, RectTransform};
use egui::epaint::CircleShape;
use egui::{epaint, Color32, Layout, Pos2, Stroke, Widget, Sense, Response, Shape};


use crate::pages::types::PageType;
use crate::types::screen_grabber::ScreenGrabber;
use crate::utils::input::Annotation;

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
        egui::SidePanel::left("left-panel-toolbox")
            .exact_width(40f32)
            .show_inside(ui, |ui| {
                ui.vertical(|ui| {
                    ui.button("--").clicked();
                    ui.button("[]").clicked();
                    ui.button("**").clicked();
                    ui.button("O").clicked();
                    ui.button("_").clicked();
                })
            });
        if app.has_captured_image() {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                let original_rect = Rect::from_min_size(
                    Pos2::ZERO,
                    (app.texture_image.clone().unwrap().size_vec2()),
                );
                let res = egui::Image::new(&app.texture_image.clone().unwrap())
                    .max_size(ui.available_size())
                    .maintain_aspect_ratio(true)
                    // .shrink_to_fit()
                    .sense(Sense::click())
                    .ui(ui);

                let to_screen = RectTransform::from_to(original_rect, res.rect);
                let scale = res.rect.size().x / app.texture_image.clone().unwrap().size()[0] as f32;
                //ctx is an Arc so clone === copy pointer
                let painter = egui::Painter::new(ctx.clone(), res.layer_id, res.rect);

                let (x, y) = (res.rect.width(), res.rect.height());
                // println!("x:{x} y:{y}");
                // println!("{:?}", res.rect);
                let offset = (ctx.screen_rect().width() - x) / 2.0;
                let input = (Pos2::new(500.0, 400.0), 100.0);

                let line = egui::Shape::LineSegment {
                    points: [
                        to_screen.transform_pos(Pos2::ZERO),
                        to_screen.transform_pos(Pos2::new(original_rect.width() / 2.0, 100.0))
                    ],
                    stroke: Stroke::new(10.0 * scale, Color32::RED),
                };
                let circle = egui::Shape::Circle(CircleShape::stroke(
                    to_screen.transform_pos(Pos2::new(input.0.x, input.0.y)),
                    input.1 * to_screen.scale()[0],
                    Stroke::new(10.0 * to_screen.scale()[0], Color32::RED),
                ));
                painter.add(line);
                painter.add(circle);

                let input_response = ui.interact(res.rect, res.id, Sense::click_and_drag());
                manage_input(app, input_response, to_screen.inverse());

                if app.editor.segment.is_some() {
                    println!("{:?}", app.editor.segment.as_ref().unwrap());
                    painter.add(app.editor.segment.as_mut()
                        .unwrap().render(to_screen.scale()[0], to_screen));
                }

                let shapes : Vec<Shape> = app.editor.annotations.iter()
                    .map(|a| a.render(to_screen.scale()[0], to_screen)).collect();
                painter.extend(shapes);
                //println!("CAZZAROLA: {:?}", check);

                // println!("ui: {:?}, res: {:?}", ui.id(), check.id);
                // if check.clicked() {
                //     let pos = check.interact_pointer_pos().unwrap();
                //     painter.add(egui::Shape::Circle(CircleShape::stroke(
                //         Pos2::new(pos.x, pos.y),
                //         50.0,
                //         Stroke::new(10.0 * to_screen.scale()[0], Color32::BLUE),
                //     )));
                // }
            });
        }
    });
}

fn manage_input(app: &mut ScreenGrabber, input_response: Response, to_orginal: RectTransform) {
    if input_response.drag_started() {
        println!("andato");
        app.editor.segment = Some(Annotation::segment(to_orginal.transform_pos_clamped(
            input_response.interact_pointer_pos().unwrap()
        )));
        return;
    }
    if input_response.drag_released() {
        //TODO
        app.editor.annotations.push(app.editor.segment.clone().unwrap());
        app.editor.segment = None;
        return;
    }
    if app.editor.segment.is_some() {
        app.editor.segment.as_mut().unwrap().update(to_orginal.transform_pos_clamped(
            input_response.interact_pointer_pos().unwrap()
        ));
        return;
    }
}