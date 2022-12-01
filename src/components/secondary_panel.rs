use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Clone, PartialEq, Eq, Properties)]
pub struct SecondaryPanelProps {
    pub show: bool,
}

#[styled_component(SecondaryPanel)]
pub fn render(SecondaryPanelProps { show }: &SecondaryPanelProps) -> Html {
    if *show {
        html! {
            <div class={css!(r#"
                        min-width: 60px;
                        border-left: 1px solid #000;
                        padding: 10px;
                    "#)}>
                {"secondary panel"}
            </div>
        }
    } else {
        html! {}
    }
}
