use std::borrow::Cow;
use std::thread::sleep;
use std::time::Duration;

use arboard;
use arboard::ImageData;
use screenshots::Screen;

fn main() {
    let mut clipboard = arboard::Clipboard::new().unwrap();

    let screenshot = Screen::all().unwrap()[0].capture().unwrap();
    let size = [screenshot.width() as usize, screenshot.height() as usize];
    let pixels = screenshot.as_flat_samples();

    // let random_text = clipboard.set_text("TEXT IN CLIPBOARD");
    let res = clipboard
        .set_image(ImageData {
            width: size[0],
            height: size[1],
            bytes: Cow::Borrowed(pixels.samples),
        })
        .unwrap();

    drop(clipboard);
    // IMAGES ARE NOT COPIED TO CLIPBOARD IN X11
    // (only a reference is copied but it will die by the end of the program)

    sleep(Duration::from_secs_f64(20f64));
}
