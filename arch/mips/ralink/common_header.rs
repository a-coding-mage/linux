/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright (C) 2013 John Crispin <john@phrozen.org>
 */

/* Translated from the C header; include guards are not applicable in Rust. */

pub const RAMIPS_SYS_TYPE_LEN: usize = 32;

#[repr(C)]
pub struct ralink_soc_info {
    pub sys_type: [u8; RAMIPS_SYS_TYPE_LEN],
    pub compatible: *mut u8,

    pub mem_base: core::ffi::c_ulong,
    pub mem_size: core::ffi::c_ulong,
    pub mem_size_min: core::ffi::c_ulong,
    pub mem_size_max: core::ffi::c_ulong,
    pub mem_detect: Option<unsafe extern "C" fn()> ,
}

extern "C" {
    pub static mut soc_info: ralink_soc_info;

    pub fn ralink_of_remap();

    /* The C declaration uses the __init annotation. */
    pub fn prom_soc_init(soc_info: *mut ralink_soc_info);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
