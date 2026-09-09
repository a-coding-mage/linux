/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: linux/netfilter_arp/arp_tables.h supplies `in_addr` and
// `ARPT_DEV_ADDR_LEN_MAX`.

pub const ARPT_MANGLE_ADDR_LEN_MAX: usize =
    core::mem::size_of::<crate::in_addr>();

#[repr(C)]
#[derive(Copy, Clone)]
pub union arpt_mangle_u_s {
    pub src_ip: crate::in_addr,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union arpt_mangle_u_t {
    pub tgt_ip: crate::in_addr,
}

#[repr(C)]
pub struct arpt_mangle {
    pub src_devaddr: [core::ffi::c_char; crate::ARPT_DEV_ADDR_LEN_MAX],
    pub tgt_devaddr: [core::ffi::c_char; crate::ARPT_DEV_ADDR_LEN_MAX],
    pub u_s: arpt_mangle_u_s,
    pub u_t: arpt_mangle_u_t,
    pub flags: u8,
    pub target: core::ffi::c_int,
}

pub const ARPT_MANGLE_SDEV: u8 = 0x01;
pub const ARPT_MANGLE_TDEV: u8 = 0x02;
pub const ARPT_MANGLE_SIP: u8 = 0x04;
pub const ARPT_MANGLE_TIP: u8 = 0x08;
pub const ARPT_MANGLE_MASK: u8 = 0x0f;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
