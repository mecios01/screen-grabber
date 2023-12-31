use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Status {
    #[default]
    Normal,
    ToCancel,
    ToReset,
    ToSave,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(skip)]
    pub status: Status,
    // 1379144576
    //general
    pub start_minimized: bool,
    example_text: String,
    //keybindings
    //appearance
    // theme: Visuals,
}

/// `MyConfig` implements `Default`
impl Default for Config {
    fn default() -> Self {
        Self {
            status: Status::default(),
            //general
            start_minimized: false,
            example_text: "".into(),
            //keybindings
            //appearance
            // theme: Visuals::dark(),
        }
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        self.start_minimized == other.start_minimized && self.example_text == other.example_text
    }
}

impl Config {
    pub fn get_example_test(&self) -> &str {
        &self.example_text
    }
    pub fn set_example_test(&mut self, text: String) {
        self.example_text = text
    }
    pub fn load_or_default() -> Self {
        confy::load("screen-grabber", "config").unwrap_or_default()
    }
}
