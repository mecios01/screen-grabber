#![allow(unused_variables, unused_imports)]

use std::fs::File;
use std::io::Write;

use skia_safe::{
    surfaces, Color, Data, EncodedImageFormat, Font, FontStyle, IRect, Image, Paint, PaintStyle,
    Path, Rect, TextBlob, Typeface,
};

fn main() {
    //surface to draw on
    let mut surface = surfaces::raster_n32_premul((1920, 1080)).expect("surface");
    //utility part of the surface to draw stuff
    let canvas = surface.canvas();
    //a path to be drawn (optionally)
    let path = Path::new();
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

    let bytes = include_bytes!("./input.png");
    let data = Data::new_copy(bytes);
    let image = Image::from_encoded(data).unwrap();

    canvas.draw_image(image, (0, 0), Some(&Paint::default()));
    //draw the screenshot instead of the background color
    // canvas.clear(Color::WHITE);

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
    canvas.draw_text_blob(&text, (60, 150), &text_paint);

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
