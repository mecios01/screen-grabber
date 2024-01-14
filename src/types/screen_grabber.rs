use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use arboard::Clipboard;
use crossbeam::channel::TryRecvError;
use egui::{
    ColorImage, Context, FontFamily, FontId, Pos2, Rect, TextStyle, TextureHandle, TextureOptions,
    Vec2, ViewportCommand, Visuals,
};
use egui_keybind::Bind;
use global_hotkey::hotkey::HotKey;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use serde::{Deserialize, Serialize};

use crate::pages::capture::capture_page;
use crate::pages::launcher::launcher_page;
use crate::pages::settings::settings_page;
use crate::pages::types::{PageType, SettingType};
use crate::types::config::{Config, Status};
use crate::types::editor::Editor;
use crate::types::keybinds::HotKeyAction;
use crate::types::rasterizer::Rasterizer;
use crate::types::save_destination::SaveDestination;
use crate::types::sync::{DoubleChannel, MasterSignal, SaveImageData, SlaveSignal};
use crate::types::utils::{
    export_color_image_to_skia_image, set_min_inner_size,
};

pub const APP_KEY: &str = "screen-grabber";

#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct ScreenGrabber {
    //app
    #[serde(skip)]
    pub clipboard: Arc<Mutex<Clipboard>>,
    #[serde(skip)]
    current_page: PageType,
    pub is_minimized: bool,
    #[serde(skip)]
    pub is_saving: bool,
    #[serde(skip)]
    pub is_capturing: bool,
    #[serde(skip)]
    pub is_binding: bool,
    #[serde(skip)]
    pub editor: Editor,
    #[serde(skip)]
    pub capture_delay_s: f32,
    //settings
    #[serde(skip)]
    pub active_section: SettingType,
    #[serde(skip)]
    pub config: Config,
    #[serde(skip)]
    pub prev_config: Config,
    //sync stuff
    #[serde(skip)]
    pub hotkey_channel: DoubleChannel<MasterSignal, SlaveSignal>,
    #[serde(skip)]
    pub save_channel: DoubleChannel<MasterSignal, SlaveSignal>,
    #[serde(skip)]
    pub thread_handles: Vec<JoinHandle<()>>,
}

impl Default for ScreenGrabber {
    fn default() -> Self {
        Self {
            //app
            clipboard: Arc::new(Mutex::new(Clipboard::new().unwrap())),
            current_page: PageType::Launcher,
            is_minimized: false,
            is_saving: false,
            is_capturing: false,
            is_binding: false,
            capture_delay_s: 0.3,
            editor: Editor::default(),
            //settings
            active_section: SettingType::General,
            config: Config::load_or_default(),
            prev_config: Config::load_or_default(),
            //sync
            hotkey_channel: DoubleChannel::new(),
            save_channel: DoubleChannel::new(),
            thread_handles: vec![],
        }
    }
}

