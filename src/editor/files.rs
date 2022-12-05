use crate::agent::worker::MipsyWebWorker;
use crate::editor::MipsyCodeEditorLink;
use std::rc::Rc;

use bounce::{Slice, UseAtomHandle};
use gloo_worker::WorkerBridge;
use yew::Reducible;

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
    /// the indexes of the files that are currently being compiled
    pub to_compile: Vec<usize>, // would use references but they break the slice derive
}

impl FileList {
    pub fn get_next_tab(&self) -> Option<UriEq> {
        // get the index of the tab after selected
        if self.files.is_empty() {
            return None;
        }

        if let Some(selected) = self.selected {
            let last_index = self.files.len() - 1;
            let next_index = (selected + 1) % self.files.len();
            if next_index == last_index {
                None
            } else {
                Some(self.files[next_index].uri.clone())
            }
        } else {
            None
        }
    }
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
    /// restore the view state (on switching tabs usually)
    RestoreViewState(UseAtomHandle<MipsyCodeEditorLink>, UriEq),
    /// updates the selected
    SetSelected(UriEq, UseAtomHandle<MipsyCodeEditorLink>),
    /// Log the current state of the FileList
    Log,

    /// Push an index of self.files to compile list
    ToggleCompile(usize),

    /// Compile all files in the to_compile list
    SendCompileCode(WorkerBridge<MipsyWebWorker>),
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

                Rc::new(Self {
                    files,
                    selected,
                    to_compile: self.to_compile.clone(),
                })
            }
            FileListAction::Remove(uri) => {
                // remove model
                monaco::sys::editor::get_model(&uri)
                    .expect("The model should exist")
                    .dispose();

                let mut files = self.files.clone();
                files.retain(|file| file.uri != uri);
                Rc::new(Self {
                    files,
                    selected: self.selected,
                    to_compile: self.to_compile.clone(),
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
                            to_compile: self.to_compile.clone(),
                        }))
                    }
                });

                return_val.unwrap_or(self)
            }
            FileListAction::SetSelected(uri, editor_link) => {
                let selected = self.files.iter().position(|file| file.uri == uri);

                editor_link.link.with_editor(|editor| {
                    editor.set_model(
                        &monaco::api::TextModel::get(&uri).expect("The model should exist"),
                    );
                });

                Rc::new(Self {
                    files: self.files.clone(),
                    selected,
                    to_compile: self.to_compile.clone(),
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
            FileListAction::RestoreViewState(editor_link, uri) => {
                editor_link.link.with_editor(|editor| {
                    let view_state: wasm_bindgen::JsValue = self
                        .files
                        .iter()
                        .find(|f| f.uri == uri)
                        .expect("The file should exist")
                        .state
                        .clone()
                        .into();

                    editor.as_ref().restore_view_state(&view_state.into());
                });

                self
            }
            FileListAction::ToggleCompile(index) => {
                // if the usize is not in the list, add it
                // if it is in the list, remove it, and shuffle the rest down

                let mut to_compile = self.to_compile.clone();
                if let Some(pos) = to_compile.iter().position(|i| *i == index) {
                    to_compile.remove(pos);
                } else {
                    to_compile.push(index);
                }
                // log the current state of the to_compile list, with filenames
                log::info!(
                    "to_compile: {:?}",
                    to_compile
                        .iter()
                        .map(|i| &self.files[*i].name)
                        .collect::<Vec<_>>()
                );
                Rc::new(Self {
                    files: self.files.clone(),
                    selected: self.selected,
                    to_compile,
                })
            }
            FileListAction::SendCompileCode(bridge) => {
                let files = self
                    .to_compile
                    .iter()
                    .map(|i| self.files[*i].clone().into())
                    .collect::<Vec<_>>();

                bridge.send(crate::agent::worker::ToWorker::CompileCode(files));
                self
            }
        }
    }
}
