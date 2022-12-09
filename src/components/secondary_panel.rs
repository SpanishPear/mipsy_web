use crate::components::{debug_pane::DebugPane, file_explorer::FileExplorer};
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub enum PanelType {
    FileExplorer,
    Debug,
}

impl PanelType {
    pub fn iter() -> impl Iterator<Item = PanelType> {
        vec![PanelType::FileExplorer, PanelType::Debug].into_iter()
    }
    pub fn title(&self) -> String {
        match self {
            PanelType::FileExplorer => "File Explorer".to_string(),
            PanelType::Debug => "Debug".to_string(),
        }
    }
}

impl From<PanelType> for Html {
    fn from(panel_type: PanelType) -> Self {
        match panel_type {
            PanelType::FileExplorer => html! {
                <FileExplorer />
            },
            PanelType::Debug => html! {
                <DebugPane />
            },
        }
    }
}

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct SecondaryPanelProps {
    pub show: bool,
    pub panel_type: PanelType,
}

#[styled_component(SecondaryPanel)]
pub fn render(SecondaryPanelProps { show, panel_type }: &SecondaryPanelProps) -> Html {
    let panel_type = panel_type.clone();
    if *show {
        html! {
            <div class={css!(r#"
                        min-width: 60px;
                        border-left: 1px solid #000;
                        padding: 10px;
                        width: 100%;
                        overflow-x: hidden;
                    "#)}>
                {panel_type}
            </div>
        }
    } else {
        html! {}
    }
}
