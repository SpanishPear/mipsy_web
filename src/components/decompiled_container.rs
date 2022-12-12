use crate::agent::worker::MipsyWebWorker;
use crate::agent::FromWorker;
use crate::components::app::ReceiverHolder;
use crate::components::icons::{StopIconFilled, StopIconOutline};
use crate::state::app::{State, StateAction};
use bounce::use_slice;
use gloo_worker::WorkerBridge;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_hooks::use_async_with_options;
use yew_hooks::UseAsyncOptions;

#[styled_component(DecompiledContainer)]
pub fn data() -> Html {
    let state = use_slice::<State>();
    let worker = use_context::<WorkerBridge<MipsyWebWorker>>().expect("worker must exist at root");
    // get the raw decompiled text, and the current instruction (PC)
    let (decompiled, current_instr) = match *state {
        State::Compiled(ref running_state) => (
            &running_state.decompiled,
            running_state
                .mipsy_internal_state
                .current_instr
                .unwrap_or(0),
        ),
        _ => unreachable!("Not possible to be rendered if not compiled"),
    };

    let receiver = use_context::<ReceiverHolder>().expect("receiver must exist at root");
    let breakpoints = {
        log::info!("getting breakpoints");
        let worker = worker.clone();
        use_async_with_options(
            async move {
                worker.send(crate::agent::ToWorker::GetBreakpoints);
                receiver.receiver.recv().await
            },
            UseAsyncOptions::enable_auto(),
        )
    };

    if let Some(breakpoints) = &breakpoints.data {
        log::debug!("breakpoints: {:?}", breakpoints);
    };
    //TODO(refactor): this is from mipsy_web_v1 - refactor to remove duped code;
    html! {
            <pre class={css!(r#"
                font-size: 11px;
            "#)}>
            <table>
            { html! {
                for decompiled.as_str().split('\n').into_iter().map(|item| {

                    // add a &nbsp; if newline
                    if item.is_empty() {
                        return html! {
                            <tr>{"\u{00a0}"}</tr>
                        }
                    }

                    // the actual hex address lives from 2-10, 01 are 0x
                    // option because the current line could be a label (and hence no addr)
                    let source_instr = if item.starts_with("0x") {
                        Some(u32::from_str_radix(&item[2..10], 16).unwrap_or(0))
                    } else {
                        None
                    };
                    let should_highlight = if let Some(source_instr) = source_instr {
                        source_instr == current_instr
                    } else {
                        false
                    };

                    let current_is_breakpoint = if let Some(source_instr) = source_instr {
                        if let Some(FromWorker::Breakpoints(breakpoints)) = &breakpoints.data {
                            breakpoints.contains(&source_instr)
                        } else {
                            false
                        }
                    } else {
                        false
                    };

                    let toggle_breakpoint = {
                        let item = String::from(item);
                        let state = state.clone();
                        let worker = worker.clone();
                        Callback::from(move |_| {
                            state.dispatch(StateAction::ToggleBreakpoint(source_instr, item.clone(), worker.clone()));
                        })

                    };


                    html! {
                        <tr
                          class={
                            if should_highlight {
                                css!(r#"
                                    background-color: #f5f5f5;
                                "#)
                            } else {
                                css!(r#"
                                    background-color: transparent;
                                "#)
                            }
                          }>
                            <td class={css!(r#"
                                &:hover button {
                                    visibility: visible;
                                }
                            "#)} >
                                <button
                                    onclick={toggle_breakpoint}
                                    z-index={0}
                                    class={css!(r#"
                                        text-align: center;
                                        font-size: 14px;
                                        visibility: ${is_invisble};
                                        background-color: transparent;
                                        border: none;
                                        &:hover {
                                            visibility: ${inverse_is_visible};
                                            cursor: pointer;
                                        }
                                    "#,
                                        is_invisble = if current_is_breakpoint {
                                            "visible"
                                        } else {
                                            "hidden"
                                        },
                                        inverse_is_visible = if current_is_breakpoint {
                                            "hidden"
                                        } else {
                                            "visible"
                                        },
                                    )}
                                >
                                    if current_is_breakpoint {
                                        <StopIconFilled />
                                    } else {
                                        <StopIconOutline />
                                    }
                                </button>
                            </td>
                            <td class={css!("vertical-align: middle;")}>
                                {item}
                            </td>
                        </tr>
                    }

                })
            }}
            </table>
            </pre>
    }
}
