use super::{new_file_button::NewFileButton, open_files_button::OpenFilesButton};
use crate::components::tab::TabProps;
use crate::editor::files::{FileList, FileListAction};
use crate::editor::MipsyCodeEditorLink;
use bounce::{use_atom, use_slice, use_slice_dispatch};
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(FileExplorer)]
pub fn file_explorer() -> Html {
    let files = &use_slice::<FileList>().files;
    let selected = &use_slice::<FileList>().selected;
    let files_dispatch = use_slice_dispatch::<FileList>();
    let to_compile = &use_slice::<FileList>().to_compile;
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
                    let files_dispatch = files_dispatch.clone();
                    let checkbox_onchange = {
                        Callback::from(move |_| {
                            files_dispatch(FileListAction::ToggleCompile(index));
                        })
                    };

                    let checked = to_compile.contains(&index);
                    let position_in_compile_list = to_compile.iter().position(|x| *x == index);

                    html! {
                        <div class={css!(r#"
                            display: flex;
                            flex-direction: row;
                            align-items: center;
                        "#)}>
                            if position_in_compile_list.is_some() {
                                <p>
                                    { position_in_compile_list.unwrap() + 1 }
                                </p>
                            }
                            // if the given index is in the compile list, then "check" the checkbox
                            <input type="checkbox" onchange={checkbox_onchange} title="add to compile command" {checked} class={
                                css!(r#"
                                    margin-right: 0.5rem;
                                    &:checked {
                                        background-color: #3f51b5;
                                    }
                                "#)
                            }/>
                            <FileName uri={file_info.uri.clone()}  name={file_info.name.clone()} {is_selected}/>
                        </div>
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
) -> Html {
    let editor_link = use_atom::<MipsyCodeEditorLink>();
    let files = use_slice::<FileList>();

    let onclick = {
        let uri = uri.clone();
        Callback::from(move |_: MouseEvent| {
            log::info!("FileName onclick");
            files.dispatch(FileListAction::SetViewState(editor_link.clone()));
            files.dispatch(FileListAction::SetSelected(uri.clone()));
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
