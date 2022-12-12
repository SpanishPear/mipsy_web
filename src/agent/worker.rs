use super::communication::{FromWorker, ToWorker};
use super::state::BinaryRuntimeState;
use crate::agent::communication::{DecompiledResponseData, ErrorResponseData};
use crate::agent::mipsy_glue;
use crate::agent::state::RuntimeState;
use crate::config::MipsyWebConfig;
use crate::state::app::breakpoint_address_from_source;
use gloo_worker::{HandlerId, Worker, WorkerScope};
use mipsy_lib::compile::breakpoints::Breakpoint;
use mipsy_lib::compile::CompilerOptions;
use mipsy_lib::Binary;
use mipsy_parser::TaggedFile;

/// A struct containing
/// state for the Worker
pub struct MipsyWebWorker {
    config: MipsyWebConfig,
    inst_set: mipsy_lib::InstSet,
    binary_runtime_state: Option<BinaryRuntimeState>,
}

/// Used for internal messaging
#[allow(dead_code)]
enum Message {
    Pong,
}

impl Worker for MipsyWebWorker {
    type Message = ();
    type Input = ToWorker;
    type Output = FromWorker;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self {
            config: MipsyWebConfig::default(),
            inst_set: mipsy_instructions::inst_set(),
            binary_runtime_state: None,
        }
    }

    fn update(&mut self, _scope: &WorkerScope<Self>, _msg: Self::Message) {
        // no messaging
    }

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, id: HandlerId) {
        // this runs in a web worker
        // and does not block the main
        // browser thread!
        log::info!("received message from main thread: {:#?}", msg);
        match msg {
            ToWorker::Ping => {
                let output = Self::Output::Pong("hello from worker".to_string());
                scope.respond(id, output);
            }
            ToWorker::CompileCode(files) => {
                let config = &self.config.lib;
                let files: Vec<mipsy_parser::TaggedFile> = files
                    .iter()
                    .map(|file| TaggedFile::new(Some(&file.name), &file.content))
                    .collect::<Vec<_>>();

                let compiled = mipsy_lib::compile(
                    &self.inst_set,
                    files.clone(),
                    &CompilerOptions::default(),
                    config,
                );

                // use if let to reduce nesting...
                if let Ok(binary) = compiled {
                    let decompiled = mipsy_glue::decompile(&self.inst_set, &binary, &files);
                    let response = Self::Output::Decompiled(DecompiledResponseData { decompiled });

                    // create a new runtime from the given binary
                    let runtime = mipsy_lib::runtime(&binary, &[]);
                    let runtime_state = BinaryRuntimeState {
                        // store the runtime on the heap, so it doesnt get huge enum
                        runtime: RuntimeState::Running(Box::new(runtime)),
                        binary,
                    };
                    self.binary_runtime_state = Some(runtime_state);
                    scope.respond(id, response)
                } else if let Err(error_type) = compiled {
                    self.binary_runtime_state = None;
                    let error_msg = match error_type {
                        mipsy_lib::MipsyError::Compiler(ref compiler_err) => {
                            log::info!("compiler error: {:#?}", compiler_err);
                            /*format!(
                                "{}\n{}\n{}",
                                generate_highlighted_line(file.clone(), compiler_err),
                                compiler_err.error().message(),
                                compiler_err.error().tips().join("\n")
                            )*/
                            "".to_string()
                        }
                        mipsy_lib::MipsyError::Parser(_) => String::from("failed to parse"),
                        mipsy_lib::MipsyError::Runtime(_) => {
                            unreachable!("runtime error should not be possible at compile time");
                        }
                    };
                    scope.respond(
                        id,
                        Self::Output::Error(ErrorResponseData {
                            error_type,
                            file_name: "".to_string(),
                            message: error_msg,
                        }),
                    )
                }
            }
            ToWorker::ToggleBreakpoint(line, source_instr) => {
                let binary = self.binary_runtime_state.as_mut();
                if let Some(binary) = binary {
                    let addr = breakpoint_address_from_source(&line, source_instr, &binary.binary);
                    if binary.binary.breakpoints.contains_key(&addr) {
                        binary.binary.breakpoints.remove(&addr);
                    } else {
                        let id = Binary::generate_id(&binary.binary.breakpoints);
                        binary.binary.breakpoints.insert(addr, Breakpoint::new(id));
                    }
                }
            }
            ToWorker::GetBreakpoints => {
                let binary = self.binary_runtime_state.as_ref();
                if let Some(binary) = binary {
                    let breakpoints = binary.binary.breakpoints.keys().cloned().collect();
                    scope.respond(id, Self::Output::Breakpoints(breakpoints));
                }
            }
        }
    }
}
