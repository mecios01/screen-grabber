use std::fs::File;
use std::io::{Cursor, Write};
use std::thread;

use egui::ColorImage;
use image::{ImageOutputFormat, RgbaImage};
use screenshots::Screen;
use skia_safe::{
    surfaces,
    textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle},
    Color,
    Data,
    EncodedImageFormat,
    Font,
    FontMgr,
    FontStyle,
    // Path,
    IRect,
    Image,
    Paint,
    PaintStyle,
    Rect,
    TextBlob,
    Typeface,
};

fn main() {
    //surface to draw on
    let mut surface = surfaces::raster_n32_premul((1920, 1080)).expect("surface");
    //utility part of the surface to draw stuff
    let canvas = surface.canvas();
    //a path to be drawn (optionally)
    // let path = Path::new();
    //the paint current in use
    let mut paint = Paint::default();
    paint.set_color(Color::RED);
    paint.set_anti_alias(true);
    paint.set_stroke_width(8.0);
    //set only stroke/stroke&fill
    paint.set_style(PaintStyle::Stroke);

    let mut text_paint = Paint::default();
    text_paint.set_color(Color::from_argb(200, 0, 0, 255));
    text_paint.set_anti_alias(true);
    text_paint.set_stroke_width(1.0);
    //set only stroke/stroke&fill
    text_paint.set_style(PaintStyle::StrokeAndFill);

    let scr = thread::spawn(|| {
        let screenshot = Screen::all().unwrap()[0].capture().unwrap();
        let size = [screenshot.width() as _, screenshot.height() as _];
        let pixels = screenshot.as_flat_samples();
        ColorImage::from_rgba_unmultiplied(size, pixels.as_slice())
    })
    .join()
    .unwrap();

    let rgba_img = RgbaImage::from_raw(
        scr.width() as u32,
        scr.height() as u32,
        Vec::from(scr.as_raw()),
    )
    .expect("cannot get an rgba image");

    //export into memory buffer
    let mut buff = Cursor::new(Vec::<u8>::new());
    rgba_img
        .write_to(&mut buff, ImageOutputFormat::Png)
        .unwrap();
    let data = Data::new_copy(&buff.into_inner());
    let image = Image::from_encoded(data).expect("Cannot make the image");
    canvas.draw_image(image, (0, 0), Some(&Paint::default()));

    canvas.draw_circle((960.0, 540.0), 200.0, &paint);

    canvas.draw_line((0.0, 0.0), (1920.0, 1080.0), &paint);

    canvas.draw_line((0.0, 1080.0), (1920.0, 0.0), &paint);

    canvas.draw_rect(
        Rect::from_point_and_size((760.0, 340.0), (400.0, 400.0)),
        &paint,
    );

    let text = TextBlob::from_str(
        "Hello, Skia!",
        &Font::from_typeface(
            Typeface::new("Arial", FontStyle::default()).unwrap(),
            Some(80.0),
        ),
    )
    .unwrap();
    //this renders simple text
    canvas.draw_text_blob(&text, (60, 150), &text_paint);

    //paragraph handles multiline text
    let mut font_collection = FontCollection::new();
    font_collection.set_default_font_manager(FontMgr::new(), None);
    let mut paragraph_style = ParagraphStyle::new();
    paragraph_style.set_replace_tab_characters(true);
    let mut paragraph_builder = ParagraphBuilder::new(&paragraph_style, font_collection);
    let mut ts = TextStyle::new();
    ts.set_foreground_paint(&text_paint);
    paragraph_builder.push_style(&ts);
    paragraph_builder.add_text("This\nis a multiline\nparagraph");
    let mut paragraph = paragraph_builder.build();
    paragraph.layout(f32::INFINITY);
    paragraph.paint(canvas, (120, 180));

    //shows the crop
    let image = surface
        .image_snapshot()
        .make_subset(None, IRect::from_pt_size((40, 50), (500, 333)).as_ref())
        .unwrap();

    let mut context = surface.direct_context();
    let d = image
        .encode(context.as_mut(), EncodedImageFormat::PNG, Some(100))
        .unwrap();
    let mut file = File::create("./examples/test.png").unwrap();
    let bytes = d.as_bytes();
    file.write_all(bytes).unwrap();
}
