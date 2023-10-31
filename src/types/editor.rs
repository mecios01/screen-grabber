use egui::Shape;
use crate::utils::input::Annotation;

pub enum StackAction {
    AddShape(Shape), //NO TEXT SHAPES HERE (THEY NEED TO BE HANDLED DIFFERENTLY)
}

pub struct Editor {
    //execution stack (the list of operations performed on the original image)
    //is_interacting (is the user dragging/selecting/adding something)
    pub execution_stack: Option<Vec<StackAction>>,
    pub segment: Option<Annotation>,
    pub annotations: Vec<Annotation>,
    // captured_image
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            execution_stack: None,
            segment : None,
            annotations: Vec::new(),
        }
    }
}

impl Editor {
    fn new() -> Self {
        todo!();
        Self {
            ..Default::default()
        }
    }
}