impl ScreenGrabber {
    pub fn spawn_threads(&mut self) {
        let hk = self.hotkey_channel.secondary_channel().unwrap();
        let sv = self.save_channel.secondary_channel().unwrap();

        let h1 = self.spawn_hotkeys_thread(*hk);
        self.thread_handles.push(h1);
        let h2 = self.spawn_capture_save_thread(*sv);
        self.thread_handles.push(h2)
    }
    fn spawn_hotkeys_thread(
        &mut self,
        channel: DoubleChannel<SlaveSignal, MasterSignal>,
    ) -> JoinHandle<()> {
        let mut hotkeys = self.config.hotkeys.clone();
        thread::spawn(move || {
            println!("HOTKEYS THREAD ON");
            let manager = GlobalHotKeyManager::new().unwrap();
            let mut hks: Vec<HotKey> = hotkeys
                .iter()
                .map(|kb| HotKey::try_from(kb.key_bind.as_str()).unwrap())
                .collect();
            manager.register_all(&hks).unwrap();

            let receiver = GlobalHotKeyEvent::receiver();

            //init stuff
            'outer: loop {
                match channel.receiver.try_recv() {
                    Ok(signal) => match signal {
                        MasterSignal::SetHotkeys(hotks) => {
                            //register all new hotkeys
                            manager.unregister_all(&hks).unwrap();
                            hotkeys = hotks;

                            hks = hotkeys
                                .iter()
                                .map(|kb| HotKey::try_from(kb.key_bind.as_str()).unwrap())
                                .collect();
                            manager.register_all(&hks).unwrap();
                            //updating hotkey ids
                        }
                        MasterSignal::Shutdown => break 'outer,
                        _ => {}
                    },
                    Err(TryRecvError::Disconnected) => break 'outer,
                    _ => {}
                }
                if let Ok(event) = receiver.try_recv() {
                    if event.state == global_hotkey::HotKeyState::Pressed {
                        let mut action = HotKeyAction::None;
                        if let Some(hotkey) = hotkeys.iter().find(|h| h.id == event.id) {
                            action = hotkey.action.clone();
                        }
                        let _ = channel.sender.send(SlaveSignal::KeyPressed(action));
                    }
                }
                thread::sleep(Duration::from_millis(100));
            }
            println!("HOTKEYS THREAD DEAD");
        })
    }

    fn spawn_capture_save_thread(
        &mut self,
        channel: DoubleChannel<SlaveSignal, MasterSignal>,
    ) -> JoinHandle<()> {
        thread::spawn(move || {
            println!("SAVE THREAD ON");
            //the call recv is blocking so we are actually pausing the thread
            'outer: loop {
                match channel.receiver.recv() {
                    Ok(signal) => match signal {
                        MasterSignal::StartCaptureAfter(duration) => {
                            thread::sleep(duration);
                            let image = {
                                let screenshot =
                                    screenshots::Screen::all().unwrap()[0].capture().unwrap();
                                let size = [screenshot.width() as _, screenshot.height() as _];
                                let pixels = screenshot.as_flat_samples();
                                ColorImage::from_rgba_unmultiplied(size, pixels.as_slice())
                            };
                            let _ = channel.sender.send(SlaveSignal::CaptureDone(image));
                        }
                        MasterSignal::SaveImage(save_data) => {
                            let guard = save_data.image_ref.lock().unwrap();
                            let image = export_color_image_to_skia_image(&guard.as_ref().unwrap());
                            if image.is_none() {
                                let _ = channel.sender.send(SlaveSignal::Aborted);
                                continue 'outer;
                            }
                            let mut rasterizer =
                                Rasterizer::new(save_data.canvas_size, save_data.crop_area);
                            rasterizer.add_screenshot(image.as_ref().unwrap(), (0, 0));
                            rasterizer.add_annotations(save_data.annotations.as_ref());
                            let signal = match rasterizer.export(save_data.path) {
                                Some(_) => SlaveSignal::SaveDone,
                                None => SlaveSignal::Aborted,
                            };
                            let _ = channel.sender.send(signal);
                        }
                        MasterSignal::Shutdown => {
                            break 'outer;
                        }
                        _ => {}
                    },
                    Err(e) => {
                        println!("{:?}", e);
                        break 'outer;
                    }
                }
            }
            println!("SAVE THREAD DEAD");
        })
    }

    fn manage_window_status(&mut self, ctx: &Context) {
        if self.is_capturing && !self.is_minimized {
            ctx.send_viewport_cmd(ViewportCommand::Minimized(true));
            self.is_minimized = true;
        }
        if !self.is_capturing && self.is_minimized {
            ctx.send_viewport_cmd(ViewportCommand::Minimized(false));
            self.is_minimized = false;
        }
    }
    fn check_signals(&mut self, ctx: &Context) {
        match self.save_channel.receiver.try_recv() {
            Ok(signal) => {
                println!("Signal received");
                match signal {
                    SlaveSignal::CaptureDone(c) => {
                        println!("Capture Done");
                        let id =
                            ctx.load_texture("screenshot", c.clone(), TextureOptions::default());
                        self.editor.crop_rect = Rect::from_min_size(Pos2::ZERO, id.size_vec2());
                        self.editor.texture = Some(id);
                        self.editor.annotations.clear();
                        let mut guard = self.editor.captured_image.lock().unwrap();
                        *guard = Some(c);
                        self.is_capturing = false;
                        self.current_page = PageType::Capture;
                        set_min_inner_size(ctx);
                    }
                    SlaveSignal::Aborted => {
                        println!("Save aborted");
                        self.is_capturing = false;
                        self.is_saving = false;
                    }
                    SlaveSignal::SaveDone => {
                        println!("Save Done");
                        self.is_saving = false;
                    }
                    _ => {}
                }
            }
            Err(e) => {
                if e == TryRecvError::Disconnected {
                    println!("{:?}", e)
                }
            }
        };
        if let Ok(signal) = self.hotkey_channel.receiver.try_recv() {
            println!("Signal received!");
            match signal {
                SlaveSignal::KeyPressed(action) => match action {
                    HotKeyAction::Capture => self.capture(),
                    HotKeyAction::None => {
                        println!("Hotkey does not exists!")
                    }
                    _ => {}
                },
                SlaveSignal::KeyReleased(id) => {
                    println!("Key released with id = {}", id)
                }
                _ => {}
            }
        }
    }
}

