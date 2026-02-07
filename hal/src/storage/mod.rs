//! Block Device Storage
//! NVMe and VirtIO storage drivers

#![allow(unused)]

pub trait BlockDevice {
    fn read_block(&self, lba: u64, buf: &mut [u8]);
    fn write_block(&self, lba: u64, data: &[u8]);
}
