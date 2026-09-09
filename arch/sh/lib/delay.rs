// SPDX-License-Identifier: GPL-2.0
/*
 *	Precise Delay Loops for SuperH
 *
 *	Copyright (C) 1999 Niibe Yutaka & Kaz Kojima
 */

// External kernel declarations and architecture-specific constants are
// supplied by the surrounding translation unit.

pub unsafe fn __delay(mut loops: usize) {
    core::arch::asm!(
        ".balign 8\n\t",
        "tst {0}, {0}\n\t",
        "1:\t",
        "bf/s 1b\n\t",
        " dt {0}",
        inout(reg) loops,
        out("t") _,
        options(nostack)
    );
}

#[inline]
pub unsafe fn __const_udelay(mut xloops: usize) {
    xloops = xloops.wrapping_mul(4);
    core::arch::asm!(
        "dmulu.l {0}, {2}\n\t",
        "sts mach, {0}",
        inout(reg) xloops,
        in(reg) cpu_data[raw_smp_processor_id()].loops_per_jiffy.wrapping_mul(HZ / 4),
        out("macl") _,
        out("mach") _,
        options(nostack)
    );
    __delay(xloops.wrapping_add(1));
}

pub unsafe fn __udelay(usecs: usize) {
    __const_udelay(usecs.wrapping_mul(0x0000_10c6));
}

pub unsafe fn __ndelay(nsecs: usize) {
    __const_udelay(nsecs.wrapping_mul(0x0000_0005));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
