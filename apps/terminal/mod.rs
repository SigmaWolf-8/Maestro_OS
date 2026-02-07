//! Ternary Terminal
//! Command-line interface with ternary-encoded I/O

#![allow(unused)]

pub struct Terminal {
    pub prompt: &'static str,
    pub history_size: usize,
}
