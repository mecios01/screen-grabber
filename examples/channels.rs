use std::collections::BTreeMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use crossbeam::channel::{unbounded, Receiver, Sender};
use egui::ColorImage;
use global_hotkey::hotkey::HotKey;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};

#[derive(Debug, Clone)]
struct AppHotKey(HotKey, String, KeyPressAction);

#[derive(Debug)]
pub struct Settings {
    pub hotkeys: Vec<AppHotKey>,
}

pub struct App {
    pub has_requested_screenshot: bool,
    pub image: Arc<Mutex<Option<ColorImage>>>,
    pub settings: Arc<RwLock<Settings>>,
}

#[derive(Debug)]
pub enum MasterSignal {
    StartCaptureAfter(Duration),
    SetHotkeys(Arc<RwLock<Settings>>),
    SaveImage(Arc<Mutex<Option<ColorImage>>>),
    Shutdown,
}

#[derive(Debug)]
pub enum SlaveSignal {
    CaptureDone(ColorImage),
    KeyPressed(u32),
    KeyReleased(u32),
}

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Ord)]
pub enum KeyPressAction {
    SaveImage,
    StartCaptureAfter,
    SetHotkeys,
    Shutdown,
}

fn main() {
    let (master_st_sender, slave_st_recv) = unbounded::<MasterSignal>();
    let (master_ht_sender, slave_ht_recv) = unbounded::<MasterSignal>();
    let (st_sender, master_st_recv) = unbounded::<SlaveSignal>();
    let (hk_sender, master_hk_recv) = unbounded::<SlaveSignal>();

    let mt = main_thread(
        master_st_sender,
        master_ht_sender,
        master_st_recv,
        master_hk_recv,
    );
    let st = capture_save_thread(st_sender, slave_st_recv);
    let hk = hotkeys_thread(hk_sender, slave_ht_recv);

    st.join().unwrap();
    hk.join().unwrap();
    mt.join().unwrap();
}

fn main_thread(
    sender_st: Sender<MasterSignal>,
    sender_hk: Sender<MasterSignal>,
    recv_st: Receiver<SlaveSignal>,
    recv_ht: Receiver<SlaveSignal>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut hk_arr = [
            (KeyPressAction::SaveImage, "CTRL+S"),
            (KeyPressAction::StartCaptureAfter, "CTRL+A"),
            (KeyPressAction::SetHotkeys, "CTRL+H"),
            (KeyPressAction::Shutdown, "CTRL+E"),
        ]
        .iter()
        .map(|e| {
            let hk = HotKey::from_str(e.1).unwrap();
            AppHotKey(hk, e.1.to_string(), e.0)
        })
        .collect::<Vec<AppHotKey>>();

        let mut app = App {
            has_requested_screenshot: false,
            image: Arc::new(Mutex::new(None)),
            settings: Arc::new(RwLock::new(Settings { hotkeys: hk_arr })),
        };
        let guard = app.settings.read().unwrap();
        let hkmap = guard
            .hotkeys
            .iter()
            .map(|e| (e.0.id(), (e)))
            .collect::<BTreeMap<u32, &AppHotKey>>();
        let from = "MAIN";
        println!("Sending hotkeys from {from}");
        sender_hk
            .send(MasterSignal::SetHotkeys(app.settings.clone()))
            .unwrap();

        'outer: loop {
            thread::sleep(Duration::from_millis(1000));
            //loop the signals until possible w/o blocking
            while let Ok(s) = recv_ht.try_recv() {
                match s {
                    SlaveSignal::KeyPressed(k) => {
                        println!("{from}: KeyPressed[{k}]");
                        let key = hkmap.get(&k);
                        if key.is_none() {
                            continue;
                        }
                        match key.unwrap().2 {
                            KeyPressAction::StartCaptureAfter => {
                                if app.has_requested_screenshot == false {
                                    app.has_requested_screenshot = true;
                                    sender_st
                                        .send(MasterSignal::StartCaptureAfter(Duration::from_secs(
                                            1,
                                        )))
                                        .unwrap();
                                }
                            }
                            KeyPressAction::SaveImage => sender_st
                                .send(MasterSignal::SaveImage(app.image.clone()))
                                .unwrap(),
                            KeyPressAction::SetHotkeys => sender_hk
                                .send(MasterSignal::SetHotkeys(app.settings.clone()))
                                .unwrap(),
                            KeyPressAction::Shutdown => {
                                let _ = sender_st.send(MasterSignal::Shutdown);
                                let _ = sender_hk.send(MasterSignal::Shutdown);
                                break 'outer;
                            }
                        };
                    }
                    SlaveSignal::KeyReleased(k) => {
                        println!("{from}: KeyReleased[{k}]")
                    }
                    _ => {}
                }
            }
            while let Ok(s) = recv_st.try_recv() {
                match s {
                    SlaveSignal::CaptureDone(c) => {
                        println!("{from}: Captured {:?}", c.size);
                        *app.image.lock().unwrap() = Some(c);
                        app.has_requested_screenshot = false;
                    }
                    _ => {}
                }
            }
        }
        println!("{from}: EXITING")
    })
}

