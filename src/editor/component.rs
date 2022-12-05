use crate::components::tab_container::TabContainer;
use crate::editor::{get_options, MipsyCodeEditorLink};
use bounce::use_atom;
use monaco::yew::CodeEditor;
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

    let link = use_atom::<MipsyCodeEditorLink>();

    html! {
        <>
            <TabContainer />
            <CodeEditor
                classes={styles}
                options={get_options()}
                link={(*link).clone().link}
            />
        </>
    }
}
