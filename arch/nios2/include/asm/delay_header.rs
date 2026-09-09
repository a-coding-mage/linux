/*
 * Copyright (C) 2014 Altera Corporation
 * Copyright (C) 2004 Microtronix Datacom Ltd
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependency supplied by asm-generic/delay.h in the original header.

/* Undefined functions to get compile-time errors */
unsafe extern "C" {
    pub fn __bad_udelay();
    pub fn __bad_ndelay();

    pub static mut loops_per_jiffy: core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
