use std::rc::Rc;

use bounce::{Slice, UseAtomHandle};
use monaco::sys::editor::ICodeEditorViewState;
use yew::Reducible;

use crate::components::app::MipsyCodeEditorLink;
use crate::components::tab_container::UriEq;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileInfo {
    pub name: String,
    pub contents: String,
    pub uri: UriEq,
    pub state: Option<js_sys::Object>,
}

#[derive(Slice, Default, Debug, Clone, PartialEq, Eq)]
pub struct FileList {
    pub files: Vec<FileInfo>,
    pub selected: Option<usize>,
}

pub enum FileListAction {
    /// Append a file to the list
    ///    name    contents create_monacotext_model
    Append(String, String),
    /// Remove a file from the FileList
    ///    name
    Remove(UriEq),
    /// update the stored view state (on switching tabs usually)
    ///          name   contents
    SetViewState(UseAtomHandle<MipsyCodeEditorLink>),
    /// updates the selected
    SetSelected(UriEq),
    /// Log the current state of the FileList
    Log,
}

impl Reducible for FileList {
    type Action = FileListAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            FileListAction::Append(name, contents) => {
                let mut files = self.files.clone();

                let uri: UriEq =
                    monaco::api::TextModel::create(contents.as_str(), "mips".into(), None)
                        .expect("Failed to create text model")
                        .uri()
                        .into();

                files.push(FileInfo {
                    name,
                    contents,
                    uri,
                    state: None,
                });

                // select the first file
                let selected = if self.files.is_empty() {
                    Some(0)
                } else {
                    self.selected
                };

                Rc::new(Self { files, selected })
            }
            FileListAction::Remove(uri) => {
                let mut files = self.files.clone();
                files.retain(|file| file.uri != uri);
                Rc::new(Self {
                    files,
                    selected: self.selected,
                })
            }
            FileListAction::SetViewState(editor_link) => {
                let mut return_val: Option<Rc<Self>> = None;
                editor_link.link.with_editor(|editor| {
                    let state = editor.as_ref().save_view_state();
                    if let Some(state) = state {
                        let item = state.value_of();

                        let mut files = self.files.clone();

                        // set the selected view state
                        if let Some(selected) = self.selected {
                            files[selected].state = Some(item);
                        }

                        return_val = Some(Rc::new(Self {
                            files,
                            selected: self.selected,
                        }))
                    }
                });

                return_val.unwrap_or(self)
            }
            FileListAction::SetSelected(uri) => {
                let selected = self.files.iter().position(|file| file.uri == uri);

                Rc::new(Self {
                    files: self.files.clone(),
                    selected,
                })
            }
            FileListAction::Log => {
                log::info!(
                    "FileList: {:?}",
                    self.files
                        .iter()
                        .map(|f| (&f.name, f.uri.to_string(false)))
                        .collect::<Vec<_>>()
                );
                self
            }
        }
    }
}
