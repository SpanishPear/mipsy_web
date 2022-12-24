use crate::{
    components::secondary_panel::{PanelType, SecondaryPanel},
    toggle_secondary_pane, SplitContainer,
};
use bounce::use_atom;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(SideBar)]
pub fn render() -> Html {
    let show_secondary_panel = use_state_eq(|| false);
    let panel_type = use_state_eq(|| PanelType::FileExplorer);
    let split_handle = use_atom::<SplitContainer>();

    // onclick to toggle the secondary panel
    let onclick = |panel_val: PanelType| {
        let show_secondary_panel = show_secondary_panel.clone();
        let panel_type = panel_type.clone();
        let split_handle = split_handle.clone();
        Callback::from(move |_| {
            if panel_val == *panel_type || !*show_secondary_panel {
                toggle_secondary_pane(&split_handle.handle, show_secondary_panel.clone());
            }
            let panel_val = panel_val.clone();
            panel_type.set(panel_val);
        })
    };

    html! {
        <div class={css!(r#"
            display: flex;
            height: 100%;
            justify-content: center;
        "#)}>
            // icon list of panels
            <div class={css!(r#"
                padding: 10px;
                display: flex;
                align-items: center;
                flex-direction: column;
            "#)}>
                {
                    for PanelType::iter().map(|panel_type| {
                        let panel_type = panel_type.clone();
                        let id = format!("{}-sidebar-button", panel_type.id());
                        html! {
                            <label tabindex=0
                                {id}
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
                                    margin-bottom: 10px;
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
                width: 2.3rem;
                height: 2.3rem;
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
        PanelType::Debug => {
            html! {
            <svg
                width="24px" height="24px" viewBox="0 0 24 24"
                fill="currentColor"
                class={css!(r#"
                    width: 2.3rem;
                    height: 2.3rem;
                "#)}
            >
                <path
                    d="M10.94 13.5l-1.32 1.32a3.73 3.73 0 0 0-7.24 0L1.06 13.5 0 14.56l1.72 1.72-.22.22V18H0v1.5h1.5v.08c.077.489.214.966.41 1.42L0 22.94 1.06 24l1.65-1.65A4.308 4.308 0 0 0 6 24a4.31 4.31 0 0 0 3.29-1.65L10.94 24 12 22.94 10.09 21c.198-.464.336-.951.41-1.45v-.1H12V18h-1.5v-1.5l-.22-.22L12 14.56l-1.06-1.06zM6 13.5a2.25 2.25 0 0 1 2.25 2.25h-4.5A2.25 2.25 0 0 1 6 13.5zm3 6a3.33 3.33 0 0 1-3 3 3.33 3.33 0 0 1-3-3v-2.25h6v2.25zm14.76-9.9v1.26L13.5 17.37V15.6l8.5-5.37L9 2v9.46a5.07 5.07 0 0 0-1.5-.72V.63L8.64 0l15.12 9.6z"/>
            </svg>
            }
        }
    }
}
