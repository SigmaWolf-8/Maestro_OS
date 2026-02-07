//! HID Input Devices
//! Keyboard, mouse, and touchscreen abstraction

#![allow(unused)]

pub enum InputEvent {
    KeyPress(u8),
    KeyRelease(u8),
    MouseMove(i32, i32),
    MouseClick(u8),
}
