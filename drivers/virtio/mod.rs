//! VirtIO Drivers — QEMU virtualization support
//! Block, network, and console devices

#![allow(unused)]

pub struct VirtioDevice {
    pub device_type: u32,
    pub queue_size: u16,
}
