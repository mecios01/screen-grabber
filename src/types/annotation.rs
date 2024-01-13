use eframe::emath::{Pos2, RectTransform, Rot2, Vec2};
use egui::epaint::{CircleShape, RectShape, TextShape};
use egui::{Color32, FontId, Painter, Rect, Shape, Stroke};

#[derive(Debug, Clone)]
pub enum Annotation {
    Segment(SegmentAnnotation),
    Circle(CircleAnnotation),
    Rect(RectAnnotation),
    Arrow(ArrowAnnotation),
    Pencil(PencilAnnotation),
    Text(TextAnnotation),
    Eraser(EraserAnnotation),
    Crop(CropAnnotation),
    Highlighter(HighlighterAnnotation),
}

impl Annotation {
    pub fn segment(starting: Pos2, color: Color32, width: f32) -> Self {
        Self::Segment(SegmentAnnotation::new(starting, color, width))
    }
    pub fn circle(center: Pos2, color: Color32, thickness: f32, fill_color: Color32) -> Self {
        Self::Circle(CircleAnnotation::new(center, color, thickness, fill_color))
    }

    pub fn rect(pos: Pos2, color: Color32, fill_color: Color32, thickness: f32) -> Self {
        Self::Rect(RectAnnotation::new(pos, pos, thickness, color, fill_color))
    }

    pub fn arrow(starting: Pos2, color: Color32, thickenss: f32) -> Self {
        Self::Arrow(ArrowAnnotation::new(starting, color, thickenss))
    }

    pub fn pencil(starting: Pos2, color: Color32, width: f32) -> Self {
        Self::Pencil(PencilAnnotation::new(starting, color, width))
    }

    pub fn highlighter(starting: Pos2, color: Color32, width: f32) -> Self {
        Self::Highlighter(HighlighterAnnotation::new(starting, color, width))
    }

    pub fn text(pos: Pos2, color: Color32, font_size: f32) -> Self {
        Self::Text(TextAnnotation::new(pos, color, font_size))
    }

    pub fn eraser(annotation: Annotation, index: usize) -> Self {
        Self::Eraser(EraserAnnotation::new(index, Box::new(annotation)))
    }

    pub fn crop(pos: Pos2) -> Self {
        Self::Crop(CropAnnotation::new(pos, pos))
    }
    pub fn render(
        &self,
        scaling: f32,
        rect_transform: RectTransform,
        painter: &Painter,
        editing: bool,
    ) -> Shape {
        match self {
            Annotation::Segment(s) => s.render(scaling, rect_transform),
            Annotation::Circle(c) => c.render(scaling, rect_transform),
            Annotation::Rect(r) => r.render(scaling, rect_transform),
            Annotation::Arrow(a) => a.render(scaling, rect_transform),
            Annotation::Pencil(p) => p.render(scaling, rect_transform),
            Annotation::Highlighter(h) => h.render(scaling, rect_transform),
            Annotation::Text(t) => t.render(scaling, rect_transform, painter, editing),
            Annotation::Eraser(_) => Shape::Noop,
            Annotation::Crop(c) => c.render(scaling, rect_transform),
        }
    }

    pub fn check_click(
        &self,
        click: Pos2,
        scaling: f32,
        rect_transform: RectTransform,
        painter: &Painter,
    ) -> bool {
        self.render(scaling, rect_transform, painter, false)
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
    pub width: f32,
}

impl SegmentAnnotation {
    fn new(starting: Pos2, color: Color32, width: f32) -> Self {
        Self {
            starting_pos: starting,
            ending_pos: starting,
            color,
            width,
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
            Stroke::new(self.width * scaling, self.color),
        )
    }
}

#[derive(Debug, Clone)]
pub struct CircleAnnotation {
    pub center: Pos2,
    pub radius: f32,
    pub color: Color32,
    pub width: f32,
    pub fill_color: Color32,
}

impl CircleAnnotation {
    pub fn new(center: Pos2, color: Color32, width: f32, fill_color: Color32) -> Self {
        Self {
            center,
            radius: 0.0,
            color,
            width,
            fill_color,
        }
    }
    pub fn render(&self, scaling: f32, rect_transform: RectTransform) -> Shape {
        Shape::Circle(CircleShape {
            center: rect_transform.transform_pos(self.center),
            radius: self.radius * scaling,
            fill: self.fill_color,
            stroke: Stroke::new(self.width * scaling, self.color),
        })
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
    pub width: f32,
    pub fill_color: Color32,
}

impl RectAnnotation {
    pub fn new(p1: Pos2, p2: Pos2, width: f32, color: Color32, fill_color: Color32) -> Self {
        Self {
            p1,
            p2,
            color,
            width,
            fill_color,
        }
    }
    pub fn render(&self, scaling: f32, rect_transform: RectTransform) -> Shape {
        Shape::Rect(RectShape::new(
            Rect::from_two_pos(
                rect_transform.transform_pos(self.p1),
                rect_transform.transform_pos(self.p2),
            ),
            0.0,
            self.fill_color,
            Stroke::new(self.width * scaling, self.color),
        ))
    }
    pub fn update_p2(&mut self, p2: Pos2) {
        self.p2 = p2;
    }
    pub fn update_color(&mut self, color: Color32) {
        self.color = color;
    }
}

#[derive(Debug, Clone)]
pub struct Tip {
    pub line1: (Pos2, Pos2),
    pub line2: (Pos2, Pos2),
}

impl Default for Tip {
    fn default() -> Self {
        Self {
            line1: (Pos2::default(), Pos2::default()),
            line2: (Pos2::default(), Pos2::default()),
        }
    }
}

impl Tip {
    pub fn set_line1(&mut self, line1: (Pos2, Pos2)) {
        self.line1 = line1;
    }
    pub fn set_line2(&mut self, line2: (Pos2, Pos2)) {
        self.line2 = line2;
    }
}

#[derive(Debug, Clone)]
pub struct ArrowAnnotation {
    pub starting_pos: Pos2,
    pub ending_pos: Pos2,
    pub color: Color32,
    pub width: f32,
    pub tip: Tip,
}

impl ArrowAnnotation {
    fn new(starting: Pos2, color: Color32, width: f32) -> Self {
        Self {
            starting_pos: starting,
            ending_pos: starting,
            color,
            width,
            tip: Tip::default(),
        }
    }

