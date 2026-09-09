/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from <linux/types.h>; the C __u* types correspond to Rust's
// fixed-width unsigned integer types.

pub const EBT_STP_TYPE: u16 = 0x0001;

pub const EBT_STP_FLAGS: u16 = 0x0002;
pub const EBT_STP_ROOTPRIO: u16 = 0x0004;
pub const EBT_STP_ROOTADDR: u16 = 0x0008;
pub const EBT_STP_ROOTCOST: u16 = 0x0010;
pub const EBT_STP_SENDERPRIO: u16 = 0x0020;
pub const EBT_STP_SENDERADDR: u16 = 0x0040;
pub const EBT_STP_PORT: u16 = 0x0080;
pub const EBT_STP_MSGAGE: u16 = 0x0100;
pub const EBT_STP_MAXAGE: u16 = 0x0200;
pub const EBT_STP_HELLOTIME: u16 = 0x0400;
pub const EBT_STP_FWDD: u16 = 0x0800;

pub const EBT_STP_MASK: u16 = 0x0fff;
pub const EBT_STP_CONFIG_MASK: u16 = 0x0ffe;

pub const EBT_STP_MATCH: &str = "stp";

#[repr(C)]
pub struct ebt_stp_config_info {
    pub flags: u8,
    pub root_priol: u16,
    pub root_priou: u16,
    pub root_addr: [core::ffi::c_char; 6],
    pub root_addrmsk: [core::ffi::c_char; 6],
    pub root_costl: u32,
    pub root_costu: u32,
    pub sender_priol: u16,
    pub sender_priou: u16,
    pub sender_addr: [core::ffi::c_char; 6],
    pub sender_addrmsk: [core::ffi::c_char; 6],
    pub portl: u16,
    pub portu: u16,
    pub msg_agel: u16,
    pub msg_ageu: u16,
    pub max_agel: u16,
    pub max_ageu: u16,
    pub hello_timel: u16,
    pub hello_timeu: u16,
    pub forward_delayl: u16,
    pub forward_delayu: u16,
}

#[repr(C)]
pub struct ebt_stp_info {
    pub r#type: u8,
    pub config: ebt_stp_config_info,
    pub bitmask: u16,
    pub invflags: u16,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
