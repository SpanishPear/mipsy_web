use crate::components::tab_container::TabContainer;
use crate::editor::files::FileList;
use crate::editor::{get_options, MipsyCodeEditorLink};
use bounce::{use_atom, use_slice};
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
    let styles: StyleSource = styles.as_str().try_into().unwrap();

    let link = use_atom::<MipsyCodeEditorLink>();
    let files = use_slice::<FileList>();

    let current_model = files
        .selected
        .and_then(|i| files.files.get(i))
        .and_then(|f| monaco::api::TextModel::get(&f.uri));

    html! {
        <>
            <TabContainer />
            <CodeEditor
                classes={styles}
                options={get_options()}
                link={(*link).clone().link}
                model={current_model}
            />
        </>
    }
}
