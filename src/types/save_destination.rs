use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use arboard::Clipboard;

pub enum SaveDestination {
    RealPath(PathBuf),
    Clipboard(Arc<Mutex<Clipboard>>),
}

impl SaveDestination {
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
        match self {
            SaveDestination::RealPath(_) => true,
            _ => false,
        }
    }

    pub fn is_clipboard(&self) -> bool {
        !self.is_path()
    }
}
