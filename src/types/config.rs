use std::ops::Deref;
use std::sync::{Arc, RwLock};
use egui_keybind::Shortcut;
use serde::{Deserialize, Serialize};
use crate::pages::types::{Binding, HotKeyAction};

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
    //general
    pub start_minimized: bool,
    example_text: String,
    //keybindings
    pub hotkeys: Arc<RwLock<Vec<Binding>>>,
    pub in_app_hotkeys: Vec<Binding>,
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
            hotkeys: Arc::new(RwLock::new(vec![
                Binding { id: 1, key_bind: String::new(), shortcut: Shortcut::default(), action: HotKeyAction::Capture }
            ])),
            in_app_hotkeys: vec![
                Binding { id: 1, key_bind: String::new(), shortcut: Shortcut::default(), action: HotKeyAction::Save },
                Binding { id: 2, key_bind: String::new(), shortcut: Shortcut::default(), action: HotKeyAction::Reset },
                Binding { id: 3, key_bind: String::new(), shortcut: Shortcut::default(), action: HotKeyAction::None },
            ],
            //appearance
            // theme: Visuals::dark(),
        }
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        self.start_minimized == other.start_minimized &&
            self.example_text == other.example_text &&
            self.in_app_hotkeys == other.in_app_hotkeys &&
            self.hotkeys.read().unwrap().deref().eq(other.hotkeys.read().unwrap().deref())
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
