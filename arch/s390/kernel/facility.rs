// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright IBM Corp. 2023
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe extern "C" {
    fn __stfle_asm(facility_list: *mut u64, nr: u32) -> u32;
}

static mut SIZE: u32 = 0;

pub unsafe fn stfle_size() -> u32 {
    let mut r: u32;
    let mut dummy: u64;

    r = core::ptr::read_volatile(&raw const SIZE);
    if r == 0 {
        r = __stfle_asm(&mut dummy as *mut u64, 1).wrapping_add(1);
        core::ptr::write_volatile(&raw mut SIZE, r);
    }
    r
}

// EXPORT_SYMBOL(stfle_size);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
