use crate::components::run_button::RunButton;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(DebugPane)]
pub fn render() -> Html {
    html! {
        <div>
            <RunButton />
        </div>
    }
}
