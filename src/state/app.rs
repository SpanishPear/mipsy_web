use crate::agent::communication::DecompiledResponseData;

use super::error::ErrorType;
use super::running::RunningState;
use bounce::Slice;
use std::rc::Rc;
use yew::Reducible;

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
}

impl Reducible for State {
    type Action = StateAction;
    fn reduce(self: Rc<Self>, action: StateAction) -> Rc<Self> {
        match action {
            StateAction::InitialiseFromDecompiled(response) => {
                Rc::new(State::new_compiled_state_from_response(response))
            }
        }
    }
}

impl State {
    pub fn new_compiled_state_from_response(response: DecompiledResponseData) -> Self {
        Self::Compiled(RunningState::new(response.decompiled, response.binary))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReadSyscalls {
    ReadInt,
    ReadFloat,
    ReadDouble,
    ReadString,
    ReadChar,
}
