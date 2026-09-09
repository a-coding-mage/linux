/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Dependencies supplied by the surrounding kernel translation:
// `phys_addr_t`, `TO_CACHE!`, `memblock_alloc`, and `PAGE_SIZE`.

macro_rules! dmi_early_remap {
    ($x:expr, $l:expr) => {
        dmi_remap($x, $l)
    };
}

macro_rules! dmi_early_unmap {
    ($x:expr, $l:expr) => {
        dmi_unmap($x)
    };
}

macro_rules! dmi_alloc {
    ($l:expr) => {
        memblock_alloc($l, PAGE_SIZE)
    };
}

#[inline]
pub unsafe fn dmi_remap(phys_addr: phys_addr_t, _size: ::core::ffi::c_ulong) -> *mut ::core::ffi::c_void {
    (TO_CACHE!(phys_addr)) as *mut ::core::ffi::c_void
}

#[inline]
pub unsafe fn dmi_unmap(_addr: *mut ::core::ffi::c_void) {
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
