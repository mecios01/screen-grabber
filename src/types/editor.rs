use std::default::Default;
use std::sync::{Arc, Mutex};

use eframe::emath::{Rect, RectTransform};
use egui::color_picker::Alpha;
use egui::{
    Color32, ColorImage, DragValue, Event, Id, Image, Key, Painter, PointerButton, Pos2, Response,
    Rounding, Sense, Shape, Stroke, TextureHandle, TextureOptions, Ui, Vec2, Widget,
};

use crate::types::annotation::{Annotation, Position};
use crate::types::icons::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Crop,
    DrawArrow,
    DrawCircle,
    DrawEllipse,
    DrawFree,
    DrawLine,
    DrawPixelate,
    DrawRect,
    Erase,
    Highlight,
    Idle,
    InsertText,
    Move,
    Redo,
    Select,
    SetWidth(f32),
    SetZoom(f32),
    Undo,
}

pub struct Editor {
    pub captured_image: Arc<Mutex<Option<ColorImage>>>,
    pub texture: Option<TextureHandle>,
    pub mode: Mode,
    pub crop_rect: Rect,
    pub current_annotation: Option<Annotation>,
    pub undone_annotations: Vec<Annotation>,
    pub annotations: Vec<Annotation>,
    pub current_color: Color32,
    pub current_width: f32,
    pub current_fill_color: Color32,
    pub current_font_size: f32,
    pub fill_color_enabled: bool,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            captured_image: Arc::new(Mutex::new(None)),
            texture: None,
            mode: Mode::Idle,
            crop_rect: Rect::NOTHING,
            current_annotation: None,
            annotations: Vec::new(),
            undone_annotations: Vec::new(),
            current_color: Color32::RED,
            current_width: 7.5,
            current_fill_color: Color32::RED,
            current_font_size: 16.0,
            fill_color_enabled: false,
        }
    }
}

impl Editor {
    pub fn manage(&mut self, ui: &mut Ui) {
        if let Some(c) = self
            .current_annotation
            .iter()
            .filter(|&a| matches!(a, Annotation::Crop(_)))
            .last()
        {}

        let image_res = Image::new(&self.texture.clone().unwrap())
            //let image_res = Image::new(&y.clone())
            .maintain_aspect_ratio(true)
            .max_size(ui.available_size())
            .shrink_to_fit()
            .ui(ui);
        let to_screen = RectTransform::from_to(self.crop_rect, image_res.rect);
        let painter = Painter::new(ui.ctx().clone(), image_res.layer_id, image_res.rect);
        self.manage_input(ui, to_screen.inverse(), &painter);
        self.manage_render(&painter, to_screen);
    }
    pub fn manage_input(&mut self, ui: &mut Ui, to_original: RectTransform, painter: &Painter) {
        match self.mode {
            Mode::Crop => self.manage_crop(ui, to_original),
            Mode::DrawArrow => self.manage_arrow(ui, to_original),
            Mode::DrawCircle => self.manage_circle(ui, to_original),
            Mode::DrawEllipse => {}
            Mode::DrawFree => self.manage_pencil(ui, to_original),
            Mode::DrawLine => self.manage_segment(ui, to_original),
            Mode::DrawPixelate => {}
            Mode::DrawRect => self.manage_rect(ui, to_original),
            Mode::Erase => self.manage_eraser(ui, to_original, painter),
            Mode::Highlight => {}
            Mode::Idle => {}
            Mode::InsertText => self.manage_text(ui, to_original),
            Mode::Move => {}
            Mode::Redo => {}
            Mode::Select => {}
            Mode::SetWidth(_width) => {}
            Mode::SetZoom(_zoom) => {}
            Mode::Undo => {}
        }
    }

    pub fn manage_render(&self, painter: &Painter, to_screen: RectTransform) {
        let shapes: Vec<Shape> = self
            .annotations
            .iter()
            .filter(|&a| !matches!(a, Annotation::Crop(_)))
            .map(|a| a.render(to_screen.scale()[0], to_screen, painter, false))
            .collect();
        painter.extend(shapes);

        if let Some(a) = &self.current_annotation {
            painter.add(a.render(to_screen.scale()[0], to_screen, painter, true));
        }
    }

    fn get_input(
        &self,
        rect: Rect,
        ui: &mut Ui,
        id: Id,
        rect_transform: RectTransform,
        sense: Sense,
    ) -> (Response, Option<Pos2>) {
        let input_res = ui.interact(rect, id, sense);
        let Some(input) = input_res.interact_pointer_pos() else {
            return (input_res, None);
        };
        let pos = rect_transform.transform_pos_clamped(input);
        return (input_res, Some(pos));
    }

