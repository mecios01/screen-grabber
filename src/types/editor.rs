use eframe::emath::RectTransform;
use egui::{Shape, Ui};
use crate::types::screen_grabber::ScreenGrabber;
use crate::utils::input::Annotation;

pub enum StackAction {
    AddShape(Shape), //NO TEXT SHAPES HERE (THEY NEED TO BE HANDLED DIFFERENTLY)
}

pub enum Mode{
    Idle,
    DrawSegment,
    DrawCircle,
}

pub struct Editor {
    pub mode: Mode,
    pub cur_annotation: Option<Annotation>,
    pub annotations: Vec<Annotation>,
    // captured_image
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            mode: Mode::Idle,
            cur_annotation: None,
            annotations: Vec::new(),
        }
    }
}

impl Editor {
    fn new() -> Self {
        todo!();
    }

    fn manage_input(app: &mut ScreenGrabber, ui: &mut Ui, to_original: RectTransform){
    }
}
