use std::fmt::{Display, Formatter};

use egui_keybind::Shortcut;
use serde::{Deserialize, Serialize};

#[derive(Eq, Serialize, Deserialize, Clone, Debug)]
pub enum HotKeyAction {
    Capture,
    Editor,
    Clipboard,
    Save,
    SaveDefault,
    Settings,
    Reset,
    None,
}

impl Display for HotKeyAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            HotKeyAction::Capture => write!(f, "Capture"),
            HotKeyAction::Editor => write!(f, "Editor"),
            HotKeyAction::Clipboard => write!(f, "Clipboard"),
            HotKeyAction::Save => write!(f, "Save"),
            HotKeyAction::SaveDefault => write!(f, "Save Default"),
            HotKeyAction::Settings => write!(f, "Settings"),
            HotKeyAction::Reset => write!(f, "Reset"),
            HotKeyAction::None => write!(f, "None"),
        }
    }
}
impl PartialEq for HotKeyAction {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (HotKeyAction::Capture, HotKeyAction::Capture)
                | (HotKeyAction::Editor, HotKeyAction::Editor)
                | (HotKeyAction::Clipboard, HotKeyAction::Clipboard)
                | (HotKeyAction::Save, HotKeyAction::Save)
                | (HotKeyAction::SaveDefault, HotKeyAction::Save)
                | (HotKeyAction::Settings, HotKeyAction::Settings)
                | (HotKeyAction::Reset, HotKeyAction::Reset)
                | (HotKeyAction::None, HotKeyAction::None)
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Binding {
    pub id: u32,
    pub key_bind: String,
    pub shortcut: Shortcut,
    pub action: HotKeyAction,
}

impl Default for Binding {
    fn default() -> Self {
        Self {
            id: 0,
            key_bind: String::new(),
            shortcut: Shortcut::NONE,
            action: HotKeyAction::None,
        }
    }
}

impl PartialEq<Self> for Binding {
    fn eq(&self, other: &Self) -> bool {
        self.shortcut.eq(&other.shortcut) /* && self.action.eq(&other.action)*/
    }
}
