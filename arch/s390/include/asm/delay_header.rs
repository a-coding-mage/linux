/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  S390 version
 *    Copyright IBM Corp. 1999
 *    Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com)
 *
 *  Derived from "include/asm-i386/delay.h"
 *    Copyright (C) 1993 Linus Torvalds
 *
 *  Delay routines calling functions in arch/s390/lib/delay.c
 */

// The C header guard is omitted; Rust modules provide equivalent item scoping.

unsafe extern "C" {
    pub fn __ndelay(nsecs: core::ffi::c_ulong);
    pub fn __udelay(usecs: core::ffi::c_ulong);
    pub fn __delay(loops: core::ffi::c_ulong);
}

#[inline(always)]
pub unsafe fn ndelay(n: core::ffi::c_ulong) {
    unsafe { __ndelay(n) };
}

#[inline(always)]
pub unsafe fn udelay(n: core::ffi::c_ulong) {
    unsafe { __udelay(n) };
}

#[inline(always)]
pub unsafe fn mdelay(n: core::ffi::c_ulong) {
    unsafe { __udelay(n.wrapping_mul(1000 as core::ffi::c_ulong)) };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
