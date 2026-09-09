/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2009 Chen Liqin <liqin.chen@sunplusct.com>
 * Copyright (C) 2016 Regents of the University of California
 */

// C header guard: _ASM_RISCV_DELAY_H

use core::ffi::c_ulong;

pub unsafe extern "C" {
    pub static mut riscv_timebase: c_ulong;

    // C macro: #define udelay udelay
    pub fn udelay(usecs: c_ulong);

    // C macro: #define ndelay ndelay
    pub fn ndelay(nsecs: c_ulong);

    pub fn __delay(cycles: c_ulong);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
