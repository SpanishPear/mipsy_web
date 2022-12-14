use bounce::Atom;
use serde::{Deserialize, Serialize};

#[derive(Atom, Default, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct Breakpoints {
    /// Vec of all the breakpoints
    /// this is sent to us from the webworker
    inner: Vec<u32>,
}

impl Breakpoints {
    pub fn inner(&self) -> &Vec<u32> {
        &self.inner
    }
}

impl From<Vec<u32>> for Breakpoints {
    fn from(inner: Vec<u32>) -> Self {
        Self { inner }
    }
}
