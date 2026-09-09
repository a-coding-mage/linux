// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2024 Kalray, Inc.  All Rights Reserved.
 */

// Dependencies supplied by the surrounding kernel translation unit.
extern "C" {
    fn __raw_writeb(value: i32, addr: *mut core::ffi::c_void);
    fn __raw_writeq(value: c_long, addr: *mut core::ffi::c_void);
    fn __raw_writel(value: c_long, addr: *mut core::ffi::c_void);
    fn __raw_readb(addr: *const core::ffi::c_void) -> u8;
    fn __raw_readq(addr: *const core::ffi::c_void) -> c_long;
    fn __raw_readl(addr: *const core::ffi::c_void) -> c_long;
}

use core::ffi::{c_long, c_void};

#[cfg(not(feature = "memset_io"))]
/// Set a range of I/O memory to a constant value.
pub unsafe extern "C" fn memset_io(addr: *mut c_void, val: i32, mut count: usize) {
    let mut qc = (val as u8) as c_long;
    qc = qc.wrapping_mul((!0usize / 0xff) as c_long);

    while count != 0 && (addr as usize) % core::mem::size_of::<c_long>() != 0 {
        __raw_writeb(val, addr);
        addr = (addr as *mut u8).add(1) as *mut c_void;
        count -= 1;
    }

    while count >= core::mem::size_of::<c_long>() {
        #[cfg(target_pointer_width = "64")]
        __raw_writeq(qc, addr);
        #[cfg(not(target_pointer_width = "64"))]
        __raw_writel(qc, addr);
        addr = (addr as *mut u8).add(core::mem::size_of::<c_long>()) as *mut c_void;
        count -= core::mem::size_of::<c_long>();
    }

    while count != 0 {
        __raw_writeb(val, addr);
        addr = (addr as *mut u8).add(1) as *mut c_void;
        count -= 1;
    }
}

#[cfg(not(feature = "memcpy_fromio"))]
/// Copy a block of data from I/O memory.
pub unsafe extern "C" fn memcpy_fromio(mut dst: *mut c_void, mut src: *const c_void, mut count: usize) {
    while count != 0 && (src as usize) % core::mem::size_of::<c_long>() != 0 {
        *(dst as *mut u8) = __raw_readb(src);
        src = (src as *const u8).add(1) as *const c_void;
        dst = (dst as *mut u8).add(1) as *mut c_void;
        count -= 1;
    }

    while count >= core::mem::size_of::<c_long>() {
        #[cfg(target_pointer_width = "64")]
        let val = __raw_readq(src);
        #[cfg(not(target_pointer_width = "64"))]
        let val = __raw_readl(src);
        core::ptr::write_unaligned(dst as *mut c_long, val);
        src = (src as *const u8).add(core::mem::size_of::<c_long>()) as *const c_void;
        dst = (dst as *mut u8).add(core::mem::size_of::<c_long>()) as *mut c_void;
        count -= core::mem::size_of::<c_long>();
    }

    while count != 0 {
        *(dst as *mut u8) = __raw_readb(src);
        src = (src as *const u8).add(1) as *const c_void;
        dst = (dst as *mut u8).add(1) as *mut c_void;
        count -= 1;
    }
}

#[cfg(not(feature = "memcpy_toio"))]
/// Copy a block of data to I/O memory.
pub unsafe extern "C" fn memcpy_toio(mut dst: *mut c_void, mut src: *const c_void, mut count: usize) {
    while count != 0 && (dst as usize) % core::mem::size_of::<c_long>() != 0 {
        __raw_writeb(*(src as *const u8) as i32, dst);
        src = (src as *const u8).add(1) as *const c_void;
        dst = (dst as *mut u8).add(1) as *mut c_void;
        count -= 1;
    }

    while count >= core::mem::size_of::<c_long>() {
        let val = core::ptr::read_unaligned(src as *const c_long);
        #[cfg(target_pointer_width = "64")]
        __raw_writeq(val, dst);
        #[cfg(not(target_pointer_width = "64"))]
        __raw_writel(val, dst);
        src = (src as *const u8).add(core::mem::size_of::<c_long>()) as *const c_void;
        dst = (dst as *mut u8).add(core::mem::size_of::<c_long>()) as *mut c_void;
        count -= core::mem::size_of::<c_long>();
    }

    while count != 0 {
        __raw_writeb(*(src as *const u8) as i32, dst);
        src = (src as *const u8).add(1) as *const c_void;
        dst = (dst as *mut u8).add(1) as *mut c_void;
        count -= 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
