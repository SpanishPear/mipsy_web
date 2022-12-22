use bounce::use_slice_dispatch;
use gloo_worker::WorkerBridge;
use stylist::yew::styled_component;
use yew::prelude::*;

use crate::agent::worker::MipsyWebWorker;
use crate::editor::files::{FileList, FileListAction};

#[styled_component(StartButton)]
pub fn render() -> Html {
    let bridge =
        use_context::<WorkerBridge<MipsyWebWorker>>().expect("context should exist at app root");

    let dispatch = use_slice_dispatch::<FileList>();
    let onclick = {
        Callback::from(move |_| {
            dispatch(FileListAction::SendCompileCode(bridge.clone()));
        })
    };

    html! {
        <svg {onclick}
             viewBox="0 0 20 20" fill="currentColor"
             class={css!(r#"
                width: 2rem;
                height: 2rem;
                cursor: pointer;
                transition: all 0.2s ease-in-out;
                &:hover {
                    transform: scale(1.1);
            }"#)}>
            <path fill-rule="evenodd"
                d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clip-rule="evenodd"
             />
        </svg>
    }
}
