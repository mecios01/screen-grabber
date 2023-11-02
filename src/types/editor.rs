use std::default::Default;

use eframe::emath::RectTransform;
use egui::color_picker::Alpha;
use egui::{Color32, Image, Rgba, Rounding, Sense, Shape, Stroke, Ui};

use crate::types::annotation::Annotation;
use crate::types::icons::*;

pub enum StackAction {
    AddShape(Shape), //NO TEXT SHAPES HERE (THEY NEED TO BE HANDLED DIFFERENTLY)
}

#[derive(Clone, Copy, PartialEq)]
pub enum Mode {
    Idle,
    DrawLine,
    DrawRect,
    DrawCircle,
    DrawEllipse,
    Erase,
    InsertText,
    Select,
    Move,
}

pub struct Editor {
    pub mode: Mode,
    pub cur_annotation: Option<Annotation>,
    pub annotations: Vec<Annotation>,
    pub current_color: Rgba,
    // captured_image
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            mode: Mode::Idle,
            cur_annotation: None,
            annotations: Vec::new(),
            current_color: Rgba::RED,
        }
    }
}

impl Editor {
    fn new() -> Self {
        todo!();
    }

    pub fn manage_input(&mut self, ui: &mut Ui, to_original: RectTransform) {
        match self.mode {
            Mode::Idle => {}
            Mode::DrawLine => self.manage_segment(ui, to_original),
            Mode::DrawRect => self.manage_rect(ui, to_original),
            Mode::DrawCircle => self.manage_circle(ui, to_original),
            Mode::DrawEllipse => {}
            Mode::Erase => self.manage_arrow(ui, to_original),
            Mode::InsertText => {}
            Mode::Select => self.manage_pencil(ui, to_original),
            Mode::Move => {}
        }
    }

    fn manage_segment(&mut self, ui: &mut Ui, to_original: RectTransform) {
        let input_res = ui.interact(*to_original.from(), ui.id(), Sense::click_and_drag());
        if input_res.interact_pointer_pos().is_none() {
            return;
        }

        let pos = to_original.transform_pos_clamped(input_res.interact_pointer_pos().unwrap());
        if input_res.drag_started() {
            self.cur_annotation = Some(Annotation::segment(pos, Color32::from(self.current_color)));
            return;
        }
        if input_res.drag_released() {
            self.annotations.push(self.cur_annotation.clone().unwrap());
            self.cur_annotation = None;
            return;
        }

        if let Annotation::Segment(ref mut s) = self.cur_annotation.as_mut().unwrap() {
            s.update_ending(pos);
        }
    }

    fn manage_circle(&mut self, ui: &mut Ui, to_original: RectTransform) {
        let input_res = ui.interact(*to_original.from(), ui.id(), Sense::click_and_drag());
        if input_res.interact_pointer_pos().is_none() {
            return;
        }

        let pos = to_original.transform_pos_clamped(input_res.interact_pointer_pos().unwrap());
        if input_res.drag_started() {
            self.cur_annotation = Some(Annotation::circle(pos, Color32::from(self.current_color)));
            return;
        }
        if input_res.drag_released() {
            self.annotations.push(self.cur_annotation.clone().unwrap());
            self.cur_annotation = None;
            return;
        }
        if let Annotation::Circle(ref mut c) = self.cur_annotation.as_mut().unwrap() {
            c.update_radius(pos);
        }
    }

    fn manage_rect(&mut self, ui: &mut Ui, to_original: RectTransform) {
        let input_res = ui.interact(*to_original.from(), ui.id(), Sense::click_and_drag());
        if input_res.interact_pointer_pos().is_none() {
            return;
        }

        let pos = to_original.transform_pos_clamped(input_res.interact_pointer_pos().unwrap());
        if input_res.drag_started() {
            self.cur_annotation = Some(Annotation::rect(pos, Color32::from(self.current_color)));
            return;
        }
        if input_res.drag_released() {
            self.annotations.push(self.cur_annotation.clone().unwrap());
            self.cur_annotation = None;
            return;
        }
        if let Annotation::Rect(ref mut r) = self.cur_annotation.as_mut().unwrap() {
            r.update_p2(pos);
        }
    }

    fn manage_arrow(&mut self, ui: &mut Ui, to_original: RectTransform) {
        let input_res = ui.interact(*to_original.from(), ui.id(), Sense::click_and_drag());
        if input_res.interact_pointer_pos().is_none() {
            return;
        }

        let pos = to_original.transform_pos_clamped(input_res.interact_pointer_pos().unwrap());
        if input_res.drag_started() {
            self.cur_annotation = Some(Annotation::arrow(pos, Color32::from(self.current_color)));
            return;
        }
        if input_res.drag_released() {
            self.annotations.push(self.cur_annotation.clone().unwrap());
            self.cur_annotation = None;
            return;
        }

        if let Annotation::Arrow(ref mut a) = self.cur_annotation.as_mut().unwrap() {
            a.update_ending(pos);
        }
    }

    fn manage_pencil(&mut self, ui: &mut Ui, to_original: RectTransform) {
        let input_res = ui.interact(*to_original.from(), ui.id(), Sense::click_and_drag());
        if input_res.interact_pointer_pos().is_none() {
            return;
        }

        let pos = to_original.transform_pos_clamped(input_res.interact_pointer_pos().unwrap());
        if input_res.drag_started() {
            self.cur_annotation = Some(Annotation::pencil(pos, Color32::from(self.current_color)));
            return;
        }
        if input_res.drag_released() {
            self.annotations.push(self.cur_annotation.clone().unwrap());
            self.cur_annotation = None;
            return;
        }
        if let Annotation::Pencil(ref mut p) = self.cur_annotation.as_mut().unwrap() {
            p.update_points(pos);
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
            self.mode = mode
        }
        response
    }
    pub fn show_tool_buttons(&mut self, ui: &mut Ui) {
        self.tool_button(ui, &CURSOR_SVG, Mode::Idle);
        self.tool_button(ui, &ELLIPSE_SVG, Mode::DrawCircle);
        self.tool_button(ui, &ERASER_SVG, Mode::Erase);
        self.tool_button(ui, &LINE_SVG, Mode::DrawLine);
        self.tool_button(ui, &MOVE_SVG, Mode::Move);
        self.tool_button(ui, &RECTANGLE_SVG, Mode::DrawRect);
        self.tool_button(ui, &SELECT_SVG, Mode::Select);
        self.tool_button(ui, &TEXT_SVG, Mode::InsertText);
        let alpha: Alpha = Alpha::BlendOrAdditive;
        egui::color_picker::color_edit_button_rgba(ui, &mut self.current_color, alpha);
    }
}
