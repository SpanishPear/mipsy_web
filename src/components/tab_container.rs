use crate::components::tab::Tab;
use bounce::prelude::*;
use monaco::yew::CodeEditorLink;
use stylist::yew::styled_component;
use yew::prelude::*;

/// A container for tab details
#[derive(Clone, Debug, PartialEq)]
pub struct TabDetails {
    /// The tab's uri
    uri: String,
    /// The full path to the file
    /// showing full_path or just name
    /// is handled by the Tab component
    display_name: String,
}

#[derive(Atom, Debug, Clone, PartialEq)]
struct TabDetailsList {
    pub inner: Vec<TabDetails>,
}

impl Default for TabDetailsList {
    fn default() -> Self {
        let all_tabs = monaco::api::TextModel::get_all().into_iter().map(|model| {
            let display_name = model.uri().path().to_string();

            TabDetails {
                uri: model.uri().to_string(false),
                display_name,
            }
        });

        Self {
            inner: all_tabs.collect(),
        }
    }
}

#[styled_component(TabContainer)]
pub fn tabs() -> Html {
    let link = use_context::<CodeEditorLink>().expect("should have a link");
    // get a vec of all model paths

    let all_tabs = &use_atom::<TabDetailsList>().inner;
    let current_tab: UseStateHandle<Option<String>> = use_state(|| None);

    // after the first render, get the current tab
    {
        let current_tab = current_tab.clone();
        let link = link.clone();
        use_effect(move || {
            let maybe_tabname = link.with_editor(|editor| {
                editor
                    .get_model()
                    .expect("There should always be one tab focused")
                    .uri()
                    .path()
            });

            if let Some(current_tab_name) = maybe_tabname {
                if *current_tab == None {
                    current_tab.set(Some(current_tab_name));
                }
            }

            || ()
        });
    }

    // loop through all tabs, and render a tab for each one
    // add the selected class to the tab that is currently selected
    // TODO(tabs): tab close button
    // TODO(tabs): tab middle click to close
    // TODO(stretch): tab drag and drop to reorder
    // TODO(tabs): tab onclick to focus
    // TODO(tabs): if the filename is not already open, show filename, else show full path
    // TODO(tabs): if the filename is too long, truncate it
    // TODO(tabs): save and restore scroll position, cursor (editor.restoreViewState)
    html! {
        <StyledTabContainer>
            {
                for all_tabs.into_iter().map(|details| {
                    let uri=details.uri.clone();
                    html! {
                        <Tab uri={uri} selected={(*current_tab).clone()} />
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
        "#)}>
            { for children.iter() }
        </ul>
    }
}
