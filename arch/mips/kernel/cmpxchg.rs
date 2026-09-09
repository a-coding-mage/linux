// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2017 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

// Dependencies supplied by the surrounding kernel translation.
unsafe extern "C" {
    fn arch_cmpxchg(ptr: *mut u32, old: u32, new: u32) -> u32;
    fn WARN_ON(condition: bool) -> bool;
}

pub unsafe fn __xchg_small(ptr: *mut core::ffi::c_void, mut val: usize, size: u32) -> usize {
    let mut old32: u32;
    let mut new32: u32;
    let mut load32: u32;
    let mut mask: u32;
    let ptr32: *mut u32;
    let mut shift: u32;

    /* Check that ptr is naturally aligned */
    WARN_ON((ptr as usize & (size as usize - 1)) != 0);

    /* Mask value to the correct size. */
    let bits = size * u32::from(u8::BITS);
    mask = if bits == u32::from(u32::BITS) {
        u32::MAX
    } else {
        (1u32 << bits) - 1
    };
    val &= mask as usize;

    /*
     * Calculate a shift & mask that correspond to the value we wish to
     * exchange within the naturally aligned 4 byte integer that includes
     * it.
     */
    shift = (ptr as usize & 0x3) as u32;
    if cfg!(target_endian = "big") {
        shift ^= 4 - size;
    }
    shift *= u32::from(u8::BITS);
    mask <<= shift;

    /*
     * Calculate a pointer to the naturally aligned 4 byte integer that
     * includes our byte of interest, and load its value.
     */
    ptr32 = (ptr as usize & !0x3) as *mut u32;
    load32 = core::ptr::read_volatile(ptr32);

    loop {
        old32 = load32;
        new32 = (load32 & !mask) | ((val as u32) << shift);
        load32 = arch_cmpxchg(ptr32, old32, new32);
        if load32 == old32 {
            break;
        }
    }

    ((load32 & mask) >> shift) as usize
}

pub unsafe fn __cmpxchg_small(
    ptr: *mut core::ffi::c_void,
    mut old: usize,
    mut new: usize,
    size: u32,
) -> usize {
    let mut mask: u32;
    let mut old32: u32;
    let mut new32: u32;
    let mut load32: u32;
    let mut load: u32;
    let ptr32: *mut u32;
    let mut shift: u32;

    /* Check that ptr is naturally aligned */
    WARN_ON((ptr as usize & (size as usize - 1)) != 0);

    /* Mask inputs to the correct size. */
    let bits = size * u32::from(u8::BITS);
    mask = if bits == u32::from(u32::BITS) {
        u32::MAX
    } else {
        (1u32 << bits) - 1
    };
    old &= mask as usize;
    new &= mask as usize;

    /*
     * Calculate a shift & mask that correspond to the value we wish to
     * compare & exchange within the naturally aligned 4 byte integer
     * that includes it.
     */
    shift = (ptr as usize & 0x3) as u32;
    if cfg!(target_endian = "big") {
        shift ^= 4 - size;
    }
    shift *= u32::from(u8::BITS);
    mask <<= shift;

    /*
     * Calculate a pointer to the naturally aligned 4 byte integer that
     * includes our byte of interest, and load its value.
     */
    ptr32 = (ptr as usize & !0x3) as *mut u32;
    load32 = core::ptr::read_volatile(ptr32);

    loop {
        /*
         * Ensure the byte we want to exchange matches the expected
         * old value, and if not then bail.
         */
        load = (load32 & mask) >> shift;
        if load != old as u32 {
            return load as usize;
        }

        /*
         * Calculate the old & new values of the naturally aligned
         * 4 byte integer that include the byte we want to exchange.
         * Attempt to exchange the old value for the new value, and
         * return if we succeed.
         */
        old32 = (load32 & !mask) | ((old as u32) << shift);
        new32 = (load32 & !mask) | ((new as u32) << shift);
        load32 = arch_cmpxchg(ptr32, old32, new32);
        if load32 == old32 {
            return old;
        }
    }
}

// EXPORT_SYMBOL(__cmpxchg_small);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
