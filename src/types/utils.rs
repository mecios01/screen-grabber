use std::io::Cursor;

use egui::ColorImage;
use image::{ImageOutputFormat, RgbaImage};
use skia_safe::{Data, Image};

pub fn export_color_image_to_skia_image(ci: &ColorImage) -> Option<Image> {
    let rgba_img = RgbaImage::from_raw(
        ci.width() as u32,
        ci.height() as u32,
        Vec::from(ci.as_raw()),
    );
    if rgba_img.is_none() {
        return None;
    }

    //export into memory buffer (works almost like a file)
    let mut buff = Cursor::new(Vec::<u8>::new());
    if rgba_img
        .unwrap()
        .write_to(&mut buff, ImageOutputFormat::Png)
        .is_err()
    {
        return None;
    }

    let data = Data::new_copy(&buff.into_inner());
    if let Some(image) = Image::from_encoded(data) {
        Some(image)
    } else {
        None
    }
}