use std::collections::HashMap;

use bounce::Atom;
use mipsy_lib::{Binary, MipsyError, Safe};
use serde::{Deserialize, Serialize};

use crate::agent::communication::DecompiledResponseData;

#[derive(Atom, Default, Debug, PartialEq, Clone)]
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

impl State {
    pub fn new_compiled_state_from_response(response: DecompiledResponseData) -> Self {
        Self::Compiled(RunningState::new(response.decompiled, response.binary))
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct RunningState {
    // the syscall input
    pub decompiled: String,
    pub mipsy_internal_state: MipsyInternalState,
    // TODO(state): is this needed?
    pub should_kill: bool,
    // tell the application that we running
    // but waiting on some syscall input
    pub input_needed: Option<ReadSyscalls>,
}

impl RunningState {
    pub fn new(decompiled: String, binary: Binary) -> Self {
        Self {
            decompiled,
            mipsy_internal_state: MipsyInternalState::new_with_binary(binary),
            should_kill: false,
            input_needed: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    CompilerOrParserError(ErrorState),
    RuntimeError(RuntimeErrorState),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ErrorState {
    pub error: MipsyError,
    pub mipsy_stdout: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeErrorState {
    pub error: MipsyError,
    pub mips_state: MipsyInternalState,
    pub decompiled: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct MipsyInternalState {
    // program stdout
    pub stdout: Vec<String>,
    // stdout from mipsy (i.e. compiler error, etc)
    pub mipsy_stdout: Vec<String>,
    // program has exited with some return value
    pub exit_status: Option<i32>,
    // keep track of previous and current
    // registers for highlighting
    pub register_values: Vec<Safe<i32>>,
    pub previous_registers: Vec<Safe<i32>>,
    pub current_instr: Option<u32>,
    // cannot be a big array due to serde not using const-generics yet
    // https://github.com/serde-rs/serde/issues/631
    pub memory: HashMap<u32, Vec<Safe<u8> /*; PAGE_SIZE] */>>,
    // TODO(state): is this needed?
    pub is_stepping: bool,
    // the mipsy binary
    pub binary: Option<Binary>,
    /// used to tell us if we have already exited from a breakpoint
    /// and if the next run should continue or not
    /// ONLY worker.rs should ever set this
    pub breakpoint_switch: bool,
}

impl MipsyInternalState {
    pub fn new() -> Self {
        Self {
            stdout: Vec::new(),
            mipsy_stdout: Vec::new(),
            exit_status: None,
            register_values: vec![Safe::Uninitialised; 32],
            previous_registers: vec![Safe::Uninitialised; 32],
            current_instr: None,
            memory: HashMap::new(),
            is_stepping: false,
            binary: None,
            breakpoint_switch: false,
        }
    }

    pub fn new_with_binary(binary: Binary) -> Self {
        Self {
            stdout: Vec::new(),
            mipsy_stdout: Vec::new(),
            exit_status: None,
            register_values: vec![Safe::Uninitialised; 32],
            previous_registers: vec![Safe::Uninitialised; 32],
            current_instr: None,
            memory: HashMap::new(),
            is_stepping: false,
            binary: Some(binary),
            breakpoint_switch: false,
        }
    }
}

impl Default for MipsyInternalState {
    fn default() -> Self {
        Self::new()
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
