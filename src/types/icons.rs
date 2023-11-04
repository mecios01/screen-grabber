use egui::Image;

lazy_static::lazy_static! {
    pub static ref CURSOR_SVG: Image<'static> = Image::new(egui::include_image!("../assets/icons/cursor.svg"));
    pub static ref ELLIPSE_SVG: Image<'static> = Image::new(egui::include_image!("../assets/icons/ellipse.svg"));
    pub static ref ERASER_SVG: Image<'static> = Image::new(egui::include_image!("../assets/icons/eraser.svg"));
    pub static ref LINE_SVG: Image<'static> = Image::new(egui::include_image!("../assets/icons/line.svg"));
    pub static ref MOVE_SVG: Image<'static> = Image::new(egui::include_image!("../assets/icons/move.svg"));
    pub static ref RECTANGLE_SVG: Image<'static> = Image::new(egui::include_image!("../assets/icons/rectangle.svg"));
    pub static ref SELECT_SVG: Image<'static> = Image::new(egui::include_image!("../assets/icons/select.svg"));
    pub static ref TEXT_SVG: Image<'static> = Image::new(egui::include_image!("../assets/icons/text.svg"));
    pub static ref SETTINGS_SVG: Image<'static> = Image::new(egui::include_image!("../assets/icons/settings.svg"));
}
