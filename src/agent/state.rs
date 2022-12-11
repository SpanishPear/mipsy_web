use mipsy_lib::Binary;

pub struct BinaryRuntimeState {
    pub binary: Binary,
    pub runtime: RuntimeState,
}

type Guard<T> = Box<dyn FnOnce(T) -> mipsy_lib::Runtime>;

// for now, the below are not implemented
pub enum RuntimeState {
    Running(Box<mipsy_lib::Runtime>),
    WaitingInt(Guard<i32>),
    WaitingFloat(Guard<f32>),
    WaitingString(Guard<Vec<u8>>),
    WaitingChar(Guard<u8>),
}
