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