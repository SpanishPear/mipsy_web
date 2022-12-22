use crate::components::debugger::start_button::StartButton;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(DebugPane)]
pub fn render() -> Html {
    html! {
        <div>
            // file explorer header container
            <div class={css!(r#"
                display: flex;
                flex-direction: row;
                justify-content: space-between;
                margin-bottom: 0.5rem;
            "#)}>
                // files title
                <div class={css!(r#"
                    font-weight: bold;
                "#)}>
                    { "Debugger" }
                </div>
            </div>
            <StartButton />

        </div>
    }
}
