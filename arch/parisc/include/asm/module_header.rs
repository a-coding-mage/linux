/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency intent: equivalent of <asm-generic/module.h>. */

/*
 * This file contains the parisc architecture specific module code.
 */

#[repr(C)]
pub struct unwind_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mod_arch_specific_section {
    pub stub_offset: core::ffi::c_ulong,
    pub stub_entries: core::ffi::c_uint,
}

#[repr(C)]
pub struct mod_arch_specific {
    pub got_offset: core::ffi::c_ulong,
    pub got_count: core::ffi::c_ulong,
    pub got_max: core::ffi::c_ulong,
    pub fdesc_offset: core::ffi::c_ulong,
    pub fdesc_count: core::ffi::c_ulong,
    pub fdesc_max: core::ffi::c_ulong,
    pub section: *mut mod_arch_specific_section,
    pub unwind_section: core::ffi::c_int,
    pub unwind: *mut unwind_table,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
