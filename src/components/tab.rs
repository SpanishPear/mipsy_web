use bounce::use_slice;
use monaco::yew::CodeEditorLink;
use stylist::yew::styled_component;
use wasm_bindgen::JsValue;
use yew::prelude::*;

use crate::components::app::{FileList, FileListAction};

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
    // TODO(tabs): tab close button
    // TODO(tabs): tab middle click to close
    // TODO(stretch): tab drag and drop to reorder
    // TODO(tabs): tab onclick to focus
    // TODO(tabs): if the filename is not already open, show filename, else show full path
    // TODO(tabs): if the filename is too long, truncate it
    // TODO(tabs): save and restore scroll position, cursor (editor.restoreViewState)
    let editor_link = use_context::<CodeEditorLink>().expect("should have a link");
    let files = use_slice::<FileList>();
    let select_onclick = {
        let uri = uri.clone();
        let files = files.clone();
        Callback::from(move |_: MouseEvent| {
            editor_link.with_editor(|editor| {
                // save editors view state

                let view_state = editor
                    .as_ref()
                    .save_view_state()
                    .expect("should have a view state");

                log::info!("made it");
                files.dispatch(FileListAction::SetViewState(view_state));

                log::info!("made it2");
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
            monaco::sys::editor::get_model(&uri)
                .expect("The model should exist")
                .dispose();

            // remove from files_list too
            files.dispatch(FileListAction::Remove(uri.clone()));
        })
    };
    html! {
        <StyledTab selected={*is_selected} {close_onclick} {select_onclick}>
            <span>{name}</span>
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
