use crate::components::tab::Tab;
use monaco::yew::CodeEditorLink;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(TabContainer)]
pub fn tabs() -> Html {
    let link = use_context::<CodeEditorLink>().expect("should have a link");
    // get a vec of all model paths
    let all_tabs = monaco::api::TextModel::get_all()
        .into_iter()
        .map(|model| model.uri().path());

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
    // TODO: tab close button
    // TODO: tab middle click to close
    // TODO(stretch): tab drag and drop to reorder
    // TODO: tab onclick to focus
    // TODO: if the filename is not already open, show filename, else show full path
    // TODO: if the filename is too long, truncate it

    html! {
        <ContextProvider<Vec<String>> context={all_tabs.clone().collect::<Vec<String>>()}>
            <StyledTabContainer>
                {
                    for ["test", "/1", "tabs", "things", "there"].into_iter().map(|uri| {
                        html! {
                            <Tab uri={uri} selected={(*current_tab).clone()} />
                        }
                    })
                }
            </StyledTabContainer>
        </ContextProvider<Vec<String>>>
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
