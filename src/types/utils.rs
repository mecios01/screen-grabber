use std::io::Cursor;
use std::path::PathBuf;

use chrono::Utc;
use egui::{ColorImage, IconData};
use image::{GenericImageView, ImageOutputFormat, RgbaImage};
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

pub fn save_dialog() -> Option<PathBuf> {
    let path = std::env::current_dir().unwrap();
    let dt = Utc::now();

    let timestamp = dt.format("%Y%m%d_%H%M%S");
    let res = rfd::FileDialog::new()
        .set_file_name(format!("{}", timestamp))
        .add_filter("png", &["png"])
        .add_filter("jpg", &["jpg"])
        .add_filter("gif", &["gif"])
        .add_filter("bmp", &["bmp"])
        .set_directory(&path)
        .save_file();
    res
}

pub fn load_icon(path: &str) -> Result<IconData, String> {
    let icon = image::open(path).map_err(|e| format!("Failed to load icon: {}", e))?;
    let (width, height) = icon.dimensions();
    let icon = IconData {
        rgba: icon.into_bytes(),
        width,
        height,
    };
    Ok(icon)
}
