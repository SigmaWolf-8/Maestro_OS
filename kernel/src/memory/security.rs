//! Memory Security — Zone enforcement
//! Prevents cross-mode memory access violations

#![allow(unused)]

pub enum SecurityZone {
    Hypervisor,  // Mode 0 — full access
    Kernel,      // Mode 1 — kernel space
    Supervisor,  // Mode phi — system services
    User,        // Mode phi+ — user applications
}
