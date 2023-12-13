use egui::{IconData, Image};
use image::{load_from_memory_with_format, GenericImageView, ImageFormat};

pub fn app_icon() -> IconData {
    let bytes = include_bytes!("../assets/icons/screengrabber.png");
    let image = load_from_memory_with_format(bytes, ImageFormat::Png).unwrap();
    let (width, height) = image.dimensions();
    IconData {
        width,
        height,
        rgba: image.into_bytes(),
    }
}

lazy_static::lazy_static! {
pub static ref ARROW :Image<'static>  = Image::new(egui::include_image!("../assets/icons/light/arrow.png"));
pub static ref CIRCLE :Image<'static> = Image::new(egui::include_image!("../assets/icons/light/circle.png"));
pub static ref CROP :Image<'static> = Image::new(egui::include_image!("../assets/icons/light/crop.png"));
pub static ref CURSOR :Image<'static> = Image::new(egui::include_image!("../assets/icons/light/cursor.png"));
pub static ref ERASER :Image<'static> = Image::new(egui::include_image!("../assets/icons/light/eraser.png"));
pub static ref HIGHLIGHT :Image<'static> = Image::new(egui::include_image!("../assets/icons/light/highlighter.png"));
pub static ref LINE :Image<'static> = Image::new(egui::include_image!("../assets/icons/light/line.png"));
pub static ref PENCIL :Image<'static> = Image::new(egui::include_image!("../assets/icons/light/pencil.png"));
pub static ref RECTANGLE :Image<'static> = Image::new(egui::include_image!("../assets/icons/light/rectangle.png"));
pub static ref REDO :Image<'static> = Image::new(egui::include_image!("../assets/icons/light/redo.png"));
pub static ref TEXT :Image<'static> = Image::new(egui::include_image!("../assets/icons/light/text.png"));
pub static ref UNDO :Image<'static> = Image::new(egui::include_image!("../assets/icons/light/undo.png"));
pub static ref ZOOMM :Image<'static> = Image::new(egui::include_image!("../assets/icons/light/zoomm.png"));
pub static ref ZOOMP :Image<'static> = Image::new(egui::include_image!("../assets/icons/light/zoomp.png"));

//dark version
pub static ref ARROW_DARK :Image<'static>  = Image::new(egui::include_image!("../assets/icons/dark/arrow_dark.png"));
pub static ref CIRCLE_DARK :Image<'static> = Image::new(egui::include_image!("../assets/icons/dark/circle_dark.png"));
pub static ref CROP_DARK :Image<'static> = Image::new(egui::include_image!("../assets/icons/dark/crop_dark.png"));
pub static ref CURSOR_DARK :Image<'static> = Image::new(egui::include_image!("../assets/icons/dark/cursor_dark.png"));
pub static ref ERASER_DARK :Image<'static> = Image::new(egui::include_image!("../assets/icons/dark/eraser_dark.png"));
pub static ref HIGHLIGHT_DARK :Image<'static> = Image::new(egui::include_image!("../assets/icons/dark/highlighter_dark.png"));
pub static ref LINE_DARK :Image<'static> = Image::new(egui::include_image!("../assets/icons/dark/line_dark.png"));
pub static ref PENCIL_DARK :Image<'static> = Image::new(egui::include_image!("../assets/icons/dark/pencil_dark.png"));
pub static ref RECTANGLE_DARK :Image<'static> = Image::new(egui::include_image!("../assets/icons/dark/rectangle_dark.png"));
pub static ref REDO_DARK :Image<'static> = Image::new(egui::include_image!("../assets/icons/dark/redo_dark.png"));
pub static ref TEXT_DARK :Image<'static> = Image::new(egui::include_image!("../assets/icons/dark/text_dark.png"));
pub static ref UNDO_DARK :Image<'static> = Image::new(egui::include_image!("../assets/icons/dark/undo_dark.png"));
pub static ref ZOOMM_DARK :Image<'static> = Image::new(egui::include_image!("../assets/icons/dark/zoomm_dark.png"));
pub static ref ZOOMP_DARK :Image<'static> = Image::new(egui::include_image!("../assets/icons/dark/zoomp_dark.png"));

pub static ref SETTINGS_SVG: Image<'static> = Image::new(egui::include_image!("../assets/icons/settings.svg"));}