    fn update_texture(&mut self, ui: &mut Ui, crop: Option<Rect>) {
        let Some(image) = self.captured_image.lock().unwrap().clone() else {
            panic!()
        };
        if let Some(crop_rect) = crop {
            self.crop_rect = crop_rect;
        } else {
            self.crop_rect = Rect::from_two_pos(
                Pos2::ZERO,
                Pos2::new(image.width() as f32, image.height() as f32),
            )
        }
        let cropped_image = image.region(&self.crop_rect, None);
        self.texture = Some(ui.ctx().load_texture(
            "image",
            cropped_image,
            TextureOptions::default(),
        ));
    }
    fn manage_crop(&mut self, ui: &mut Ui, to_original: RectTransform) {
        let mut crop = None;
        if let Some(Annotation::Crop(ref mut c)) = self.current_annotation.as_mut() {
            if c.resizing {
                let input_res = ui.interact(*to_original.from(), ui.id(), Sense::click_and_drag());
                let Some(input) = input_res.interact_pointer_pos() else {
                    return;
                };
                let pos = to_original.transform_pos_clamped(input);
                c.update(pos);
                if input_res.drag_released_by(PointerButton::Primary) {
                    if c.p1 != c.p2 {
                        c.update_resize(false);
                        c.reset_points();
                    } else {
                        self.current_annotation = None;
                    }
                }
                return;
            }
            let control_points = c.get_control_points(to_original.inverse());
            let size = Vec2::splat(8.0);
            control_points.into_iter().enumerate().for_each(|(i, cp)| {
                let point_rect = Rect::from_center_size(cp.pos, size);
                let point_response =
                    ui.interact(point_rect, ui.id().with(i), Sense::click_and_drag());
                if point_response.dragged_by(PointerButton::Primary) {
                    match cp.label {
                        Position::LeftTop => {
                            c.p1 += point_response.drag_delta() * to_original.scale()[0];
                        }
                        Position::CenterTop => {
                            c.p1.y += point_response.drag_delta().y * to_original.scale()[0];
                        }
                        Position::RightTop => {
                            c.p1.y += point_response.drag_delta().y * to_original.scale()[0];
                            c.p2.x += point_response.drag_delta().x * to_original.scale()[0];
                        }
                        Position::LeftCenter => {
                            c.p1.x += point_response.drag_delta().x * to_original.scale()[0];
                        }
                        Position::RightCenter => {
                            c.p2.x += point_response.drag_delta().x * to_original.scale()[0];
                        }
                        Position::LeftBottom => {
                            c.p1.x += point_response.drag_delta().x * to_original.scale()[0];
                            c.p2.y += point_response.drag_delta().y * to_original.scale()[0];
                        }
                        Position::CenterBottom => {
                            c.p2.y += point_response.drag_delta().y * to_original.scale()[0];
                        }
                        Position::RightBottom => {
                            c.p2 += point_response.drag_delta() * to_original.scale()[0];
                        }
                    }
                    c.p1 = to_original.to().clamp(c.p1);
                    c.p2 = to_original.to().clamp(c.p2);
                }

                if point_response.drag_released_by(PointerButton::Primary) {
                    c.reset_points();
                }
            });
            let x = ui.input(|s| s.events.clone());
            for event in &x {
                match event {
                    Event::Key {
                        key: Key::Enter,
                        pressed: true,
                        ..
                    } => {
                        c.reset_points();
                        c.update_finished(true);
                        crop = Some(c.get_rect());
                    }
                    _ => {}
                }
            }
        } else {
            let input_res = ui.interact(*to_original.from(), ui.id(), Sense::click_and_drag());
            let Some(input) = input_res.interact_pointer_pos() else {
                return;
            };
            let pos = to_original.transform_pos_clamped(input);
            if input_res.drag_started_by(PointerButton::Primary) {
                self.current_annotation = Some(Annotation::crop(pos));
            }
        }

        if let Some(crop_rect) = crop {
            self.update_texture(ui, Some(crop_rect));
            self.annotations
                .push(self.current_annotation.clone().unwrap());
            self.current_annotation = None;
        }
    }

    fn manage_segment(&mut self, ui: &mut Ui, to_original: RectTransform) {
        let input_res = ui.interact(*to_original.from(), ui.id(), Sense::click_and_drag());
        let Some(input) = input_res.interact_pointer_pos() else {
            return;
        };

        let pos = to_original.transform_pos_clamped(input);
        if input_res.drag_started_by(PointerButton::Primary) {
            self.current_annotation = Some(Annotation::segment(
                pos,
                self.current_color,
                self.current_width,
            ));
            return;
        }
        if let Some(Annotation::Segment(ref mut s)) = self.current_annotation.as_mut() {
            s.update_ending(pos);
            if input_res.drag_released_by(PointerButton::Primary) {
                if s.starting_pos != s.ending_pos {
                    self.annotations
                        .push(self.current_annotation.clone().unwrap());
                }
                self.current_annotation = None;
            }
        }
    }

