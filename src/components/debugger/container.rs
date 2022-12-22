use crate::components::icons::{
    IconButton, ResetIcon, RunIconOutline, StepForwardIcon, StopIconOutline,
};
use crate::components::{debugger::start_button::StartButton, icons::StepBackIcon};
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
            <div class={css!(r#"
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
            "#)}>
            <IconButton>
                <RunIconOutline />
            </IconButton>

            <IconButton >
                <StepBackIcon />
            </IconButton>

            <IconButton>
                <StepForwardIcon />
            </IconButton>

            <IconButton>
                <StopIconOutline/>
            </IconButton>

            <IconButton>
                <ResetIcon />
            </IconButton>

            </div>
        </div>
    }
}
