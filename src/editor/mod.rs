pub mod component;
pub mod files;

use monaco::sys::editor::{IEditorMinimapOptions, IStandaloneEditorConstructionOptions};
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

fn get_options() -> IStandaloneEditorConstructionOptions {
    let options = IStandaloneEditorConstructionOptions::default();
    options.set_theme("vs-dark".into());
    options.set_language("mips".into());
    options.set_scroll_beyond_last_line(false.into());
    options.set_automatic_layout(true.into());

    let minimap = IEditorMinimapOptions::default();
    minimap.set_enabled(false.into());
    options.set_minimap(Some(&minimap));

    options
}
