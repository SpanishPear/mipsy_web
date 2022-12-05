use bounce::Atom;

#[derive(Atom, Default, PartialEq, Eq)]
pub struct MipsyWebConfig {
    pub web: MipsyWebUserConfig,
    pub lib: mipsy_utils::MipsyConfig,
}

#[derive(Default, PartialEq, Eq)]
pub struct MipsyWebUserConfig {}
