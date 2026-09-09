/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the Linux timer header.

pub const HEARTBEAT_INVERTED: u32 = 1 << 0;

#[repr(C)]
pub struct heartbeat_data {
    pub base: *mut core::ffi::c_void,
    pub bit_pos: *mut u8,
    pub nr_bits: u32,
    pub timer: timer_list,
    pub regsize: u32,
    pub mask: u32,
    pub flags: core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
