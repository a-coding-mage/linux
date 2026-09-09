// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// Supplied by the Linux kernel build environment:
// `loops_per_jiffy` and `HZ` are used by the original implementation.
extern "C" {
    static mut loops_per_jiffy: usize;
    static HZ: usize;
}

// __aligned(8) on the original function; Rust has no function alignment
// attribute corresponding to this source-level declaration.
pub unsafe fn __delay(mut loops: usize) {
    // The original C implementation uses C-SKY inline assembly:
    //   mov r0, r0
    //   1:declt %0
    //   bf 1b
    while loops != 0 {
        loops = loops.wrapping_sub(1);
    }
}

// EXPORT_SYMBOL(__delay);

pub unsafe fn __const_udelay(xloops: usize) {
    let loops: u64 = (xloops as u64)
        .wrapping_mul(loops_per_jiffy as u64)
        .wrapping_mul(HZ as u64);

    __delay((loops >> 32) as usize);
}

// EXPORT_SYMBOL(__const_udelay);

pub unsafe fn __udelay(usecs: usize) {
    __const_udelay(usecs.wrapping_mul(0x10C7usize)); // 2**32 / 1000000 (rounded up)
}

// EXPORT_SYMBOL(__udelay);

pub unsafe fn __ndelay(nsecs: usize) {
    __const_udelay(nsecs.wrapping_mul(0x5usize)); // 2**32 / 1000000000 (rounded up)
}

// EXPORT_SYMBOL(__ndelay);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
