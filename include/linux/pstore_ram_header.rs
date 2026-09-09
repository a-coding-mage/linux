/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2010 Marco Stornelli <marco.stornelli@gmail.com>
 * Copyright (C) 2011 Kees Cook <keescook@chromium.org>
 * Copyright (C) 2011 Google, Inc.
 */

// Dependency intent: declarations from <linux/pstore.h> are supplied elsewhere.

#[repr(C)]
pub struct persistent_ram_ecc_info {
    pub block_size: i32,
    pub ecc_size: i32,
    pub symsize: i32,
    pub poly: i32,
    pub par: *mut u16,
}

/*
 * Ramoops platform data
 * @mem_size\tmemory size for ramoops
 * @mem_address\tphysical memory address to contain ramoops
 */

pub const RAMOOPS_FLAG_FTRACE_PER_CPU: u32 = 1u32 << 0;

#[repr(C)]
pub struct ramoops_platform_data {
    pub mem_size: core::ffi::c_ulong,
    pub mem_address: phys_addr_t,
    pub mem_type: u32,
    pub record_size: core::ffi::c_ulong,
    pub console_size: core::ffi::c_ulong,
    pub ftrace_size: core::ffi::c_ulong,
    pub pmsg_size: core::ffi::c_ulong,
    pub max_reason: i32,
    pub flags: u32,
    pub ecc_info: persistent_ram_ecc_info,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
