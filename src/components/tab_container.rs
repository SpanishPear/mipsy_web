use std::ops::Deref;

use crate::components::app::{FileList, MipsyCodeEditorLink};
use crate::components::tab::Tab;
use bounce::prelude::*;
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Debug)]
pub struct UriEq(monaco::sys::Uri);

impl PartialEq for UriEq {
    /// Compare two `Uri`s by their `path`
    /// Given that we only ever use `Uri`s to represent files, this is sufficient
    fn eq(&self, other: &Self) -> bool {
        self.0.path() == other.0.path()
    }
}

impl Eq for UriEq {}

impl Clone for UriEq {
    fn clone(&self) -> Self {
        Self(monaco::sys::Uri::from(&self.0.clone()))
    }
}

impl Deref for UriEq {
    type Target = monaco::sys::Uri;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<monaco::sys::Uri> for UriEq {
    fn from(uri: monaco::sys::Uri) -> Self {
        Self(uri)
    }
}

#[styled_component(TabContainer)]
pub fn tabs() -> Html {
    // get a vec of all model paths
    let all_tabs = &use_slice_value::<FileList>().files;
    let selected = &use_slice_value::<FileList>().selected;

    // loop through all tabs, and render a tab for each one
    // add the selected class to the tab that is currently selected
    html! {
        <StyledTabContainer>
            {
                for all_tabs.iter().enumerate().map(|(index, details)| {
                    let uri=details.uri.clone();
                    let is_selected = selected.is_some() && selected.unwrap() == index;
                    html! {
                        <Tab uri={uri} name={details.name.clone()} {is_selected} />
                    }
                })
            }
        </StyledTabContainer>
    }
}

#[derive(Properties, PartialEq)]
pub struct StyledTabContainerProps {
    pub children: Children,
}

#[styled_component(StyledTabContainer)]
pub fn styled_tabs(StyledTabContainerProps { children }: &StyledTabContainerProps) -> Html {
    html! {
        <ul class={css!(r#"
            display: flex;
            flex-direction: row;
            min-height: 60px;
            border: 1px solid black;
            border-bottom: none;
        "#)}>
            { for children.iter() }
        </ul>
    }
}
