use bounce::{use_atom, use_slice};
use stylist::yew::styled_component;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::components::app::{FileList, FileListAction, MipsyCodeEditorLink};

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
        let editor_link = editor_link.clone();
        Callback::from(move |_: MouseEvent| {
            editor_link.link.with_editor(|editor| {
                // save editors view state

                let view_state = editor.as_ref().save_view_state();

                files.dispatch(FileListAction::SetViewState(view_state));

                files.dispatch(FileListAction::SetSelected(uri.clone()));

                editor
                    .set_model(&monaco::api::TextModel::get(&uri).expect("The model should exist"));

                // restore editors view state

                let view_state: JsValue = files
                    .files
                    .iter()
                    .find(|f| f.uri == uri)
                    .expect("The file should exist")
                    .state
                    .clone()
                    .into();

                editor.as_ref().restore_view_state(&view_state.into());
            });
        })
    };

    let close_onclick = {
        let uri = uri.clone();
        Callback::from(move |_: MouseEvent| {
            // always have one tab open...
            if files.files.len() == 1 {
                return;
            }

            // change selected model to avail
            // handle wrapping
            let next = files
                .files
                .iter()
                .position(|f| f.uri == uri)
                .map(|i| i + 1)
                .unwrap_or(0);

            let next = if next >= files.files.len() - 1 {
                next - 1
            } else {
                next
            };

            let next = std::cmp::max(next, 0);

            let next = files.files.get(next).map(|f| f.uri.clone());
            // set model to next
            if let Some(next) = next {
                files.dispatch(FileListAction::SetSelected(next.clone()));
                editor_link.link.with_editor(|editor| {
                    editor.set_model(
                        &monaco::api::TextModel::get(&next).expect("The model should exist"),
                    );
                });
            }

            // remove model
            monaco::sys::editor::get_model(&uri)
                .expect("The model should exist")
                .dispose();

            // remove from files_list too
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
