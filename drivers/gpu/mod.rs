//! GPU Graphics Driver
//! Hardware-accelerated rendering pipeline

#![allow(unused)]

pub struct GpuDevice {
    pub vram_mb: u32,
    pub max_resolution: (u32, u32),
}