fn capture_save_thread(s2: Sender<SlaveSignal>, r2: Receiver<MasterSignal>) -> JoinHandle<()> {
    thread::spawn(move || {
        let from = "CSAVE";
        'outer: loop {
            //the following recv is blocking so no explicit sleep is needed
            match r2.recv() {
                Ok(r) => match r {
                    MasterSignal::StartCaptureAfter(u) => {
                        println!("{from}: StartCaptureAfter {:?}", u.as_millis());
                        thread::sleep(u);
                        let image = {
                            let screenshot =
                                screenshots::Screen::all().unwrap()[0].capture().unwrap();
                            let size = [screenshot.width() as _, screenshot.height() as _];
                            let pixels = screenshot.as_flat_samples();
                            ColorImage::from_rgba_unmultiplied(size, pixels.as_slice())
                        };
                        s2.send(SlaveSignal::CaptureDone(image)).unwrap();
                    }
                    MasterSignal::SaveImage(s) => {
                        let guard = s.lock().unwrap();
                        if guard.is_none() {
                            println!("{from}: you must capture an image before")
                        } else {
                            println!("{from}: SaveImage, {:?}", guard.as_ref().unwrap().size)
                        }
                    }
                    MasterSignal::Shutdown => {
                        break 'outer;
                    }
                    _ => {}
                },
                Err(_) => {
                    break 'outer;
                }
            }
        }
        println!("{}: EXITING", from)
    })
}

fn hotkeys_thread(s3: Sender<SlaveSignal>, r3: Receiver<MasterSignal>) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut manager = GlobalHotKeyManager::new().unwrap();
        let receiver = GlobalHotKeyEvent::receiver();
        let mut hotkeys: Vec<HotKey> = vec![];
        let from = "HOTK";

        'outer: loop {
            thread::sleep(Duration::from_millis(100));
            match r3.try_recv() {
                Ok(r) => match r {
                    MasterSignal::SetHotkeys(h) => {
                        println!("{from}: registering hotkeys");
                        let hk = h
                            .read()
                            .unwrap()
                            .hotkeys
                            .iter()
                            .by_ref()
                            .map(|e| e.0.clone())
                            .collect();
                        let _ = manager.unregister_all(&hotkeys);
                        hotkeys = hk;
                        drop(h);
                        manager.register_all(&hotkeys).unwrap();
                    }
                    MasterSignal::Shutdown => {
                        break 'outer;
                    }
                    _ => {}
                },
                _ => {}
            }

            match receiver.try_recv() {
                Ok(e) => match e.state {
                    HotKeyState::Pressed => s3.send(SlaveSignal::KeyPressed(e.id)).unwrap(),
                    HotKeyState::Released => s3.send(SlaveSignal::KeyReleased(e.id)).unwrap(),
                },
                Err(e) => {}
            }
        }
        println!("{from}: EXITING")
    })
}
