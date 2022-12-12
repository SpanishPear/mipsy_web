use crate::editor::EditorFile;
use serde::{Deserialize, Serialize};

/// The type that a worker
/// can receive
#[derive(Serialize, Deserialize, Debug)]
pub enum ToWorker {
    Ping,
    CompileCode(Vec<EditorFile>),
    ToggleBreakpoint(String, Option<u32>),
    GetBreakpoints,
}

/// The type that a Worker
/// can send back
#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum FromWorker {
    Pong(String),
    Error(ErrorResponseData),
    Decompiled(DecompiledResponseData),
    Breakpoints(Vec<u32>),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ErrorResponseData {
    pub error_type: mipsy_lib::MipsyError,
    pub file_name: String,
    pub message: String,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct DecompiledResponseData {
    pub decompiled: String,
}
