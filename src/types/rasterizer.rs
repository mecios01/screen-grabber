use std::fs::File;
use std::io::Write;
use std::path::Path;

use egui::{Color32, Pos2};
use skia_safe::paint::Cap;
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};
use skia_safe::{
    path, surfaces, AlphaType, Color, ColorSpace, EncodedImageFormat, FontMgr, IRect, Image,
    ImageInfo, Paint, PaintStyle, Point, Rect, Surface,
};

use crate::types::annotation::{
    Annotation, ArrowAnnotation, CircleAnnotation, PencilAnnotation, RectAnnotation,
    SegmentAnnotation, TextAnnotation,
};

pub struct Rasterizer {
    //size of the desired cropped area
    crop_area: (Pos2, Pos2),
    //rendering surface. It exposes a canvas to draw on
    surface: Surface,
}

impl Rasterizer {
    fn default_stroke_paint() -> Paint {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Stroke);
        paint
    }
    fn default_fill_paint() -> Paint {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        paint
    }
    fn color_from_color32(color: Color32) -> Color {
        Color::from_argb(color.a(), color.r(), color.g(), color.b())
    }
}

impl Rasterizer {
    ///create a new rasterizer given a surface size
    pub fn new(canvas_size: (u32, u32), crop_size: (Pos2, Pos2)) -> Self {
        let surface = surfaces::raster(
            &ImageInfo::new_n32(
                (canvas_size.0 as i32, canvas_size.1 as i32),
                AlphaType::Opaque,
                ColorSpace::new_srgb(),
            ),
            None,
            None,
        )
        .unwrap();

        Self {
            // canvas_size,
            crop_area: (crop_size.0, crop_size.1),
            surface,
        }
    }

