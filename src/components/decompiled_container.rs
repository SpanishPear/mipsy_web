use bounce::use_atom;
use stylist::yew::styled_component;
use yew::prelude::*;

use crate::components::icons::{StopIconFilled, StopIconOutline};
use crate::state::{ErrorType, State};

#[styled_component(DecompiledContainer)]
pub fn data() -> Html {
    let state = use_atom::<State>();

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

    //TODO(refactor): this is from mipsy_web_v1 - refactor to remove duped code;
    html! {
            <pre class={css!(r#"
                font-size: 11px;
            "#)}>
            <table>
            { html! {
                for decompiled.as_str().split('\n').into_iter().map(|item| {
                    if item.is_empty() {
                        // this is &nbsp;
                        html! {
                            <tr>{"\u{00a0}"}</tr>
                        }
                    }
                    else {
                        // the actual hex address lives from 2-10, 01 are 0x
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
                        let current_is_breakpoint = match &*state {
                            State::NoBinary => unreachable!("cannot have decompiled if no file"),
                            State::Error(error_type) => {
                                if let ErrorType::RuntimeError(_error) = error_type {
                                    false
                                } else {
                                    unreachable!("Error in decompiled not possible if not compiled");
                                }
                            },
                            State::Compiled(curr) => {
                                let binary = curr.mipsy_internal_state.binary.as_ref().expect("binary must exist");
                                let addr = if let Some(source_instr) = source_instr {
                                    source_instr
                                } else {
                                    binary.get_label(&item.trim().replace(':', "")).expect("label must exist")
                                };
                                binary.breakpoints.contains_key(&addr)
                            }

                        };

                        let toggle_breakpoint = {
                            let item = String::from(item);
                            // let worker = props.worker.clone();
                            let state = state.clone();
                            Callback::from(move |_| {
                                match &*state {
                                    State::NoBinary=> unreachable!(),
                                    State::Error(error_type) =>  {
                                        if let ErrorType::RuntimeError(error) = error_type {
                                            let binary = error.mips_state.binary.as_ref().expect("binary must exist");
                                            let addr = if let Some(source_instr) = source_instr {
                                                source_instr
                                            } else {
                                                binary.get_label(&item.trim().replace(':', "")).expect("label must exist")
                                            };
                                            //TODO(breakpoints): toggle breakpoint
                                            //worker.send(WorkerRequest::ToggleBreakpoint(addr));
                                        } else {
                                            unreachable!("Error in decompiled not possible if not compiled");
                                        }
                                    }
                                    State::Compiled(curr) => {
                                        let binary = curr.mipsy_internal_state.binary.as_ref().expect("binary must exist");
                                        let addr = if let Some(source_instr) = source_instr {
                                            source_instr
                                        } else {
                                            binary.get_label(&item.trim().replace(':', "")).expect("label must exist")
                                        };
                                        //TODO(breakpoints): toggle breakpoint
                                        // worker.send(WorkerRequest::ToggleBreakpoint(addr));
                                    },
                                }
                            })
                        };


                        // TODO(breakpoints): make css styling work to hover correctly
                        html! {
                            <tr
                              class={
                                classes!("", if should_highlight {
                                  "bg-th-highlighting"
                                } else {
                                  ""
                                })
                              }>
                                <td class="breakpoint_button_td" >
                                    <button
                                        onclick={toggle_breakpoint}
                                        z-index={0}
                                        class={css!(r#"
                                            text-align: center;
                                            font-size: 11px;
                                            visibility: ${is_invisble};
                                            &:hover {
                                                visibility: ${inverse_is_visible};
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
                                        //TODO(breakpoints): toggle breakpoint
                                        //classes!("text-center", "text-xs", if !current_is_breakpoint {"group-hover:visible invisible"} else {""})}
                                    >
                                        if current_is_breakpoint {
                                            <StopIconFilled />
                                        } else {
                                            <StopIconOutline />
                                        }
                                    </button>
                                </td>
                                <td>
                                    {item}
                                </td>
                            </tr>
                        }
                    }
                })
            }}
            </table>
            </pre>
    }
}
