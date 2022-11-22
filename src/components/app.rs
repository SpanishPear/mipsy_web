use crate::agent::MipsyWebWorker;
use crate::{
    components::{layout::ResizableLayout, menubar::MenuBar},
    editor::component::Editor,
};
use bounce::{use_atom, use_slice, Atom, Slice};
use gloo_worker::Spawnable;
use js_sys::{Object, Promise};
use monaco::sys::editor::ICodeEditorViewState;
use monaco::sys::Uri;
use monaco::yew::CodeEditorLink;
use std::rc::Rc;
use stylist::css;
use stylist::yew::styled_component;
use wasm_bindgen_futures::{spawn_local, JsFuture};
use yew::prelude::*;

use super::tab_container::UriEq;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileInfo {
    pub name: String,
    pub contents: String,
    pub uri: UriEq,
    pub state: Option<Object>,
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
    SetViewState(Option<ICodeEditorViewState>),
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
            FileListAction::SetViewState(state) => {
                if let Some(state) = state {
                    let item = state.value_of();

                    let mut files = self.files.clone();

                    // set the selected view state
                    if let Some(selected) = self.selected {
                        files[selected].state = Some(item);
                    }

                    Rc::new(Self {
                        files,
                        selected: self.selected,
                    })
                } else {
                    self
                }
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

#[derive(Atom, Default, Debug, Clone, PartialEq)]
pub struct MipsyCodeEditorLink {
    pub link: CodeEditorLink,
}

#[styled_component(App)]
pub fn app() -> Html {
    let code_editor_link = use_atom::<MipsyCodeEditorLink>();

    let bridge = MipsyWebWorker::spawner()
        .callback(move |m| {
            // this runs in the main browser thread
            // and does not block the web worker
            log::info!("received message from worker: {:?}", m);
        })
        .spawn("/worker.js");

    spawn_local(async move {
        bridge.send(crate::agent::ToWorker::Ping);
        // We need to hold the bridge until the worker resolves.
        let promise = Promise::new(&mut |_, _| {});
        let a = JsFuture::from(promise).await;
        //TODO: use channels to send messages/await
        // responses from the worker
        log::error!("{:?}", a);
    });

    let files = use_slice::<FileList>();
    use_effect_with_deps(
        move |_| {
            code_editor_link.link.with_editor(|editor| {
                let model = editor.get_model().expect("editor has no model");

                model.set_value(include_str!("../main.s"));

                files.dispatch(FileListAction::Append(
                    "main.s".into(),
                    include_str!("../main.s").into(),
                ));
            });

            || ()
        },
        (),
    );

    html! {
        <AppContainer>
            <ResizableLayout
                menu_container={{ html_nested! {
                    <MenuContainer />
                }}}
                editor_container={{ html_nested!{
                    <EditorContainer />
                }}}
                runtime_container={{html_nested!{
                    <RuntimeContainer />
                }}}
            >
            </ResizableLayout>
        </AppContainer>
    }
}

#[derive(Properties, PartialEq)]
struct AppContainerProps {
    children: Children,
}

#[styled_component(AppContainer)]
fn app_container(props: &AppContainerProps) -> Html {
    html! {
        <div class={css!(r#"
            min-width: 100vw;
            min-height: 100vh;
            height: 100%;
            width: 100%;
            background-color: pink;
        "#)}>
            {
                for props.children.iter()
            }
        </div>
    }
}

#[styled_component(MenuContainer)]
pub fn menu_container() -> Html {
    html! {
        <div class={css!(r#"
            width: 100%;
            height: 100%;
        "#)}>
           <MenuBar />
        </div>
    }
}

#[styled_component(EditorContainer)]
pub fn editor_container() -> Html {
    let styles: String = "width: 100%; height: 100%; max-height: 90vh;".into();
    html! {
        <div class={css!(r#"
            width: 100%;
            height: 100%;
            min-width: 100%;
            min-height: 100%;
        "#)}>
            <Editor {styles}/>
        </div>
    }
}

#[styled_component(RuntimeContainer)]
pub fn runtime_container() -> Html {
    html! {
        <div class={css!(r#"
            background-color: green;
        "#)}>
            {"runtime"}
        </div>
    }
}
