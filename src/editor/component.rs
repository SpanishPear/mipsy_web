use crate::editor::get_options;
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
    html! {
        <CodeEditor
            classes={styles}
            options={get_options()}
        />
    }
}