    pub fn update_ending(&mut self, updating: Pos2) {
        self.ending_pos = updating;
    }
    ///this allows us to export the tip (if done in the render/update phase causes the tip to glitch)
    pub fn consolidate(&mut self) {
        let rot = Rot2::from_angle(std::f32::consts::TAU / 10.0);
        let vec = self.ending_pos - self.starting_pos;
        let tip_length = vec.length() / 4.0;
        let tip = self.ending_pos;
        let dir = vec.normalized();
        self.tip
            .set_line1((tip, self.ending_pos - tip_length * (rot * dir)));
        self.tip
            .set_line2((tip, self.ending_pos - tip_length * (rot.inverse() * dir)));
    }
    fn render(&self, scaling: f32, rect_transform: RectTransform) -> Shape {
        let rot = Rot2::from_angle(std::f32::consts::TAU / 10.0);
        let vec = self.ending_pos - self.starting_pos;
        let tip_length = vec.length() / 4.0;
        let tip = rect_transform.transform_pos(self.ending_pos);
        let dir = vec.normalized();
        let line1 = [
            tip,
            rect_transform.transform_pos(self.ending_pos - tip_length * (rot * dir)),
        ];
        let line2 = [
            tip,
            rect_transform.transform_pos(self.ending_pos - tip_length * (rot.inverse() * dir)),
        ];
        let body = Shape::line_segment(
            [rect_transform.transform_pos(self.starting_pos), tip],
            Stroke::new(self.width * scaling, self.color),
        );
        let tip1 = Shape::line_segment(line1, Stroke::new(self.width * scaling, self.color));
        let tip2 = Shape::line_segment(line2, Stroke::new(self.width * scaling, self.color));

        Shape::Vec(vec![body, tip1, tip2])
    }
}

#[derive(Debug, Clone)]
pub struct TextAnnotation {
    pub pos: Pos2,
    pub text: String,
    pub size: f32,
    pub color: Color32,
}

impl TextAnnotation {
    fn new(pos: Pos2, color: Color32, font_size: f32) -> Self {
        Self {
            pos,
            text: String::from(""),
            size: font_size,
            color,
        }
    }

    pub fn update_text(&mut self, new_text: &String) {
        self.text = self.text.clone() + new_text.as_str();
    }

