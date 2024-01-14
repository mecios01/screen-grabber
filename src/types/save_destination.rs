use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use arboard::Clipboard;

pub enum SaveDestination {
    RealPath(PathBuf),
    Clipboard(Arc<Mutex<Clipboard>>),
}

impl SaveDestination {
    pub fn default_path() -> PathBuf {
        let picture_dir = match dirs::picture_dir() {
            Some(path) => path,
            None => {
                eprintln!("Unable to determine user's Pictures directory");
                return std::env::current_dir().expect("If not exists, you are running this on a potato xD");
            }
        };
        let screenshot_dir = picture_dir.join("Screenshots");
        if !screenshot_dir.exists() {
            if let Err(e) = fs::create_dir(&screenshot_dir) {
                eprintln!("Failed to create screenshots directory: {}", e);
                return std::env::current_dir().expect("If not exists, you are running this on a potato xD");
            }
        }
        screenshot_dir
    }
    pub fn clipboard(self) -> Option<Arc<Mutex<Clipboard>>> {
        match self {
            SaveDestination::Clipboard(c) => Some(c),
            _ => None,
        }
    }
    pub fn path(self) -> Option<PathBuf> {
        match self {
            SaveDestination::RealPath(p) => Some(p),
            _ => None,
        }
    }
    pub fn is_path(&self) -> bool {
        matches!(self, SaveDestination::RealPath(_))
    }

    pub fn is_clipboard(&self) -> bool {
        !self.is_path()
    }
}
