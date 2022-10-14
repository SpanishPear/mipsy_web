use serde::{Deserialize, Serialize};

/// An instance of a file
/// that is open
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct EditorFile {
    pub name: String,
    pub content: String,
}

impl EditorFile {
    pub fn new(name: String, content: String) -> Self {
        Self { name, content }
    }
}
