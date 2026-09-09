/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::c_int;

#[repr(C)]
pub struct skey_region {
    pub start: ::core::ffi::c_ulong,
    pub end: ::core::ffi::c_ulong,
}

/*
 * SKEY_REGION(_start, _end) emits entries in the .skey_region section:
 *   .section .skey_region,"a"; .balign 8; .quad (_start); .quad (_end);
 *   .previous
 * The assembler-section emission is retained as intent; Rust has no direct
 * file-local equivalent for this macro.
 */

extern "C" {
    pub static mut skey_regions_initialized: c_int;
    pub static mut __skey_region_start: [skey_region; 0];
    pub static mut __skey_region_end: [skey_region; 0];

    pub fn __skey_regions_initialize();
}

#[inline]
pub unsafe fn skey_regions_initialize() {
    if core::ptr::read_volatile(&skey_regions_initialized) != 0 {
        return;
    }
    __skey_regions_initialize();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
