use serde::{Deserialize, Serialize};
use yew_agent::{Agent, AgentLink, HandlerId, Public};

/// A struct containing
/// state for the Worker
pub struct Worker {
    link: AgentLink<Self>,
}

/// The type that a worker
/// can receive
#[derive(Serialize, Deserialize)]
pub enum ToWorker {
    Ping,
}

/// The type that a Worker
/// can send back
#[derive(Serialize, Deserialize, Debug)]
pub enum FromWorker {
    Pong(String),
}

impl Agent for Worker {
    type Input = ToWorker;
    type Message = ();
    type Output = FromWorker;
    type Reach = Public<Self>;

    fn create(link: AgentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) {
        // no messaging
    }

    fn handle_input(&mut self, msg: Self::Input, id: HandlerId) {
        // this runs in a web worker
        // and does not block the main
        // browser thread!

        let output = Self::Output::Pong("hello from worker".to_string());

        self.link.respond(id, output);
    }

    fn name_of_resource() -> &'static str {
        "worker.js"
    }

    fn resource_path_is_relative() -> bool {
        true
    }
}
