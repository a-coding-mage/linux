/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * Byte swapping for NOLIBC
 * Copyright (C) 2026 Thomas Weißschuh <linux@weissschuh.net>
 */

/* make sure to include all global symbols */
/* C source included "nolibc.h", "stdint.h", and <linux/swab.h>. */

unsafe extern "C" {
    pub fn __swab16(_x: u16) -> u16;
    pub fn __swab32(_x: u32) -> u32;
    pub fn __swab64(_x: u64) -> u64;
}

#[inline]
pub unsafe fn bswap_16(_x: u16) -> u16 {
    unsafe { __swab16(_x) }
}

#[inline]
pub unsafe fn bswap_32(_x: u32) -> u32 {
    unsafe { __swab32(_x) }
}

#[inline]
pub unsafe fn bswap_64(_x: u64) -> u64 {
    unsafe { __swab64(_x) }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
