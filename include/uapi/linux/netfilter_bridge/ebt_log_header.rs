/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: __u8 and __u32 from <linux/types.h> map to Rust's fixed-width
// unsigned integer types.

pub const EBT_LOG_IP: u8 = 0x01; // if the frame is made by ip, log the ip information
pub const EBT_LOG_ARP: u8 = 0x02;
pub const EBT_LOG_NFLOG: u8 = 0x04;
pub const EBT_LOG_IP6: u8 = 0x08;
pub const EBT_LOG_MASK: u8 = EBT_LOG_IP | EBT_LOG_ARP | EBT_LOG_IP6;
pub const EBT_LOG_PREFIX_SIZE: usize = 30;
pub const EBT_LOG_WATCHER: &str = "log";

#[repr(C)]
pub struct ebt_log_info {
    pub loglevel: u8,
    pub prefix: [u8; EBT_LOG_PREFIX_SIZE],
    pub bitmask: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
