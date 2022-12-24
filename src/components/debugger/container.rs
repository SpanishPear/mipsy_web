use crate::agent::worker::MipsyWebWorker;
use crate::components::icons::StepBackIcon;
use crate::components::icons::{
    CompileCodeIcon, IconButton, ResetIcon, RunIconOutline, StepForwardIcon, StopIconOutline,
};
use crate::editor::files::{FileList, FileListAction};
use bounce::use_slice_dispatch;
use gloo_worker::WorkerBridge;
use stylist::yew::styled_component;
use yew::prelude::*;

#[styled_component(DebugPane)]
pub fn render() -> Html {
    let bridge = use_context::<WorkerBridge<MipsyWebWorker>>().expect("context should exist");
    let dispatch = use_slice_dispatch::<FileList>();

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

                <IconButton title="compile" onclick={Callback::from(move |_| {
                    dispatch(FileListAction::SendCompileCode(bridge.clone()));
                })}>
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
