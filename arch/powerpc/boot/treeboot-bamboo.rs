// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright IBM Corporation, 2007
 * Josh Boyer <jwboyer@linux.vnet.ibm.com>
 *
 * Based on ebony wrapper:
 * Copyright 2007 David Gibson, IBM Corporation.
 */

// #include "ops.h"
// #include "stdio.h"
// #include "44x.h"
// #include "stdlib.h"

// BSS_STACK(4096);

const PIBS_MAC0: usize = 0xfffc0400;
const PIBS_MAC1: usize = 0xfffc0500;

#[no_mangle]
pub static mut pibs_mac0: [i8; 6] = [0; 6];
#[no_mangle]
pub static mut pibs_mac1: [i8; 6] = [0; 6];

extern "C" {
    static mut _end: u8;

    fn strtoull(s: *const i8, endptr: *mut *mut i8, base: i32) -> u64;
    fn memcpy(dest: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize)
        -> *mut core::ffi::c_void;
    fn simple_alloc_init(start: *mut u8, size: usize, align: usize, min_alloc: usize);
    fn bamboo_init(mac0: *const u8, mac1: *const u8);
}

unsafe fn read_pibs_mac() {
    let mut mac64: u64;

    mac64 = strtoull(PIBS_MAC0 as *const i8, core::ptr::null_mut(), 16);
    memcpy(
        pibs_mac0.as_mut_ptr() as *mut core::ffi::c_void,
        (core::ptr::addr_of!(mac64) as *const u8).add(2) as *const core::ffi::c_void,
        6,
    );

    mac64 = strtoull(PIBS_MAC1 as *const i8, core::ptr::null_mut(), 16);
    memcpy(
        pibs_mac1.as_mut_ptr() as *mut core::ffi::c_void,
        (core::ptr::addr_of!(mac64) as *const u8).add(2) as *const core::ffi::c_void,
        6,
    );
}

#[no_mangle]
pub unsafe extern "C" fn platform_init() {
    let end_of_ram: usize = 0x8000000;
    let avail_ram = end_of_ram - (core::ptr::addr_of_mut!(_end) as usize);

    simple_alloc_init(core::ptr::addr_of_mut!(_end), avail_ram, 32, 64);
    read_pibs_mac();
    bamboo_init(pibs_mac0.as_ptr() as *const u8, pibs_mac1.as_ptr() as *const u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
