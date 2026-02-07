//! Ternary Switch — Three-state toggle
//! States: Negative (-1), Neutral (0), Positive (+1)

#![allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TernaryState {
    Negative,  // -1
    Neutral,   //  0
    Positive,  // +1
}

pub struct TernarySwitch {
    pub state: TernaryState,
    pub label: &'static str,
}
