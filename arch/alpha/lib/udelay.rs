// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 1993, 2000 Linus Torvalds
 *
 * Delay routines, using a pre-computed "loops_per_jiffy" value.
 */

use core::ffi::{c_int, c_ulong};

extern "C" {
    static mut loops_per_jiffy: c_ulong;
}

#[cfg(feature = "CONFIG_SMP")]
#[repr(C)]
pub struct CpuData {
    pub loops_per_jiffy: c_ulong,
}

#[cfg(feature = "CONFIG_SMP")]
extern "C" {
    static mut cpu_data: *mut CpuData;
    fn smp_processor_id() -> usize;
}

/*
 * Use only for very small delays (< 1 msec).
 *
 * The active part of our cycle counter is only 32-bits wide, and
 * we're treating the difference between two marks as signed.  On
 * a 1GHz box, that's about 2 seconds.
 */
pub unsafe fn __delay(mut loops: c_int) {
    let mut tmp: c_int;
    // Alpha-specific inline assembly from the original implementation:
    // rpcc tmp; addl loops,tmp,loops; 1: rpcc tmp; subl loops,tmp,tmp; bgt tmp,1b
    core::arch::asm!(
        "rpcc {tmp}",
        "addl {loops}, {tmp}, {loops}",
        "1: rpcc {tmp}",
        "subl {loops}, {tmp}, {tmp}",
        "bgt {tmp}, 1b",
        tmp = lateout(reg) tmp,
        loops = inout(reg) loops,
        options(nostack)
    );
}

#[cfg(feature = "CONFIG_SMP")]
#[inline]
unsafe fn lpj() -> c_ulong {
    (*cpu_data.add(smp_processor_id())).loops_per_jiffy
}

#[cfg(not(feature = "CONFIG_SMP"))]
#[inline]
unsafe fn lpj() -> c_ulong {
    loops_per_jiffy
}

pub unsafe fn udelay(mut usecs: c_ulong) {
    usecs = usecs.wrapping_mul(
        (((crate::HZ as c_ulong) << 32) / 1_000_000).wrapping_mul(lpj()),
    );
    __delay((usecs as i64 >> 32) as c_int);
}

pub unsafe fn ndelay(mut nsecs: c_ulong) {
    nsecs = nsecs.wrapping_mul(
        (((crate::HZ as c_ulong) << 32) / 1_000_000_000).wrapping_mul(lpj()),
    );
    __delay((nsecs as i64 >> 32) as c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
