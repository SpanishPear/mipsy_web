use crate::editor::get_options;
use monaco::yew::{CodeEditor, CodeEditorLink};
use stylist::yew::styled_component;
use stylist::StyleSource;
use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct EditorProps {
    pub styles: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TabName(String);

#[styled_component(Editor)]
pub fn editor(EditorProps { styles }: &EditorProps) -> Html {
    let styles: StyleSource = styles.as_str().into();

    let link: UseStateHandle<CodeEditorLink> = use_state(CodeEditorLink::default);

    html! {
        <>
            <Tabs link={link.clone()} />
            <CodeEditor
                classes={styles}
                options={get_options()}
                link={(*link).clone()}
            />
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct TabProps {
    pub link: UseStateHandle<CodeEditorLink>,
}

// TODO - make this a styled component, in a separate file
#[function_component(Tabs)]
pub fn tabs(TabProps { link }: &TabProps) -> Html {
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
            let maybe_tabname = (*link).with_editor(|editor| {
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
    // TODO: the above TODO's should be done in a Tab component
    html! {
        {
            for all_tabs.into_iter().map(|display_name| {
                if let Some(current_tab_name) = &*current_tab
                    && current_tab_name == &display_name {

                    html! {
                        <div class="tab selected">{display_name}</div>
                    }

                } else {
                    html! {
                        <div class="tab">{display_name}</div>
                    }
                }
            })
        }
    }
}
