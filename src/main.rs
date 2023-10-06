pub mod types;
pub mod pages;

use types::screen_grabber::ScreenGrabber;
fn main() -> eframe::Result<()> {
    let mut native_options = eframe::NativeOptions::default();
    // native_options.transparent = true;
    eframe::run_native("App",native_options,Box::new(|cc|Box::new(ScreenGrabber::new(cc))))
}