    pub fn delete_char(&mut self) {
        self.text.pop();
    }
    fn render(
        &self,
        scaling: f32,
        to_screen: RectTransform,
        painter: &Painter,
        editing: bool,
    ) -> Shape {
        let galley = painter.layout_no_wrap(
            self.text.clone(),
            FontId::monospace(self.size * scaling),
            self.color,
        );
        let text_shape = Shape::Text(TextShape::new(
            to_screen.transform_pos(self.pos),
            galley,
            Color32::BLACK,
        )); //added Color32 field after egui update
        if !editing {
            return text_shape;
        }

        let mut rect = text_shape.visual_bounding_rect().expand(4.0);
        if rect.any_nan() {
            rect = Rect::from_two_pos(
                to_screen.transform_pos(self.pos),
                to_screen.transform_pos(
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
    pub width: f32,
}

impl PencilAnnotation {
    fn new(pos: Pos2, color: Color32, width: f32) -> Self {
        Self {
            points: vec![pos],
            color,
            width,
        }
    }
    pub fn update_points(&mut self, pos: Pos2) {
        if let Some(last) = self.points.last() {
            if pos == *last {
                return;
            }
        }
        self.points.push(pos);
    }

    pub fn render(&self, scaling: f32, rect_transform: RectTransform) -> Shape {
        let line: Vec<Pos2> = self
            .points
            .iter()
            .map(|p| rect_transform.transform_pos(*p))
            .collect();

        Shape::line(line, Stroke::new(self.width * scaling, self.color))
    }
}

#[derive(Debug, Clone)]
pub struct HighlighterAnnotation {
    pub points: Vec<Pos2>,
    pub color: Color32,
    pub width: f32,
}

impl HighlighterAnnotation {
    fn new(pos: Pos2, color: Color32, width: f32) -> Self {
        Self {
            points: vec![pos],
            color,
            width,
        }
    }
    pub fn update_points(&mut self, pos: Pos2) {
        if let Some(last) = self.points.last() {
            if pos == *last {
                return;
            }
        }
        self.points.push(pos);
    }

    pub fn render(&self, scaling: f32, rect_transform: RectTransform) -> Shape {
        let line: Vec<Pos2> = self
            .points
            .iter()
            .map(|p| rect_transform.transform_pos(*p))
            .collect();

        Shape::line(line, Stroke::new(self.width * scaling, self.color))
    }
}

#[derive(Debug, Clone)]
pub struct EraserAnnotation {
    pub annotation: Box<Annotation>,
    pub index: usize,
}

impl EraserAnnotation {
    pub fn new(index: usize, annotation: Box<Annotation>) -> Self {
        Self { index, annotation }
    }
}

#[derive(Debug, Clone)]
pub struct CropAnnotation {
    pub p1: Pos2,
    pub p2: Pos2,
    pub resizing: bool,
    pub finished: bool,
}

impl CropAnnotation {
    pub fn new(p1: Pos2, p2: Pos2) -> Self {
        Self {
            p1,
            p2,
            resizing: true,
            finished: false,
        }
    }

    pub fn reset_points(&mut self) {
        let rect = Rect::from_two_pos(self.p1, self.p2);
        self.p1 = rect.min;
        self.p2 = rect.max;
    }
    pub fn render(&self, _scaling: f32, rect_transform: RectTransform) -> Shape {
        if self.finished {
            return Shape::Noop;
        };
        let color = Color32::from_rgb(255, 255, 255);
        let rect = Rect::from_two_pos(
            rect_transform.transform_pos(self.p1),
            rect_transform.transform_pos(self.p2),
        );
        let border = Shape::rect_stroke(rect, 0.0, Stroke::new(1.0, color)); //no scaling it's virtual
        let mut cps: Vec<Shape> = self
            .get_points(rect_transform)
            .iter()
            .map(|p| Shape::circle_filled(*p, 5.0, color)) //no scaling virtual
            .collect();

        let top = Rect::everything_above(rect.min.y);
        let bot = Rect::everything_below(rect.max.y);
        let mut left = Rect::everything_left_of(rect.min.x);
        left.set_top(rect.min.y);
        left.set_bottom(rect.max.y);
        let mut right = Rect::everything_right_of(rect.right());
        right.set_top(rect.top());
        right.set_bottom(rect.bottom());
        cps.push(Shape::rect_filled(top, 0.0, Color32::from_white_alpha(5)));
        cps.push(Shape::rect_filled(bot, 0.0, Color32::from_white_alpha(5)));
        cps.push(Shape::rect_filled(left, 0.0, Color32::from_white_alpha(5)));
        cps.push(Shape::rect_filled(right, 0.0, Color32::from_white_alpha(5)));
        cps.push(border);
        Shape::Vec(cps)
    }

    pub fn get_points(&self, to_screen: RectTransform) -> Vec<Pos2> {
        let rect = to_screen.transform_rect(Rect::from_two_pos(self.p1, self.p2));
        vec![
            rect.left_top(),
            rect.center_top(),
            rect.right_top(),
            rect.left_center(),
            rect.right_center(),
            rect.left_bottom(),
            rect.center_bottom(),
            rect.right_bottom(),
        ]
    }
    pub fn get_control_points(&self, to_screen: RectTransform) -> Vec<ControlPoint> {
        let rect = to_screen.transform_rect(Rect::from_two_pos(self.p1, self.p2));
        vec![
            ControlPoint::new(rect.left_top(), Position::LeftTop),
            ControlPoint::new(rect.center_top(), Position::CenterTop),
            ControlPoint::new(rect.right_top(), Position::RightTop),
            ControlPoint::new(rect.left_center(), Position::LeftCenter),
            ControlPoint::new(rect.right_center(), Position::RightCenter),
            ControlPoint::new(rect.left_bottom(), Position::LeftBottom),
            ControlPoint::new(rect.center_bottom(), Position::CenterBottom),
            ControlPoint::new(rect.right_bottom(), Position::RightBottom),
        ]
    }
    pub fn update(&mut self, pos: Pos2) {
        self.p2 = pos;
    }
    pub fn update_resize(&mut self, value: bool) {
        self.resizing = value;
    }

    pub fn update_finished(&mut self, value: bool) {
        self.finished = value;
    }

    pub fn get_rect(&self) -> Rect {
        Rect::from_two_pos(self.p1, self.p2)
    }
}

pub enum Position {
    LeftTop,
    CenterTop,
    RightTop,
    LeftCenter,
    RightCenter,
    LeftBottom,
    CenterBottom,
    RightBottom,
}
pub struct ControlPoint {
    pub pos: Pos2,
    pub label: Position,
}

impl ControlPoint {
    pub fn new(pos: Pos2, label: Position) -> Self {
        Self { pos, label }
    }
}
