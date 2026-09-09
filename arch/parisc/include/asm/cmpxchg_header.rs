/* SPDX-License-Identifier: GPL-2.0 */
/*
 * forked from parisc asm/atomic.h which was:
 *	Copyright (C) 2000 Philipp Rumpf <prumpf@tux.org>
 *	Copyright (C) 2006 Kyle McMartin <kyle@parisc-linux.org>
 */

/* This should get optimized out since it's never called.
 * Or get a link error if xchg is used "wrong".
 */
unsafe extern "C" {
    pub fn __xchg_called_with_bad_pointer();
}

/* __xchg32/64 defined in arch/parisc/lib/bitops.c */
unsafe extern "C" {
    pub fn __xchg8(x: core::ffi::c_char, ptr: *mut core::ffi::c_char) -> usize;
    pub fn __xchg32(x: i32, ptr: *mut i32) -> usize;
    /* CONFIG_64BIT */
    pub fn __xchg64(x: usize, ptr: *mut usize) -> usize;
}

/* optimizer better get rid of switch since size is a constant */
#[inline]
pub unsafe fn __arch_xchg(x: usize, ptr: *mut core::ffi::c_void, size: i32) -> usize {
    match size {
        /* CONFIG_64BIT: case 8 */
        8 => unsafe { __xchg64(x, ptr.cast::<usize>()) },
        4 => unsafe { __xchg32(x as i32, ptr.cast::<i32>()) },
        1 => unsafe { __xchg8(x as core::ffi::c_char, ptr.cast::<core::ffi::c_char>()) },
        _ => {
            unsafe { __xchg_called_with_bad_pointer() };
            x
        }
    }
}

/*
 * REVISIT - Abandoned use of LDCW in xchg() for now:
 * o need to test sizeof(*ptr) to avoid clearing adjacent bytes
 * o and while we are at it, could CONFIG_64BIT code use LDCD too?
 *
 *	if (__builtin_constant_p(x) && (x == NULL))
 *		if (((unsigned long)p & 0xf) == 0)
 *			return __ldcw(p);
 */
#[inline]
pub unsafe fn arch_xchg<T: Copy>(ptr: *mut T, x: T) -> T {
    unsafe { __arch_xchg(x_as_usize(x), ptr.cast(), core::mem::size_of::<T>() as i32) as T }
}

/* bug catcher for when unsupported size is used - won't link */
unsafe extern "C" {
    pub fn __cmpxchg_called_with_bad_pointer();
    /* __cmpxchg_u... defined in arch/parisc/lib/bitops.c */
    pub fn __cmpxchg_u8(ptr: *mut u8, old: u8, new_: u8) -> u8;
    pub fn __cmpxchg_u16(ptr: *mut u16, old: u16, new_: u16) -> u16;
    pub fn __cmpxchg_u32(ptr: *mut u32, old: u32, new_: u32) -> u32;
    pub fn __cmpxchg_u64(ptr: *mut u64, old: u64, new_: u64) -> u64;
}

/* don't worry...optimizer will get rid of most of this */
#[inline]
pub unsafe fn __cmpxchg(ptr: *mut core::ffi::c_void, old: usize, new_: usize, size: i32) -> usize {
    match size {
        /* CONFIG_64BIT: size == 8 */
        8 => unsafe { __cmpxchg_u64(ptr.cast(), old as u64, new_ as u64) as usize },
        4 => unsafe { __cmpxchg_u32(ptr.cast(), old as u32, new_ as u32) as usize },
        2 => unsafe { __cmpxchg_u16(ptr.cast(), old as u16, new_ as u16) as usize },
        1 => unsafe { __cmpxchg_u8(ptr.cast(), old as u8, new_ as u8) as usize },
        _ => {
            unsafe { __cmpxchg_called_with_bad_pointer() };
            old
        }
    }
}

/* Supplied by <asm-generic/cmpxchg-local.h>. */
unsafe extern "C" {
    pub fn __generic_cmpxchg_local(ptr: *mut core::ffi::c_void, old: usize, new_: usize, size: i32) -> usize;
    pub fn __generic_cmpxchg64_local<T>(ptr: *mut T, old: T, new_: T) -> T;
}

#[inline]
pub unsafe fn __cmpxchg_local(ptr: *mut core::ffi::c_void, old: usize, new_: usize, size: i32) -> usize {
    match size {
        /* CONFIG_64BIT: case 8 */
        8 => unsafe { __cmpxchg_u64(ptr.cast(), old as u64, new_ as u64) as usize },
        4 => unsafe { __cmpxchg_u32(ptr.cast(), old as u32, new_ as u32) as usize },
        _ => unsafe { __generic_cmpxchg_local(ptr, old, new_, size) },
    }
}

#[inline]
pub unsafe fn arch_cmpxchg_local<T: Copy>(ptr: *mut T, o: T, n: T) -> T {
    unsafe { __cmpxchg_local(ptr.cast(), x_as_usize(o), x_as_usize(n), core::mem::size_of::<T>() as i32) as T }
}

/* CONFIG_64BIT: arch_cmpxchg64_local includes BUILD_BUG_ON(sizeof(*(ptr)) != 8). */
#[inline]
pub unsafe fn arch_cmpxchg64_local<T: Copy>(ptr: *mut T, o: T, n: T) -> T {
    unsafe { arch_cmpxchg_local(ptr, o, n) }
}

#[inline]
pub unsafe fn arch_cmpxchg64(ptr: *mut u64, o: u64, n: u64) -> u64 {
    unsafe { __cmpxchg_u64(ptr, o, n) }
}

/* C's __typeof__ casts preserve the pointed-to integer type; callers should provide integer T. */
#[inline]
unsafe fn x_as_usize<T>(x: T) -> usize {
    unsafe { core::mem::transmute_copy(&x) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
