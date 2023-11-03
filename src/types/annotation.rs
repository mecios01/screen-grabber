use eframe::emath::{Pos2, RectTransform, Rot2, Vec2};
use egui::epaint::TextShape;
use egui::{Color32, FontId, Painter, Rect, Shape, Stroke};

#[derive(Debug, Clone)]
pub enum Annotation {
    Segment(SegmentAnnotation),
    Circle(CircleAnnotation),
    Rect(RectAnnotation),
    Arrow(ArrowAnnotation),
    Pencil(PencilAnnotation),
    Text(TextAnnotation),
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

    pub fn arrow(starting: Pos2, color: Color32) -> Self {
        Self::Arrow(ArrowAnnotation::new(starting, color))
    }

    pub fn pencil(starting: Pos2, color: Color32) -> Self {
        Self::Pencil(PencilAnnotation::new(starting, color))
    }

    pub fn text(pos: Pos2, color: Color32) -> Self {
        Self::Text(TextAnnotation::new(pos, color))
    }
    pub fn render(&self, scaling: f32, rect_transform: RectTransform, painter: &Painter) -> Shape {
        match self {
            Annotation::Segment(s) => s.render(scaling, rect_transform),
            Annotation::Circle(c) => c.render(scaling, rect_transform),
            Annotation::Rect(r) => r.render(scaling, rect_transform),
            Annotation::Arrow(a) => a.render(scaling, rect_transform),
            Annotation::Pencil(p) => p.render(scaling, rect_transform),
            Annotation::Text(t) => t.render(scaling, rect_transform, painter),
        }
    }

    pub fn check_click(
        &self,
        click: Pos2,
        scaling: f32,
        rect_transform: RectTransform,
        painter: &Painter,
    ) -> bool {
        self.render(scaling, rect_transform, painter)
            .visual_bounding_rect()
            .contains(click)
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
        Shape::line_segment(
            [
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
    pub fn new(p1: Pos2, p2: Pos2, color: Color32) -> Self {
        Self { p1, p2, color }
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
    pub fn update_p2(&mut self, p2: Pos2) {
        self.p2 = p2;
    }
    pub fn update_color(&mut self, color: Color32) {
        self.color = color;
    }
}

#[derive(Debug, Clone)]
pub struct ArrowAnnotation {
    pub starting_pos: Pos2,
    pub ending_pos: Pos2,
    pub color: Color32,
}

impl ArrowAnnotation {
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
        let rot = Rot2::from_angle(std::f32::consts::TAU / 10.0);
        let vec = self.ending_pos - self.starting_pos;
        let tip_length = vec.length() / 4.0;
        let tip = self.ending_pos;
        let dir = vec.normalized();
        let body = Shape::line_segment(
            [
                rect_transform.transform_pos(self.starting_pos),
                rect_transform.transform_pos(self.ending_pos),
            ],
            Stroke::new(10.0 * scaling, self.color),
        );
        let tip1 = Shape::line_segment(
            [
                rect_transform.transform_pos(tip),
                rect_transform.transform_pos(tip - tip_length * (rot * dir)),
            ],
            Stroke::new(10.0 * scaling, self.color),
        );
        let tip2 = Shape::line_segment(
            [
                rect_transform.transform_pos(tip),
                rect_transform.transform_pos(tip - tip_length * (rot.inverse() * dir)),
            ],
            Stroke::new(10.0 * scaling, self.color),
        );

        Shape::Vec(vec![body, tip1, tip2])
    }
}

#[derive(Debug, Clone)]
pub struct TextAnnotation {
    pub pos: Pos2,
    pub text: String,
    pub size: f32,
    pub color: Color32,
    pub editing: bool,
}

impl TextAnnotation {
    fn new(pos: Pos2, color: Color32) -> Self {
        Self {
            pos,
            text: String::from(""),
            size: 32.0,
            color,
            editing: true,
        }
    }

    pub fn update_text(&mut self, new_text: &String) {
        self.text = self.text.clone() + new_text.as_str();
    }

    pub fn delete_char(&mut self) {
        self.text.pop();
    }

    pub fn update_editing(&mut self, value: bool) {
        self.editing = value;
    }

    fn render(&self, scaling: f32, to_screen: RectTransform, painter: &Painter) -> Shape {
        let galley = painter.layout_no_wrap(
            self.text.clone(),
            FontId::monospace(self.size * scaling),
            self.color,
        );
        let text_shape = Shape::Text(TextShape::new(
            to_screen.transform_pos_clamped(self.pos),
            galley,
        ));
        if !self.editing {
            return text_shape;
        }

        let mut rect = text_shape.visual_bounding_rect().expand(4.0);
        if rect.any_nan() {
            rect = Rect::from_two_pos(
                to_screen.transform_pos_clamped(self.pos),
                to_screen.transform_pos_clamped(
                    self.pos + Vec2::angled(std::f32::consts::TAU / 8.0) * self.size * scaling,
                ),
            )
            .expand(4.0);
        }

        let count = self.text.chars().rev().take_while(|c| *c == '\n').count() as f32;
        rect.extend_with_y((rect.max.y) + (self.size * count * scaling));

        let mut dashed_rect = Shape::dashed_line(
            [
                rect.left_top(),
                rect.right_top(),
                rect.right_bottom(),
                rect.left_bottom(),
                rect.left_top(),
            ]
            .as_slice(),
            Stroke::new(1.0, Color32::LIGHT_GRAY),
            1.0,
            3.0,
        );
        dashed_rect.push(text_shape);
        Shape::Vec(dashed_rect)
    }
}

#[derive(Debug, Clone)]
pub struct PencilAnnotation {
    pub points: Vec<Pos2>,
    pub color: Color32,
}

impl PencilAnnotation {
    fn new(pos: Pos2, color: Color32) -> Self {
        Self {
            points: vec![pos],
            color,
        }
    }
    pub fn update_points(&mut self, pos: Pos2) {
        self.points.push(pos);
    }

    pub fn render(&self, scaling: f32, rect_transform: RectTransform) -> Shape {
        let line: Vec<Pos2> = self
            .points
            .iter()
            .map(|p| rect_transform.transform_pos_clamped(*p))
            .collect();

        Shape::line(line, Stroke::new(10.0 * scaling, self.color))
    }
}
