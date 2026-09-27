// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2012 Freescale Semiconductor, Inc.
 */

// Dependencies supplied by the surrounding kernel translation.
use kernel::ffi::c_void;

extern "C" {
    static mut jiffies: kernel::ffi::c_ulong;
    fn msecs_to_jiffies(msecs: kernel::ffi::c_ulong) -> kernel::ffi::c_ulong;
    fn readl_relaxed(reg: *const c_void) -> u32;
    fn time_after(a: kernel::ffi::c_ulong, b: kernel::ffi::c_ulong) -> bool;
}

// The concrete spinlock representation is supplied by the translated headers.
#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

#[no_mangle]
pub static mut mxs_lock: spinlock_t = spinlock_t { _private: [] };

extern "C" {
    static ETIMEDOUT: kernel::ffi::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn mxs_clk_wait(reg: *mut c_void, shift: u8) -> kernel::ffi::c_int {
    let timeout = jiffies.wrapping_add(msecs_to_jiffies(10));

    while readl_relaxed(reg as *const c_void) & (1u32 << shift) != 0 {
        if time_after(jiffies, timeout) {
            return -ETIMEDOUT;
        }
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
