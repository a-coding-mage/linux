// SPDX-License-Identifier: GPL-2.0-only
/*
 * Based on arch/arm/kernel/io.c
 *
 * Copyright (C) 2012 ARM Ltd.
 */

use core::ffi::c_void;

extern "C" {
    fn __const_memcpy_toio_aligned64(to: *mut u64, from: *const u64, count: usize);
    fn __const_memcpy_toio_aligned32(to: *mut u32, from: *const u32, count: usize);
    fn dgh();
}

/*
 * This generates a memcpy that works on a from/to address which is aligned to
 * bits. Count is in terms of the number of bits sized quantities to copy. It
 * optimizes to use the STR groupings when possible so that it is WC friendly.
 */

pub unsafe fn __iowrite64_copy_full(to: *mut c_void, from: *const c_void, count: usize) {
    let mut _to = to as *mut u64;
    let mut _from = from as *const u64;
    let _count = count;
    let _end_from = _from.add(_count & !7usize);

    while _from < _end_from {
        __const_memcpy_toio_aligned64(_to, _from, 8);
        _from = _from.add(8);
        _to = _to.add(8);
    }
    if (_count % 8) >= 4 {
        __const_memcpy_toio_aligned64(_to, _from, 4);
        _from = _from.add(4);
        _to = _to.add(4);
    }
    if (_count % 4) >= 2 {
        __const_memcpy_toio_aligned64(_to, _from, 2);
        _from = _from.add(2);
        _to = _to.add(2);
    }
    if _count % 2 != 0 {
        __const_memcpy_toio_aligned64(_to, _from, 1);
    }
    dgh();
}

pub unsafe fn __iowrite32_copy_full(to: *mut c_void, from: *const c_void, count: usize) {
    let mut _to = to as *mut u32;
    let mut _from = from as *const u32;
    let _count = count;
    let _end_from = _from.add(_count & !7usize);

    while _from < _end_from {
        __const_memcpy_toio_aligned32(_to, _from, 8);
        _from = _from.add(8);
        _to = _to.add(8);
    }
    if (_count % 8) >= 4 {
        __const_memcpy_toio_aligned32(_to, _from, 4);
        _from = _from.add(4);
        _to = _to.add(4);
    }
    if (_count % 4) >= 2 {
        __const_memcpy_toio_aligned32(_to, _from, 2);
        _from = _from.add(2);
        _to = _to.add(2);
    }
    if _count % 2 != 0 {
        __const_memcpy_toio_aligned32(_to, _from, 1);
    }
    dgh();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
