/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2024 Loongson Technology Corporation Limited
 */

/*
 * Functions to change memory attributes.
 */
extern "C" {
    pub fn set_memory_x(addr: kernel::ffi::c_ulong, numpages: kernel::ffi::c_int)
        -> kernel::ffi::c_int;
    pub fn set_memory_nx(addr: kernel::ffi::c_ulong, numpages: kernel::ffi::c_int)
        -> kernel::ffi::c_int;
    pub fn set_memory_ro(addr: kernel::ffi::c_ulong, numpages: kernel::ffi::c_int)
        -> kernel::ffi::c_int;
    pub fn set_memory_rw(addr: kernel::ffi::c_ulong, numpages: kernel::ffi::c_int)
        -> kernel::ffi::c_int;

    pub fn kernel_page_present(page: *mut page) -> bool;
    pub fn set_direct_map_default_noflush(page: *mut page) -> kernel::ffi::c_int;
    pub fn set_direct_map_invalid_noflush(page: *mut page) -> kernel::ffi::c_int;
    pub fn set_direct_map_valid_noflush(
        page: *mut page,
        nr: kernel::ffi::c_uint,
        valid: bool,
    ) -> kernel::ffi::c_int;
}

/* External dependency supplied by the surrounding kernel translation. */
pub enum page {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
