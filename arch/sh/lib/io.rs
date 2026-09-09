// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/lib/io.c - SH32 optimized I/O routines
 *
 * Copyright (C) 2000  Stuart Menefy
 * Copyright (C) 2005  Paul Mundt
 *
 * Provide real functions which expand to whatever the header file defined.
 * Also definitions of machine independent IO functions.
 */

use core::ffi::c_void;

// Supplied by linux/io.h; `__iomem` is an address-space annotation in C.
extern "C" {
    pub fn __raw_readl(addr: *const c_void) -> u32;
}

#[inline]
pub unsafe fn __raw_readsl(addr: *const c_void, datap: *mut c_void, mut len: i32) {
    let mut data = datap as *mut u32;

    while len != 0 && ((data as usize as u32) & 0x1f) != 0 {
        *data = __raw_readl(addr);
        data = data.add(1);
        len -= 1;
    }

    if len >= (0x20 >> 2) {
        /*
         * The original SH inline assembly reads eight words from `addr` and
         * writes them as one 0x20-byte burst, repeating while at least eight
         * words remain.  The architecture-specific cache/burst instruction
         * has no file-local Rust equivalent, so preserve its exact observable
         * word ordering with the equivalent volatile-independent operations.
         */
        while len >= (0x20 >> 2) {
            let tmp1 = __raw_readl(addr);
            let tmp2 = __raw_readl(addr);
            let tmp3 = __raw_readl(addr);
            let tmp4 = __raw_readl(addr);
            let tmp5 = __raw_readl(addr);
            let tmp6 = __raw_readl(addr);
            let tmp7 = __raw_readl(addr);
            let tmp8 = __raw_readl(addr);
            *data = tmp1;
            *data.add(1) = tmp2;
            *data.add(2) = tmp3;
            *data.add(3) = tmp4;
            *data.add(4) = tmp5;
            *data.add(5) = tmp6;
            *data.add(6) = tmp7;
            *data.add(7) = tmp8;
            data = data.add(8);
            len -= 0x20 >> 2;
        }
    }

    while len != 0 {
        *data = __raw_readl(addr);
        data = data.add(1);
        len -= 1;
    }
}

pub unsafe fn __raw_writesl(addr: *mut c_void, datap: *const c_void, mut len: i32) {
    let mut data = datap as *const u32;

    // Equivalent of the original SH `mov.l @data+, tmp; dt len; bf.s; mov.l tmp,@addr` loop.
    while len != 0 {
        let value = *data;
        data = data.add(1);
        core::ptr::write_volatile(addr as *mut u32, value);
        len -= 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
