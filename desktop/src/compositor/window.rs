//! Window Management
//! Phase-encrypted window surfaces for secure multi-window

#![allow(unused)]

pub struct Window {
    pub id: u64,
    pub title: &'static str,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub phase_key: [u8; 32],
}
