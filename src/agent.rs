use gloo_worker::{HandlerId, Worker, WorkerScope};
use serde::{Deserialize, Serialize};

/// A struct containing
/// state for the Worker
pub struct MipsyWebWorker {}

/// The type that a worker
/// can receive
#[derive(Serialize, Deserialize)]
pub enum ToWorker {
    Ping,
}

/// Used for internal messaging
pub enum Message {
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
        Self {}
    }

    fn update(&mut self, _scope: &WorkerScope<Self>, _msg: Self::Message) {
        // no messaging
    }

    fn received(&mut self, scope: &WorkerScope<Self>, _msg: Self::Input, id: HandlerId) {
        // this runs in a web worker
        // and does not block the main
        // browser thread!

        let output = Self::Output::Pong("hello from worker".to_string());
        scope.respond(id, output);
    }
}
