// SPDX-License-Identifier: GPL-2.0-only
// Copyright 2022 Google LLC
// Author: Ard Biesheuvel <ardb@google.com>

// NOTE: code in this file runs *very* early, and is not permitted to use
// global variables or anything that relies on absolute addressing.

use core::ffi::{c_char, c_void};

extern "C" {
    fn fdt_getprop_w(
        fdt: *mut c_void,
        node: i32,
        name: *const c_char,
        lenp: *mut i32,
    ) -> *mut u64;
    fn fdt64_to_cpu(value: u64) -> u64;
    fn kaslr_disabled_cmdline() -> bool;
    fn __early_cpu_has_rndr() -> bool;
    fn __arm64_rndr(value: *mut u64) -> bool;
}

extern "C" {
    static VMALLOC_END: u64;
    static KIMAGE_VADDR: u64;
}

unsafe fn get_kaslr_seed(fdt: *mut c_void, node: i32) -> u64 {
    static SEED_STR: &[u8] = b"kaslr-seed\0";
    let mut prop: *mut u64;
    let mut ret: u64;
    let mut len: i32 = 0;

    if node < 0 {
        return 0;
    }

    prop = fdt_getprop_w(fdt, node, SEED_STR.as_ptr() as *const c_char, &mut len);
    if prop.is_null() || len != core::mem::size_of::<u64>() as i32 {
        return 0;
    }

    ret = fdt64_to_cpu(*prop);
    *prop = 0;
    ret
}

pub unsafe fn kaslr_early_init(fdt: *mut c_void, chosen: i32) -> u64 {
    let seed: u64;
    let range: u64;

    if kaslr_disabled_cmdline() {
        return 0;
    }

    seed = get_kaslr_seed(fdt, chosen);
    let seed = if seed == 0 {
        let mut rndr_seed: u64 = 0;
        if !__early_cpu_has_rndr() || !__arm64_rndr(&mut rndr_seed) {
            return 0;
        }
        rndr_seed
    } else {
        seed
    };

    /*
     * OK, so we are proceeding with KASLR enabled. Calculate a suitable
     * kernel image offset from the seed. Let's place the kernel in the
     * 'middle' half of the VMALLOC area, and stay clear of the lower and
     * upper quarters to avoid colliding with other allocations.
     */
    range = (VMALLOC_END - KIMAGE_VADDR) / 2;
    range / 2 + (((range as u128 * seed as u128) >> 64) as u64)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
