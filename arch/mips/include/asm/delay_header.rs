/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1994 by Waldorf Electronics
 * Copyright (C) 1995 - 2000, 01, 03 by Ralf Baechle
 * Copyright (C) 1999, 2000 Silicon Graphics, Inc.
 * Copyright (C) 2007  Maciej W. Rozycki
 */

// Dependency: <linux/param.h> supplies the build-time HZ value.

use core::ffi::c_ulong;

unsafe extern "C" {
    pub fn __delay(loops: c_ulong);
    pub fn __ndelay(ns: c_ulong);
    pub fn __udelay(us: c_ulong);
}

#[inline]
pub unsafe fn ndelay(ns: c_ulong) {
    unsafe { __ndelay(ns) }
}

#[inline]
pub unsafe fn udelay(us: c_ulong) {
    unsafe { __udelay(us) }
}

/* make sure "usecs *= ..." in udelay do not overflow. */
// The following cfg conditions represent the original build-time HZ tests.
#[cfg(hz_ge_1000)]
pub const MAX_UDELAY_MS: c_ulong = 1;

#[cfg(hz_le_200)]
pub const MAX_UDELAY_MS: c_ulong = 5;

#[cfg(hz_between_201_and_999)]
pub const MAX_UDELAY_MS: c_ulong = 1000 / HZ;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
