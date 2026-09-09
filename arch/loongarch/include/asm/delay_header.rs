/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

use core::ffi::c_ulong;

extern "C" {
    pub fn __delay(cycles: c_ulong);
    pub fn __ndelay(ns: c_ulong);
    pub fn __udelay(us: c_ulong);
}

#[inline]
pub unsafe fn ndelay(ns: c_ulong) {
    __ndelay(ns);
}

#[inline]
pub unsafe fn udelay(us: c_ulong) {
    __udelay(us);
}

/* make sure "usecs *= ..." in udelay do not overflow. */
/* The original value is selected at build time from the externally supplied HZ. */
pub const MAX_UDELAY_MS: usize = if HZ >= 1000 {
    1
} else if HZ <= 200 {
    5
} else {
    1000 / HZ
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
