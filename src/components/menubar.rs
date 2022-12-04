use crate::components::secondary_panel::{PanelType, SecondaryPanel};
use crate::{toggle_secondary_pane, SplitContainer};
use bounce::use_atom;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(MenuBar)]
pub fn menubar() -> Html {
    let show_secondary_panel = use_state_eq(|| false);
    let panel_type = use_state_eq(|| PanelType::FileExplorer);

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
