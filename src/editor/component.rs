use crate::components::tab_container::TabContainer;
use crate::editor::get_options;
use monaco::yew::{CodeEditor, CodeEditorLink};
use stylist::yew::styled_component;
use stylist::StyleSource;
use yew::prelude::*;

#[derive(Properties, PartialEq, Eq)]
pub struct EditorProps {
    pub styles: String,
}

#[styled_component(Editor)]
pub fn editor(EditorProps { styles }: &EditorProps) -> Html {
    let styles: StyleSource = styles.as_str().into();

    let link: UseStateHandle<CodeEditorLink> = use_state(CodeEditorLink::default);

    html! {
        <ContextProvider<CodeEditorLink> context={(*link).clone()}>
            <TabContainer />
            <CodeEditor
                classes={styles}
                options={get_options()}
                link={(*link).clone()}
            />
        </ContextProvider<CodeEditorLink>>
    }
}
