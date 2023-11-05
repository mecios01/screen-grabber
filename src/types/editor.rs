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
    pub mode: Mode,
    pub current_annotation: Option<Annotation>,
    pub undone_annotations: Option<Vec<Annotation>>,
    pub annotations: Vec<Annotation>,
    pub current_color: Rgba,
    // captured_image
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            mode: Mode::Idle,
            current_annotation: None,
            annotations: Vec::new(),
            current_color: Rgba::RED,
            undone_annotations: None,
        }
    }
}

impl Editor {
    pub fn manage_input(&mut self, ui: &mut Ui, to_original: RectTransform) {
        match self.mode {
            Mode::Crop => {}
            Mode::DrawArrow => {}
            Mode::DrawCircle => self.manage_circle(ui, to_original),
            Mode::DrawEllipse => {}
            Mode::DrawFree => {}
            Mode::DrawLine => self.manage_segment(ui, to_original),
            Mode::DrawPixelate => {}
            Mode::DrawRect => self.manage_rect(ui, to_original),
            Mode::Erase => self.manage_arrow(ui, to_original),
            Mode::Highlight => {}
            Mode::Idle => {}
            Mode::InsertText => {}
            Mode::Move => {}
            Mode::Redo => self.redo(),
            Mode::Select => {}
            Mode::SetWidth(width) => {}
            Mode::SetZoom(zoom) => {}
            Mode::Undo => self.undo(),
        }
    }

    fn manage_segment(&mut self, ui: &mut Ui, to_original: RectTransform) {
        let input_res = ui.interact(*to_original.from(), ui.id(), Sense::click_and_drag());
        if input_res.interact_pointer_pos().is_none() {
            return;
        }

        let pos = to_original.transform_pos_clamped(input_res.interact_pointer_pos().unwrap());
        if input_res.drag_started() {
            self.current_annotation =
                Some(Annotation::segment(pos, Color32::from(self.current_color)));
            return;
        }
        if input_res.drag_released() {
            self.annotations
                .push(self.current_annotation.clone().unwrap());
            self.current_annotation = None;
            return;
        }

        if let Annotation::Segment(ref mut s) = self.current_annotation.as_mut().unwrap() {
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
            self.current_annotation =
                Some(Annotation::circle(pos, Color32::from(self.current_color)));
            return;
        }
        if input_res.drag_released() {
            self.annotations
                .push(self.current_annotation.clone().unwrap());
            self.current_annotation = None;
            return;
        }
        if let Annotation::Circle(ref mut c) = self.current_annotation.as_mut().unwrap() {
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
            self.current_annotation =
                Some(Annotation::rect(pos, Color32::from(self.current_color)));
            return;
        }
        if input_res.drag_released() {
            self.annotations
                .push(self.current_annotation.clone().unwrap());
            self.current_annotation = None;
            return;
        }
        if let Annotation::Rect(ref mut r) = self.current_annotation.as_mut().unwrap() {
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
            self.current_annotation =
                Some(Annotation::arrow(pos, Color32::from(self.current_color)));
            return;
        }
        if input_res.drag_released() {
            self.annotations
                .push(self.current_annotation.clone().unwrap());
            self.current_annotation = None;
            return;
        }

        if let Annotation::Arrow(ref mut a) = self.current_annotation.as_mut().unwrap() {
            a.update_ending(pos);
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
        //dark mode
        if ui.visuals().dark_mode {
            // self.tool_button(ui, &CURSOR_DARK, Mode::Idle);
            // self.tool_button(ui, &SELECT_DARK, Mode::Select);
            // self.tool_button(ui, &MOVE_DARK, Mode::Move);
            // self.tool_button(ui, &ERASER_DARK, Mode::Erase);
            // self.tool_button(ui, &RECTANGLE_DARK, Mode::DrawRect);
            // self.tool_button(ui, &CIRCLE_DARK, Mode::DrawCircle);
            // self.tool_button(ui, &LINE_DARK, Mode::DrawLine);
            // self.tool_button(ui, &TEXT_DARK, Mode::InsertText);
        }
        //light mode
        else {
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
            self.tool_button(ui, &WIDTH, Mode::Idle);
            self.tool_button(ui, &ZOOMM, Mode::Erase);
            self.tool_button(ui, &ZOOMP, Mode::Erase);
        }
        let alpha: Alpha = Alpha::BlendOrAdditive;
        egui::color_picker::color_edit_button_rgba(ui, &mut self.current_color, alpha);
    }

    fn undo(&mut self) {
        if self.annotations.len() > 0 {
            if self.undone_annotations.is_none() {
                self.undone_annotations = Some(Vec::<Annotation>::new());
            }
            let undone = self.annotations.pop().unwrap();
            self.undone_annotations.as_deref_mut().unwrap().push(undone);
        }
    }
    fn redo(&mut self) {}
}
