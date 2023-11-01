use std::default::Default;

use eframe::emath::RectTransform;
use egui::color_picker::Alpha;
use egui::{Image, Rgba, Rounding, Sense, Shape, Stroke, Ui};

use crate::types::annotation::Annotation;
use crate::types::icons::*;
use crate::types::screen_grabber::ScreenGrabber;

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
    fn manage_input(app: &mut ScreenGrabber, ui: &mut Ui, to_original: RectTransform) {}

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
        self.tool_button(ui, &ELLIPSE_SVG, Mode::DrawEllipse);
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
