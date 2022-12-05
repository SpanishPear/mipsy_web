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
}

impl Worker for MipsyWebWorker {
    type Input = ToWorker;
    type Message = ();
    type Output = FromWorker;

    fn create(_scope: &WorkerScope<Self>) -> Self {
        Self {
            config: MipsyWebConfig::default(),
            inst_set: mipsy_instructions::inst_set(),
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

                log::debug!("compiled: {:#?}", compiled);
            }
        }
    }
}
