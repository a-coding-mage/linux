/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright (C) 2016 Red Hat, Inc.
 * Author: Michael S. Tsirkin <mst@redhat.com>
 */

/*
 * Portable implementations of 1 and 2 byte xchg using a 4 byte cmpxchg.
 * Note: this header isn't self-contained: before including it, __cmpxchg_u32
 * must be defined first.
 */

#[inline]
pub unsafe fn __xchg_cmpxchg(ptr: *mut core::ffi::c_void, x: u32, size: i32) -> u32 {
    let off = (ptr as usize) % core::mem::size_of::<u32>();
    let p = (ptr as *mut u8).sub(off) as *mut u32;

    #[cfg(target_endian = "big")]
    let bitoff = (core::mem::size_of::<u32>() - size as usize - off) * u8::BITS as usize;
    #[cfg(target_endian = "little")]
    let bitoff = off * u8::BITS as usize;

    let bitmask = ((0x1u32 << (size as usize * u8::BITS as usize)) - 1) << bitoff;
    let mut oldv: u32;
    let mut newv: u32;
    let mut ret: u32;

    loop {
        oldv = core::ptr::read_volatile(p);
        ret = (oldv & bitmask) >> bitoff;
        newv = (oldv & !bitmask) | (x << bitoff);
        if __cmpxchg_u32(p, oldv, newv) == oldv {
            break;
        }
    }

    ret
}

#[inline]
pub unsafe fn xchg_u16(m: *mut u16, val: usize) -> usize {
    __xchg_cmpxchg(
        m as *mut core::ffi::c_void,
        val as u32,
        core::mem::size_of::<u16>() as i32,
    ) as usize
}

#[inline]
pub unsafe fn xchg_u8(m: *mut u8, val: usize) -> usize {
    __xchg_cmpxchg(
        m as *mut core::ffi::c_void,
        val as u32,
        core::mem::size_of::<u8>() as i32,
    ) as usize
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
