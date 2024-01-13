use std::io::Cursor;

use egui::{ColorImage, IconData, Vec2, ViewportCommand, Visuals};
use global_hotkey::hotkey::HotKey;
use image::{GenericImageView, ImageOutputFormat, RgbaImage};
use rand::random;
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

pub fn new_hotkey_from_str(str: &str) -> u32 {
    if !str.is_empty() {
        HotKey::try_from(str).unwrap().id()
    } else {
        random::<u32>()
    }
}

pub fn set_theme(is_dark: bool) -> Visuals {
    if is_dark {
        Visuals::dark()
    } else {
        Visuals::light()
    }
}
const MIN_INNER_SIZE: Vec2 = Vec2::new(1000.0, 800.0);
const MAX_INNER_SIZE: Vec2 = Vec2::new(500.0, 600.0);

pub fn set_min_inner_size(ctx: &egui::Context) {
    let rect = ctx.available_rect();
    if rect.width() <= MIN_INNER_SIZE.x || rect.height() <= MIN_INNER_SIZE.y {
        ctx.send_viewport_cmd(ViewportCommand::InnerSize(MIN_INNER_SIZE));
    }
}
pub fn set_max_inner_size(ctx: &egui::Context) {
    let rect = ctx.available_rect();
    if rect.width() >= MAX_INNER_SIZE.x || rect.height() >= MAX_INNER_SIZE.y {
        ctx.send_viewport_cmd(ViewportCommand::InnerSize(MAX_INNER_SIZE));
    }
}
