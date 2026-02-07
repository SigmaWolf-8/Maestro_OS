//! Salvi Kernel — Core kernel for Maestro OS
//! Ternary-native memory management, process scheduling, and security modes

#![no_std]

pub mod memory;
pub mod scheduler;
pub mod syscall;
pub mod security;
pub mod ipc;
