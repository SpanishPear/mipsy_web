use mipsy_lib::Binary;
use std::collections::HashMap;

use mipsy_lib::Safe;
use serde::{Deserialize, Serialize};

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
    pub binary: Option<mipsy_lib::Binary>,
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
