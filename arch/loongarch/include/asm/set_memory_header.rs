/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2024 Loongson Technology Corporation Limited
 */

/*
 * Functions to change memory attributes.
 */
extern "C" {
    pub fn set_memory_x(addr: core::ffi::c_ulong, numpages: core::ffi::c_int)
        -> core::ffi::c_int;
    pub fn set_memory_nx(addr: core::ffi::c_ulong, numpages: core::ffi::c_int)
        -> core::ffi::c_int;
    pub fn set_memory_ro(addr: core::ffi::c_ulong, numpages: core::ffi::c_int)
        -> core::ffi::c_int;
    pub fn set_memory_rw(addr: core::ffi::c_ulong, numpages: core::ffi::c_int)
        -> core::ffi::c_int;

    pub fn kernel_page_present(page: *mut page) -> bool;
    pub fn set_direct_map_default_noflush(page: *mut page) -> core::ffi::c_int;
    pub fn set_direct_map_invalid_noflush(page: *mut page) -> core::ffi::c_int;
    pub fn set_direct_map_valid_noflush(
        page: *mut page,
        nr: core::ffi::c_uint,
        valid: bool,
    ) -> core::ffi::c_int;
}

/* External dependency supplied by the surrounding kernel translation. */
pub enum page {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
