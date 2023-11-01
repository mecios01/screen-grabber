use eframe::emath::{Pos2, RectTransform};
use egui::{Color32, Rect, Response, Shape, Stroke};

use crate::types::screen_grabber::ScreenGrabber;

#[derive(Debug, Clone)]
pub enum Annotation {
    Segment(SegmentAnnotation),
    Circle(CircleAnnotation),
    Rect(RectAnnotation),
}

impl Annotation {
    pub fn segment(starting: Pos2) -> Self {
        Self::Segment(SegmentAnnotation::new(starting))
    }
    pub fn circle(center: Pos2) -> Self {
        Self::Circle(CircleAnnotation::new(center))
    }

    pub fn rect(pos: Pos2) -> Self {
        Self::Rect(RectAnnotation::new(pos, pos))
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
}

impl SegmentAnnotation {
    fn new(starting: Pos2) -> Self {
        Self {
            starting_pos: starting,
            ending_pos: starting,
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
            Stroke::new(
                10.0 * scaling,
                Color32::from_rgba_unmultiplied(255, 0, 0, 25),
            ),
        )
    }
}

#[derive(Debug, Clone)]
pub struct CircleAnnotation {
    pub center: Pos2,
    pub radius: f32,
}

impl CircleAnnotation {
    pub fn new(center: Pos2) -> Self {
        Self {
            center: center,
            radius: 0.0,
        }
    }
    pub fn render(&self, scaling: f32, rect_transform: RectTransform) -> Shape {
        Shape::circle_stroke(
            rect_transform.transform_pos_clamped(self.center),
            self.radius * scaling,
            Stroke::new(
                10.0 * scaling,
                Color32::from_rgba_unmultiplied(255, 0, 0, 25),
            ),
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
    pub min: Pos2,
    pub max: Pos2,
}

impl RectAnnotation {
    pub fn new(min: Pos2, max: Pos2) -> Self {
        Self { min, max }
    }
    pub fn render(&self, scaling: f32, rect_transform: RectTransform) -> Shape {
        Shape::rect_stroke(
            Rect::from([
                rect_transform.transform_pos_clamped(self.min),
                rect_transform.transform_pos_clamped(self.max),
            ]),
            0.0,
            Stroke::new(
                10.0 * scaling,
                Color32::from_rgba_unmultiplied(255, 0, 0, 25),
            ),
        )
    }
    pub fn update_max(&mut self, max: Pos2) {
        self.max = max;
    }
}
