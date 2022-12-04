use crate::components::app::MipsyCodeEditorLink;
use crate::components::new_file_button::NewFileButton;
use crate::components::open_files_button::OpenFilesButton;
use crate::editor::files::{FileList, FileListAction};
use bounce::{use_atom, use_slice};
use stylist::yew::styled_component;
use yew::prelude::*;

use super::tab::TabProps;

#[styled_component(FileExplorer)]
pub fn file_explorer() -> Html {
    let files = &use_slice::<FileList>().files;
    let selected = &use_slice::<FileList>().selected;
    html! {
        <>
            // file explorer header container
            <div class={css!(r#"
                display: flex;
                flex-direction: row;
                justify-content: space-between;
                margin-bottom: 0.5rem;
            "#)}>
                // files title
                <div class={css!(r#"
                    font-weight: bold;
                "#)}>
                    { "Files" }
                </div>

                // right hand side buttons
                <div class={css!(r#"
                    min-width: 40px;
                    display: flex;
                    justify-content: space-between;
                "#)}>
                    <NewFileButton />
                    <OpenFilesButton />
                </div>
            </div>
            {
                for files.iter().enumerate().map(|(index, file_info)| {
                    let is_selected = selected.is_some() && selected.unwrap() == index;
                    html! {
                        <FileName uri={file_info.uri.clone()}  name={file_info.name.clone()} {is_selected}/>
                    }
                })
            }
        </>

    }
}

#[styled_component(FileName)]
fn from(
    TabProps {
        name,
        uri,
        is_selected,
    }: &TabProps,
) -> Self {
    let editor_link = use_atom::<MipsyCodeEditorLink>();
    let files = use_slice::<FileList>();

    let onclick = {
        let uri = uri.clone();
        Callback::from(move |_: MouseEvent| {
            files.dispatch(FileListAction::SetViewState(editor_link.clone()));
            files.dispatch(FileListAction::SetSelected(
                uri.clone(),
                editor_link.clone(),
            ));
            files.dispatch(FileListAction::RestoreViewState(
                editor_link.clone(),
                uri.clone(),
            ));
        })
    };

    html! {
        <div {onclick} data-selected={if *is_selected {"true"} else {"false"}} class={css!(r#"
                display: flex;
                align-items: center;
                padding: 0.5rem;
                cursor: pointer;
                transition: background-color 0.2s ease-in-out;
                &:hover {
                    background-color: rgba(0, 0, 0, 0.1);
                }
                &[data-selected="true"] {
                    background-color: rgba(0, 0, 0, 0.1);
                }
            "#)}>
            {name}
        </div>
    }
}
