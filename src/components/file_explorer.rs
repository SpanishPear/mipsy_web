use crate::components::new_file_button::NewFileButton;
use crate::editor::files::FileList;
use bounce::use_slice;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(FileExplorer)]
pub fn file_explorer() -> Html {
    let files = &use_slice::<FileList>().files;
    let selected = &use_slice::<FileList>().selected;
    html! {
        <>
            <div class={css!(r#"
                display: flex;
                flex-direction: row;
                justify-content: space-between;
                margin-bottom: 0.5rem;
            "#)}>
                <div class={css!(r#"
                    font-weight: bold;
                "#)}>
                    { "Files" }
                </div>
                // add three checkboxes
                <div>
                    <NewFileButton />
                </div>
            </div>
            {
                for files.iter().enumerate().map(|(index, file_info)| {
                    let selected = selected.is_some() && selected.unwrap() == index;
                    html! {
                        <FileName name={file_info.name.clone()} {selected}/>
                    }
                })
            }
        </>

    }
}

#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct FileNameProps {
    pub name: String,
    pub selected: bool,
}

#[styled_component(FileName)]
fn from(FileNameProps { name, selected }: &FileNameProps) -> Self {
    html! {
        <div data-selected={if *selected {"true"} else {"false"}} class={css!(r#"
                display: flex;
                align-items: center;
                padding: 0.5rem;
                cursor: pointer;
                transition: background-color 0.2s ease-in-out;
                &:hover {
                    background-color: rgba(0, 0, 0, 0.1);
                }
                &[data-selected="true"] {
                    background-color: rgba(0, 0, 0, 0.1);
                }
            "#)}>
            {name}
        </div>
    }
}
