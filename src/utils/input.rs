use eframe::emath::{Pos2, RectTransform, Vec2};
use egui::{Color32, Shape, Stroke};

#[derive(Debug, Clone)]
pub enum Annotation {
    Segment(SegmentAnnotation)
}


impl Annotation {
    pub fn segment(starting: Pos2) -> Self {
        Self::Segment(SegmentAnnotation::new(starting))
    }
    pub fn update(& mut self, position: Pos2){
        match self {
            Annotation::Segment(ref mut s) => s.update(position),
        }

    }

    pub fn render(&self, scaling: f32, rect_transform: RectTransform) -> Shape{
        match self {
            Annotation::Segment(s) => s.render(scaling, rect_transform),
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

    fn update(&mut self, updating: Pos2) {
        self.ending_pos = updating;
    }

    fn render(&self, scaling: f32, rect_transform: RectTransform) -> Shape {

        Shape::line(
            vec![
                rect_transform.transform_pos(self.starting_pos),
                rect_transform.transform_pos(self.ending_pos)],
            Stroke::new(10.0 * scaling, Color32::RED))
    }
}