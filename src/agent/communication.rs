use crate::editor::EditorFile;
use crate::state::breakpoints::Breakpoints;
use serde::{Deserialize, Serialize};

/// The type that a worker
/// can receive
#[derive(Serialize, Deserialize, Debug)]
pub enum ToWorker {
    Ping,
    CompileCode(Vec<EditorFile>),
    ToggleBreakpoint(Option<u32>, String),
    GetBreakpoints,
}

/// The type that a Worker
/// can send back
#[derive(Serialize, Deserialize, Debug)]
pub enum FromWorker {
    Pong(String),
    Error(ErrorResponseData),
    Decompiled(DecompiledResponseData),
    /// A vector of addresses of all active breakpoints
    // TODO(breakpoints): extend to include enabled,ignore_count, etc
    Breakpoints(Breakpoints),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponseData {
    pub error_type: mipsy_lib::MipsyError,
    pub file_name: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DecompiledResponseData {
    pub decompiled: String,
}
