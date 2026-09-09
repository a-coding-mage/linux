/* SPDX-License-Identifier: GPL-2.0 */
/* 32-bit atomic xchg() and cmpxchg() definitions.
 *
 * Copyright (C) 1996 David S. Miller (davem@davemloft.net)
 * Copyright (C) 2000 Anton Blanchard (anton@linuxcare.com.au)
 * Copyright (C) 2007 Kyle McMartin (kyle@parisc-linux.org)
 *
 * Additions by Keith M Wesolowski (wesolows@foobazco.org) based
 * on asm-parisc/atomic.h Copyright (C) 2000 Philipp Rumpf <prumpf@tux.org>.
 */

extern "C" {
    pub fn __xchg_u32(m: *mut u32, new: u32) -> usize;
    pub fn __xchg_called_with_bad_pointer();
}

pub unsafe fn __arch_xchg(x: usize, ptr: *mut core::ffi::c_void, size: i32) -> usize {
    match size {
        4 => __xchg_u32(ptr as *mut u32, x as u32),
        _ => {
            __xchg_called_with_bad_pointer();
            x
        }
    }
}

/* arch_xchg(ptr, x): cast the result of __arch_xchg to the pointed-to type. */

/* Emulate cmpxchg() the same way we emulate atomics,
 * by hashing the object address and indexing into an array
 * of spinlocks to get a bit of performance...
 *
 * See arch/sparc/lib/atomic32.c for implementation.
 *
 * Cribbed from <asm-parisc/atomic.h>
 */

/* bug catcher for when unsupported size is used - won't link */
extern "C" {
    pub fn __cmpxchg_called_with_bad_pointer();
    pub fn __cmpxchg_u8(m: *mut u8, old: u8, new_: u8) -> usize;
    pub fn __cmpxchg_u16(m: *mut u16, old: u16, new_: u16) -> usize;
    pub fn __cmpxchg_u32(m: *mut u32, old: u32, new_: u32) -> usize;
}

/* don't worry...optimizer will get rid of most of this */
pub unsafe fn __cmpxchg(
    ptr: *mut core::ffi::c_void,
    old: usize,
    new_: usize,
    size: i32,
) -> usize {
    match size {
        1 => __cmpxchg_u8(ptr as *mut u8, old as u8, new_ as u8),
        2 => __cmpxchg_u16(ptr as *mut u16, old as u16, new_ as u16),
        4 => __cmpxchg_u32(ptr as *mut u32, old as u32, new_ as u32),
        _ => {
            __cmpxchg_called_with_bad_pointer();
            old
        }
    }
}

extern "C" {
    pub fn __cmpxchg_u64(ptr: *mut u64, old: u64, new: u64) -> u64;
}

/* arch_cmpxchg64(ptr, old, new) is __cmpxchg_u64(ptr, old, new). */

/* C include: <asm-generic/cmpxchg-local.h> */

/*
 * cmpxchg_local and cmpxchg64_local are atomic wrt current CPU. Always make
 * them available.
 */
extern "C" {
    pub fn __generic_cmpxchg_local(
        ptr: *mut core::ffi::c_void,
        old: usize,
        new_: usize,
        size: usize,
    ) -> usize;
    pub fn __generic_cmpxchg64_local(ptr: *mut u64, old: u64, new_: u64) -> u64;
}

/* arch_cmpxchg_local(ptr, o, n) invokes __generic_cmpxchg_local. */
/* arch_cmpxchg64_local(ptr, o, n) invokes __generic_cmpxchg64_local. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
