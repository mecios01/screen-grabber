use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Result};
use egui_keybind::Shortcut;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum PageType {
    Launcher,
    Capture,
    Settings,
}

pub enum SettingType {
    General,
    Keybindings,
    Appearance,
    About,
}

#[derive(Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub enum HotKeyAction {
    Capture,
    Save,
    Reset,
    None,
}

impl Display for HotKeyAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            HotKeyAction::Capture => write!(f, "Capture"),
            HotKeyAction::Save => write!(f, "Save"),
            HotKeyAction::Reset => write!(f, "Reset"),
            HotKeyAction::None => write!(f, "None"),
        }
    }
}
impl PartialEq for HotKeyAction {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (HotKeyAction::Capture, HotKeyAction::Capture) |
            (HotKeyAction::Save, HotKeyAction::Save) |
            (HotKeyAction::Reset, HotKeyAction::Reset) |
            (HotKeyAction::None, HotKeyAction::None)
        )
    }
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Clone, Debug)]
pub struct GlobalBinding {
    pub id: u32,
    pub key_bind: String,
    pub action: HotKeyAction,
}

impl Default for GlobalBinding {
    fn default() -> Self {
        Self {
            id: 0,
            key_bind: String::new(),
            action: HotKeyAction::None,
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppBinding {
    pub id: u32,
    pub shortcut: Shortcut,
    pub action: HotKeyAction,
}

impl PartialEq<Self> for AppBinding {
    fn eq(&self, other: &Self) -> bool {
        self.shortcut.eq(&other.shortcut)/* && self.action.eq(&other.action)*/
    }
}

impl Default for AppBinding {
    fn default() -> Self {
        Self {
            id: 0,
            shortcut: Shortcut::default(),
            action: HotKeyAction::None,
        }
    }
}

