use std::thread;

use egui::{
    ColorImage, FontFamily, FontId, Pos2, Rect, TextStyle, TextureHandle, TextureOptions, Vec2,
};
use screenshots::Screen;
use serde::{Deserialize, Serialize};

use crate::pages::capture::capture_page;
use crate::pages::launcher::launcher_page;
use crate::pages::settings::settings_page;
use crate::pages::types::{PageType, SettingType};
use crate::types::config::Config;
use crate::types::editor::Editor;
use crate::types::rasterizer::Rasterizer;
use crate::types::utils::export_color_image_to_skia_image;

pub const APP_KEY: &str = "screen-grabber";

#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ScreenGrabber {
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

    //settings
    #[serde(skip)]
    pub active_section: SettingType,
    #[serde(skip)]
    pub config: Config,
    #[serde(skip)]
    pub prev_config: Config,
}

impl Default for ScreenGrabber {
    fn default() -> Self {
        Self {
            current_page: PageType::Launcher,
            is_minimized: false,
            texture_image: None,
            captured_image: None,
            editor: Editor::default(),
            //settings
            active_section: SettingType::General,
            config: Config::load_or_default(),
            prev_config: Config::load_or_default(),
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
        self.texture_image = Some(id.clone());
        self.captured_image = Some(image.clone());
        self.editor.texture = Some(id.clone());
        self.editor.captured_image = Some(image.clone());
        self.editor.original_rect = Rect::from_min_size(Pos2::ZERO, id.size_vec2());
        self.editor.crop_rect = Rect::from_min_size(Pos2::ZERO, id.size_vec2());
    }

    pub fn save_as(&mut self) -> Option<()> {
        if !self.has_captured_image() {
            return None;
        }
        //Here we should get the output path (from config or rfd)
        let size = self
            .captured_image
            .as_ref()
            .expect("cannot get captured image")
            .size
            .clone();
        let image = export_color_image_to_skia_image(&self.captured_image.as_ref().unwrap());
        if image.is_none() {
            return None;
        }
        let annotations = self.editor.annotations.clone();
        thread::spawn(move || {
            let mut rasterizer = Rasterizer::new((size[0] as u32, size[1] as u32), (1920, 1080));
            rasterizer.add_screenshot(image.as_ref().unwrap(), (0, 0));
            rasterizer.add_annotations(annotations.as_ref());
            match rasterizer.export("./out.png") {
                Some(_) => Some(()),
                None => None,
            }
        })
        .join()
        .unwrap()
    }
    ///settings (to understand if this is the right place for setters of settings)
    pub fn set_active_section(&mut self, session: SettingType) {
        self.active_section = session
    }

    // pub fn load_config(&mut self) -> Result<(), confy::ConfyError> {
    //     self.config = confy::load("screen-grabber", "config")?;
    //     Ok(())
    // }
    pub fn store_config(&mut self) -> Result<(), confy::ConfyError> {
        println!("{}", &self.config.get_example_test());
        confy::store("screen-grabber", "config", &self.config)?;
        Ok(())
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
    //Defaults are pretty good but in case we want to change them or allow the user to do so this
    // is the way to do it (at least one possible way)

    use FontFamily::{Monospace, Proportional};

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(16.0, Proportional)),
        (TextStyle::Body, FontId::new(16.0, Proportional)),
        (TextStyle::Monospace, FontId::new(16.0, Monospace)),
        (TextStyle::Button, FontId::new(16.0, Proportional)),
        (TextStyle::Small, FontId::new(16.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}
