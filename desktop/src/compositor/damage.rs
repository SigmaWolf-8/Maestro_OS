//! Damage Tracking
//! Efficient partial-screen updates using damage rectangles

#![allow(unused)]

pub struct DamageRect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub struct DamageTracker {
    regions: Vec<DamageRect>,
}
