//! Security Badge Widget
//! Displays current security mode with color-coded indicator

#![allow(unused)]

pub struct SecurityBadge {
    pub mode: u8,
    pub label: &'static str,
}
