pub mod component;
pub mod files;

use self::files::FileInfo;
use bounce::Atom;
use monaco::sys::editor::{IEditorMinimapOptions, IStandaloneEditorConstructionOptions};
use monaco::yew::CodeEditorLink;
use serde::{Deserialize, Serialize};

/// A wrapper struct for holding the handle to the Monaco Editor
#[derive(Atom, Default, Debug, Clone, PartialEq)]
pub struct MipsyCodeEditorLink {
    pub link: CodeEditorLink,
}

/// An instance of a file in the Editor
/// but only the name and contents
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

/// The rest of MipsyWeb uses FileInfo struct, it's only IndexDb that uses EditorFile
impl From<FileInfo> for EditorFile {
    fn from(file: FileInfo) -> Self {
        Self {
            name: file.name,
            content: file.contents,
        }
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
