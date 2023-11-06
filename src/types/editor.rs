use eframe::emath::RectTransform;
use egui::color_picker::Alpha;
use egui::{Color32, Event, Image, Key, Painter, Rgba, Rounding, Sense, Shape, Stroke, Ui};
use std::default::Default;
use std::mem;

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
    pub last_mode: Option<Mode>,
    pub current_annotation: Option<Annotation>,
    pub undone_annotations: Vec<Annotation>,
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
            undone_annotations: Vec::new(),
            last_mode: None,
        }
    }
}

impl Editor {
    pub fn manage_input(&mut self, ui: &mut Ui, to_original: RectTransform, painter: &Painter) {
        match self.mode {
            Mode::Crop => {}
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
            Mode::Redo => self.redo(),
            Mode::Select => {}
            Mode::SetWidth(_width) => {}
            Mode::SetZoom(_zoom) => {}
            Mode::Undo => self.undo(),
        }
    }

    pub fn manage_render(&self, painter: &Painter, to_screen: RectTransform) {
        let shapes: Vec<Shape> = self
            .annotations
            .iter()
            .map(|a| a.render(to_screen.scale()[0], to_screen, painter, false))
            .collect();
        painter.extend(shapes);

        if self.current_annotation.is_some() {
            painter.add(self.current_annotation.as_ref().unwrap().render(
                to_screen.scale()[0],
                to_screen,
                painter,
                true,
            ));
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

    fn manage_pencil(&mut self, ui: &mut Ui, to_original: RectTransform) {
        let input_res = ui.interact(*to_original.from(), ui.id(), Sense::click_and_drag());
        if input_res.interact_pointer_pos().is_none() {
            return;
        }

        let pos = to_original.transform_pos_clamped(input_res.interact_pointer_pos().unwrap());
        if input_res.drag_started() {
            self.current_annotation =
                Some(Annotation::pencil(pos, Color32::from(self.current_color)));
            return;
        }
        if input_res.drag_released() {
            self.annotations
                .push(self.current_annotation.clone().unwrap());
            self.current_annotation = None;
            return;
        }
        if let Annotation::Pencil(ref mut p) = self.current_annotation.as_mut().unwrap() {
            p.update_points(pos);
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
                            if let Annotation::Text(ref mut t) =
                                self.current_annotation.as_mut().unwrap()
                            {
                                t.update_text(text_to_insert)
                            }
                        }
                        Event::Key {
                            key: Key::Backspace,
                            pressed: true,
                            ..
                        } => {
                            if let Annotation::Text(ref mut t) =
                                self.current_annotation.as_mut().unwrap()
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
                            if let Annotation::Text(ref mut t) =
                                self.current_annotation.as_mut().unwrap()
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
            self.current_annotation =
                Some(Annotation::text(pos, Color32::from(self.current_color)));
            return;
        }
    }

    fn manage_eraser(&mut self, ui: &mut Ui, to_original: RectTransform, painter: &Painter) {
        let input_res = ui.interact(*to_original.from(), ui.id(), Sense::click());
        if input_res.interact_pointer_pos().is_none() {
            return;
        }
        let pos = input_res.interact_pointer_pos().unwrap();

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
            if self.mode == mode {
                self.mode = Mode::Idle;
            } else {
                self.mode = mode;
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
        let alpha: Alpha = Alpha::BlendOrAdditive;
        egui::color_picker::color_edit_button_rgba(ui, &mut self.current_color, alpha);
    }

    fn undo(&mut self) {
        if self.annotations.len() > 0 {
            let undone = self.annotations.pop().unwrap();
            if let Annotation::Eraser(e) = undone.clone() {
                self.annotations.push(*e);
            }
            self.undone_annotations.push(undone);
        }
        match mem::take(&mut self.last_mode) {
            Some(mode) => self.mode = mode,
            None => self.mode = Mode::Idle,
        }
    }
    fn redo(&mut self) {
        if self.undone_annotations.len() > 0 {
            let redo = self.undone_annotations.pop().unwrap();
            if let Annotation::Eraser(_) = redo {
                self.annotations.pop();
            }
            self.annotations.push(redo);
        }
        match mem::take(&mut self.last_mode) {
            Some(mode) => self.mode = mode,
            None => self.mode = Mode::Idle,
        }
    }
}
