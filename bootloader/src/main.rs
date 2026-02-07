//! Salvi-Boot — UEFI bootloader for Maestro OS
//! Stage 0: UEFI/BIOS handoff
//! Stage 1: Rust stub loader
//! Stage 2: Full HAL initialization

#![no_std]
#![no_main]

mod stage0;
mod stage1;
mod stage2;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
