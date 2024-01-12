use egui::{Key, Modifiers};
use egui_keybind::Shortcut;
use serde::{Deserialize, Serialize};
use crate::types::keybinds::{Binding, HotKeyAction};
use crate::types::utils::new_hotkey_from_str;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub enum Status {
    #[default]
    Normal,
    ToGoBack,
    ToDiscard,
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
    pub hotkeys: Vec<Binding>,
    pub in_app_hotkeys: Vec<Binding>,
    //appearance
    pub is_dark: bool
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
            hotkeys: vec![
                Binding { id: new_hotkey_from_str("Alt+C"), key_bind: String::from("Alt+C"), shortcut: Shortcut::new(Some(egui::KeyboardShortcut::new(Modifiers::ALT, Key::C)), None), action: HotKeyAction::Capture },
            ],
            in_app_hotkeys: vec![
                Binding { id: new_hotkey_from_str("Alt+E"), key_bind: String::from("Alt+E"), shortcut: Shortcut::new(Some(egui::KeyboardShortcut::new(Modifiers::ALT, Key::E)), None), action: HotKeyAction::Editor },
                Binding { id: new_hotkey_from_str("Alt+Shift+C"), key_bind: String::from("Alt+Shift+C"), shortcut: Shortcut::new(Some(egui::KeyboardShortcut::new(Modifiers::ALT | Modifiers::SHIFT, Key::C)), None), action: HotKeyAction::Clipboard },
                Binding { id: new_hotkey_from_str("Alt+S"), key_bind: String::from("Alt+S"), shortcut: Shortcut::new(Some(egui::KeyboardShortcut::new(Modifiers::ALT, Key::S)), None), action: HotKeyAction::Save },
                Binding { id: new_hotkey_from_str("Alt+I"), key_bind: String::from("Alt+I"), shortcut: Shortcut::new(Some(egui::KeyboardShortcut::new(Modifiers::ALT, Key::I)), None), action: HotKeyAction::Settings },
                Binding { id: new_hotkey_from_str("Alt+R"), key_bind: String::from("Alt+R"), shortcut: Shortcut::new(Some(egui::KeyboardShortcut::new(Modifiers::ALT, Key::R)), None), action: HotKeyAction::Reset },
            ],
            //appearance
            is_dark: true
        }
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        self.start_minimized == other.start_minimized &&
            self.example_text == other.example_text &&
            self.in_app_hotkeys == other.in_app_hotkeys &&
            self.hotkeys == other.hotkeys &&
            self.is_dark == other.is_dark
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
