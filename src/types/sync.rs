use std::sync::{Arc, Mutex};
use std::time::Duration;

use crossbeam::channel::{unbounded, Receiver, Sender};
use egui::{ColorImage, Pos2};

use crate::types::annotation::Annotation;
use crate::types::keybinds::{Binding, HotKeyAction};
use crate::types::save_destination::SaveDestination;

///Signals send from the main thread to the auxiliary threads
pub enum MasterSignal {
    ///asks the thread to capture the screenshot after Duration
    StartCaptureAfter(Duration),
    ///setup the hotkeys or renew them after changes
    SetHotkeys(Vec<Binding>),
    ///asks the thread to export and save the image into the path
    SaveImage(SaveImageData),
    ///makes the thread exit the main loop then terminate (so we can join it before exiting)
    Shutdown,
}

///Signals sent from the auxiliary threads back to the main thread
pub enum SlaveSignal {
    CaptureDone(ColorImage),
    SaveDone,
    KeyPressed(HotKeyAction),
    KeyReleased(u32),
    Aborted,
}

pub struct DoubleChannel<S, R>
where
    S: Sized,
    R: Sized,
{
    pub sender: Sender<S>,
    pub receiver: Receiver<R>,
    secondary_channel: Option<Box<DoubleChannel<R, S>>>,
}

impl<S, R> DoubleChannel<S, R>
where
    S: Sized,
    R: Sized,
{
    pub fn new() -> DoubleChannel<S, R> {
        let (ch1_s, ch1_r) = unbounded::<S>();
        let (ch2_s, ch2_r) = unbounded::<R>();

        //this is the one to be given away to the thread
        let secondary = DoubleChannel::<R, S> {
            sender: ch2_s,
            receiver: ch1_r,
            secondary_channel: None,
        };

        Self {
            sender: ch1_s,
            receiver: ch2_r,
            secondary_channel: Some(Box::new(secondary)),
        }
    }

    ///this should be called only once at the start of the application, every other subsequent call
    ///will return a None value
    #[must_use]
    pub fn secondary_channel(&mut self) -> Option<Box<DoubleChannel<R, S>>> {
        if self.secondary_channel.is_none() {
            None
        } else {
            self.secondary_channel.take()
        }
    }
}

///use this struct to wrap the data needed to export the screenshot image(s)
pub struct SaveImageData {
    pub image_ref: Arc<Mutex<Option<ColorImage>>>,
    pub path: SaveDestination,
    pub canvas_size: (u32, u32),
    pub crop_area: (Pos2, Pos2),
    pub annotations: Vec<Annotation>,
}

impl SaveImageData {
    #[inline]
    pub fn new(
        image_ref: Arc<Mutex<Option<ColorImage>>>,
        path: SaveDestination,
        crop_area: (Pos2, Pos2),
        canvas_size: (u32, u32),
        annotations: Vec<Annotation>,
    ) -> Self {
        Self {
            image_ref,
            path,
            canvas_size,
            crop_area,
            annotations,
        }
    }
}
