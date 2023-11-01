use std::thread;

use egui::{ColorImage, TextureHandle, TextureOptions, Vec2};
use screenshots::Screen;
use serde::{Deserialize, Serialize};

use crate::pages::capture::capture_page;
use crate::pages::launcher::launcher_page;
use crate::pages::settings::settings_page;
use crate::pages::types::PageType;
use crate::types::editor::Editor;

pub const APP_KEY: &str = "screen-grabber";

#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ScreenGrabber {
    config: bool,
    //it should be an entire config loaded at start of the app
    current_page: PageType,
    //image captured
    #[serde(skip)]
    pub texture_image: Option<TextureHandle>,

    #[serde(skip)]
    pub captured_image: Option<ColorImage>,
    pub is_minimized: bool,
    #[serde(skip)]
    pub editor: Editor,
}

impl Default for ScreenGrabber {
    fn default() -> Self {
        Self {
            config: false,
            current_page: PageType::Launcher,
            is_minimized: false,
            texture_image: None,
            captured_image: None,
            editor: Editor::default(),
        }
    }
}

impl ScreenGrabber {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        set_font_style(&cc.egui_ctx);

        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
    pub fn set_page(&mut self, page: PageType) {
        self.current_page = page
    }
    #[inline]
    pub fn has_captured_image(&self) -> bool {
        self.texture_image.is_some()
    }

    pub fn get_original_size(&self) -> Vec2 {
        if let Some(image) = &self.texture_image {
            return image.size_vec2();
        }
        Vec2::ZERO
    }
    pub fn set_new_captured_image(&mut self, image: TextureHandle) {
        self.texture_image = Some(image);
        self.is_minimized = false;
    }
    pub fn capture(&mut self, ctx: &egui::Context) {
        let image = thread::spawn(|| {
            let screenshot = Screen::all().unwrap()[0].capture().unwrap();
            let size = [screenshot.width() as _, screenshot.height() as _];
            let pixels = screenshot.as_flat_samples();
            ColorImage::from_rgba_unmultiplied(size, pixels.as_slice())
        })
        .join()
        .unwrap();
        let id = ctx.load_texture("screenshot", image.clone(), TextureOptions::default());
        self.texture_image = Some(id);
        self.captured_image = Some(image);
    }
}

impl eframe::App for ScreenGrabber {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.current_page {
            PageType::Launcher => launcher_page(self, ctx, frame),
            PageType::Capture => capture_page(self, ctx, frame),
            PageType::Settings => settings_page(self, ctx, frame),
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, APP_KEY, self);
    }
}

fn set_font_style(ctx: &egui::Context) {
    // use FontFamily::{Monospace, Proportional};
    //
    // let mut style = (*ctx.style()).clone();
    // style.text_styles = [
    //     (TextStyle::Heading, FontId::new(25.0, Proportional)),
    //     (TextStyle::Body, FontId::new(16.0, Proportional)),
    //     (TextStyle::Monospace, FontId::new(16.0, Monospace)),
    //     (TextStyle::Button, FontId::new(22.0, Proportional)),
    //     (TextStyle::Small, FontId::new(12.0, Proportional)),
    // ]
    // .into();
    // ctx.set_style(style);
}
