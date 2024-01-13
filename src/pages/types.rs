use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub enum PageType {
    Launcher,
    Capture,
    Settings,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SettingType {
    General,
    Keybindings,
    Appearance,
    About,
}
