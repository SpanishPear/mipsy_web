use stylist::yew::styled_component;
use yew::prelude::*;

use super::tab_container::UriEq;

#[derive(Properties, PartialEq, Eq)]
pub struct TabProps {
    pub uri: UriEq,
    pub selected: Option<String>,
}

#[styled_component(Tab)]
pub fn tab(TabProps { uri, selected }: &TabProps) -> Html {
    // TODO(tabs): tab close button
    // TODO(tabs): tab middle click to close
    // TODO(stretch): tab drag and drop to reorder
    // TODO(tabs): tab onclick to focus
    // TODO(tabs): if the filename is not already open, show filename, else show full path
    // TODO(tabs): if the filename is too long, truncate it
    // TODO(tabs): save and restore scroll position, cursor (editor.restoreViewState)
    let selected = selected.as_ref().map(|s| *s == uri.path()).unwrap_or(false);
    let uri_string = uri.to_string(false);
    let display_name = uri_string.split('/').last().unwrap_or_default();

    html! {
        <StyledTab selected={selected}>
            <span>{display_name}</span>
        </StyledTab>
    }
}

#[derive(Properties, PartialEq)]
pub struct StyledTabProps {
    pub selected: bool,
    pub children: Children,
}

#[styled_component(StyledTab)]
pub fn styled_tab(StyledTabProps { selected, children }: &StyledTabProps) -> Html {
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
            >
                {"✕"}
            </span>
        </li>
    }
}
