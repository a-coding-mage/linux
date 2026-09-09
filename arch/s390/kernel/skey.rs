// SPDX-License-Identifier: GPL-2.0

use core::arch::asm;

// Dependencies supplied by the corresponding architecture headers/build.
#[repr(C)]
pub struct skey_region {
    pub start: usize,
    pub end: usize,
}

extern "C" {
    static __skey_region_start: *mut skey_region;
    static __skey_region_end: *mut skey_region;

    fn page_set_storage_key(address: usize, key: usize, fetch_protection: i32);
    fn barrier();
}

// PAGE_MASK, PAGE_SIZE, and PAGE_DEFAULT_KEY are supplied by asm/page.h and
// asm/skey.h in the surrounding translation unit.
extern "C" {
    static PAGE_MASK: usize;
    static PAGE_SIZE: usize;
    static PAGE_DEFAULT_KEY: usize;
}

pub static mut skey_regions_initialized: i32 = 0;

unsafe fn load_real_address(address: usize) -> usize {
    let real: usize;

    asm!(
        "lra {real},0({address})",
        real = out(reg) real,
        address = in(reg) address,
        options(nostack, preserves_flags),
    );
    real
}

/*
 * Initialize storage keys of registered memory regions with the
 * default key. This is useful for code which is executed with a
 * non-default access key.
 */
pub unsafe fn __skey_regions_initialize() {
    let mut address: usize;
    let mut real: usize;
    let mut r: *mut skey_region;
    let end: *mut skey_region;

    r = __skey_region_start;
    end = __skey_region_end;
    while r < end {
        address = (*r).start & PAGE_MASK;
        loop {
            real = load_real_address(address);
            page_set_storage_key(real, PAGE_DEFAULT_KEY, 1);
            address = address.wrapping_add(PAGE_SIZE);
            if !(address < (*r).end) {
                break;
            }
        }
        r = r.add(1);
    }
    /*
     * Make sure storage keys are initialized before
     * skey_regions_initialized is changed.
     */
    barrier();
    core::ptr::write_volatile(&mut skey_regions_initialized, 1);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
