use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Programs {
    pub aeneas: String,
    pub primitives: String,
}

#[derive(Debug, Deserialize)]
pub struct Coq {
    pub original: String,
    pub refactored: String,
}

#[derive(Debug, Deserialize)]
pub struct Llbc {
    pub original: String,
    pub refactored: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub programs: Programs,
    pub coq: Coq,
    pub llbc: Llbc,
}