    fn manage_circle(&mut self, ui: &mut Ui, to_original: RectTransform) {
        let input_res = ui.interact(*to_original.from(), ui.id(), Sense::click_and_drag());
        let Some(input) = input_res.interact_pointer_pos() else {
            return;
        };
        let fill = match self.fill_color_enabled {
            true => self.current_fill_color,
            false => Color32::TRANSPARENT,
        };
        let pos = to_original.transform_pos_clamped(input);
        if input_res.drag_started_by(PointerButton::Primary) {
            self.current_annotation = Some(Annotation::circle(
                pos,
                self.current_color,
                self.current_width,
                fill,
            ));
            return;
        }

        if let Some(Annotation::Circle(ref mut c)) = self.current_annotation.as_mut() {
            c.update_radius(pos);
            if input_res.drag_released_by(PointerButton::Primary) {
                if c.radius != 0.0 {
                    self.annotations
                        .push(self.current_annotation.clone().unwrap());
                }
                self.current_annotation = None;
            }
        }
    }

    fn manage_rect(&mut self, ui: &mut Ui, to_original: RectTransform) {
        let input_res = ui.interact(*to_original.from(), ui.id(), Sense::click_and_drag());
        let Some(input) = input_res.interact_pointer_pos() else {
            return;
        };

        let pos = to_original.transform_pos_clamped(input);
        if input_res.drag_started_by(PointerButton::Primary) {
            let fill = match self.fill_color_enabled {
                true => self.current_fill_color,
                false => Color32::TRANSPARENT,
            };
            self.current_annotation = Some(Annotation::rect(
                pos,
                self.current_color,
                fill,
                self.current_width,
            ));
            return;
        }

        if let Some(Annotation::Rect(ref mut r)) = self.current_annotation.as_mut() {
            r.update_p2(pos);
            if input_res.drag_released_by(PointerButton::Primary) {
                if r.p1 != r.p2 {
                    self.annotations
                        .push(self.current_annotation.clone().unwrap());
                }
                self.current_annotation = None;
            }
        }
    }

    fn manage_arrow(&mut self, ui: &mut Ui, to_original: RectTransform) {
        let input_res = ui.interact(*to_original.from(), ui.id(), Sense::click_and_drag());
        let Some(input) = input_res.interact_pointer_pos() else {
            return;
        };

        let pos = to_original.transform_pos_clamped(input);
        if input_res.drag_started_by(PointerButton::Primary) {
            self.current_annotation = Some(Annotation::arrow(
                pos,
                self.current_color,
                self.current_width,
            ));
            return;
        }

        if let Some(Annotation::Arrow(ref mut a)) = self.current_annotation.as_mut() {
            a.update_ending(pos);
            if input_res.drag_released_by(PointerButton::Primary) {
                if a.starting_pos != a.ending_pos {
                    a.consolidate();
                    self.annotations
                        .push(self.current_annotation.clone().unwrap());
                }
                self.current_annotation = None;
            }
        }
    }

    fn manage_pencil(&mut self, ui: &mut Ui, to_original: RectTransform) {
        let input_res = ui.interact(*to_original.from(), ui.id(), Sense::click_and_drag());
        let Some(input) = input_res.interact_pointer_pos() else {
            return;
        };

        let pos = to_original.transform_pos_clamped(input);
        if input_res.drag_started_by(PointerButton::Primary) {
            self.current_annotation = Some(Annotation::pencil(
                pos,
                self.current_color,
                self.current_width,
            ));
            return;
        }

        if let Some(Annotation::Pencil(ref mut p)) = self.current_annotation.as_mut() {
            p.update_points(pos);
            if input_res.drag_released_by(PointerButton::Primary) {
                if p.points.len() > 1 {
                    self.annotations
                        .push(self.current_annotation.clone().unwrap());
                }
                self.current_annotation = None;
            }
        }
    }

