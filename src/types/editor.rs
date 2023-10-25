use egui::Shape;

pub enum StackAction {
    AddShape(Shape), //NO TEXT SHAPES HERE (THEY NEED TO BE HANDLED DIFFERENTLY)
}

pub struct Editor {
    //execution stack (the list of operations performed on the original image)
    //is_interacting (is the user dragging/selecting/adding something)
    pub execution_stack: Option<Vec<StackAction>>,
    // captured_image
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            execution_stack: None,
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
