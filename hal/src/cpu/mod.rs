//! CPU Abstraction
//! x86_64 with ternary instruction extensions

#![allow(unused)]

pub struct CpuInfo {
    pub cores: u32,
    pub ternary_capable: bool,
}
