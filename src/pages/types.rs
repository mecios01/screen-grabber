use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum PageType {
    Launcher,
    Capture,
    Settings,
}

pub enum SettingSection {
    General,
    Keybindings,
    Appearance,
    About,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MyConfig {
    //general
    pub start_minimized: bool,
    example_text: String,
    //appearance
    // theme: Visuals,
}

/// `MyConfig` implements `Default`
impl Default for MyConfig {
    fn default() -> Self {
        Self {
            //general
            start_minimized: false,
            example_text: "".into(),
            //appearance
            // theme: Visuals::dark(),
        }
    }
}

impl MyConfig {
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
