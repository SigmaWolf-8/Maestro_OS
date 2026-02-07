//! Maestro OS Installer
//! GUI-based installation wizard with disk partitioning and configuration

#![allow(unused)]

pub struct Installer {
    pub target_disk: &'static str,
    pub install_desktop: bool,
}