    fn manage_text(&mut self, ui: &mut Ui, to_original: RectTransform) {
        let input_res = ui.interact(*to_original.from(), ui.id(), Sense::click());

        if input_res.interact_pointer_pos().is_none() {
            if self.current_annotation.is_some() {
                let x = ui.input(|s| s.events.clone());
                for event in &x {
                    match event {
                        Event::Text(text_to_insert) => {
                            if let Some(Annotation::Text(ref mut t)) =
                                self.current_annotation.as_mut()
                            {
                                t.update_text(text_to_insert)
                            }
                        }
                        Event::Key {
                            key: Key::Backspace,
                            pressed: true,
                            ..
                        } => {
                            if let Some(Annotation::Text(ref mut t)) =
                                self.current_annotation.as_mut()
                            {
                                t.delete_char()
                            }
                        }
                        Event::Key {
                            key: Key::Enter,
                            pressed: true,
                            modifiers: egui::Modifiers::NONE,
                            ..
                        } => {
                            self.annotations
                                .push(self.current_annotation.clone().unwrap());
                            self.current_annotation = None;
                            return;
                        }
                        Event::Key {
                            key: Key::Enter,
                            pressed: true,
                            modifiers: egui::Modifiers::SHIFT,
                            ..
                        } => {
                            if let Some(Annotation::Text(ref mut t)) =
                                self.current_annotation.as_mut()
                            {
                                t.update_text(&"\n".to_string());
                            }
                        }
                        _ => {}
                    }
                }
            }
            return;
        }

        let pos = to_original.transform_pos_clamped(input_res.interact_pointer_pos().unwrap());
        if input_res.clicked() {
            self.current_annotation = Some(Annotation::text(pos, self.current_color, 32.0));
            return;
        }
    }

    fn manage_eraser(&mut self, ui: &mut Ui, to_original: RectTransform, painter: &Painter) {
        let input_res = ui.interact(*to_original.from(), ui.id(), Sense::click());
        let Some(input) = input_res.interact_pointer_pos() else {
            return;
        };
        let pos = input;
        if input_res.clicked() {
            let index = self.annotations.iter().rposition(|a| {
                a.check_click(
                    pos,
                    to_original.inverse().scale()[0],
                    to_original.inverse(),
                    painter,
                )
            });
            if index.is_some() {
                let removed = self.annotations.remove(index.unwrap());
                self.annotations.push(Annotation::eraser(removed));
            }
        }
    }

    pub fn tool_button(&mut self, ui: &mut Ui, image: &Image<'_>, mode: Mode) -> egui::Response {
        let size_points = egui::Vec2::splat(24.0);

        let (id, rect) = ui.allocate_space(size_points);
        let response = ui.interact(rect, id, Sense::click());
        let tint = if response.hovered() || self.mode == mode {
            ui.painter().rect(
                rect,
                Rounding::same(4.0),
                ui.style().visuals.extreme_bg_color,
                Stroke::new(1f32, ui.visuals().widgets.active.bg_stroke.color),
            );
            ui.visuals().widgets.active.fg_stroke.color
        } else {
            ui.visuals().widgets.inactive.fg_stroke.color
        };
        let image = image
            .clone()
            .maintain_aspect_ratio(true)
            .tint(tint)
            .fit_to_exact_size(size_points);
        image.paint_at(ui, rect);

        if response.clicked() {
            match mode {
                Mode::Undo => self.undo(ui),
                Mode::Redo => self.redo(ui),
                _ => {
                    if self.mode == mode {
                        self.mode = Mode::Idle;
                    } else {
                        self.mode = mode;
                    }
                }
            }
            self.current_annotation = None;
        }
        response
    }
    pub fn show_tool_buttons(&mut self, ui: &mut Ui) {
        //dark mode
        if ui.visuals().dark_mode {
            self.tool_button(ui, &ARROW_DARK, Mode::DrawArrow);
            self.tool_button(ui, &CIRCLE_DARK, Mode::DrawCircle);
            self.tool_button(ui, &CROP_DARK, Mode::Crop);
            self.tool_button(ui, &CURSOR_DARK, Mode::Idle);
            self.tool_button(ui, &ERASER_DARK, Mode::Erase);
            self.tool_button(ui, &HIGHLIGHT_DARK, Mode::Highlight);
            self.tool_button(ui, &LINE_DARK, Mode::DrawLine);
            self.tool_button(ui, &MOVE_DARK, Mode::Move);
            self.tool_button(ui, &PENCIL_DARK, Mode::DrawFree);
            self.tool_button(ui, &PIXELATE_DARK, Mode::DrawPixelate);
            self.tool_button(ui, &RECTANGLE_DARK, Mode::DrawRect);
            self.tool_button(ui, &SELECT_DARK, Mode::Select);
            self.tool_button(ui, &TEXT_DARK, Mode::InsertText);
            //TODO: render differently
            self.tool_button(ui, &UNDO_DARK, Mode::Undo);
            self.tool_button(ui, &REDO_DARK, Mode::Redo);
            self.tool_button(ui, &WIDTH_DARK, Mode::SetWidth(10.0));
            self.tool_button(ui, &ZOOMM_DARK, Mode::SetZoom(100.0));
            self.tool_button(ui, &ZOOMP_DARK, Mode::SetZoom(50.0));
        }
        //light mode
        else {
            self.tool_button(ui, &ARROW, Mode::DrawArrow);
            self.tool_button(ui, &CIRCLE, Mode::DrawCircle);
            self.tool_button(ui, &CROP, Mode::Crop);
            self.tool_button(ui, &CURSOR, Mode::Idle);
            self.tool_button(ui, &ERASER, Mode::Erase);
            self.tool_button(ui, &HIGHLIGHT, Mode::Highlight);
            self.tool_button(ui, &LINE, Mode::DrawLine);
            self.tool_button(ui, &MOVE, Mode::Move);
            self.tool_button(ui, &PENCIL, Mode::DrawFree);
            self.tool_button(ui, &PIXELATE, Mode::DrawPixelate);
            self.tool_button(ui, &RECTANGLE, Mode::DrawRect);
            self.tool_button(ui, &SELECT, Mode::Select);
            self.tool_button(ui, &TEXT, Mode::InsertText);
            //TODO: render differently
            self.tool_button(ui, &UNDO, Mode::Undo);
            self.tool_button(ui, &REDO, Mode::Redo);
            self.tool_button(ui, &WIDTH, Mode::SetWidth(10.0));
            self.tool_button(ui, &ZOOMM, Mode::SetZoom(100.0));
            self.tool_button(ui, &ZOOMP, Mode::SetZoom(50.0));
        }
        Editor::make_stroke_ui(ui, &mut self.current_width, &mut self.current_color);
    }
    pub fn show_fill_dropdown(&mut self, ui: &mut Ui) {
        let enabled_stoke = ui.visuals().widgets.hovered.fg_stroke;
        let mut fill = egui::Button::new("Color");
        let mut none = egui::Button::new("None");

        if self.fill_color_enabled {
            fill = fill.stroke(enabled_stoke);
        } else {
            none = none.stroke(enabled_stoke);
        }

        ui.menu_button("Fill mode", |ui| {
            if fill.ui(ui).clicked() {
                self.fill_color_enabled = true;
            };
            if none.ui(ui).clicked() {
                self.fill_color_enabled = false;
            }
        });
    }

