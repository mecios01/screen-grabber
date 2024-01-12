use std::borrow::Cow;
use std::fs::File;
use std::io::{Cursor, Write};
use std::path::PathBuf;

use arboard::ImageData;
use egui::{Color32, Pos2};
use image::ImageFormat;
use skia_safe::paint::Cap;
use skia_safe::textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle};
use skia_safe::{
    path, surfaces, AlphaType, BlendMode, Color, ColorSpace, EncodedImageFormat, FontMgr, IRect,
    Image, ImageInfo, Paint, PaintStyle, Point, Rect, Surface,
};

use crate::types::annotation::{
    Annotation, ArrowAnnotation, CircleAnnotation, HighlighterAnnotation, PencilAnnotation,
    RectAnnotation, SegmentAnnotation, TextAnnotation,
};
use crate::types::save_destination::SaveDestination;

pub struct Rasterizer {
    //size of the desired cropped area
    crop_area: (Pos2, Pos2),
    //rendering surface. It exposes a canvas to draw on
    surface: Surface,
}

impl Rasterizer {
    fn default_screenshot_paint() -> Paint {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Stroke);
        paint
    }
    fn default_stroke_paint() -> Paint {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Stroke);
        paint.set_blend_mode(BlendMode::SrcOver);
        paint
    }
    fn default_fill_paint() -> Paint {
        let mut paint = Paint::default();
        paint.set_anti_alias(true);
        paint.set_style(PaintStyle::Fill);
        paint.set_blend_mode(BlendMode::SrcOver);
        paint
    }
    fn color_from_color32(color: Color32) -> Color {
        let [r, g, b, a] = color.to_srgba_unmultiplied();
        Color::from_argb(a, r, g, b)
    }
}

impl Rasterizer {
    ///create a new rasterizer given a surface size
    pub fn new(canvas_size: (u32, u32), crop_size: (Pos2, Pos2)) -> Self {
        let surface = surfaces::raster(
            &ImageInfo::new_n32(
                (canvas_size.0 as i32, canvas_size.1 as i32),
                AlphaType::Premul,
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
        let paint = Rasterizer::default_screenshot_paint();
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
            Annotation::Highlighter(highlighter) => self.draw_highlighter(highlighter),
            _ => {}
        })
    }

    //this handles all the extensions different from jpg, png
    fn save_other_formats(data: skia_safe::Data, mut path: PathBuf) -> Option<()> {
        println!("SAVE PATH:{}", path.display());
        let format = match path.extension() {
            Some(osstr) => {
                match osstr.to_str() {
                    Some(e) => match e {
                        "gif" => ImageFormat::Gif,
                        "bmp" => ImageFormat::Bmp,
                        _ => {
                            //we do not support other formats
                            path.set_extension("png");
                            ImageFormat::Png
                        }
                    },
                    None => return None,
                }
            }
            _ => return None,
        };
        let reader = image::io::Reader::with_format(Cursor::new(data.as_bytes()), ImageFormat::Png);
        let decoded = reader.decode();
        if decoded.is_err() {
            return None;
        }

        let res = decoded.unwrap().save_with_format(path, format);
        if res.is_err() {
            println!("{:?}", res.err());
            return None;
        }
        return Some(());
    }
    //export the image into the given format (or PNG then convert)
    pub fn export(self, dest: SaveDestination) -> Option<()> {
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
                let mut default_encoder = EncodedImageFormat::PNG;

                match dest {
                    SaveDestination::RealPath(ref p) => {
                        if p.extension().is_some_and(|e| e.to_str().unwrap() == "jpg") {
                            default_encoder = EncodedImageFormat::JPEG
                        }
                    }
                    _ => {}
                }

                let res = image.encode(context.as_mut(), default_encoder, Some(100));
                if res.is_none() {
                    eprintln!("Cannot decode");
                    return None;
                }

                if dest.is_path() {
                    let mut pb = dest.path().unwrap();
                    match pb.extension() {
                        Some(e) => match e.to_str() {
                            Some("png") => {}
                            Some("jpg") => {}
                            Some(_) => return Self::save_other_formats(res.unwrap(), pb),
                            None => {
                                pb.set_extension("png");
                            }
                        },
                        _ => {}
                    }
                    //fallback save as PNG
                    let d = res.unwrap();
                    let bytes = d.as_bytes();
                    let mut file = File::create(pb.as_path()).unwrap();
                    match file.write_all(bytes) {
                        Ok(()) => Some(()),
                        Err(e) => {
                            eprintln!("{:?}", e);
                            None
                        }
                    }
                } else {
                    let d = res.unwrap();
                    let unc = d.as_bytes();
                    let img = image::load_from_memory(unc).unwrap();
                    let fs = img.as_flat_samples_u8().unwrap();

                    let bytes = Cow::Borrowed(fs.samples);
                    let arc = dest.clipboard().unwrap();
                    let mut guard = match arc.lock() {
                        Ok(guard) => guard,
                        Err(_) => return None,
                    };

                    let result = guard.set_image(ImageData {
                        width: image.width() as usize,
                        height: image.height() as usize,
                        bytes,
                    });
                    return match result {
                        Ok(_) => Some(()),
                        Err(_e) => None,
                    };
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

    fn draw_highlighter(&mut self, highlighter: &HighlighterAnnotation) {
        let canvas = self.get_canvas();
        let mut paint = Rasterizer::default_stroke_paint();
        let color = Rasterizer::color_from_color32(highlighter.color);

        paint.set_color(color);
        paint.set_stroke_width(highlighter.width);
        paint.set_style(PaintStyle::Stroke);
        let points = highlighter
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
