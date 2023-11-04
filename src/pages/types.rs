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
