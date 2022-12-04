use crate::components::secondary_panel::{PanelType, SecondaryPanel};
use crate::editor::files::{FileList, FileListAction};
use crate::{toggle_secondary_pane, SplitContainer};
use bounce::{use_atom, use_slice};
use gloo::file::File;
use stylist::yew::styled_component;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[styled_component(MenuBar)]
pub fn menubar() -> Html {
    let files = use_slice::<FileList>();
    let show_secondary_panel = use_state_eq(|| false);
    let panel_type = use_state_eq(|| PanelType::FileExplorer);

    let _onchange = {
        Callback::from(move |e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let files = files.clone();
            // for every file, read it and
            // add it to the FileList
            let mut result = Vec::new();

            if let Some(files) = input.files() {
                let files = js_sys::try_iter(&files)
                    .unwrap()
                    .unwrap()
                    .map(|v| web_sys::File::from(v.unwrap()))
                    .map(File::from);
                result.extend(files);
            }

            for file in result.into_iter() {
                let name = file.name();
                let contents = gloo::file::futures::read_as_bytes(&file);
                let files = files.clone();
                spawn_local(async move {
                    let contents = contents.await;

                    // TODO(error handling): handle invalid utf8
                    let contents = contents.expect("File contains invalid utf8");

                    let contents = String::from_utf8(contents).unwrap();

                    files.dispatch(FileListAction::Append(name, contents));

                    files.dispatch(FileListAction::Log);
                });
            }
        })
    };

    // onclick to toggle the secondary panel
    let onclick = |panel_val: PanelType| {
        let show_secondary_panel = show_secondary_panel.clone();
        let split_handle = use_atom::<SplitContainer>();
        let panel_type = panel_type.clone();
        Callback::from(move |_| {
            toggle_secondary_pane(&split_handle.handle, *show_secondary_panel);
            let panel_val = panel_val.clone();
            panel_type.set(panel_val);
            show_secondary_panel.set(!(*show_secondary_panel));
        })
    };

    html! {
        <div class={css!(r#"
            display: flex;
            height: 100%;
        "#)}>
            // icon list of panels
            <div class={css!(r#"
                padding: 10px;
            "#)}>
                {
                    for PanelType::iter().map(|panel_type| {
                        let panel_type = panel_type.clone();
                        html! {
                            <label tabindex=0
                                title={panel_type.title()}
                                onclick={onclick(panel_type.clone())}
                                class={css!(r#"
                                    display: flex;
                                    align-items: center;
                                    justify-content: center;
                                    &:hover {
                                        cursor: pointer;
                                        stroke: white;
                                    } 
                            "#)}>
                                <PanelIcon panel={panel_type} />
                            </label>
                        }
                    })
                }
            </div>
            // secondary panel
            <SecondaryPanel
                panel_type={(*panel_type).clone()}
                show={*show_secondary_panel}
            />
        </div>
    }
}

#[derive(Properties, PartialEq, Eq)]
pub struct IconProps {
    pub panel: PanelType,
}

#[styled_component(PanelIcon)]
fn icon_renderer(props: &IconProps) -> Html {
    match props.panel {
        PanelType::FileExplorer => html! {
            <svg viewBox="0 0 20 20" fill="currentColor" class={css!(r#"
                width: 2.0rem;
                height: 2.0rem;
            "#)} >
              <path
                stroke-width="3%"
                fill-rule="evenodd"
                d="M2 6a2 2 0 012-2h4l2 2h4a2 2 0 012 2v1H8a3 3 0 00-3 3v1.5a1.5 1.5 0 01-3 0V6z"
                clip-rule="evenodd"
              />
              <path
                stroke-width="3%"
                d="M6 12a2 2 0 012-2h8a2 2 0 012 2v2a2 2 0 01-2 2H2h2a2 2 0 002-2v-2z"
              />
            </svg>
        },
    }
}
