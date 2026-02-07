//! Security Mode Transitions
//! Mode 0 (Hypervisor) -> Mode 1 (Kernel) -> Mode phi (Supervisor) -> Mode phi+ (User)

#![allow(unused)]

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SecurityMode {
    Hypervisor,
    Kernel,
    Supervisor,
    User,
}

pub fn transition(from: SecurityMode, to: SecurityMode) -> bool {
    // Only allow transitions to equal or lower privilege
    (to as u8) >= (from as u8)
}
