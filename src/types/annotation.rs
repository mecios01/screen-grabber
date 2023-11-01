use eframe::emath::{Pos2, RectTransform};
use eframe::wgpu::Color;
use egui::color_picker::color_edit_button_hsva;
use egui::{Color32, Rect, Response, Shape, Stroke};

use crate::types::screen_grabber::ScreenGrabber;

#[derive(Debug, Clone)]
pub enum Annotation {
    Segment(SegmentAnnotation),
    Circle(CircleAnnotation),
    Rect(RectAnnotation),
}

impl Annotation {
    pub fn segment(starting: Pos2, color: Color32) -> Self {
        Self::Segment(SegmentAnnotation::new(starting, color))
    }
    pub fn circle(center: Pos2, color: Color32) -> Self {
        Self::Circle(CircleAnnotation::new(center, color))
    }

    pub fn rect(pos: Pos2, color: Color32) -> Self {
        Self::Rect(RectAnnotation::new(pos, pos, color))
    }

    pub fn render(&self, scaling: f32, rect_transform: RectTransform) -> Shape {
        match self {
            Annotation::Segment(s) => s.render(scaling, rect_transform),
            Annotation::Circle(c) => c.render(scaling, rect_transform),
            Annotation::Rect(r) => r.render(scaling, rect_transform),
        }
    }
    //TODO
}

#[derive(Debug, Clone)]
pub struct SegmentAnnotation {
    pub starting_pos: Pos2,
    pub ending_pos: Pos2,
    pub color: Color32,
}

impl SegmentAnnotation {
    fn new(starting: Pos2, color: Color32) -> Self {
        Self {
            starting_pos: starting,
            ending_pos: starting,
            color,
        }
    }

    pub fn update_ending(&mut self, updating: Pos2) {
        self.ending_pos = updating;
    }

    fn render(&self, scaling: f32, rect_transform: RectTransform) -> Shape {
        Shape::line(
            vec![
                rect_transform.transform_pos(self.starting_pos),
                rect_transform.transform_pos(self.ending_pos),
            ],
            Stroke::new(10.0 * scaling, self.color),
        )
    }
}

#[derive(Debug, Clone)]
pub struct CircleAnnotation {
    pub center: Pos2,
    pub radius: f32,
    pub color: Color32,
}

impl CircleAnnotation {
    pub fn new(center: Pos2, color: Color32) -> Self {
        Self {
            center,
            radius: 0.0,
            color,
        }
    }
    pub fn render(&self, scaling: f32, rect_transform: RectTransform) -> Shape {
        Shape::circle_stroke(
            rect_transform.transform_pos_clamped(self.center),
            self.radius * scaling,
            Stroke::new(10.0 * scaling, self.color),
        )
    }
    pub fn update_center(&mut self, center: Pos2) {
        self.center = center;
    }
    pub fn update_radius(&mut self, pos: Pos2) {
        self.radius = (pos - self.center).length();
    }
}

#[derive(Debug, Clone)]
pub struct RectAnnotation {
    pub p1: Pos2,
    pub p2: Pos2,
    pub color: Color32,
}

impl RectAnnotation {
    pub fn new(min: Pos2, max: Pos2, color: Color32) -> Self {
        Self {
            p1: min,
            p2: max,
            color,
        }
    }
    pub fn render(&self, scaling: f32, rect_transform: RectTransform) -> Shape {
        Shape::rect_stroke(
            Rect::from_two_pos(
                rect_transform.transform_pos_clamped(self.p1),
                rect_transform.transform_pos_clamped(self.p2),
            ),
            0.0,
            Stroke::new(10.0 * scaling, self.color),
        )
    }
    pub fn update_max(&mut self, max: Pos2) {
        self.p2 = max;
    }
    pub fn update_color(&mut self, color: Color32) {
        self.color = color;
    }
}
