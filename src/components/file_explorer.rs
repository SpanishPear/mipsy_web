use crate::editor::files::{FileInfo, FileList};
use bounce::use_slice;
use stylist::yew::styled_component;
use yew::prelude::*;

impl From<FileInfo> for Html {
    fn from(file_info: FileInfo) -> Self {
        html! {
            <div>
                {file_info.name}
            </div>
        }
    }
}

#[styled_component(FileExplorer)]
pub fn file_explorer() -> Html {
    let files = &use_slice::<FileList>().files;

    html! {
        <>
            <div class={css!(r#"
                display: flex;
                flex-direction: row;
                justify-content: flex-end;
            "#)}>
                // add three checkboxes
                <input type="button" title="test button" />
                <input type="button" title="test buton2" />
                <input type="button" title="test button3" />
            </div>
            {
                for files.iter().map(|file_info| {
                    html! {
                        {file_info.clone()}
                    }
                })
            }
        </>

    }
}
