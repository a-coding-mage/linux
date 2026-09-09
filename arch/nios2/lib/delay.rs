// SPDX-License-Identifier: GPL-2.0-only
/* Copyright Altera Corporation (C) 2014. All rights reserved.
 */

// The declarations below are supplied by the kernel architecture code.
use core::ffi::c_ulong;

unsafe extern "C" {
    fn get_cycles() -> u64;
    fn cpu_relax();
    static loops_per_jiffy: c_ulong;
    static HZ: c_ulong;
}

pub unsafe fn __delay(cycles: c_ulong) {
    let start: u64 = unsafe { get_cycles() };

    while unsafe { get_cycles() }.wrapping_sub(start) < cycles as u64 {
        unsafe { cpu_relax() };
    }
}

// EXPORT_SYMBOL(__delay);

pub unsafe fn __const_udelay(xloops: c_ulong) {
    let loops: u64;

    loops = (xloops as u64)
        .wrapping_mul(unsafe { loops_per_jiffy } as u64)
        .wrapping_mul(unsafe { HZ } as u64);

    unsafe { __delay((loops >> 32) as c_ulong) };
}

// EXPORT_SYMBOL(__const_udelay);

pub unsafe fn __udelay(usecs: c_ulong) {
    unsafe { __const_udelay(usecs.wrapping_mul(0x10C7 as c_ulong)) }; /* 2**32 / 1000000 (rounded up) */
}

// EXPORT_SYMBOL(__udelay);

pub unsafe fn __ndelay(nsecs: c_ulong) {
    unsafe { __const_udelay(nsecs.wrapping_mul(0x5 as c_ulong)) }; /* 2**32 / 1000000000 (rounded up) */
}

// EXPORT_SYMBOL(__ndelay);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
