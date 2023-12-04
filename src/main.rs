use std::str::FromStr;
use std::time::Duration;

use global_hotkey::{hotkey::HotKey, GlobalHotKeyEvent, GlobalHotKeyManager};

use types::screen_grabber::ScreenGrabber;

pub mod pages;
pub mod types;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        // changed a bit after upgrading to egui 0.24.0
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 400.0])
            .with_min_inner_size([500.0, 400.0]),
        ..Default::default()
    };

    //Hotkey handling
    let manager = GlobalHotKeyManager::new().unwrap();
    let hotkey = HotKey::from_str("shift+q").unwrap();
    manager.register(hotkey).unwrap();
    let receiver = GlobalHotKeyEvent::receiver();
    std::thread::spawn(|| loop {
        if let Ok(event) = receiver.try_recv() {
            println!("tray event: {event:?}");
        }
        std::thread::sleep(Duration::from_millis(100));
    });

    // native_options.transparent = true;
    eframe::run_native(
        "Screengrabber",
        native_options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(ScreenGrabber::new(cc))
        }),
    )
}
