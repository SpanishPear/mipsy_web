use super::app::ReadSyscalls;
use super::mipsy_internal::MipsyInternalState;

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
    pub fn new(decompiled: String, binary: mipsy_lib::Binary) -> Self {
        Self {
            decompiled,
            mipsy_internal_state: MipsyInternalState::new_with_binary(binary),
            should_kill: false,
            input_needed: None,
        }
    }
}
