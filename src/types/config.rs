use std::fmt::Display;
use std::path::PathBuf;

use chrono::Utc;
use egui::{Key, Modifiers};
use egui_keybind::Shortcut;
use serde::{Deserialize, Serialize};

use crate::types::keybinds::{Binding, HotKeyAction};
use crate::types::save_destination::SaveDestination;
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

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Filename {
    pub timestamp: bool,
    pub prefix: String,
    pub postfix: String,
}

impl Default for Filename {
    fn default() -> Self {
        Self {
            timestamp: true,
            prefix: String::new(),
            postfix: String::new(),
        }
    }
}

impl Display for Filename {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let dt = Utc::now();
        let mut name = self.prefix.clone();
        if self.timestamp || (self.prefix.is_empty() && self.postfix.is_empty()) {
            let timestamp = dt.format("%Y%m%d_%H%M%S");
            name.push_str(&timestamp.to_string());
        }
        name.push_str(&self.postfix);
        f.write_str(&name)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    #[serde(skip)]
    pub status: Status,
    //general
    pub start_minimized: bool,
    pub default_path: PathBuf,
    pub default_filename: Filename,

    //keybindings
    pub hotkeys: Vec<Binding>,
    pub in_app_hotkeys: Vec<Binding>,
    //appearance
    pub is_dark: bool,
}
impl Config {
    #[inline]
    fn default_in_app_keybindings() -> Vec<Binding> {
        vec![
            Binding {
                id: new_hotkey_from_str("Alt+E"),
                key_bind: String::from("Alt+E"),
                shortcut: Shortcut::new(
                    Some(egui::KeyboardShortcut::new(Modifiers::ALT, Key::E)),
                    None,
                ),
                action: HotKeyAction::Editor,
            },
            Binding {
                id: new_hotkey_from_str("Alt+Shift+C"),
                key_bind: String::from("Alt+Shift+C"),
                shortcut: Shortcut::new(
                    Some(egui::KeyboardShortcut::new(
                        Modifiers::ALT | Modifiers::SHIFT,
                        Key::C,
                    )),
                    None,
                ),
                action: HotKeyAction::Clipboard,
            },
            Binding {
                id: new_hotkey_from_str("Alt+S"),
                key_bind: String::from("Alt+S"),
                shortcut: Shortcut::new(
                    Some(egui::KeyboardShortcut::new(Modifiers::ALT, Key::S)),
                    None,
                ),
                action: HotKeyAction::Save,
            },
            Binding {
                id: new_hotkey_from_str("Alt+D"),
                key_bind: String::from("Alt+D"),
                shortcut: Shortcut::new(
                    Some(egui::KeyboardShortcut::new(Modifiers::ALT, Key::D)),
                    None,
                ),
                action: HotKeyAction::SaveDefault,
            },
            Binding {
                id: new_hotkey_from_str("Alt+I"),
                key_bind: String::from("Alt+I"),
                shortcut: Shortcut::new(
                    Some(egui::KeyboardShortcut::new(Modifiers::ALT, Key::I)),
                    None,
                ),
                action: HotKeyAction::Settings,
            },
            Binding {
                id: new_hotkey_from_str("Alt+R"),
                key_bind: String::from("Alt+R"),
                shortcut: Shortcut::new(
                    Some(egui::KeyboardShortcut::new(Modifiers::ALT, Key::R)),
                    None,
                ),
                action: HotKeyAction::Reset,
            },
        ]
    }
}
/// `MyConfig` implements `Default`
impl Default for Config {
    fn default() -> Self {
        Self {
            status: Status::default(),
            //general
            start_minimized: false,
            default_path: SaveDestination::default_path(),
            default_filename: Filename::default(),
            //keybindings
            hotkeys: vec![Binding {
                id: new_hotkey_from_str("Alt+C"),
                key_bind: String::from("Alt+C"),
                shortcut: Shortcut::new(
                    Some(egui::KeyboardShortcut::new(Modifiers::ALT, Key::C)),
                    None,
                ),
                action: HotKeyAction::Capture,
            }],
            in_app_hotkeys: Self::default_in_app_keybindings(),
            //appearance
            is_dark: true,
        }
    }
}

impl PartialEq for Config {
    fn eq(&self, other: &Self) -> bool {
        self.start_minimized == other.start_minimized
            && self.in_app_hotkeys == other.in_app_hotkeys
            && self.hotkeys == other.hotkeys
            && self.is_dark == other.is_dark
            && self.default_filename == other.default_filename
            && self.default_path == other.default_path
    }
}

impl Config {
    pub fn load_or_default() -> Self {
        confy::load("screen-grabber", "config").unwrap_or_default()
    }
}
