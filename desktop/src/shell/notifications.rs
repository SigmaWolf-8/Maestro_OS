//! Notification System
//! Toast-style notifications with security level indicators

#![allow(unused)]

pub struct Notification {
    pub title: &'static str,
    pub body: &'static str,
    pub urgency: u8,
}
