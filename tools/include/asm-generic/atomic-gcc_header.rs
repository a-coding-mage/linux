/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Original C header included:
 * <linux/compiler.h>
 * <linux/types.h>
 * <linux/bitops.h>
 *
 * The types and helper macros/functions referenced here, such as atomic_t,
 * BIT_MASK, and BIT_WORD, are supplied by those translated dependencies.
 */

use core::ffi::{c_int, c_long, c_ulong};
use core::sync::atomic::{AtomicI32, Ordering};

#[cfg(target_pointer_width = "64")]
type AtomicUlong = core::sync::atomic::AtomicU64;

#[cfg(target_pointer_width = "32")]
type AtomicUlong = core::sync::atomic::AtomicU32;

/*
 * Atomic operations that C can't guarantee us.  Useful for
 * resource counting etc..
 *
 * Excerpts obtained from the Linux kernel sources.
 */

#[inline]
pub const fn ATOMIC_INIT(i: c_int) -> atomic_t {
    atomic_t { counter: i }
}

/**
 * atomic_read - read atomic variable
 * @v: pointer of type atomic_t
 *
 * Atomically reads the value of @v.
 */
#[inline]
pub unsafe fn atomic_read(v: *const atomic_t) -> c_int {
    core::ptr::read_volatile(core::ptr::addr_of!((*v).counter))
}

/**
 * atomic_set - set atomic variable
 * @v: pointer of type atomic_t
 * @i: required value
 *
 * Atomically sets the value of @v to @i.
 */
#[inline]
pub unsafe fn atomic_set(v: *mut atomic_t, i: c_int) {
    (*v).counter = i;
}

/**
 * atomic_inc - increment atomic variable
 * @v: pointer of type atomic_t
 *
 * Atomically increments @v by 1.
 */
#[inline]
pub unsafe fn atomic_inc(v: *mut atomic_t) {
    (*(core::ptr::addr_of_mut!((*v).counter) as *mut AtomicI32)).fetch_add(1, Ordering::SeqCst);
}

/**
 * atomic_dec_and_test - decrement and test
 * @v: pointer of type atomic_t
 *
 * Atomically decrements @v by 1 and
 * returns true if the result is 0, or false for all other
 * cases.
 */
#[inline]
pub unsafe fn atomic_dec_and_test(v: *mut atomic_t) -> c_int {
    let new_value =
        (*(core::ptr::addr_of_mut!((*v).counter) as *mut AtomicI32)).fetch_sub(1, Ordering::SeqCst) - 1;
    (new_value == 0) as c_int
}

#[inline]
pub unsafe fn cmpxchg(ptr: *mut c_int, oldval: c_int, newval: c_int) -> c_int {
    match (*(ptr as *mut AtomicI32)).compare_exchange(oldval, newval, Ordering::SeqCst, Ordering::SeqCst) {
        Ok(previous) => previous,
        Err(previous) => previous,
    }
}

#[inline]
pub unsafe fn atomic_cmpxchg(v: *mut atomic_t, oldval: c_int, newval: c_int) -> c_int {
    cmpxchg(core::ptr::addr_of_mut!((*v).counter), oldval, newval)
}

#[inline]
pub unsafe fn test_and_set_bit(nr: c_long, mut addr: *mut c_ulong) -> c_int {
    let mask: c_ulong = BIT_MASK(nr);
    let old: c_ulong;

    addr = addr.add(BIT_WORD(nr) as usize);

    old = (*(addr as *mut AtomicUlong)).fetch_or(mask as _, Ordering::SeqCst) as c_ulong;
    ((old & mask) != 0) as c_int
}

#[inline]
pub unsafe fn test_and_clear_bit(nr: c_long, mut addr: *mut c_ulong) -> c_int {
    let mask: c_ulong = BIT_MASK(nr);
    let old: c_ulong;

    addr = addr.add(BIT_WORD(nr) as usize);

    old = (*(addr as *mut AtomicUlong)).fetch_and(!mask as _, Ordering::SeqCst) as c_ulong;
    ((old & mask) != 0) as c_int
}