impl ScreenGrabber {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        let config = Config::load_or_default();
        if config.is_dark {
            cc.egui_ctx.set_visuals(Visuals::dark())
        } else {
            cc.egui_ctx.set_visuals(Visuals::light())
        }
        cc.egui_ctx
            .send_viewport_cmd(ViewportCommand::Minimized(config.start_minimized));
        cc.egui_ctx.style_mut(|style| {
            style.spacing.slider_width = 200.0;
            style.visuals.slider_trailing_fill = true;
        });
        // cc.egui_ctx.set_visuals(Visuals::light());
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
        !self.is_capturing && self.editor.texture.is_some()
    }

    pub fn get_original_size(&self) -> Vec2 {
        if let Some(image) = &self.editor.texture {
            return image.size_vec2();
        }
        Vec2::ZERO
    }
    pub fn set_new_captured_image(&mut self, image: TextureHandle) {
        self.editor.texture = Some(image);
    }
    pub fn capture(&mut self) {
        // send signal to the worker
        if self.is_capturing {
            return;
        }
        println!("Sending capture signal");
        //minimize the window

        let _ = self
            .save_channel
            .sender
            .send(MasterSignal::StartCaptureAfter(Duration::from_secs_f32(
                self.capture_delay_s,
            )));
        self.is_capturing = true;
    }
    pub fn choose_folder_dialog(&self) -> Option<PathBuf> {
        rfd::FileDialog::new().pick_folder()
    }
    pub fn save_dialog(&self) -> Option<PathBuf> {
        let path = self.config.default_path.clone();

        let mut dialog = rfd::FileDialog::new()
            .add_filter("png", &["png"])
            .add_filter("jpg", &["jpg"])
            .add_filter("gif", &["gif"])
            .add_filter("bmp", &["bmp"]);

        dialog = dialog.set_file_name(format!("{}", self.config.default_filename.to_string()));
        let p = path;
        println!("Default path is {:?}", p);
        dialog = dialog.set_directory(&p);

        let res = dialog.save_file();
        res
    }
    fn _save(&mut self, path: SaveDestination) {
        let guard = self.editor.captured_image.lock().unwrap();
        let size = guard
            .as_ref()
            .expect("cannot get captured image")
            .size
            .clone();
        drop(guard);
        let annotations = self.editor.annotations.clone();
        let canvas_size = (size[0] as u32, size[1] as u32);
        let crop_size = (self.editor.crop_rect.min, self.editor.crop_rect.max);

        let image_data = SaveImageData::new(
            self.editor.captured_image.clone(),
            path,
            crop_size,
            canvas_size,
            annotations,
        );
        let _ = self
            .save_channel
            .sender
            .send(MasterSignal::SaveImage(image_data));
        self.is_saving = true;
    }
    pub fn save_default(&mut self) {
        if !self.has_captured_image() {
            return;
        }
        let mut path = self.config.default_path.clone();
        path.push(self.config.default_filename.to_string());
        path.set_extension("png");
        println!("{:?}", path);
        self._save(SaveDestination::RealPath(path));
    }
    pub fn save_as(&mut self) {
        if !self.has_captured_image() {
            return;
        }
        let path = self.save_dialog();
        if path.is_none() {
            return;
        }
        self._save(SaveDestination::RealPath(path.unwrap()));
    }
    pub fn save_clipboard(&mut self) {
        if !self.has_captured_image() {
            return;
        }
        self._save(SaveDestination::Clipboard(self.clipboard.clone()));
    }
    pub fn set_active_section(&mut self, session: SettingType) {
        self.active_section = session
    }

    pub fn store_config(&mut self) -> Result<(), confy::ConfyError> {
        confy::store("screen-grabber", "config", &self.config)?;
        Ok(())
    }
    pub fn manage_in_app_hotkeys(&mut self, ctx: &Context) {
        for index in 0..self.config.in_app_hotkeys.len() {
            let s = &self.config.in_app_hotkeys[index];
            if ctx.input_mut(|i| s.shortcut.pressed(i)) {
                match s.action {
                    HotKeyAction::Editor => {
                        if self.has_captured_image() {
                            self.set_page(PageType::Capture)
                        }
                    }
                    HotKeyAction::Clipboard => {
                        if self.has_captured_image() {
                            self.save_clipboard()
                        }
                    }
                    HotKeyAction::Save => {
                        if self.has_captured_image() {
                            self.save_as()
                        }
                    }
                    HotKeyAction::SaveDefault => {
                        if self.has_captured_image() {
                            self.save_default()
                        }
                    }
                    HotKeyAction::Settings => self.set_page(PageType::Settings),
                    HotKeyAction::Reset => {
                        if let PageType::Settings = self.current_page {
                            if !self.prev_config.eq(&Config::default()) {
                                self.config.status = Status::ToReset;
                            }
                        }
                    }
                    _ => {}
                }
            };
        }
    }
}

impl eframe::App for ScreenGrabber {
    fn update(&mut self, ctx: &Context, frame: &mut eframe::Frame) {
        self.config.is_dark = ctx.style().visuals.dark_mode;
        self.check_signals(ctx);
        self.manage_window_status(ctx);
        ctx.request_repaint();
        if !self.is_binding {
            self.manage_in_app_hotkeys(ctx)
        };
        match self.current_page {
            PageType::Launcher => launcher_page(self, ctx, frame),
            PageType::Capture => capture_page(self, ctx, frame),
            PageType::Settings => settings_page(self, ctx, frame),
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, APP_KEY, self);
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        //sends the shutdown signal to the threads
        let _ = self.save_channel.sender.send(MasterSignal::Shutdown);
        let _ = self.hotkey_channel.sender.send(MasterSignal::Shutdown);
        //join them
        while let Some(h) = self.thread_handles.pop() {
            let _ = h.join();
        }
    }
}

fn set_font_style(ctx: &Context) {
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
