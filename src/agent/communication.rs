use crate::editor::EditorFile;
use serde::{Deserialize, Serialize};

/// The type that a worker
/// can receive
#[derive(Serialize, Deserialize, Debug)]
pub enum ToWorker {
    Ping,
    CompileCode(Vec<EditorFile>),
}

/// The type that a Worker
/// can send back
#[derive(Serialize, Deserialize, Debug)]
pub enum FromWorker {
    Pong(String),
    Error(ErrorResponseData),
    Decompiled(DecompiledResponseData),
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
    pub binary: mipsy_lib::Binary,
}