    pub fn show_fill_color_picker(&mut self, ui: &mut Ui) {
        ui.add_enabled(self.fill_color_enabled, |ui: &mut Ui| {
            egui::color_picker::color_edit_button_srgba(
                ui,
                &mut self.current_fill_color,
                Alpha::OnlyBlend,
            )
        })
        .on_hover_text("Fill")
        .on_disabled_hover_text("Fill (disabled)");
    }
    pub fn make_stroke_ui(ui: &mut Ui, width: &mut f32, color: &mut Color32) {
        ui.add(DragValue::new(width).speed(0.1).clamp_range(0.0..=100.0))
            .on_hover_text("Width");
        let (_id, stroke_rect) = ui.allocate_space(ui.spacing().interact_size);
        let left = stroke_rect.left_center();
        let right = stroke_rect.right_center();
        ui.painter().line_segment([left, right], (*width, *color));
        egui::color_picker::color_edit_button_srgba(ui, color, Alpha::OnlyBlend)
            .on_hover_text("Stroke");
        ui.add_space(ui.spacing().item_spacing.y)
    }

    fn undo(&mut self, ui: &mut Ui) {
        if self.annotations.len() > 0 {
            let undone = self.annotations.pop().unwrap();
            if let Annotation::Eraser(e) = undone.clone() {
                self.annotations.push(*e);
            }
            if let Annotation::Crop(_) = undone.clone() {
                if let Some(Annotation::Crop(old_crop)) = self
                    .annotations
                    .iter()
                    .filter(|a| matches!(a, Annotation::Crop(_)))
                    .last()
                {
                    self.update_texture(ui, Some(old_crop.get_rect()));
                } else {
                    self.update_texture(ui, None);
                }
            }
            self.undone_annotations.push(undone);
        }
    }
    fn redo(&mut self, ui: &mut Ui) {
        if self.undone_annotations.len() > 0 {
            let redo = self.undone_annotations.pop().unwrap();
            if let Annotation::Eraser(_) = redo {
                self.annotations.pop();
            }
            if let Annotation::Crop(c) = &redo {
                self.update_texture(ui, Some(c.get_rect()));
            }
            self.annotations.push(redo);
        }
    }
}
