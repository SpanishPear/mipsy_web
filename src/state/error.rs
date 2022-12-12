use super::mipsy_internal::MipsyInternalState;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    CompilerOrParserError(ErrorState),
    RuntimeError(RuntimeErrorState),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ErrorState {
    pub error: mipsy_lib::MipsyError,
    pub mipsy_stdout: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeErrorState {
    pub error: mipsy_lib::MipsyError,
    pub mips_state: MipsyInternalState,
    pub decompiled: String,
}
