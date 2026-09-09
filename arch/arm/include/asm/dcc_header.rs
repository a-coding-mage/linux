/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2010, 2014 The Linux Foundation. All rights reserved.
 */

// Dependency corresponding to <asm/barrier.h>; `isb` is supplied externally.

#[inline]
pub unsafe fn __dcc_getstatus() -> u32 {
    let mut __ret: u32;
    core::arch::asm!(
        "mrc p14, 0, {ret}, c0, c1, 0 // read comms ctrl reg",
        ret = out(reg) __ret,
        options(nostack)
    );

    __ret
}

#[inline]
pub unsafe fn __dcc_getchar() -> i8 {
    let mut __c: i8;

    core::arch::asm!(
        "mrc p14, 0, {c}, c0, c5, 0 // read comms data reg",
        c = out(reg) __c,
        options(nostack)
    );
    isb();

    __c
}

#[inline]
pub unsafe fn __dcc_putchar(c: i8) {
    core::arch::asm!(
        "mcr p14, 0, {c}, c0, c5, 0 // write a char",
        c = in(reg) c,
        options(nostack)
    );
    isb();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
