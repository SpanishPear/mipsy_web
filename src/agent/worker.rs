use super::state::BinaryRuntimeState;
use crate::agent::state::ErrorResponseData;
use crate::config::MipsyWebConfig;
use crate::editor::EditorFile;
use gloo_worker::{HandlerId, Worker, WorkerScope};
use mipsy_lib::compile::CompilerOptions;
use mipsy_parser::TaggedFile;
use serde::{Deserialize, Serialize};

/// A struct containing
/// state for the Worker
pub struct MipsyWebWorker {
    config: MipsyWebConfig,
    inst_set: mipsy_lib::InstSet,
    binary_runtime_state: Option<BinaryRuntimeState>,
}

/// The type that a worker
/// can receive
#[derive(Serialize, Deserialize, Debug)]
pub enum ToWorker {
    Ping,
    CompileCode(Vec<EditorFile>),
}

/// Used for internal messaging
#[allow(dead_code)]
enum Message {
    Pong,
}

/// The type that a Worker
/// can send back
#[derive(Serialize, Deserialize, Debug)]
pub enum FromWorker {
    Pong(String),
    Error(ErrorResponseData),
}

impl Worker for MipsyWebWorker {
    type Input = ToWorker;
    type Message = ();
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
                let files = files
                    .iter()
                    .map(|file| TaggedFile::new(Some(&file.name), &file.content))
                    .collect::<Vec<_>>();

                let compiled =
                    mipsy_lib::compile(&self.inst_set, files, &CompilerOptions::default(), config);

                match compiled {
                    Ok(binary) => {
                        /*let decompiled = decompile(&binary, &self.inst_set, Some(file.clone()));
                        let response = Self::Output::DecompiledCode(DecompiledResponse {
                            decompiled,
                            file: Some(file.clone()),
                            binary: binary.to_owned(),
                        });
                        let runtime = mipsy_lib::runtime(&binary, &[]);
                        self.binary = Some(binary);
                        self.runtime = Some(RuntimeState::Running(runtime));
                        self.file = Some(file);
                        self.link.respond(id, response)
                        */
                    }

                    Err(error_type) => {
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
                                unreachable!(
                                    "runtime error should not be possible at compile time"
                                );
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
            }
        }
    }
}
