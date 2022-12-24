use crate::components::icons::{
    CompileCodeIcon, IconButton, ResetIcon, RunIconOutline, StepForwardIcon, StopIconOutline,
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

                <IconButton title="compile">
                    <CompileCodeIcon />
                </IconButton>

                <IconButton title="compile and run">
                    <RunIconOutline />
                </IconButton>

                <IconButton title="step back">
                    <StepBackIcon />
                </IconButton>

                <IconButton title="step forward">
                    <StepForwardIcon />
                </IconButton>

                <IconButton title="stop">
                    <StopIconOutline/>
                </IconButton>

                <IconButton title="reset">
                    <ResetIcon />
                </IconButton>

            </div>
        </div>
    }
}
