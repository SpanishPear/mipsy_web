use bounce::{use_atom, use_slice};
use stylist::yew::styled_component;
use yew::prelude::*;

use crate::editor::{
    files::{FileList, FileListAction},
    MipsyCodeEditorLink,
};

use super::tab_container::UriEq;

#[derive(Properties, PartialEq, Eq)]
pub struct TabProps {
    pub name: String,
    pub uri: UriEq,
    pub is_selected: bool,
}

#[styled_component(Tab)]
pub fn tab(
    TabProps {
        name,
        uri,
        is_selected,
    }: &TabProps,
) -> Html {
    // TODO(tabs): tab middle click to close
    // TODO(tabs): tab drag and drop to reorder
    // TODO(tabs): should opening file with same name as existing file replace existing file?
    // TODO(tabs): move onclicks to separate functions
    let editor_link = use_atom::<MipsyCodeEditorLink>();
    let files = use_slice::<FileList>();
    let select_onclick = {
        let uri = uri.clone();
        let files = files.clone();
        Callback::from(move |_: MouseEvent| {
            log::info!("tab select onclick");
            files.dispatch(FileListAction::SetViewState(editor_link.clone()));
            files.dispatch(FileListAction::SetSelected(uri.clone()));
            files.dispatch(FileListAction::RestoreViewState(
                editor_link.clone(),
                uri.clone(),
            ));
        })
    };

    let close_onclick = {
        // clone so we can hold onto it in the callback
        let uri = uri.clone();
        // deref here just to make a copy
        let is_selected = *is_selected;
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();

            // can't delete the last file!
            if files.files.len() == 1 {
                return;
            }

            // if current file is selected, select next file
            if is_selected {
                if let Some(next) = files.get_next_tab() {
                    files.dispatch(FileListAction::SetSelected(next));
                }
            }

            // remove from files_list, and the monaco editor
            files.dispatch(FileListAction::Remove(uri.clone()));
        })
    };

    html! {
        <StyledTab selected={*is_selected} {close_onclick} {select_onclick}>
            <span class={css!(r#"
                white-space: nowrap;
                overflow: hidden;
                text-overflow: ellipsis;
            "#)}>
                {name}
            </span>
        </StyledTab>
    }
}

#[derive(Properties, PartialEq)]
pub struct StyledTabProps {
    pub selected: bool,
    pub children: Children,
    pub select_onclick: Callback<MouseEvent>,
    pub close_onclick: Callback<MouseEvent>,
}

#[styled_component(StyledTab)]
pub fn styled_tab(
    StyledTabProps {
        selected,
        children,
        select_onclick,
        close_onclick,
    }: &StyledTabProps,
) -> Html {
    // TODO(tabs): disable/remove close button on last tab
    html! {
        <li
            style={ if *selected { "color: #fff; background-color: #1e1e1e; border-bottom: none;" } else { "" } }
            class={css!(r#"
                padding: 5px 10px;
                min-width: 100px;
                text-align: center;
                border-bottom: 1px solid #000;
                cursor: pointer;
                background-color: #fff;
                user-select: none;
                display: flex;
                align-items: center;
                background-color: #2d2d2d;
                color: #666666; 
                font-family: 'Roboto', sans-serif;
                justify-content: space-between;
                min-width: 0;
            "#)}
            onclick={select_onclick}
        >

            {   for children.iter() }
            <span
                class={css!(r#"
                    float: right;
                    cursor: pointer;
                    user-select: none;
                    padding: 2px 4px;
                    border-radius: 25%;
                    color: #fff;
                    &:hover {
                        background-color: darkgray;
                    }
                "#)}
                onclick={close_onclick}
            >
                {"✕"}
            </span>
        </li>
    }
}