    pub fn add_screenshot(&mut self, image: &Image, pos: (u32, u32)) {
        let canvas = self.get_canvas();
        let paint = Rasterizer::default_stroke_paint();
        canvas.draw_image(&image, (pos.0 as i32, pos.1 as i32), Some(&paint));
    }
    pub fn add_annotations(&mut self, annotations: &[Annotation]) {
        annotations.iter().for_each(|a| match a {
            Annotation::Segment(segment) => self.draw_line(segment),
            Annotation::Circle(circle) => self.draw_circle(circle),
            Annotation::Rect(rect) => self.draw_rectangle(rect),
            Annotation::Arrow(arrow) => self.draw_arrow(arrow),
            Annotation::Pencil(pencil) => self.draw_pencil(pencil),
            Annotation::Text(text) => self.draw_text(text),
            _ => {}
        })
    }
    //export the image into the given format (or PNG then convert)
    pub fn export<P: AsRef<Path>>(self, path: P) -> Option<()> {
        let mut _self = self;
        let ref dim = _self.crop_area;
        let image_res = _self.surface.image_snapshot().make_subset(
            None,
            IRect::from_pt_size(
                (dim.0.x as i32, dim.0.y as i32),
                ((dim.1.x - dim.0.x) as i32, (dim.1.y - dim.0.y) as i32),
            )
            .as_ref(),
        );
        match image_res {
            Some(image) => {
                let mut context = _self.surface.direct_context();
                let d = image
                    .encode(context.as_mut(), EncodedImageFormat::PNG, Some(100))
                    .unwrap();
                let bytes = d.as_bytes();
                let mut file = File::create(path).unwrap();
                match file.write_all(bytes) {
                    Ok(()) => Some(()),
                    Err(e) => {
                        eprintln!("{:?}", e);
                        None
                    }
                }
            }
            None => {
                return None;
            }
        }
    }
    #[inline]
    fn get_canvas(&mut self) -> &skia_safe::Canvas {
        self.surface.canvas()
    }
    #[inline]
    fn draw_circle(&mut self, circle: &CircleAnnotation) {
        let canvas = self.get_canvas();
        let mut paint = Rasterizer::default_stroke_paint();
        let color = Rasterizer::color_from_color32(circle.color);
        paint.set_color(color);
        paint.set_stroke_width(circle.width);
        paint.set_style(PaintStyle::Stroke);

        if circle.fill_color != Color32::TRANSPARENT {
            let mut fill_paint = Rasterizer::default_fill_paint();
            let c1 = (circle.center.x, circle.center.y);
            let r1 = 1.0 + circle.radius - circle.width / 2.0;
            fill_paint.set_color(Rasterizer::color_from_color32(circle.fill_color));
            canvas.draw_circle(c1, r1, &fill_paint);
        }

        canvas.draw_circle((circle.center.x, circle.center.y), circle.radius, &paint);
    }
    #[inline]
    fn draw_rectangle(&mut self, rect: &RectAnnotation) {
        let canvas = self.get_canvas();
        let color = Rasterizer::color_from_color32(rect.color);
        let mut paint = Rasterizer::default_stroke_paint();
        paint.set_color(color);
        paint.set_stroke_width(rect.width);
        let size = (rect.p2.x - rect.p1.x, rect.p2.y - rect.p1.y);

        if rect.fill_color != Color32::TRANSPARENT {
            let mut fill_paint = Rasterizer::default_fill_paint();
            let hw = rect.width / 2.0 - 1.0;
            let p1 = (rect.p1.x + hw, rect.p1.y + hw);
            let s1 = (1.0 + size.0 - rect.width, 1.0 + size.1 - rect.width);
            fill_paint.set_color(Rasterizer::color_from_color32(rect.fill_color));
            //lets draw the fill color
            canvas.draw_rect(Rect::from_point_and_size(p1, s1), &fill_paint);
        }

        canvas.draw_rect(
            Rect::from_point_and_size((rect.p1.x, rect.p1.y), size),
            &paint,
        );
    }
    #[inline]
    fn draw_line(&mut self, line: &SegmentAnnotation) {
        let canvas = self.get_canvas();
        let mut paint = Rasterizer::default_stroke_paint();
        let color = Rasterizer::color_from_color32(line.color);

        paint.set_color(color);
        paint.set_style(PaintStyle::Stroke);
        paint.set_stroke_width(line.width);

        canvas.draw_line(
            (line.starting_pos.x, line.starting_pos.y),
            (line.ending_pos.x, line.ending_pos.y),
            &paint,
        );
    }
    #[inline]
    fn draw_arrow(&mut self, arrow: &ArrowAnnotation) {
        let canvas = self.get_canvas();
        let mut paint = Rasterizer::default_stroke_paint();
        let color = Rasterizer::color_from_color32(arrow.color);

        paint.set_color(color);
        paint.set_style(PaintStyle::Stroke);
        paint.set_stroke_width(arrow.width);
        paint.set_stroke_cap(Cap::Round);
        canvas.draw_line(
            (arrow.starting_pos.x, arrow.starting_pos.y),
            (arrow.ending_pos.x, arrow.ending_pos.y),
            &paint,
        );
        canvas.draw_line(
            (arrow.tip.line1.0.x, arrow.tip.line1.0.y),
            (arrow.tip.line1.1.x, arrow.tip.line1.1.y),
            &paint,
        );
        canvas.draw_line(
            (arrow.tip.line2.0.x, arrow.tip.line2.0.y),
            (arrow.tip.line2.1.x, arrow.tip.line2.1.y),
            &paint,
        );
    }
    #[inline]
    fn draw_text(&mut self, text: &TextAnnotation) {
        //TODO set font to match the one in egui
        let canvas = self.get_canvas();
        let mut paint = Rasterizer::default_stroke_paint();
        let color = Rasterizer::color_from_color32(text.color);

        paint.set_color(color);
        paint.set_style(PaintStyle::StrokeAndFill);
        paint.set_stroke_width(1.0);
        let mut font_collection = FontCollection::new();
        font_collection.set_default_font_manager(FontMgr::new(), None);
        let mut paragraph_style = ParagraphStyle::new();
        paragraph_style.set_replace_tab_characters(true);
        let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);
        let mut ts = TextStyle::new();
        ts.set_foreground_paint(&paint);
        ts.set_font_size(text.size);
        paragraph_builder.push_style(&ts);
        paragraph_builder.add_text(text.text.as_str());
        let mut paragraph = paragraph_builder.build();
        paragraph.layout(f32::INFINITY); //do not wrap unless a new line is found
        paragraph.paint(canvas, (text.pos.x, text.pos.y));
    }
    #[inline]
    fn draw_pencil(&mut self, pencil: &PencilAnnotation) {
        let canvas = self.get_canvas();
        let mut paint = Rasterizer::default_stroke_paint();
        let color = Rasterizer::color_from_color32(pencil.color);

        paint.set_color(color);
        paint.set_stroke_width(pencil.width);
        paint.set_style(PaintStyle::Stroke);
        let points = pencil
            .points
            .clone()
            .into_iter()
            .map(|p| Point { x: p.x, y: p.y })
            .collect::<Vec<Point>>();
        let mut path = path::Path::new();
        path.add_poly(&points, false);
        canvas.draw_path(&path, &paint);
    }
}
