use crate::agent::{communication::DecompiledResponseData, worker::MipsyWebWorker, ToWorker};

use super::{error::ErrorType, running::RunningState};
use bounce::Slice;
use gloo_worker::WorkerBridge;
use std::rc::Rc;
use yew::Reducible;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReadSyscalls {
    ReadInt,
    ReadFloat,
    ReadDouble,
    ReadString,
    ReadChar,
}

#[derive(Slice, Default, Debug, PartialEq, Clone)]
pub enum State {
    /// There is not a currently compiled set of files
    #[default]
    NoBinary,
    /// There was an attempt to compile,
    /// or we have experienced a runtime error
    Error(ErrorType),
    /// There is a currently compiled set of files
    Compiled(RunningState),
}

pub enum StateAction {
    InitialiseFromDecompiled(DecompiledResponseData),
    ToggleBreakpoint(Option<u32>, String, WorkerBridge<MipsyWebWorker>),
}

impl Reducible for State {
    type Action = StateAction;
    fn reduce(self: Rc<Self>, action: StateAction) -> Rc<Self> {
        match action {
            StateAction::InitialiseFromDecompiled(response) => {
                Rc::new(State::new_compiled_state_from_response(response))
            }
            StateAction::ToggleBreakpoint(source_instr, line, bridge) => {
                bridge.send(ToWorker::ToggleBreakpoint(source_instr, line));
                self
            }
        }
    }
}

impl State {
    pub fn new_compiled_state_from_response(response: DecompiledResponseData) -> Self {
        Self::Compiled(RunningState::new(response.decompiled))
    }
}

pub fn breakpoint_address_from_source(
    line: &str,
    source_instr: Option<u32>,
    binary: &mipsy_lib::Binary,
) -> u32 {
    if let Some(source_instr) = source_instr {
        source_instr
    } else {
        binary
            .get_label(&line.trim().replace(':', ""))
            .expect("label must exist")
    }
}
